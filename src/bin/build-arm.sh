#!/bin/bash
# ARM Linux build script for C4m1r's Wiki (Raspberry Pi, etc.)

set -e

echo "Building C4m1r's Wiki for ARM..."

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed."
    echo "Please install Rust first: https://rustup.rs/"
    exit 1
fi

# Check architecture
ARCH=$(uname -m)
echo "Detected architecture: $ARCH"

# Build the Rust binary for ARM
echo "Compiling Rust builder for ARM..."
cargo build --release --target arm-unknown-linux-gnueabihf --manifest-path src/bin/Cargo.toml

if [ $? -ne 0 ]; then
    echo "Error: Failed to compile Rust builder for ARM."
    exit 1
fi

# Run the Rust builder
echo "Running wiki builder..."
./target/arm-unknown-linux-gnueabihf/release/nervaweb ru

if [ $? -ne 0 ]; then
    echo "Error: Failed to build the wiki."
    exit 1
fi

echo "Success! Wiki built successfully for ARM."
echo "Output available in the 'book' folder."
