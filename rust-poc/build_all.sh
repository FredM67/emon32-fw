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
echo "Binary Comparison:"
echo "------------------"
echo "Simple POC: $(ls -lh target/thumbv6m-none-eabi/release/emon32-poc | awk '{print $5}')"
echo "RTIC:       $(ls -lh target/thumbv6m-none-eabi/release/emon32-rtic | awk '{print $5}')"
echo ""
echo "✅ Ready for hardware deployment!"
echo ""
echo "Next steps:"
echo "- Flash to SAMD21 hardware"  
echo "- Test real-time performance"
echo "- Implement ADC and UART drivers"
