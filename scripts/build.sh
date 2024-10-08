#!/bin/bash
set -euo pipefail

export PACKAGE_NAME="net-ssr"
export ARTIFACT_NAME=$PACKAGE_NAME"-"$1
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc

export NET_SSR_BUILD_WITH_MAN=1
export NET_SSR_BUILD_WITH_COMPLETIONS=1

# Setup Rust toolchain
if [[ "$1" == *-musl ]]; then
	rustup target add "$1"
else
	rustup toolchain install stable --profile minimal --target "$1"
fi

# Build for the target
cargo build --release --locked --target "$1"

# Create the artifact
mkdir -p "$ARTIFACT_NAME"
cp "target/$1/release/interrogator" "$ARTIFACT_NAME"
cp "target/$1/release/transponder" "$ARTIFACT_NAME"
cp README.* LICENSE "$ARTIFACT_NAME"
cp -r man "$ARTIFACT_NAME"
cp -r completions "$ARTIFACT_NAME"

# Zip the artifact
if ! command -v zip &> /dev/null
then
	sudo apt-get update && sudo apt-get install -yq zip
fi
zip -r "$ARTIFACT_NAME.zip" "$ARTIFACT_NAME"

# Install cargo-deb and make deb while $1 is end with "-gnu"
if [[ "$1" == *-gnu ]]; then
	cargo install cargo-deb
	cargo deb --target "$1"
	cp target/$1/debian/$PACKAGE_NAME*.deb ./$ARTIFACT_NAME.deb
fi
