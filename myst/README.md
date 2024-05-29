# `fly`-ing `huginn` close to the ground

konami are remaking MGS3. some people are trying to sell [electric ekranoplans](https://www.regentcraft.com/seagliders/viceroy#specifications). and
here i am being a freeloader nerd for the 2nd decade. ingredients:

- [huginn/huginn image](https://ghcr.io/huginn/huginn) (sponsor [co-author & maintainer knu](https://github.com/sponsors/knu))
- [sentry DSN](https://sentry.io/pricing/) and/or [GlitchTip](https://glitchtip.com/pricing) ([donation page](https://liberapay.com/GlitchTip/donate))
- [fly.io dashboard](https://fly.io/docs/about/pricing/)
- [inoreader](https://jp.inoreader.com/pricing) and/or [feedly](https://feedly.com/i/pro) and/or [feedbin](https://feedbin.com/)

i use its agent to crawl comic/blogs' syndication feeds, then reformat & augment them to read easily in [feedme](https://github.com/seazon/feedme). The comics update at most daily ([SMBC](https://www.smbc-comics.com/comic/archive) has been running since 2002), or thrice-weekly (Ryan North's [dinosaurs](https://qwantz.com/index.php?comic=1) have been talkin' since 2003 (lovingly crafted in [XP's MS Paint](https://www.qwantz.com/index.php?comic=4005)). and the blogs update less frequently than that. there's no need to keep huginn web-part running 24/7, and we can also reduce the scheduler wake-up frequency.

## Getting `flyctl` on MediaTek ramips mt7621

it's a softfloat Little-Endian MIPS cpu, we'll build `flyctl` like this:

```shell
scalar clone http://github.com/superfly/flyctl
cd flyctl/src
git sparse-checkout disable
env GOARCH=mipsle GOMIPS=softfloat make build
gzip bin/flyctl
# deliver flyctl.gz to the mips
```

## Making sentry crons and/or glitchtip uptime monitor

on sentry, SMBC will have a daily check-in schedule. Dinosaur Comics posts on Mon-Wed-Fri, accounting for TZ & scheduling delay, we'll have a Tue-Thu-Sat check-in regimen. both will allow some further ~2h slack time. the monitoring slugs are readable text now, not UUIDv4 anymore. but that's guessable. along with the project's `SENTRY_DSN` and quickstart for rails, we're good to go.

on glitchtip, set up a 86399-second interval uptime heartbeat (they don't accept >=86400s). we'll use the URL later.

## Launch pad

we start simple with the `supervisord`-based image. that'd keep a 1GB fly machine running all the time.

Fly people use docker images as a packaging format, they define the
environment around that. here, we bake in postgresql and the sentry
SDK gems. remember to [load `stackprof` first](https://docs.sentry.io/platforms/ruby/profiling/#enable-profiling):

```bash
mkdir hug
cd hug

echo -e '.dockerignore\nDockerfile\nfly.toml\nhug-db.toml\nsentry.rb' | tee .dockerignore

tee Dockerfile <<-EOD
FROM ghcr.io/huginn/huginn:cbeb5c293ec3394dcd202b132b80e14398da8a11

ENV ADDITIONAL_GEMS=stackprof,sentry-ruby,sentry-rails
RUN APP_SECRET_TOKEN=nbd DATABASE_ADAPTER=postgresql bundle install -j 4

# we tune this often
COPY sentry.rb /app/config/initializers/sentry.rb

EOD

tee sentry.rb <<-EOR
Sentry.init do |config|
 config.breadcrumbs_logger = [:active_support_logger, :http_logger]
 config.traces_sample_rate = 0.5
 config.profiles_sample_rate = 1.0
end
EOR
```

generate random for `APP_SECRET_TOKEN`:

```sh
podman run --rm -it --entrypoint sh ghcr.io/huginn/huginn:cbeb5c293ec3394dcd202b132b80e14398da8a11 rake secret
```

Build initial fly config, along with a managed postgres:

```bash
flyctl launch \
--org personal \
--name hug \
--region fra \
--push \
--vm-cpukind shared \
--vm-cpus 1 \
--vm-memory 1024
```

for the postgres, you can use development, 256MB, auto scale to zero (default after 1h, but [you can be more aggressive/conservative](https://fly.io/docs/postgres/managing/scale-to-zero/#turn-off-the-scale-to-zero-feature)). let's write down its config file here too:

```bash
flyctl config save \
--app hug-db \
--config hug-db.toml
```

and it's a good idea to save the running postgres image for later use:

```bash
flyctl image show -a hug-db
```

load some more secrets into the app:

```bash
flyctl secrets import <<-EOS
SENTRY_DSN=your_dsn
APP_SECRET_TOKEN=your_random
INVITATION_CODE=your_knock_password
EOS
```

and put more app config

```bash
tee -a fly.toml <<-EOT

[[statics]]
  guest_path = "/app/public"
  url_prefix = "/public"

[env]
  RAILS_SERVE_STATIC_FILES="false"

  TIMEZONE="Hanoi"
  TZ="Asia/Phnom_Penh"

  DOMAIN="hug.fly.dev"
  FORCE_SSL="true"
  DATABASE_ADAPTER="postgresql"
  RACK_ENV="production"
  RAILS_LOG_TO_STDOUT="enabled"

  DO_NOT_SEED="true"
  IMPORT_DEFAULT_SCENARIO_FOR_ALL_USERS="true"

  AGENT_LOG_LENGTH="10" # used to fit heroku 10_000 rows limit
  SCHEDULER_FREQUENCY="5"
  DELAYED_JOB_SLEEP_DELAY="900" # s

  USE_GRAPHVIZ_DOT="dot"
  DIAGRAM_DEFAULT_LAYOUT="neato"
EOT
```

now you try to `flyctl launch` it, see the resulting 2.0GB image starts on fly regino `fra`

if you get context deadline exceeded for build, try wesockets mode

```bash
flyctl wg websockets enable
```

to attach in for debugging

```bash
flyctl ssh console -s
```

in there, multi-process img has `pstree` to view supervisor's tree

and tail the logs

```bash
flyctl logs
```

here, you can open the public URL, and create an account with the invitation code. there's no `admin` account because we didn't run SEED job. huginn will have created sample agents for you, explore them for idea

## Split out the workers

The current vm resource is

```bash
flyctl machine show
```

Inside the one machine for `app` process group, there's supervisord, then foreman, managing an unicorn `web.1` (master + 2workers), and the rufus scheduler `jobs.1` task. the unicorn is only useful when i open the site, or inoreader comes to fetch the feed. rufus should be running all the time, to execute scheduled tasks (agents, events propagation, and cleanups). Let's split out `app` to `web` and `jobs`, then we can let web autostop/start according to load.
