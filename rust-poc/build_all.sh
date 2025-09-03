#!/bin/bash
# Build script for emon32 Rust POC - demonstrates both simple and RTIC versions

echo "🚀 Building emon32 Rust Proof-of-Concept"
echo "========================================"

# Simple POC version
echo ""
echo "📦 Building Simple POC version..."
cargo build --release --bin emon32-poc
if [ $? -eq 0 ]; then
    echo "✅ Simple POC build successful"
    cargo size --release --bin emon32-poc
else
    echo "❌ Simple POC build failed"
    exit 1
fi

echo ""
echo "📦 Building RTIC version..."
cargo build --release --bin emon32-rtic  
if [ $? -eq 0 ]; then
    echo "✅ RTIC build successful"
    cargo size --release --bin emon32-rtic
else
    echo "❌ RTIC build failed"
    exit 1
fi

echo ""
echo "🎉 Both versions built successfully!"
echo ""
echo "📦 Generating firmware binaries..."

# Generate binary files for flashing
cargo objcopy --release --bin emon32-poc -- -O binary target/emon32-poc.bin
cargo objcopy --release --bin emon32-rtic -- -O binary target/emon32-rtic.bin

# Generate UF2 files for easy drag-and-drop uploading
echo "🔄 Converting to UF2 format for Arduino Zero..."
python3 ../scripts/bin_to_uf2.py target/emon32-poc.bin target/emon32-poc.uf2 --base 0x2000 --family SAMD21 --linker ../linker/samd21j17.ld
python3 ../scripts/bin_to_uf2.py target/emon32-rtic.bin target/emon32-rtic.uf2 --base 0x2000 --family SAMD21 --linker ../linker/samd21j17.ld

echo ""
echo "Binary Comparison:"
echo "------------------"
echo "Simple POC: $(ls -lh target/thumbv6m-none-eabi/release/emon32-poc | awk '{print $5}')"
echo "RTIC:       $(ls -lh target/thumbv6m-none-eabi/release/emon32-rtic | awk '{print $5}')"
echo ""
echo "📊 Generated Files:"
echo "------------------"
echo "Binary files:"
ls -lh target/emon32-poc.bin target/emon32-rtic.bin
echo "UF2 files (for Arduino Zero upload):"
ls -lh target/emon32-poc.uf2 target/emon32-rtic.uf2
echo ""
echo "✅ Ready for hardware deployment!"
echo ""
echo "Next steps:"
echo "- Upload UF2 files to Arduino Zero (see FIRMWARE_UPLOAD_GUIDE.md)"
echo "- Test real-time performance"
echo "- Validate with oscilloscope (see OSCILLOSCOPE_VALIDATION.md)"
echo "- Compare timing accuracy vs C version"