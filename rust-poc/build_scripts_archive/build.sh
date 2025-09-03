#!/bin/bash

# Build script for emon32 Rust POC

set -e  # Exit on any error

echo "Building emon32 Rust POC..."

# Change to the rust-poc directory
cd "$(dirname "$0")"

# Check if Rust embedded target is installed
if ! rustup target list --installed | grep -q "thumbv6m-none-eabi"; then
    echo "Installing Rust embedded target..."
    rustup target add thumbv6m-none-eabi
fi

# Clean previous build
echo "Cleaning previous build..."
cargo clean

# Build the project
echo "Building project..."
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # Generate binary file
    echo "Generating binary file..."
    cargo objcopy --release --bin emon32-poc -- -O binary bin/emon32-poc.bin
    
    # Show size information
    echo "Binary size information:"
    cargo size --release
    
    echo ""
    echo "Binary file: bin/emon32-poc.bin"
    echo "Ready for flashing to emonPi3 hardware!"
    
else
    echo "Build failed!"
    exit 1
fi