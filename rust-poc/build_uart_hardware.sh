#!/bin/bash
# Build script for hardware UART demo binaries
# Generates UF2 files for Arduino Zero deployment

set -e

echo "Building UART Hardware Demo Binaries..."

# Build hardware UART demo
echo "Building emon32-uart-hardware..."
cargo build --release --bin emon32-uart-hardware --target thumbv6m-none-eabi

# Build RTIC hardware UART demo  
echo "Building emon32-rtic-uart-hardware..."
cargo build --release --bin emon32-rtic-uart-hardware --target thumbv6m-none-eabi

# Generate UF2 files for bootloader upload
echo "Generating UF2 files..."

# Convert to UF2 format for Arduino Zero bootloader
python3 ../scripts/bin_to_uf2.py \
    target/thumbv6m-none-eabi/release/emon32-uart-hardware \
    bin/emon32-uart-hardware.uf2 \
    --linker ../linker/samd21j17.ld

python3 ../scripts/bin_to_uf2.py \
    target/thumbv6m-none-eabi/release/emon32-rtic-uart-hardware \
    bin/emon32-rtic-uart-hardware.uf2 \
    --linker ../linker/samd21j17.ld

echo "Build complete!"
echo ""
echo "Generated files:"
echo "  - bin/emon32-uart-hardware.uf2 (Simple hardware UART demo)"
echo "  - bin/emon32-rtic-uart-hardware.uf2 (RTIC hardware UART demo)"
echo ""
echo "To upload to Arduino Zero:"
echo "1. Double-press reset button to enter bootloader mode"
echo "2. Copy .uf2 file to the EMONBOOT drive that appears"
echo ""
echo "Connect serial terminal to Arduino Zero at 115200 baud to see output"
echo "UART pins: TX=pin 2 (PA14), RX=pin 5 (PA15)"
echo ""
echo "🔌 Serial Connection Options:"
echo "  Option A: Direct USB-serial (if board has native USB-UART)"
echo "  Option B: FTDI Adapter:"
echo "    • FTDI RX → Arduino Pin 2 (TX/PA14)"
echo "    • FTDI TX → Arduino Pin 5 (RX/PA15)" 
echo "    • FTDI GND → Arduino GND"
echo "    • FTDI VCC → Not connected (Arduino powered via USB)"
echo ""
echo "📟 Terminal Commands:"
echo "  Linux/WSL: sudo minicom -D /dev/ttyUSB0 -b 115200"
echo "  Windows: PuTTY, TeraTerm, or Arduino IDE Serial Monitor"