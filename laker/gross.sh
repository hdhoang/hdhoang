#!/bin/bash
env RUST_LOG=debug /app/rss-forwarder --debug /app/feeds.toml &
/app/miniserve tests/
