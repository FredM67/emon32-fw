#!/bin/bash

# Build script for qfplib performance testing on ARM Cortex-M0+
# This script builds the firmware with qfplib integration enabled

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Building emon32 with qfplib integration...${NC}"

# Check if we're building for the right target
TARGET="thumbv6m-none-eabi"
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo -e "${YELLOW}Installing target $TARGET...${NC}"
    rustup target add "$TARGET"
fi

# Check for required tools
if ! command -v arm-none-eabi-objcopy &> /dev/null; then
    echo -e "${RED}Error: arm-none-eabi-objcopy not found${NC}"
    echo "Please install arm-none-eabi-gcc toolchain"
    echo "On Ubuntu/Debian: sudo apt install gcc-arm-none-eabi"
    echo "On macOS: brew install arm-none-eabi-gcc"
    exit 1
fi

# Build configurations
FEATURES="rtt,qfplib"
PROFILE="release"

echo -e "${BLUE}Building with features: $FEATURES${NC}"
echo -e "${BLUE}Profile: $PROFILE${NC}"

# Build the main POC with qfplib
echo -e "${YELLOW}Building main POC with qfplib...${NC}"
cargo build --target "$TARGET" --profile "$PROFILE" \
    --features "$FEATURES" \
    --bin emon32-poc

# Build the qfplib performance test
echo -e "${YELLOW}Building qfplib performance test...${NC}"
cargo build --target "$TARGET" --profile "$PROFILE" \
    --features "$FEATURES" \
    --bin emon32-qfplib-performance

# Build the energy calculator with qfplib
echo -e "${YELLOW}Building RTIC version with qfplib...${NC}"
cargo build --target "$TARGET" --profile "$PROFILE" \
    --features "$FEATURES" \
    --bin emon32-rtic || echo -e "${YELLOW}RTIC build failed (expected due to timer issues)${NC}"

# Convert to binary and UF2 format
TARGET_DIR="target/$TARGET/$PROFILE"

echo -e "${BLUE}Converting to binary format...${NC}"

# Main POC binary
if [ -f "$TARGET_DIR/emon32-poc" ]; then
    arm-none-eabi-objcopy -O binary "$TARGET_DIR/emon32-poc" "bin/emon32-poc-qfplib.bin"
    echo -e "${GREEN}Created: bin/emon32-poc-qfplib.bin${NC}"
    
    # Create UF2 file
    python3 ../scripts/bin_to_uf2.py --linker ../linker/samd21j17.ld "bin/emon32-poc-qfplib.bin" "bin/emon32-poc-qfplib.uf2"
    echo -e "${GREEN}Created: bin/emon32-poc-qfplib.uf2${NC}"
fi

# qfplib performance test binary
if [ -f "$TARGET_DIR/emon32-qfplib-performance" ]; then
    arm-none-eabi-objcopy -O binary "$TARGET_DIR/emon32-qfplib-performance" "bin/emon32-qfplib-performance.bin"
    echo -e "${GREEN}Created: bin/emon32-qfplib-performance.bin${NC}"
    
    # Create UF2 file
    python3 ../scripts/bin_to_uf2.py --linker ../linker/samd21j17.ld "bin/emon32-qfplib-performance.bin" "bin/emon32-qfplib-performance.uf2"
    echo -e "${GREEN}Created: bin/emon32-qfplib-performance.uf2${NC}"
fi

# RTIC binary (if it built successfully)
if [ -f "$TARGET_DIR/emon32-rtic" ]; then
    arm-none-eabi-objcopy -O binary "$TARGET_DIR/emon32-rtic" "bin/emon32-rtic-qfplib.bin"
    echo -e "${GREEN}Created: bin/emon32-rtic-qfplib.bin${NC}"
    
    # Create UF2 file
    python3 ../scripts/bin_to_uf2.py --linker ../linker/samd21j17.ld "bin/emon32-rtic-qfplib.bin" "bin/emon32-rtic-qfplib.uf2"
    echo -e "${GREEN}Created: bin/emon32-rtic-qfplib.uf2${NC}"
fi

echo ""
echo -e "${GREEN}qfplib build complete!${NC}"
echo ""
echo "Built firmware files:"
echo "  • bin/emon32-poc-qfplib.uf2 - Main POC with qfplib optimization"
echo "  • bin/emon32-qfplib-performance.uf2 - Performance comparison test"
echo ""
echo "Flash these to your Arduino Zero to test qfplib performance!"
echo ""
echo "To monitor output:"
echo "  1. Flash the firmware"
echo "  2. Connect probe-rs or SEGGER RTT Viewer"
echo "  3. View performance comparison results"