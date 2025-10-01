# syntax=docker/dockerfile:1.18
FROM ghcr.io/openwrt/sdk:ramips-mt7621-24.10.3

RUN <<EOS
set -xeu
sed -i s,.openwrt.org/feed/,hub.com/openwrt/, feeds.conf.default
./scripts/feeds update packages
make defconfig
./scripts/feeds install bottom
EOS

# 1.87 https://github.com/openwrt/packages/commit/95eef0fd580a411bd487ccf61a8d4bd25beab5c2
# 1.89 with xz https://github.com/openwrt/packages/commit/10862df850ae012b34ec9c57a9005b1f7e1e2aca
RUN <<EOS
set -xeu
sed -i s,1.85,1.89, feeds/packages/lang/rust/Makefile
sed -i s,src.tar.gz,src.tar.xz, feeds/packages/lang/rust/Makefile
sed -i /config[.]toml/d feeds/packages/lang/rust/Makefile
sed -i s,2f4f3142ffb7c8402139cfa0796e24baaac8b9fd3f96b2deec3b94b4045c6a8a,0b9d55610d8270e06c44f459d1e2b7918a5e673809c592abed9b9c600e33d95a, feeds/packages/lang/rust/Makefile
rm -vf feeds/packages/lang/rust/patches/0003-bump-libc-deps-to-0.2.146.patch
cat >feeds/packages/lang/rust/patches/0001-Update-xz2-and-use-it-static.patch <<EOD
From d3000458501d339ea2043006924d431ead18769e Mon Sep 17 00:00:00 2001
From: Luca Barbato <lu_zero@gentoo.org>
Date: Sun, 4 Jun 2023 19:32:28 +0000
Subject: [PATCH] Update xz2 and use it static

---
 src/bootstrap/Cargo.toml | 2 +-
 3 files changed, 9 insertions(+), 9 deletions(-)

--- a/src/bootstrap/Cargo.toml
+++ b/src/bootstrap/Cargo.toml
@@ -55,7 +55,7 @@ tar = "0.4"
 termcolor = "1.4"
 toml = "0.5"
 walkdir = "2.4"
-xz2 = "0.1"
+xz2 = { version = "0.1", features = ["static"] }

 # Dependencies needed by the build-metrics feature
 sysinfo = { version = "0.35.0", default-features = false, optional = true, features = ["system"] }
EOD
EOS

RUN <<EOS
set -xeu
make package/bottom/clean package/bottom/compile -j1 V=sc
EOS

ENV FISH_BUILD_DOCS=0 PATH=${PATH}:/builder/staging_dir/target-mipsel_24kc_musl/host/bin/:/builder/staging_dir/toolchain-mipsel_24kc_gcc-13.3.0_musl/bin/

ADD --link=true --chown=1000 https://github.com/uutils/coreutils/archive/refs/tags/0.2.2.tar.gz /builder/
ADD --link=true --chown=1000 https://github.com/fish-shell/fish-shell/releases/download/4.1.1/fish-4.1.1.tar.xz /builder/

RUN <<EOS
set -xeu
tar -xf 0.2.2.tar.gz
tar -xf fish-4.1.1.tar.xz
EOS

# CARGO_BUILD_TARGET=mipsel-unknown-linux-musl CARGO_HOME=/builder/dl/cargo CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 CARGO_PROFILE_RELEASE_DEBUG=false CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false CARGO_PROFILE_RELEASE_LTO=true CARGO_PROFILE_RELEASE_OPT_LEVEL=z CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true CARGO_PROFILE_RELEASE_PANIC=unwind CARGO_PROFILE_RELEASE_RPATH=false CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_LINKER=mipsel-openwrt-linux-musl-gcc RUSTFLAGS="-Ctarget-feature=-crt-static -lssp_nonshared" TARGET_CC=mipsel-openwrt-linux-musl-gcc TARGET_CFLAGS="-Os -pipe -mno-branch-likely -mips32r2 -mtune=24kc -fno-caller-saves -fno-plt -fhonour-copts -msoft-float -ffile-prefix-map=/builder/build_dir/target-mipsel_24kc_musl/fish-4.1.0=fish-4.1.0 -mips16 -minterlink-mips16 -Wformat -Werror=format-security -fstack-protector -D_FORTIFY_SOURCE=1 -Wl,-z,now -Wl,-z,relro " CC=gcc MAKEFLAGS="-j4" cargo b --profile release --bin fish
