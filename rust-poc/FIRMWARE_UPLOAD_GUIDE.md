# 📦 emon32 Rust Firmware Upload Guide

This guide covers how to upload the Rust firmware to your SAMD21-based hardware using the UF2 bootloader system.

## 🎯 **Quick Start**

For Arduino Zero users, use the automated script:
```bash
./build_debug.sh  # Builds firmware + generates UF2 files
# Then drag-and-drop the .uf2 file to your board
```

## 📋 **Prerequisites**

### Hardware Requirements
- **Arduino Zero** (ABX00003) - Primary development target
- **SAMD21** compatible board with UF2 bootloader
- USB cable for programming and power

### Software Requirements
```bash
# Install required Rust tools
rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview

# Install Python for UF2 conversion (if not using automated scripts)
python3 -m pip install --user -r scripts/requirements.txt
```

## 🔧 **Firmware Build Process**

### Option 1: Automated Build (Recommended)
```bash
# Build debug firmware for Arduino Zero with oscilloscope validation
./build_debug.sh

# OR build production firmware
./build_all.sh
```

### Option 2: Manual Build
```bash
# 1. Build the firmware
cargo build --release --bin emon32-poc

# 2. Extract binary
cargo objcopy --release --bin emon32-poc -- -O binary emon32-poc.bin

# 3. Convert to UF2 format
python3 ../scripts/bin_to_uf2.py emon32-poc.bin emon32-poc.uf2 \
    --base 0x2000 --family SAMD21
```

## 📂 **Understanding File Types**

### **Binary Files (.bin)**
- Raw machine code for the SAMD21 processor
- Used with JTAG programmers or advanced tools
- Files: `emon32-poc.bin`, `emon32-debug.bin`, `emon32-rtic.bin`

### **UF2 Files (.uf2)**
- User-friendly format for drag-and-drop programming
- Contains metadata and checksums for safe uploading
- **Preferred method** for Arduino Zero and compatible boards
- Files: `emon32-poc.uf2`, `emon32-debug.uf2`, `emon32-rtic.uf2`

### **ELF Files (.elf)**
- Contains debugging symbols and metadata
- Used by debuggers and analysis tools
- Not directly flashable to hardware

## 🚀 **Upload Methods**

### Method 1: UF2 Bootloader (Arduino Zero - Recommended)

> ⚠️ **IMPORTANT**: Standard Arduino Zero boards do NOT ship with UF2 bootloader!
> 
> If double-pressing RESET doesn't show an `EMONBOOT` drive, your Arduino Zero has the standard Arduino bootloader. **Skip to Method 2 (Arduino IDE Upload) below.**
> 
> The UF2 bootloader is primarily used on emonPi3 hardware and some custom boards.

#### **Step 1: Check for UF2 Bootloader**
1. **Connect** your Arduino Zero via USB
2. **Double-press** the RESET button quickly (< 1 second between presses)
3. **Look for**: 
   - ✅ LED starts breathing (fading in/out) → UF2 bootloader present
   - ✅ Drive named `EMONBOOT` appears → Continue with UF2 method
   - ❌ No drive appears → Standard Arduino bootloader → Use Method 2

#### **Step 2: Enter Bootloader Mode (UF2 Only)**
1. **Connect** your Arduino Zero via USB
2. **Double-press** the RESET button quickly (< 1 second between presses)  
3. **Wait** for the onboard LED to start breathing (fading in/out)
4. **Check** that a new drive named `EMONBOOT` appears

```bash
# Verify bootloader mode
ls /media/$USER/  # Should show EMONBOOT drive
```

#### **Step 3: Flash Firmware (UF2 Only)**
```bash
```bash
# Option A: Use the automated script
./build_debug.sh  # Generates UF2 files in target/ directory

# Option B: Manual drag-and-drop
cp target/emon32-debug.uf2 /media/$USER/EMONBOOT/
# OR simply drag the .uf2 file to the drive in your file manager
```

#### **Step 4: Verify Upload (UF2 Only)**
- The drive will **disappear** automatically after upload
- The onboard LED will **stop breathing** and show normal operation

---

### Method 2: Arduino IDE Upload (Standard Arduino Zero)

> ✅ **Use this method if Method 1 (UF2) didn't work**
> 
> This method works with the standard Arduino bootloader that ships with Arduino Zero boards.

#### **Step 1: Install Arduino IDE**
```bash
# Ubuntu/Debian
sudo apt install arduino

# Or download from: https://www.arduino.cc/en/software
```

#### **Step 2: Convert to Arduino-Compatible Format**
```bash
# Build the firmware 
./build_debug.sh

# Convert .bin to .hex format for Arduino IDE
arm-none-eabi-objcopy -I binary -O ihex \
  --change-addresses 0x2000 \
  target/emon32-debug.bin target/emon32-debug.hex
```

#### **Step 3: Upload via Arduino IDE**
1. **Open** Arduino IDE
2. **Select** Tools → Board → Arduino Zero (Programming Port)
3. **Select** correct USB port in Tools → Port
4. **Press** RESET button on Arduino Zero (single press)
5. **Upload** the .hex file via Tools → Burn Bootloader (for .hex files)

#### **Alternative: Use avrdude directly**
```bash
# Find the correct port
ls /dev/ttyACM*

# Upload firmware (replace /dev/ttyACM0 with your port)
avrdude -v -p atsamd21g18 -c arduino -P /dev/ttyACM0 -b 115200 \
  -U flash:w:target/emon32-debug.hex:i
```

---

### Method 3: OpenOCD/SWD Programming (Advanced)
```

#### **Step 3: Verify Upload**
- The drive will **disappear** automatically after upload
- The onboard LED will **stop breathing** and show normal operation
- The device will **reset** and start running your firmware

### Method 2: Arduino IDE Upload (Alternative)

If you have Arduino IDE installed:

1. **Open** Arduino IDE
2. **Select** Tools → Board → Arduino Zero (Programming Port)
3. **Select** Tools → Port → (your Arduino Zero port)
4. **Use** Sketch → Upload Using Programmer with a .hex file

### Method 3: JTAG/SWD Programming (Advanced)

For development boards with JTAG/SWD connectors:

```bash
# Using OpenOCD (configuration in project root)
openocd -f openocd.cfg -c \"program emon32-poc.bin 0x2000 verify reset exit\"

# Using probe-rs (if available)
probe-rs download --chip ATSAMD21J17A --format bin emon32-poc.bin
```

## 🎛️ **Available Firmware Variants**

### **emon32-poc** - Basic POC
- **Purpose**: Core energy monitoring functionality
- **Size**: ~4.4KB
- **Features**: Basic ADC sampling and energy calculation
- **Use Case**: Production firmware base

### **emon32-debug** - Debug Version  
- **Purpose**: Oscilloscope validation and timing analysis
- **Size**: ~4.8KB
- **Features**: Debug pin outputs for scope measurement
- **Pin Assignments** (Arduino Zero):
  - Pin 2 (PA14): ADC sampling timing
  - Pin 5 (PA15): Processing duration 
  - Pin 7 (PA21): Interrupt response timing
  - Pin 13 (PA17): Status LED
- **Use Case**: Hardware validation and performance tuning

### **emon32-rtic** - Real-Time Version
- **Purpose**: RTIC-based concurrent firmware  
- **Size**: ~5.8KB
- **Features**: Real-time task scheduling and resource sharing
- **Use Case**: Advanced real-time applications

### **emon32-rtic-debug** - RTIC + Debug
- **Purpose**: RTIC with oscilloscope validation
- **Size**: ~6.3KB  
- **Features**: RTIC tasks + debug pin outputs
- **Use Case**: Real-time system validation

## 🔍 **Troubleshooting Upload Issues**

### **Bootloader Not Detected**

**Problem**: `EMONBOOT` drive doesn't appear after double-press

**Solutions**:
```bash
# Check USB connection
lsusb | grep -i arduino

# Try different timing for double-press
# - Press RESET, wait 0.5s, press RESET again
# - Sometimes a triple-press works better

# Check for port availability  
ls /dev/ttyACM*  # Should show Arduino port

# Reset bootloader timeout
# The bootloader times out after ~15 seconds
# If missed, double-press RESET again
```

### **Upload Stalls or Fails**

**Problem**: File copy seems to hang or fails

**Solutions**:
```bash
# Ensure file size is reasonable
ls -la target/*.uf2  # Should be < 20KB for these firmwares

# Verify UF2 file integrity
python3 ../scripts/bin_to_uf2.py --verify target/emon32-debug.uf2

# Try copying via command line instead of GUI
cp target/emon32-debug.uf2 /media/$USER/EMONBOOT/
sync  # Force filesystem flush
```

### **Device Doesn't Reset After Upload**

**Problem**: Drive disappears but device seems unresponsive

**Solutions**:
```bash
# Manual reset
# Press RESET button once after upload

# Check serial output
# Connect to USB serial at 115200 baud
screen /dev/ttyACM0 115200
# OR
minicom -D /dev/ttyACM0 -b 115200

# Verify firmware is running
# Look for energy monitoring output or LED activity
```

### **Wrong Firmware Uploaded**

**Problem**: Uploaded wrong variant or corrupted firmware

**Solutions**:
```bash
# Re-enter bootloader mode
# Double-press RESET button again

# Upload correct firmware
cp target/emon32-poc.uf2 /media/$USER/EMONBOOT/

# For complete recovery, upload known-good firmware
# Or restore original Arduino bootloader/firmware if needed
```

## 🔧 **Advanced: Custom UF2 Generation**

### **Manual UF2 Creation**
```bash
# Convert any .bin file to UF2 format
python3 ../scripts/bin_to_uf2.py input.bin output.uf2 \
    --base 0x2000 \
    --family SAMD21 \
    --convert

# With custom family ID
python3 ../scripts/bin_to_uf2.py input.bin output.uf2 \
    --base 0x2000 \
    --family-id 0x68ED2B88
```

### **Memory Layout Considerations**
- **Bootloader**: 0x0000 - 0x1FFF (8KB reserved)
- **Application**: 0x2000 - 0x1FFFF (120KB available)  
- **EEPROM**: 0x20000000+ (separate address space)

The `--base 0x2000` parameter ensures firmware is loaded after the bootloader region.

### **Verification**
```bash
# Verify UF2 file structure
python3 ../scripts/bin_to_uf2.py --info target/emon32-poc.uf2

# Check that firmware size fits in available space
ls -la target/*.bin  # Should be < 120KB (122,880 bytes)
```

## 📊 **Firmware Size Analysis**

Current size usage on SAMD21J17A (128KB Flash):

| Firmware Variant | Binary Size | Flash Usage | Available Space |
|-------------------|-------------|-------------|-----------------|
| emon32-poc        | 4.4KB       | 3.4%        | 115.6KB        |
| emon32-debug      | 4.8KB       | 3.8%        | 115.2KB        |  
| emon32-rtic       | 5.8KB       | 4.5%        | 114.2KB        |
| emon32-rtic-debug | 6.3KB       | 4.9%        | 113.7KB        |

All variants leave plenty of space for additional features.

## ✅ **Verification Checklist**

After successful upload:

- [ ] **LED Activity**: Onboard LED should show normal pattern
- [ ] **USB Serial**: Connect at 115200 baud, expect energy monitoring output
- [ ] **Debug Pins** (debug versions): Verify scope signals on configured pins
- [ ] **Reset Behavior**: Manual RESET should restart firmware cleanly
- [ ] **Power Cycle**: Disconnect/reconnect USB should auto-start firmware

## 🆘 **Recovery Procedures**

### **Firmware Boot Failure**
If firmware doesn't start or device becomes unresponsive:

1. **Force Bootloader Mode**: 
   - Double-press RESET rapidly
   - Some boards support holding RESET while connecting USB

2. **Upload Known-Good Firmware**:
   ```bash
   # Use the basic POC version
   cp target/emon32-poc.uf2 /media/$USER/EMONBOOT/
   ```

3. **Factory Reset** (if available):
   - Some SAMD21 boards support factory reset procedures
   - Consult your board's documentation

### **Bootloader Corruption** (Rare)
If the bootloader itself is corrupted:

1. **JTAG/SWD Recovery**: Requires external programmer
2. **Arduino Zero Recovery**: May require Arduino IDE recovery mode  
3. **Professional Recovery**: Contact board manufacturer

For most use cases, the UF2 bootloader is very robust and self-recovering.

---

## 📚 **Related Documentation**

- [**Arduino Zero Validation Guide**](ARDUINO_ZERO_FINAL_GUIDE.md) - Complete testing procedures
- [**Oscilloscope Validation**](OSCILLOSCOPE_VALIDATION.md) - Hardware debugging with scopes  
- [**RTIC Integration Guide**](RTIC_INTEGRATION.md) - Real-time firmware details
- [**Performance Test Guide**](PERFORMANCE_TESTS_SUMMARY.md) - Validation and benchmarking

For questions or issues, refer to the main [README.md](README.md) or the specific validation guides.