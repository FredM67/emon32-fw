#!/bin/bash
set -e

echo "🔧 Building qfplib-sys LTO Performance Tests"
echo "============================================="

# Clean previous builds to ensure fresh LTO compilation with new crate structure
echo "Cleaning previous builds..."
cargo clean

echo ""
echo "Building qfplib-sys crate with maximum LTO optimizations..."

# Set environment variable for maximum LTO optimization
export QFPLIB_LTO_LEVEL=aggressive

# Build with the custom lto-max profile for maximum performance using qfplib-sys
echo "Building qfplib performance test with qfplib-sys crate..."
cargo build --bin emon32-qfplib-performance --target thumbv6m-none-eabi --features "rtt qfplib" --profile lto-max

# Copy and convert the LTO-optimized binary
cp target/thumbv6m-none-eabi/lto-max/emon32-qfplib-performance bin/emon32-qfplib-sys-lto.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-qfplib-sys-lto.elf bin/emon32-qfplib-sys-lto.uf2 --linker ../linker/samd21j17.ld

echo ""
echo "Building micromath baseline with LTO for fair comparison..."
cargo build --bin emon32-performance --target thumbv6m-none-eabi --features rtt --profile lto-max
cp target/thumbv6m-none-eabi/lto-max/emon32-performance bin/emon32-micromath-sys-lto.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-micromath-sys-lto.elf bin/emon32-micromath-sys-lto.uf2 --linker ../linker/samd21j17.ld

# Build with different LTO levels to compare
echo ""
echo "Building with size-optimized LTO..."
export QFPLIB_LTO_LEVEL=size
cargo build --bin emon32-qfplib-performance --target thumbv6m-none-eabi --features "rtt qfplib" --profile lto-size
cp target/thumbv6m-none-eabi/lto-size/emon32-qfplib-performance bin/emon32-qfplib-sys-size.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-qfplib-sys-size.elf bin/emon32-qfplib-sys-size.uf2 --linker ../linker/samd21j17.ld

echo ""
echo "✅ qfplib-sys LTO Performance Binaries Ready"
echo "============================================="
echo "📁 qfplib-sys LTO max:  bin/emon32-qfplib-sys-lto.uf2"
echo "📁 micromath LTO:       bin/emon32-micromath-sys-lto.uf2"
echo "📁 qfplib-sys size opt: bin/emon32-qfplib-sys-size.uf2"

echo ""
echo "📋 qfplib-sys LTO Testing Instructions:"
echo "1. Test qfplib-sys with maximum LTO optimization:"
echo "   Upload: bin/emon32-qfplib-sys-lto.uf2"
echo "   Expected: Zero FFI overhead through aggressive LTO inlining"
echo ""
echo "2. Compare with LTO-optimized micromath baseline:"
echo "   Upload: bin/emon32-micromath-sys-lto.uf2"
echo "   Expected: Fair comparison, both with same LTO optimizations"
echo ""
echo "3. Test size-optimized version:"
echo "   Upload: bin/emon32-qfplib-sys-size.uf2"
echo "   Expected: Smallest binary size while maintaining performance gains"
echo ""
echo "🎯 qfplib-sys Advantages:"
echo "   - Dedicated crate with proper separation of concerns"
echo "   - Multiple LTO optimization levels (aggressive, size, minimal)"
echo "   - Safe wrapper API with zero-cost abstractions"
echo "   - Static library integration for better LTO optimization"
echo "   - Environment variable control: QFPLIB_LTO_LEVEL=(debug|minimal|aggressive|size)"