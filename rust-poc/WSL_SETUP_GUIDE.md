# WSL Arduino Zero Setup Guide

Complete guide for using Arduino Zero with emon32 Rust firmware in Windows Subsystem for Linux (WSL).

# WSL Arduino Zero Setup Guide

Complete guide for using Arduino Zero with emon32 Rust firmware in Windows Subsystem for Linux (WSL).

## 🎯 Overview

WSL doesn't have direct USB access by default. This guide shows how to:
- Share Arduino Zero USB device with WSL
- Upload firmware (.uf2 files) to Arduino Zero
- Monitor RTT/serial output
- Troubleshoot common issues

## 📁 File Structure Note

**Important**: Different tools use different file locations:
- **probe-run** uses raw binaries: `target/thumbv6m-none-eabi/release/emon32-performance`
- **UF2 upload** uses processed files: `bin/emon32-performance-standard.uf2`

The build scripts rename binaries when creating UF2 files for clarity.

## 🔧 Prerequisites

### Windows Host Requirements

1. **Windows 10/11** with WSL2 installed
2. **usbipd-win** for USB device sharing
3. **Windows Terminal** (recommended)

### WSL Requirements

1. **Ubuntu/Debian** WSL distribution
2. **Rust toolchain** with ARM target
3. **probe-run** for RTT monitoring

## 📦 Installation Steps

### Step 1: Install usbipd-win on Windows

```powershell
# Run in Windows PowerShell as Administrator
winget install usbipd
```

### Step 2: Connect Arduino Zero

1. **Connect Arduino Zero** to Windows host via USB
2. **Install Arduino IDE drivers** (if not already installed)
3. **Verify device recognition** in Device Manager

### Step 3: Share USB Device with WSL

```powershell
# In Windows PowerShell as Administrator

# 1. List all USB devices
usbipd list

# Example output:
# BUSID  VID:PID    DEVICE
# 2-5    2341:804b  Arduino Zero

# 2. Bind the device (required first step)
usbipd bind --busid 2-5

# 3. Attach Arduino Zero to WSL (replace 2-5 with your BUSID)
usbipd attach --wsl --busid 2-5

# 4. Verify attachment
usbipd list
# Should show "Attached - Ubuntu" in the STATE column
```

### Step 4: Verify in WSL

```bash
# Check USB device is visible
lsusb | grep -i arduino
# Expected: Bus 001 Device 003: ID 2341:804b Arduino SA Arduino Zero

# Check serial device (when in normal mode)
ls /dev/ttyACM* 2>/dev/null || echo "No serial devices found"

# Check dmesg for USB events
dmesg | tail -10 | grep -i usb
```

## 📁 Firmware Upload Methods

### Method 1: Windows Explorer (Recommended)

**Easiest approach - use Windows host for file operations:**

1. **Build firmware in WSL:**
   ```bash
   cd /home/username/git/emon32-fw/rust-poc
   ./build_qfplib_performance.sh
   ```

2. **Navigate to WSL files from Windows:**
   - Open Windows Explorer
   - Go to: `\\wsl$\Ubuntu\home\username\git\emon32-fw\rust-poc\bin\`
   - You should see .uf2 files

3. **Upload firmware:**
   - Double-press RESET on Arduino Zero
   - EMONBOOT drive appears in Windows
   - Drag/copy .uf2 file to EMONBOOT drive
   - Device automatically reboots

### Method 2: WSL Drive Mount

**If EMONBOOT auto-mounts in WSL:**

```bash
# Check if drive auto-mounted
ls /mnt/*/EMONBOOT/ 2>/dev/null

# If found, copy directly
cp bin/emon32-performance-standard.uf2 /mnt/*/EMONBOOT/
```

### Method 3: Manual WSL Mount

**Advanced - manual drive mounting:**

```bash
# 1. Create mount point
sudo mkdir -p /mnt/emonboot

# 2. Find Windows drive letter (usually E:, F:, etc.)
# Check in Windows Explorer when EMONBOOT appears

# 3. Mount drive (replace E: with actual drive letter)
sudo mount -t drvfs E: /mnt/emonboot

# 4. Copy firmware
cp bin/emon32-performance-standard.uf2 /mnt/emonboot/

# 5. Unmount when done
sudo umount /mnt/emonboot
```

## 🔍 Monitoring Firmware Output

### RTT (Real-Time Transfer)

**Option 1: WSL probe-run (if USB attached)**
```bash
# Ensure Arduino Zero is attached to WSL
# Use the raw binary from target/ folder (not the renamed UF2)
probe-run --chip ATSAMD21J17A target/thumbv6m-none-eabi/release/emon32-performance
```

**Option 2: Windows probe-run**
```powershell
# Install probe-run on Windows
cargo install probe-run

# Detach from WSL first
usbipd detach --wsl --busid 2-5

# Run from Windows (using WSL path)
# Use the raw binary from target/ folder (not the renamed UF2)
probe-run --chip ATSAMD21J17A \\wsl$\Ubuntu\home\username\git\emon32-fw\rust-poc\target\thumbv6m-none-eabi\release\emon32-performance
```

### Serial UART

**If using UART demos:**
```bash
# WSL (if USB attached)
sudo minicom -D /dev/ttyACM0 -b 115200

# Windows alternatives:
# - PuTTY
# - Arduino IDE Serial Monitor
# - Windows Terminal with serial connection
```

### FTDI Serial Adapter (Recommended for WSL) 🌟

**Simplest approach for reliable serial monitoring:**

```bash
# Connect FTDI adapter to Arduino Zero:
# FTDI RX → Arduino Pin 2 (TX)
# FTDI TX → Arduino Pin 5 (RX)  
# FTDI GND → Arduino GND

# Monitor in WSL (no USB sharing needed):
sudo minicom -D /dev/ttyUSB0 -b 115200

# Expected output:
# 1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W
```

**Advantages:**
- No USB device sharing required
- Works directly in WSL
- Independent of Arduino USB connection
- Reliable for performance testing

📖 **Detailed FTDI setup**: See `FTDI_CONNECTION_GUIDE.md`

## 🚨 Troubleshooting

### Arduino Zero Not Detected

**Issue**: `lsusb` doesn't show Arduino Zero

**Solutions**:
```powershell
# 1. Verify Windows detection
# Device Manager → Ports (COM & LPT) → Should show Arduino Zero

# 2. Reinstall/update drivers
# Download Arduino IDE and install drivers

# 3. Try different USB cable/port

# 4. Detach and reattach in WSL
usbipd detach --wsl --busid 2-5
usbipd attach --wsl --busid 2-5
```

### EMONBOOT Drive Not Mounting

**Issue**: Double-press RESET doesn't show EMONBOOT drive

**Solutions**:
1. **Verify bootloader mode:**
   - LED should pulse slowly
   - New USB device appears in Device Manager

2. **Check USB attachment:**
   ```powershell
   # If attached to WSL, detach first
   usbipd detach --wsl --busid 2-5
   ```

3. **Try different timing:**
   - Press and release RESET quickly twice
   - Wait 1-2 seconds between presses

### RTT Connection Issues

**Issue**: `probe-run` can't connect

**Solutions**:
1. **Verify USB attachment:**
   ```bash
   lsusb | grep -i arduino
   ls /dev/ttyACM*
   ```

2. **Try Windows probe-run:**
   - Detach from WSL
   - Use Windows-installed probe-run

3. **Check firmware state:**
   - Re-upload firmware if device stuck
   - Verify correct binary is flashed

### Permission Issues

**Issue**: USB device access denied

**Solutions**:
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER

# Logout and login to apply changes
# Or restart WSL:
wsl --shutdown
# Restart WSL from Windows
```

## 📋 Quick Reference Commands

### Windows PowerShell (Administrator)
```powershell
# List USB devices
usbipd list

# Bind device (required first step)
usbipd bind --busid 2-5

# Attach Arduino Zero to WSL
usbipd attach --wsl --busid 2-5

# Detach from WSL
usbipd detach --wsl --busid 2-5
```

### WSL Commands
```bash
# Verify Arduino Zero
lsusb | grep -i arduino

# Check serial devices  
ls /dev/ttyACM*

# Build firmware
./build_qfplib_performance.sh

# Monitor RTT
probe-run --chip ATSAMD21J17A target/thumbv6m-none-eabi/release/emon32-performance
```

### File Paths
- **WSL Source**: `/home/username/git/emon32-fw/rust-poc/bin/`
- **Windows Access**: `\\wsl$\Ubuntu\home\username\git\emon32-fw\rust-poc\bin\`
- **EMONBOOT Drive**: `E:\` (or F:, G:, etc. - varies by system)

## 🎯 Recommended Workflow

1. **Development in WSL**: Build, test, modify code
2. **File transfer via Windows**: Use Explorer to copy .uf2 files
3. **Monitoring flexible**: RTT from WSL or Windows as needed
4. **USB management**: Attach to WSL only when needed for RTT

This hybrid approach provides the best of both environments!