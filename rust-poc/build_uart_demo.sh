#!/bin/bash

# Build script for emon32 UART serial output demo
# This builds firmware that demonstrates the energy monitoring output format

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Building emon32 UART Serial Output Demo...${NC}"

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
FEATURES="rtt"
PROFILE="release"

echo -e "${BLUE}Building with features: $FEATURES${NC}"
echo -e "${BLUE}Profile: $PROFILE${NC}"

# Build the simple UART demo
echo -e "${YELLOW}Building UART serial output demo...${NC}"
cargo build --target "$TARGET" --profile "$PROFILE" \
    --features "$FEATURES" \
    --bin emon32-uart

# Build the RTIC UART demo
echo -e "${YELLOW}Building RTIC UART demo...${NC}"
cargo build --target "$TARGET" --profile "$PROFILE" \
    --features "$FEATURES" \
    --bin emon32-rtic-uart

# Convert to binary and UF2 format
TARGET_DIR="target/$TARGET/$PROFILE"

echo -e "${BLUE}Converting to binary format...${NC}"

# Simple UART demo
if [ -f "$TARGET_DIR/emon32-uart" ]; then
    arm-none-eabi-objcopy -O binary "$TARGET_DIR/emon32-uart" "emon32-uart-demo.bin"
    echo -e "${GREEN}Created: emon32-uart-demo.bin${NC}"
    
    # Create UF2 file
    python3 ../scripts/bin_to_uf2.py --linker ../linker/samd21j17.ld "emon32-uart-demo.bin" "emon32-uart-demo.uf2"
    echo -e "${GREEN}Created: emon32-uart-demo.uf2${NC}"
fi

# RTIC UART demo
if [ -f "$TARGET_DIR/emon32-rtic-uart" ]; then
    arm-none-eabi-objcopy -O binary "$TARGET_DIR/emon32-rtic-uart" "emon32-rtic-uart-demo.bin"
    echo -e "${GREEN}Created: emon32-rtic-uart-demo.bin${NC}"
    
    # Create UF2 file
    python3 ../scripts/bin_to_uf2.py --linker ../linker/samd21j17.ld "emon32-rtic-uart-demo.bin" "emon32-rtic-uart-demo.uf2"
    echo -e "${GREEN}Created: emon32-rtic-uart-demo.uf2${NC}"
fi

echo ""
echo -e "${GREEN}UART Serial Output Demo Build Complete!${NC}"
echo ""
echo "Built firmware files:"
echo "  • emon32-uart-demo.uf2 - Simple UART serial output demo"
echo "  • emon32-rtic-uart-demo.uf2 - RTIC-based UART demo"
echo ""
echo "These demonstrate the energy monitoring output format:"
echo "  \"1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W\""
echo ""
echo "Flash to your Arduino Zero and monitor via:"
echo "  • RTT (if RTT debugger connected): shows formatted output"
echo "  • UART (future): 115200 baud on pins 2/5"
echo ""
echo "Note: Current implementation uses RTT for demo purposes."
echo "      Real UART hardware integration pending."