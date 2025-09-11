# syntax=docker/dockerfile:1.18
FROM ghcr.io/huginn/huginn-single-process:14db2db194a5da7c5626656bb9824931338e016e

USER 0
RUN <<EORUN
set -xeu
apt update -qq
apt install -y libjemalloc2
EORUN

RUN <<EORUN
set -xeu
sed -i.bak -e "s,1m' do,30m' do," -e "/[125]m'/d" lib/huginn_scheduler.rb
sed -i.bak -e "/twitter/d" -e "/jabber/d" /app/lib/agent_runner.rb
sed -i.bak -e "/twilio|-ftp|eather|mqtt|xmpp|slack|pdf|hipchat|weibo/d" \
    -e "/Optional lib/,/google-oauth2/d" \
    -e "/bsd|BSD/d" \
    Gemfile
rm -v app/models/agents/hipchat* app/models/agents/dropbox* app/models/agents/tumblr* app/models/agents/twilio* app/models/agents/telegram* app/models/agents/slack* app/models/agents/s3* app/models/agents/twitter* app/models/agents/google* app/models/agents/pdf* app/models/agents/jabber* app/models/agents/jira* app/models/agents/imap* app/models/agents/ftp* app/concerns/twitter*
EORUN

RUN <<EORUN
set -xeu
# gem install bundler:$(tac Gemfile.lock | grep -o -m1 '[1-9].*')
# unnecessary anymore?
gem install bundler:2.7.2
EORUN

USER default
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2

ENV ADDITIONAL_GEMS=mini_racer,faraday-multipart
RUN APP_SECRET_TOKEN=secret DATABASE_ADAPTER=postgresql bundler install
# COPY --link sentry.rb /app/config/initializers/sentry.rb
