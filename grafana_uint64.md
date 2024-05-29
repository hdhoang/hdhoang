```bash
podman run --rm -d --name grafana    --network host -p 3000:3000 docker.io/grafana/grafana-oss:10.0.3
podman run --rm -d --name clickhouse --network host -p 8123:8123 docker.io/altinity/clickhouse-server:23.3.8.22.altinitystable
```
