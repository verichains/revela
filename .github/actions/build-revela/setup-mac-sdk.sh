#!/bin/bash
# ref: https://github.com/rust-lang/rust/issues/112501#issuecomment-1682426620

version=""
checksum=""
install_dir=""
while getopts "v:c:i:" opt; do
    case $opt in
    v) version=$OPTARG ;;
    c) checksum=$OPTARG ;;
    i) install_dir=$OPTARG ;;
    \?)
        echo "Invalid option -$OPTARG" >&2
        exit 1
        ;;
    esac
done

if [ -z "$version" ]; then
    echo "version is not set"
    exit 1
fi

if [ -z "$checksum" ]; then
    echo "checksum is not set"
    exit 1
fi

if [ -z "$install_dir" ]; then
    echo "install_dir is not set"
    exit 1
fi

set -e

url="https://github.com/roblabla/MacOSX-SDKs/releases/download/$version/MacOSX$version.sdk.tar.xz"
echo "Downloading $url"
curl -L -o /tmp/MacOSX$version.sdk.tar.xz $url

echo "Verifying checksum"
echo "$checksum  /tmp/MacOSX$version.sdk.tar.xz" | shasum -a 256 -c -

echo "Extracting SDK"
mkdir -p $install_dir
tar -C $install_dir -xJf /tmp/MacOSX$version.sdk.tar.xz
