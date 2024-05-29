Some environment setup note for myself. We start on debian11

comfort tooling:

```sh
apt -y install fish ripgrep rsync
```

docker to run binary-builder:

```sh
apt -y install docker.io apparmor-{utils,profiles}
docker pull docker.io/clickhouse/binary-builder:39450-amd64
# long wait
```

code tooling & build CH:

```sh
mkdir -p /root/ch-binary/
apt -y install build-essential ccache git ca-certificates
git clone https://github.com/ClickHouse/ClickHouse --recursive
# long wait
cd ClickHouse
git checkout v22.3.11.12-lts --recurse-submodules
# long wait, with manual cleaning
git status
# very long wait
docker/packager/packager --cache=ccache --output-dir=/root/ch-binary/ --package-type=binary --build-type=debug --compiler=clang-13 --docker-image-version=39450-amd64
```

kafka barebone, from https://kafka.apache.org/quickstart:

```sh
apt -y install default-jre-headless curl
curl -LO https://dlcdn.apache.org/kafka/3.2.1/kafka_2.13-3.2.1.tgz
tar xf kafka_2.13-3.2.1.tgz
cd kafka_2.13-3.2.1/
bin/zookeeper-server-start.sh config/zookeeper.properties &
bin/kafka-server-start.sh config/server.properties &
```

```sh
cd kafka_2.13-3.2.1/
bin/kafka-topics.sh --create --topic 2-partitions --partitions 2 --bootstrap-server localhost:9092
bin/kafka-topics.sh --create --topic 1-partition --bootstrap-server localhost:9092
bin/kafka-topics.sh --bootstrap-server 127.1:9092 --describe
```

```text
Topic: 1-partition      TopicId: M70Y5dsDQeu7oE-nIZeDbg PartitionCount: 1       ReplicationFactor: 1    Configs: segment.bytes=1073741824
        Topic: 1-partition      Partition: 0    Leader: 0       Replicas: 0     Isr: 0
Topic: 2-partitions     TopicId: 8nzR9D6hSJWiyolwy_DN_Q PartitionCount: 2       ReplicationFactor: 1    Configs: segment.bytes=1073741824
        Topic: 2-partitions     Partition: 0    Leader: 0       Replicas: 0     Isr: 0
        Topic: 2-partitions     Partition: 1    Leader: 0       Replicas: 0     Isr: 0
```

optionally otel/zipkin export

```sh
docker run --network host otel/opentelemetry-collector:0.54.0
```

```sql
CREATE MATERIALIZED VIEW IF NOT EXISTS system.zipkin_spans ENGINE = URL('http://127.4.1.1:9411/api/v2/spans', 'JSONEachRow') SETTINGS output_format_json_named_tuples_as_objects = 1, output_format_json_quote_64bit_integers = 0, output_format_json_array_of_rows = 1 AS SELECT lower(hex(reinterpretAsFixedString(trace_id))) AS traceId, lower(hex(parent_span_id)) AS parentId, lower(hex(span_id)) AS id, operation_name AS name, start_time_us AS timestamp, finish_time_us - start_time_us AS duration, cast(tuple('clickhouse'), 'Tuple(serviceName text)') AS localEndpoint, cast(tuple(attribute.values[indexOf(attribute.names, 'db.statement')]), 'Tuple("db.statement" text)') AS tags FROM system.opentelemetry_span_log;

CREATE MATERIALIZED VIEW IF NOT EXISTS system.zipkin_spans (`traceId` String, `parentId` String, `id` String, `name` String, `timestamp` UInt64, `duration` Int64, `localEndpoint` Tuple(serviceName String), `tags` Tuple(`db.statement` String, `db.host` String, `ch.thread_id` String, `ch.query_id` String, `ch.query_status` String)) ENGINE = URL('http://127.4.1.1:9411/api/v2/spans', 'JSONEachRow') SETTINGS output_format_json_named_tuples_as_objects = 1, output_format_json_quote_64bit_integers = 0, output_format_json_array_of_rows = 1 AS SELECT lower(hex(reinterpretAsFixedString(trace_id))) AS traceId, lower(hex(parent_span_id)) AS parentId, lower(hex(span_id)) AS id, operation_name AS name, start_time_us AS timestamp, finish_time_us - start_time_us AS duration, CAST(tuple('vm415v'), 'Tuple(serviceName text)') AS localEndpoint, CAST((attribute.values[indexOf(attribute.names, 'db.statement')], FQDN(), attribute.values[indexOf(attribute.names, 'clickhouse.thread_id')], attribute.values[indexOf(attribute.names, 'clickhouse.query_id')], attribute.values[indexOf(attribute.names, 'clickhouse.query_status')]), 'Tuple("db.statement" text,"db.host" text,"ch.thread_id" text,"ch.query_id" text, "ch.query_status" text)') AS tags FROM system.opentelemetry_span_log;
```

```sql
CREATE DATABASE IF NOT EXISTS etl;
CREATE TABLE IF NOT EXISTS etl.trashbin(s String) ENGINE = Null;
```

rr, bytehound (nee memory-profiler) tooling:

```sh
apt -y install yarnpkg gdb libc6-dbg heaptrack lsof
curl -LO https://github.com/rr-debugger/rr/releases/download/5.6.0/rr-5.6.0-Linux-$(uname -m).deb
apt install ./rr*deb
# git version
git clone https://github.com/koute/bytehound
cd bytehound
cargo b --release -p bytehound-preload
cargo b --release -p bytehound-cli
# 0.9.0
mkdir 0.9.0/
curl -LO https://github.com/koute/bytehound/releases/download/0.9.0/bytehound-x86_64-unknown-linux-gnu.tgz | tar -C 0.9.0/ zxf -
```

bytehound environment:

```fish
set -x MEMORY_PROFILER_LOG=debug
```

silent-ish CH config, though the opentelemetry table doesn't show up:

```xml
<yandex>
    <shutdown_wait_unfinished>60</shutdown_wait_unfinished>

    <opentelemetry_span_log>
        <engine>
            engine MergeTree
            partition by toYYYYMM(finish_date)
            order by (finish_date, finish_time_us, trace_id)
        </engine>
        <database>system</database>
        <table>opentelemetry_span_log</table>
        <flush_interval_milliseconds>7500</flush_interval_milliseconds>
    </opentelemetry_span_log>
    <logger><level>trace</level></logger>

    <query_thread_log remove="1" />
    <query_log remove="1" />
    <query_views_log remove="1" />
    <part_log remove="1"/>
    <session_log remove="1"/>
    <text_log remove="1" />
    <trace_log remove="1"/>
    <crash_log remove="1"/>
    <zookeeper_log remove="1"/>
    <disable_internal_dns_cache>1</disable_internal_dns_cache>
</yandex>
```

internal graphite metrics logging:

```xml
<yandex>
        <graphite>
                <host>in-carbon</host>
                <port>2023</port>
                <timeout>5</timeout>
                <interval>60</interval>
                <root_path>direct.vm415</root_path>
                <metrics>true</metrics>
                <events>true</events>
                <asynchronous_metrics>true</asynchronous_metrics>
        </graphite>
</yandex>
```

Now the leak:

```sql
CREATE DATABASE IF NOT EXISTS etl;
CREATE TABLE IF NOT EXISTS etl.trashbin(s String) ENGINE = Null;
```

```sql
CREATE TABLE IF NOT EXISTS etl.consume_kafka1 (s String) ENGINE = Kafka()
SETTINGS kafka_format = 'CSV', kafka_group_name = 'CHgroup'
, kafka_broker_list = '127.1.1.1:9092', kafka_topic_list = '1-partition'
, kafka_num_consumers=1;
CREATE MATERIALIZED VIEW IF NOT EXISTS etl.pipeline1 TO etl.trashbin AS SELECT s FROM etl.consume_kafka1;

CREATE TABLE IF NOT EXISTS etl.consume_kafka2 (s String) ENGINE = Kafka()
SETTINGS kafka_format = 'CSV', kafka_group_name = 'CHgroup'
, kafka_broker_list = '127.2.2.2:9092', kafka_topic_list = '1-partition'
, kafka_num_consumers=1;
CREATE MATERIALIZED VIEW IF NOT EXISTS etl.pipeline2 TO etl.trashbin AS SELECT s FROM etl.consume_kafka2;
```

when 1 pipeline is idle, something keeps leaking memory. With more `kafka_num_consumers` per pipeline, it also leaks faster. Dropping either MV stops the polling, thus stops the leaking. I hoped to inspect the threads more easily with a 2 pipelines setup.

how about a error streaming table?

```sql
CREATE TABLE IF NOT EXISTS etl.consume_kafka1 (s String) ENGINE = Kafka()
SETTINGS kafka_format = 'CSV', kafka_group_name = 'CHgroup'
, kafka_broker_list = '127.1.1.1:9092', kafka_topic_list = '1-partition'
, kafka_handle_error_mode = 'stream'
, kafka_num_consumers=1;

CREATE MATERIALIZED VIEW IF NOT EXISTS etl.pipeline1 TO etl.trashbin AS SELECT s FROM etl.consume_kafka1
WHERE length(_error) >0
;

CREATE TABLE IF NOT EXISTS etl.consume_kafka2 (s String) ENGINE = Kafka()
SETTINGS kafka_format = 'CSV', kafka_group_name = 'CHgroup'
, kafka_broker_list = '127.2.2.2:9092', kafka_topic_list = '1-partition'
, kafka_handle_error_mode = 'stream'
, kafka_num_consumers=1;

CREATE MATERIALIZED VIEW IF NOT EXISTS etl.pipeline2 TO etl.trashbin AS SELECT s FROM etl.consume_kafka2
WHERE length(_error) >0
;
```
