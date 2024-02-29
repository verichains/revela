#!/bin/bash

if [ -z "$RELEASE_DIR" ]; then
    echo "RELEASE_DIR is not set"
    exit 1
fi

#parse arguments
opt_target=""
while getopts "t:" opt; do
    case $opt in
    t) opt_target=$OPTARG ;;
    \?)
        echo "Invalid option -$OPTARG" >&2
        exit 1
        ;;
    esac
done

sudo apt-get update

sudo apt-get install build-essential ca-certificates clang curl git libpq-dev libssl-dev pkg-config lsof lld --no-install-recommends --assume-yes

case $opt_target in
x86_64-pc-windows-gnu)
    TARGET="x86_64-pc-windows-gnu"
    sudo apt-get install gcc-mingw-w64-x86-64-posix --no-install-recommends --assume-yes
    ;;

x86_64-apple-darwin)
    TARGET="x86_64-apple-darwin"
    export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER=rust-lld
    ;;
aarch64-apple-darwin)
    TARGET="aarch64-apple-darwin"
    export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=rust-lld
    ;;

x86_64-unknown-linux-gnu)
    TARGET="x86_64-unknown-linux-gnu"
    ;;
aarch64-unknown-linux-gnu)
    TARGET="aarch64-unknown-linux-gnu"
    sudo apt-get install gcc-aarch64-linux-gnu libc6-dev-arm64-cross --no-install-recommends --assume-yes
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
    ;;
*)
    echo "Invalid target: $target"
    exit 1
    ;;
esac

echo "Building for target: $TARGET"
set -e
rustup target add $TARGET

export PATH=$PATH:~/.rustup/toolchains/stable-$TARGET/lib/rustlib/$TARGET/bin/

RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --profile cli --target $TARGET -p revela

BIN_SUFFIX=""
if [[ $TARGET == "x86_64-pc-windows-gnu" ]]; then
    BIN_SUFFIX=".exe"
fi
cp target/$TARGET/cli/revela$BIN_SUFFIX $RELEASE_DIR/revela-$TARGET$BIN_SUFFIX
echo "Build completed successfully"
