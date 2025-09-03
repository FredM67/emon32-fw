#!/bin/bash

# Build debug versions with oscilloscope support
# For use with Siglent SDS1202X-E validation

set -e

echo "🔬 Building OSCILLOSCOPE DEBUG versions..."

# Clean previous builds
cargo clean

echo "🔧 Building simple debug version..."
cargo build --release --bin emon32-debug

echo "⚡ Building RTIC debug version..."
cargo build --release --bin emon32-rtic-debug

echo "📦 Generating debug firmware binaries..."

# Generate binary files for flashing
cargo objcopy --release --bin emon32-debug -- -O binary target/emon32-debug.bin
cargo objcopy --release --bin emon32-rtic-debug -- -O binary target/emon32-rtic-debug.bin

# Generate UF2 files for easy drag-and-drop uploading
echo "🔄 Converting to UF2 format for Arduino Zero..."
python3 ../scripts/bin_to_uf2.py target/emon32-debug.bin target/emon32-debug.uf2 --base 0x2000 --family SAMD21 --linker ../linker/samd21j17.ld
python3 ../scripts/bin_to_uf2.py target/emon32-rtic-debug.bin target/emon32-rtic-debug.uf2 --base 0x2000 --family SAMD21 --linker ../linker/samd21j17.ld

echo "📊 Binary sizes:"
ls -lh target/emon32-debug.bin target/emon32-rtic-debug.bin
echo "📊 UF2 file sizes:"
ls -lh target/emon32-debug.uf2 target/emon32-rtic-debug.uf2

echo ""
echo "✅ DEBUG FIRMWARE READY FOR ARDUINO ZERO OSCILLOSCOPE VALIDATION!"
echo ""
echo "📋 Flash commands:"
echo "   Simple version: flash target/emon32-debug.bin"
echo "   RTIC version:   flash target/emon32-rtic-debug.bin"
echo ""
echo "🔌 Arduino Zero connections (Siglent SDS1202X-E):"
echo "   CH1 → Pin 2  (PA14 - ADC timing signals)"
echo "   CH2 → Pin 5  (PA15 - Processing duration)"
echo "   Trigger → Pin 7  (PA21 - Interrupt response)"
echo "   Visual → Pin 13 (PA17 - Onboard LED status)"
echo ""
echo "⚙️  Recommended scope settings:"
echo "   Time/Div: 50μs (for 4800 Hz sample rate)"
echo "   Voltage: 1V/div (3.3V logic)"
echo "   Trigger: Pin 2 (PA14) rising edge, auto mode"
echo ""