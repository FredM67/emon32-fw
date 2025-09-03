# FTDI Serial Adapter Connection Guide

How to connect an FTDI USB-to-Serial adapter to Arduino Zero for monitoring emon32 UART output.

## 🔌 Why Use FTDI Adapter?

**Advantages:**
- **Simple wiring** - Just 3 connections (TX, RX, GND)
- **WSL-friendly** - Shows up as `/dev/ttyUSB0` in Linux
- **Isolated** - No USB conflicts with Arduino bootloader
- **Reliable** - Dedicated serial connection
- **Universal** - Works with any OS

**Use Cases:**
- Monitoring energy calculation output
- Debugging firmware in real-time
- Performance testing data collection
- WSL development workflow

## 📦 Hardware Requirements

### FTDI Adapter
- **FTDI FT232RL** or **FT234X** based adapter
- **3.3V/5V compatible** (Arduino Zero is 3.3V but 5V tolerant)
- **USB-A or USB-C** connector for your computer

### Connections Required
- **3 wires**: TX, RX, GND
- **Jumper wires** or breadboard connections
- **Arduino Zero** with emon32 firmware

## 🔧 Wiring Diagram

### Arduino Zero ↔ FTDI Adapter

```
Arduino Zero          FTDI Adapter
┌─────────────┐       ┌──────────────┐
│             │       │              │
│  Pin 2 (TX) ├───────┤ RX           │
│  Pin 5 (RX) ├───────┤ TX           │
│  GND        ├───────┤ GND          │
│             │       │              │
│  USB (PWR)  │       │ VCC (N/C)    │
└─────────────┘       └──────────────┘
                            │
                            └─── USB to Computer
```

### Pin Details

| Arduino Zero | SAMD21 Pin | FTDI Adapter | Function |
|--------------|------------|--------------|----------|
| Pin 2        | PA14       | RX           | TX Output from Arduino |
| Pin 5        | PA15       | TX           | RX Input to Arduino (unused) |
| GND          | GND        | GND          | Common Ground |
| -            | -          | VCC          | **Not Connected** |

**Important**: Don't connect FTDI VCC - Arduino Zero is powered via USB.

## 🛠️ Setup Steps

### Step 1: Hardware Connection

1. **Power off Arduino Zero** (disconnect USB)
2. **Connect wires**:
   - FTDI RX → Arduino Pin 2 (PA14/TX)
   - FTDI TX → Arduino Pin 5 (PA15/RX)  
   - FTDI GND → Arduino GND pin
3. **Leave FTDI VCC disconnected**
4. **Connect FTDI to computer** via USB
5. **Connect Arduino Zero** to computer via USB (for power)

### Step 2: Driver Installation

**Linux/WSL:**
```bash
# FTDI drivers usually included in kernel
# Check if detected:
lsusb | grep -i ftdi
# Should show: Future Technology Devices International, Ltd FT232 Serial

# Check device:
ls /dev/ttyUSB* 
# Should show: /dev/ttyUSB0 (or similar)
```

**Windows:**
- Download FTDI VCP drivers from ftdichip.com
- Or use Windows built-in drivers (usually auto-detected)
- Device appears as COM port in Device Manager

### Step 3: Terminal Setup

**Linux/WSL:**
```bash
# Install minicom
sudo apt update && sudo apt install minicom

# Connect to FTDI adapter
sudo minicom -D /dev/ttyUSB0 -b 115200

# Alternative: screen
screen /dev/ttyUSB0 115200

# Alternative: picocom  
picocom -b 115200 /dev/ttyUSB0
```

**Windows:**
- **PuTTY**: Serial mode, COM port, 115200 baud
- **TeraTerm**: Serial connection to COM port
- **Arduino IDE**: Tools → Serial Monitor
- **Windows Terminal**: PowerShell with serial commands

## 🚀 Testing Procedure

### Step 1: Build and Upload Firmware

```bash
cd /home/username/git/emon32-fw/rust-poc

# Build UART demo firmware
./build_uart_hardware.sh

# Upload to Arduino Zero:
# 1. Double-press RESET
# 2. Copy bin/emon32-uart-hardware.uf2 to EMONBOOT drive
```

### Step 2: Start Serial Monitor

```bash
# Linux/WSL
sudo minicom -D /dev/ttyUSB0 -b 115200

# Expected output every 1000ms:
# 1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W
# 2000 ms: V1=230.3V P1=148.7W P2=76.2W P3=0.0W
```

### Step 3: Verify Output

**Expected Serial Output:**
```
1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W
2000 ms: V1=230.3V P1=148.7W P2=76.2W P3=0.0W
3000 ms: V1=230.7V P1=151.1W P2=74.8W P3=0.0W
4000 ms: V1=230.1V P1=149.6W P2=75.5W P3=0.0W
```

**Data Format:**
- **Timestamp**: Milliseconds since boot
- **V1**: Voltage on channel 1 (simulated ~230V)
- **P1/P2/P3**: Power on CT channels 1, 2, 3 (simulated loads)

## 🚨 Troubleshooting

### No Serial Output

**Check Wiring:**
```bash
# Verify FTDI detection
lsusb | grep -i ftdi

# Check permissions
ls -l /dev/ttyUSB0
# Should show: crw-rw---- 1 root dialout

# Add user to dialout group if needed
sudo usermod -a -G dialout $USER
# Logout/login or restart WSL
```

**Check Firmware:**
- Verify Arduino Zero boots (LED should blink/pulse)
- Re-upload firmware if device seems stuck
- Try RTIC version: `bin/emon32-rtic-uart-hardware.uf2`

### Garbled Output

**Baud Rate Mismatch:**
```bash
# Ensure 115200 baud in terminal
minicom -D /dev/ttyUSB0 -b 115200

# Firmware uses 115200 baud (hardcoded)
```

**Wiring Issues:**
- Double-check TX/RX connections (TX→RX, RX→TX)
- Verify ground connection
- Check for loose connections

### Permission Denied

**Linux/WSL:**
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER

# Or run with sudo temporarily
sudo minicom -D /dev/ttyUSB0 -b 115200

# Check udev rules for FTDI
ls /etc/udev/rules.d/*ftdi*
```

### WSL-Specific Issues

**USB Device Sharing:**
```powershell
# Windows PowerShell (if needed for FTDI)
usbipd wsl list
usbipd attach --wsl --busid <BUSID>  # Only if FTDI not visible in WSL
```

**Usually not needed** - FTDI adapters typically work directly in WSL.

## 📋 Quick Reference

### Pin Connections
| Function | Arduino Zero | FTDI Adapter |
|----------|--------------|--------------|
| Serial TX | Pin 2 (PA14) | RX |
| Serial RX | Pin 5 (PA15) | TX |
| Ground | GND | GND |
| Power | USB | VCC (N/C) |

### Terminal Commands
```bash
# Linux/WSL
sudo minicom -D /dev/ttyUSB0 -b 115200
screen /dev/ttyUSB0 115200
picocom -b 115200 /dev/ttyUSB0

# Check device
ls /dev/ttyUSB*
lsusb | grep -i ftdi
```

### Windows Tools
- **PuTTY**: Serial, COM port, 115200 baud
- **Arduino IDE**: Tools → Serial Monitor  
- **TeraTerm**: File → New Connection → Serial

## 🎯 Performance Testing with FTDI

**For qfplib performance tests:**

1. **Connect FTDI** as described above
2. **Build performance firmware**:
   ```bash
   ./build_qfplib_performance.sh
   ```
3. **Upload standard version**: `bin/emon32-performance-standard.uf2`
4. **Capture output** via FTDI serial
5. **Upload qfplib version**: `bin/emon32-qfplib-performance.uf2`  
6. **Compare performance data**

**Advantage**: FTDI serial is independent of RTT/USB, providing reliable data capture for performance analysis.

This setup gives you a robust serial monitoring solution that works excellently with WSL and provides clean separation between firmware upload (Arduino USB) and data monitoring (FTDI serial)!