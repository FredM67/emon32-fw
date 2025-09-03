#!/bin/bash

# Build and test qfplib performance on ARM Cortex-M0+
# This script builds both standard and qfplib versions for comparison

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Building qfplib Performance Tests${NC}"
echo "==============================================="

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
cargo clean

# Build standard math baseline (using emon32-performance which doesn't require qfplib)
echo -e "${YELLOW}Building standard math baseline...${NC}"
cargo build --bin emon32-performance --release --target thumbv6m-none-eabi --features rtt
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Standard math build successful${NC}"
    
    # Generate UF2 for standard math
    arm-none-eabi-objcopy -O binary \
        target/thumbv6m-none-eabi/release/emon32-performance \
        bin/emon32-performance-standard.bin
    
    python3 ../scripts/bin_to_uf2.py \
        bin/emon32-performance-standard.bin \
        bin/emon32-performance-standard.uf2 \
        --linker ../linker/samd21j17.ld
    
    echo -e "${GREEN}✓ Standard performance UF2 created: emon32-performance-standard.uf2${NC}"
else
    echo -e "${RED}✗ Standard math build failed${NC}"
    exit 1
fi

echo ""

# Build qfplib optimized version
echo -e "${YELLOW}Building qfplib optimized version...${NC}"
cargo build --bin emon32-qfplib-performance --release --target thumbv6m-none-eabi --features "rtt,qfplib"
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ qfplib build successful${NC}"
    
    # Generate UF2 for qfplib
    arm-none-eabi-objcopy -O binary \
        target/thumbv6m-none-eabi/release/emon32-qfplib-performance \
        bin/emon32-qfplib-performance.bin
    
    python3 ../scripts/bin_to_uf2.py \
        bin/emon32-qfplib-performance.bin \
        bin/emon32-qfplib-performance.uf2 \
        --linker ../linker/samd21j17.ld
    
    echo -e "${GREEN}✓ qfplib performance UF2 created: emon32-qfplib-performance.uf2${NC}"
else
    echo -e "${RED}✗ qfplib build failed${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}📊 Performance Test Instructions${NC}"
echo "================================="
echo ""
echo "Two firmware files have been created for performance comparison:"
echo ""
echo -e "${YELLOW}1. Standard Math Baseline:${NC}"
echo "   File: bin/emon32-performance-standard.uf2"
echo "   Uses: micromath library (standard Rust embedded math)"
echo ""
echo -e "${YELLOW}2. qfplib Optimized:${NC}"
echo "   File: bin/emon32-qfplib-performance.uf2"
echo "   Uses: qfplib (ARM Cortex-M optimized fast math)"
echo ""
echo -e "${BLUE}Hardware Testing Procedure:${NC}"
echo "1. Connect Arduino Zero to your computer via USB"
echo "2. Double-press the reset button to enter bootloader mode"
echo "3. Copy bin/emon32-performance-standard.uf2 to the ARDUINO drive"
echo "4. Wait for device to restart and connect RTT viewer:"
echo "   ${GREEN}probe-run --chip ATSAMD21J17A target/thumbv6m-none-eabi/release/emon32-qfplib-performance${NC}"
echo "5. Record the performance results"
echo "6. Repeat steps 2-5 with bin/emon32-qfplib-performance.uf2"
echo "7. Compare the cycle counts and timing results"
echo ""
echo -e "${BLUE}Expected Results:${NC}"
echo "• qfplib should show significantly lower cycle counts"
echo "• Square root operations should be 2-3x faster with qfplib"
echo "• Division operations should be 2-4x faster with qfplib"
echo "• Overall energy calculations should be noticeably faster"
echo ""
echo -e "${BLUE}RTT Output Format:${NC}"
echo "The firmware will output cycle counts and microsecond timings for:"
echo "• Square root operations (RMS calculations)"
echo "• Division operations (power calculations)"
echo "• Multiplication operations"
echo "• Combined energy calculations"
echo ""
echo -e "${GREEN}✓ Build complete! Ready for hardware performance testing.${NC}"