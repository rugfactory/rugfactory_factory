#!/bin/bash

# Generate ABI
cargo near abi

# Build reproducible WASM
cargo near build reproducible-wasm

# Create build_near directory if it doesn't exist
mkdir -p build_near

# Copy build artifacts to build_near directory
cp target/near/* build_near/

echo "Build completed successfully!"