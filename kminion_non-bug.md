One near-failure story to keep myself writing. I almost reported an issue, right up to the point where bisecting doesn't reproduce it reliably.

I run both kafka-minion [v1, known under cloudhut org](https://github.com/redpanda-data/kminion/blob/v1.0.2/options/options.go#L63) and kminion [v2, now under redpanda-data org](https://github.com/redpanda-data/kminion/blob/v2.2.0/docs/reference-config.yaml#L13) in parallel, in order to gauge how their metrics compare. (Truth be told, i wrote some suboptimal configs for kafka-minion, and the dashboards are too tied up with such config.) They use the same configuration via a systemd `EnvironmentFile`. That's the only method supported by kafka-minion whose `Options` are deserialized by struct definion:

> string `envconfig:"KAFKA_TLS_CA_FILE_PATH"`

kminion can load config from some `koanf` providers: env and/or yaml. There's a transformation step to collapse yaml maps into a flat environment variable name:

> kafka.tls.caFilepath => `KAFKA_TLS_CAFILEPATH`

Due to some recent issue, I wanted to build kminion from git repo. However, moving beyond https://github.com/redpanda-data/kminion/pull/147 gave a weird error:

```json
"caller": "kminion/main.go:43",
"msg": "failed to parse config",
"error": "1 error(s) decoding:\n\n* 'kafka.tls.ca[0]' expected type 'uint8', got unconvertible type 'map[string]interface {}', value: 'map[file:map[path:kafka_chain.pem]]'",
"stacktrace": "main.main\n\t/home/me/github/kminion/main.go:43\nruntime.main\n\t/usr/lib/golang/src/runtime/proc.go:250"
```

`map[file:map[path:kafka_chain.pem]]` okay, I see my CA file here (actually the content is ISRG root CA). But what's this `'kafka.tls.ca[0]' expected type 'uint8'`? Why is something plugging a map into an integer? At least I have a clean commit to write a bisect demo.

So I copied over the `EnvironmentFile`, removed anything about `SASL` authentication, exporter, set `0:9092` as `bootstrap.servers` for a quick failure once the app passes config parsing. And let `git bisect run` rip!

It pointed somewhere around the merge commit, as i expected. But then i had doubt, and set kafka-minion's envvar in a shell:

```sh
env KAFKA_TLS_CAFILEPATH=kafka_chain.pem kafka-minion
...snip...Connection refused
```

eh, it got right up to trying connection? what happened to my bug?

As it happened, the envvar intended for `kafka-minion` (`KAFKA_TLS_CA_FILE_PATH`) matches `koanf` auto-hydration(?), and it tried to construct this structure:

```yaml
kafka:
  tls:
    ca:
      file:
        path: "kafka_chain.pem"
```

because the new feature now accepts this structure:

```yaml
kafka:
  tls:
    ca: MIIF....base64 DER data....
```

Thus, the `expected` first element should have been a byte from the certificate.

With the new-issue tab firmly closed, I added to kminion's systemd unit file:

```conf
[Service]
UnsetEnvironment=KAFKA_TLS_CA_FILE_PATH
```

As usual, I succeeded by nearly failing.
