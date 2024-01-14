#!/usr/bin/env bash
wasm-pack build --target web --out-dir ../static/rust --no-pack --no-typescript
rm ../static/rust/.gitignore
