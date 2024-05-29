FROM docker.io/rust:1.63-slim-bullseye AS build-env
WORKDIR /build
RUN cargo install --locked --no-default-features task-keeper miniserve
COPY rust-toolchain.toml Cargo.toml Cargo.lock /build
RUN mkdir -p /build/src && \
    echo "fn main(){}" > /build/src/main.rs && \
    cargo build --release
COPY src/ /build/src
RUN cargo build --release --bins

FROM docker.io/buildpack-deps:bullseye-curl AS runtime
WORKDIR /app
VOLUME /data
COPY --from=build-env /usr/local/cargo/bin/tk /usr/local/cargo/bin/miniserve /app/
COPY --from=ghcr.io/morphy2k/rss-forwarder:0.6.0-alpha.0 /usr/local/bin/rss-forwarder /app/
CMD ["/bin/bash", "/app/gross.sh"]
COPY gross.sh feeds.toml /app/
COPY tests/output.json /app/tests/
COPY --from=build-env /build/target/release/laker-taker /build/target/release/grab /app/
