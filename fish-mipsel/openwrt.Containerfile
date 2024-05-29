# syntax=docker/dockerfile:1.4
FROM ghcr.io/openwrt/sdk:ramips-mt7621-23.05.3

ENV PREFIX=/usr PATH=${PATH}:/builder/staging_dir/target-mipsel_24kc_musl/host/bin/:/builder/staging_dir/toolchain-mipsel_24kc_gcc-12.3.0_musl/bin/

RUN true && \
    sed -i s,.openwrt.org/feed/,hub.com/openwrt/, feeds.conf.default && \
    ./scripts/feeds update packages &&\
    true
RUN true && \
    make defconfig &&\
    true
RUN true && \
    ./scripts/feeds install bottom &&\
    true
RUN true && \
    make package/bottom/clean package/bottom/compile -j$(nproc) &&\
    true

RUN true &&\
#    git clone --depth 1 https://github.com/fish-shell/fish-shell /var/cache/fish-shell &&\
    cd /var/cache/fish-shell && cargo add portable-atomic@1.6.0 --no-default-features --features fallback &&\
    true
WORKDIR /var/cache/fish-shell

RUN cargo fetch
RUN cargo build --offline || true

COPY gnu-mips.patch .
RUN patch -p1 <gnu-mips.patch || true
