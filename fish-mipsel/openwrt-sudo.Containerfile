# syntax=docker/dockerfile:1.8
FROM ghcr.io/openwrt/sdk:ramips-mt7621-23.05.4

ENV PREFIX=/usr PATH=${PATH}:/builder/staging_dir/target-mipsel_24kc_musl/host/bin/:/builder/staging_dir/toolchain-mipsel_24kc_gcc-12.3.0_musl/bin/

RUN true && \
    sed -i s,.openwrt.org/feed/,hub.com/openwrt/, feeds.conf.default && \
    ./scripts/feeds update packages &&\
    true
RUN true && \
    make defconfig &&\
    true
RUN true && \
    ./scripts/feeds install rust &&\
    true
RUN true && \
    make package/rust/clean package/rust/compile -j$(nproc) &&\
    true

RUN true &&\
    git clone --depth 1 https://github.com/trifectatechfoundation/sudo-rs /var/cache/sudo-rs &&\
    true
WORKDIR /var/cache/sudo-rs

RUN cargo fetch
RUN cargo build --offline || true
