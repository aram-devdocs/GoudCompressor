#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Build the Rust project with wasm-pack and generate TypeScript bindings
wasm-pack build --target web --out-dir ts-wrapper
