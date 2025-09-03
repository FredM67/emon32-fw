#!/bin/bash
# Arduino Zero Upload Helper for emon32 Rust Firmware

echo "🔧 Arduino Zero Upload Helper"
echo "============================="

# Check if firmware is built
if [ ! -f "target/emon32-debug.bin" ]; then
    echo "❌ Firmware not found. Building..."
    ./build_debug.sh
fi

echo ""
echo "📡 Detecting Arduino Zero bootloader type..."

# Check if UF2 bootloader is available
if ls /media/$USER/EMONBOOT* 2>/dev/null; then
    echo "✅ UF2 bootloader detected (EMONBOOT drive found)"
    echo "📁 Copying UF2 file..."
    cp target/emon32-debug.uf2 /media/$USER/EMONBOOT*/
    echo "✅ Upload complete via UF2!"
    exit 0
fi

# Check for Arduino serial port
ARDUINO_PORT=$(ls /dev/ttyACM* 2>/dev/null | head -1)

if [ -z "$ARDUINO_PORT" ]; then
    echo "❌ No Arduino Zero detected."
    echo ""
    echo "💡 Troubleshooting:"
    echo "   1. Connect Arduino Zero via USB"
    echo "   2. For UF2: Double-press RESET button"
    echo "   3. For Arduino IDE: Single-press RESET button"
    echo "   4. Check that device appears in /dev/ttyACM*"
    exit 1
fi

echo "📍 Standard Arduino bootloader detected"
echo "🔌 Port: $ARDUINO_PORT"

# Check if we have arm-none-eabi-objcopy
if ! command -v arm-none-eabi-objcopy &> /dev/null; then
    echo "❌ arm-none-eabi-objcopy not found"
    echo "📦 Install with: sudo apt install gcc-arm-none-eabi"
    exit 1
fi

# Convert to hex format
echo "🔄 Converting to .hex format..."
arm-none-eabi-objcopy -I binary -O ihex \
  --change-addresses 0x2000 \
  target/emon32-debug.bin target/emon32-debug.hex

if [ $? -ne 0 ]; then
    echo "❌ Conversion to .hex failed"
    exit 1
fi

# Check if we have avrdude
if ! command -v avrdude &> /dev/null; then
    echo "❌ avrdude not found"
    echo "📦 Install with: sudo apt install avrdude"
    echo ""
    echo "💡 Alternative: Use Arduino IDE"
    echo "   1. Open Arduino IDE"
    echo "   2. Tools → Board → Arduino Zero"  
    echo "   3. Tools → Port → $ARDUINO_PORT"
    echo "   4. File → Open → target/emon32-debug.hex"
    echo "   5. Upload"
    exit 1
fi

echo "📤 Uploading via avrdude..."
echo "   (Press RESET button on Arduino Zero now if needed)"

avrdude -v -p atsamd21g18 -c arduino -P $ARDUINO_PORT -b 115200 \
  -U flash:w:target/emon32-debug.hex:i

if [ $? -eq 0 ]; then
    echo "✅ Upload successful!"
    echo "🔍 Monitor serial output at 115200 baud on $ARDUINO_PORT"
else
    echo "❌ Upload failed"
    echo ""
    echo "💡 Try:"
    echo "   1. Press RESET button and try again"
    echo "   2. Check Arduino IDE works with this board"
    echo "   3. Use Arduino IDE upload instead"
fi