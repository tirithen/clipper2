#!/usr/bin/env bash
# Usage: scripts/wasm-check.sh
#
# Build verification for wasm32-unknown-unknown using the WASI SDK clang
# toolchain. clipper2 transitively builds Clipper2 C++ through the
# clipper2c-sys crate, so a working C++ toolchain targeting wasm is
# required. Downloads WASI SDK into .tmp/ on first run.

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

WASI_SDK_VERSION="${WASI_SDK_VERSION:-24}"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)   wasi_arch="x86_64-linux" ;;
  Linux-aarch64)  wasi_arch="arm64-linux" ;;
  Darwin-x86_64)  wasi_arch="x86_64-macos" ;;
  Darwin-arm64)   wasi_arch="arm64-macos" ;;
  *) echo "wasm-check.sh: unsupported host $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac

WASI_SDK_DIR=".tmp/wasi-sdk-${WASI_SDK_VERSION}.0-${wasi_arch}"

if [[ ! -d "$WASI_SDK_DIR" ]]; then
  mkdir -p .tmp
  curl -fsSL \
    "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_SDK_VERSION}/wasi-sdk-${WASI_SDK_VERSION}.0-${wasi_arch}.tar.gz" \
    | tar -xz -C .tmp
fi

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true

export CC="$PWD/$WASI_SDK_DIR/bin/clang"
export CXX="$PWD/$WASI_SDK_DIR/bin/clang++"
export AR="$PWD/$WASI_SDK_DIR/bin/llvm-ar"

# WASI SDK ships libc++ only under wasm32-wasip1 sysroot paths. Override
# clang's target so it finds those headers; the generated .o files are
# still link-compatible with the wasm32-unknown-unknown rustc output.
export CFLAGS_wasm32_unknown_unknown="--target=wasm32-wasip1"
export CXXFLAGS_wasm32_unknown_unknown="--target=wasm32-wasip1"

cargo build --target wasm32-unknown-unknown
