#!/bin/bash
set -e

echo "🔧 Building qfplib Performance Tests"
echo "===================================="

# Build qfplib debug test
echo "Building qfplib integration debug test..."
cargo build --bin emon32-qfplib-debug --target thumbv6m-none-eabi --features "rtt qfplib" --release
cp target/thumbv6m-none-eabi/release/emon32-qfplib-debug bin/emon32-qfplib-debug.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-qfplib-debug.elf bin/emon32-qfplib-debug.uf2 --linker ../linker/samd21j17.ld

# Build standard micromath version
echo "Building standard math baseline..."
cargo build --bin emon32-performance --target thumbv6m-none-eabi --features rtt --release
cp target/thumbv6m-none-eabi/release/emon32-performance bin/emon32-performance-micromath.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-performance-micromath.elf bin/emon32-performance-micromath.uf2 --linker ../linker/samd21j17.ld

# Build qfplib optimized version  
echo "Building qfplib optimized version..."
cargo build --bin emon32-qfplib-performance --target thumbv6m-none-eabi --features "rtt qfplib" --release
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/emon32-qfplib-performance bin/emon32-performance-qfplib.bin
cp target/thumbv6m-none-eabi/release/emon32-qfplib-performance bin/emon32-performance-qfplib.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-performance-qfplib.elf bin/emon32-performance-qfplib.uf2 --linker ../linker/samd21j17.ld

# Build qfplib complex math test
echo "Building qfplib complex math performance test..."
cargo build --bin emon32-qfplib-complex --target thumbv6m-none-eabi --features "rtt qfplib" --release
cp target/thumbv6m-none-eabi/release/emon32-qfplib-complex bin/emon32-qfplib-complex.elf
python3 ../scripts/bin_to_uf2.py bin/emon32-qfplib-complex.elf bin/emon32-qfplib-complex.uf2 --linker ../linker/samd21j17.ld

echo ""
echo "✅ qfplib Performance Test Binaries Ready"
echo "=========================================="
echo "📁 Debug test:         bin/emon32-qfplib-debug.uf2"
echo "📁 Simple comparison:  bin/emon32-performance-micromath.uf2 vs bin/emon32-performance-qfplib.uf2"
echo "📁 Complex math test:  bin/emon32-qfplib-complex.uf2"

echo ""
echo "📋 Testing Instructions:"
echo "1. DEBUG TEST (verify qfplib integration):"
echo "   Upload: bin/emon32-qfplib-debug.uf2"
echo "   Purpose: Confirm qfplib functions are being called correctly"
echo ""
echo "2. SIMPLE MATH COMPARISON (shows function call overhead):"
echo "   Upload: bin/emon32-performance-micromath.uf2"
echo "   Then:   bin/emon32-performance-qfplib.uf2"
echo "   Purpose: Shows qfplib has overhead for simple operations"
echo ""
echo "3. COMPLEX MATH TEST (where qfplib should excel):"
echo "   Upload: bin/emon32-qfplib-complex.uf2"
echo "   Purpose: Shows qfplib's advantages for trigonometric, exponential operations"
echo ""
echo "� Expected Results:"
echo "   - Debug test: Should show different results for qfplib vs micromath"
echo "   - Simple test: qfplib may be slower due to function call overhead" 
echo "   - Complex test: qfplib should be 2-10x faster for sin/cos/exp/ln operations"