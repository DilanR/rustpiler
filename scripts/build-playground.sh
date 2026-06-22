#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Building WASM..."
wasm-pack build \
  "$ROOT/crates/wasm" \
  --target web \
  --out-dir "$ROOT/playground/src/features/compiler/pkg"

echo "Installing dependencies..."
cd "$ROOT/playground"
npm ci

echo "Building playground..."
npm run build
