#!/bin/bash

# Generate ABI
cargo near abi

# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM using wasm-opt
wasm-opt -Oz -o target/wasm32-unknown-unknown/release/cargo_build_rugfactory_factory.wasm target/wasm32-unknown-unknown/release/*.wasm

# Create build_near directory if it doesn't exist
mkdir -p build_near

# Copy build artifacts to build_near directory
cp target/wasm32-unknown-unknown/release/cargo_build_rugfactory_factory.wasm build_near/
cp target/near/rugfactory_factory_abi.json build_near/

echo "Build completed successfully!"