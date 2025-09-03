#!/bin/bash
set -e

echo "🔧 Building LTO-Optimized qfplib Performance Tests"
echo "=================================================="

# Clean previous builds to ensure fresh LTO compilation
echo "Cleaning previous builds..."
cargo clean

echo ""
echo "Building qfplib with maximum LTO optimizations..."

# Build with the custom lto-max profile for maximum performance
cargo build --bin emon32-qfplib-performance --target thumbv6m-none-eabi --features "rtt qfplib" --profile lto-max

# Copy and convert the LTO-optimized binary
cp target/thumbv6m-none-eabi/lto-max/emon32-qfplib-performance bin/emon32-qfplib-lto.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-qfplib-lto.elf bin/emon32-qfplib-lto.uf2 --linker ../linker/samd21j17.ld

echo ""
echo "Building micromath baseline with LTO for fair comparison..."
cargo build --bin emon32-performance --target thumbv6m-none-eabi --features rtt --profile lto-max
cp target/thumbv6m-none-eabi/lto-max/emon32-performance bin/emon32-micromath-lto.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-micromath-lto.elf bin/emon32-micromath-lto.uf2 --linker ../linker/samd21j17.ld

# Also build the hybrid performance test with LTO
echo ""
echo "Building hybrid FastMath with LTO..."
cargo build --bin emon32-hybrid-performance --target thumbv6m-none-eabi --features "rtt qfplib" --profile lto-max
cp target/thumbv6m-none-eabi/lto-max/emon32-hybrid-performance bin/emon32-hybrid-lto.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-hybrid-lto.elf bin/emon32-hybrid-lto.uf2 --linker ../linker/samd21j17.ld

echo ""
echo "✅ LTO-Optimized Performance Binaries Ready"
echo "============================================"
echo "📁 qfplib LTO:    bin/emon32-qfplib-lto.uf2"
echo "📁 micromath LTO: bin/emon32-micromath-lto.uf2"
echo "📁 hybrid LTO:    bin/emon32-hybrid-lto.uf2"

echo ""
echo "📋 LTO Testing Instructions:"
echo "1. Test qfplib with maximum LTO optimization:"
echo "   Upload: bin/emon32-qfplib-lto.uf2"
echo "   Expected: Fastest possible qfplib performance with FFI overhead eliminated"
echo ""
echo "2. Compare with LTO-optimized micromath baseline:"
echo "   Upload: bin/emon32-micromath-lto.uf2"
echo "   Expected: Fair comparison, both with same LTO optimizations"
echo ""
echo "3. Test hybrid approach with LTO:"
echo "   Upload: bin/emon32-hybrid-lto.uf2"
echo "   Expected: Best of both worlds - qfplib for exp/div, micromath for others"
echo ""
echo "🎯 LTO Benefits:"
echo "   - Function call overhead eliminated through inlining"
echo "   - Cross-module optimizations between Rust and C code"
echo "   - Aggressive dead code elimination"
echo "   - Maximum possible qfplib performance"