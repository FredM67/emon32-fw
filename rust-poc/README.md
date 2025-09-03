# Rust Embedded Project for emon32

This is a proof-of-concept implementation of the emon32 energy monitoring firmware in Rust.

> **🎯 Project Status**: See [PROJECT_STATUS.md](./PROJECT_STATUS.md) for complete overview and current status.

## Features

- Real-time ADC sampling with timer interrupts
- Energy calculation with power, RMS, and energy accumulation
- **Hardware UART output** at 115200 baud (Arduino Zero pins 2/5)
- **Production-ready serial communication** with structured data format
- Event-driven architecture using RTIC
- Memory-safe embedded programming
- **UF2 bootloader compatibility** for easy deployment

## Hardware Target

- Microchip SAMD21J17A (ARM Cortex-M0+)
- 128KB Flash, 16KB RAM
- Compatible with emonPi3 hardware

## Building

1. Install Rust embedded toolchain:
```bash
rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
```

2. Build the project:
```bash
cd rust-poc
cargo build --release
```

3. Create binary for flashing:
```bash
cargo objcopy --release -- -O binary target/thumbv6m-none-eabi/release/emon32-poc.bin
```

## Flashing

For detailed upload instructions, see [**📦 FIRMWARE_UPLOAD_GUIDE.md**](FIRMWARE_UPLOAD_GUIDE.md)

### Quick Start (Arduino Zero)

> ⚠️ **Arduino Zero Users**: Standard Arduino Zero boards do NOT have UF2 bootloader!
> If double-pressing RESET doesn't show `EMONBOOT` drive, see Method 2 in the upload guide.

```bash
# Build firmware
./build_debug.sh

# Option A: UF2 (if UF2 bootloader installed)
# Double-press RESET, drag target/emon32-debug.uf2 to EMONBOOT drive

# Option B: Standard Arduino (most Arduino Zero boards)
# See FIRMWARE_UPLOAD_GUIDE.md Method 2 for Arduino IDE upload
```

### UF2 Bootloader Method (emonPi3 Hardware)

> ⚠️ **Arduino Zero Users**: This method only works if you have UF2 bootloader installed!
> Standard Arduino Zero boards ship with different bootloader - see upload guide Method 2.

The firmware uses the same UF2 bootloader system as the original emonPi3 C firmware:

1. **Enter bootloader**: Double-press the RESET button quickly
2. **Upload firmware**: Copy/drag the `.uf2` file to the `EMONBOOT` drive  
3. **Auto-reset**: Device automatically resets and runs the new firmware

The build scripts automatically generate both `.bin` and `.uf2` files for convenience.

### Standard Arduino Zero Upload

For standard Arduino Zero boards (most common), use Arduino IDE or avrdude:

```bash
# Convert to .hex format
arm-none-eabi-objcopy -I binary -O ihex --change-addresses 0x2000 \
  target/emon32-debug.bin target/emon32-debug.hex

# Upload via Arduino IDE or avrdude (see upload guide for details)
```

## Serial Output

The system now supports **hardware UART output** on Arduino Zero pins:

### Hardware UART (Production Ready)
- **Pins**: TX=Pin 2 (PA14), RX=Pin 5 (PA15)
- **Baud Rate**: 115200, 8N1
- **Output**: Real-time energy monitoring data

```
emon32 Rust Energy Monitor v0.1.0
Hardware UART Output at 115200 baud
Connected on PA14(TX)/PA15(RX) - Arduino Zero pins 2/5
Format: timestamp ms: V1=voltage P1=power P2=power P3=power
Ready...

1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W
2000 ms: V1=231.1V P1=152.3W P2=73.8W P3=5.2W
...
```

> **📋 Implementation Status**: Hardware UART output is now **fully implemented**!  
> See [**HARDWARE_UART_INTEGRATION_COMPLETE.md**](HARDWARE_UART_INTEGRATION_COMPLETE.md) for complete details.

### Build and Deploy

```bash
# Build hardware UART firmware
./build_uart_hardware.sh

# Generates:
# - emon32-uart-hardware.uf2 (Simple hardware demo)  
# - emon32-rtic-uart-hardware.uf2 (RTIC hardware demo)

# Deploy to Arduino Zero:
# 1. Double-press reset → EMONBOOT drive appears
# 2. Copy .uf2 file to EMONBOOT drive  
# 3. Connect serial terminal at 115200 baud to pins 2(TX)/5(RX)

# RTT demo versions also available:
./build_uart_demo.sh  # For development without serial connections
```

## Code Structure

- `src/main.rs` - Main application with RTIC tasks
- `src/board/` - Board support package (pin definitions, constants)
- `src/energy/` - Energy calculation algorithms
- `memory.x` - Memory layout for SAMD21J17A
- `Cargo.toml` - Project dependencies and configuration

## Performance Testing

### Host-based Tests

Run comprehensive accuracy and performance tests on your development machine:

```bash
# Run all tests including performance benchmarks
cargo test

# Run performance tests with detailed output
cargo test --release -- --nocapture performance
```

### ARM Hardware Performance Tests

For real-world ARM Cortex-M0+ performance validation on Arduino Zero:

```bash
# Build ARM performance test firmware
cargo build --bin emon32-performance --target thumbv6m-none-eabi --release --features rtt

# Generate UF2 file for Arduino Zero deployment
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/emon32-performance emon32-performance.bin
python3 ../scripts/bin_to_uf2.py emon32-performance.bin emon32-performance.uf2
```

### qfplib Optimization Performance Tests

Compare standard floating-point math vs qfplib optimized ARM assembly:

```bash
# Build both standard and qfplib optimized versions
./build_qfplib_performance.sh

# This generates:
# - emon32-performance-standard.uf2 (micromath baseline)
# - emon32-qfplib-performance.uf2 (qfplib optimized)

# Deploy to Arduino Zero and use RTT to collect performance data
# Install RTT tool: cargo install probe-run
probe-run --chip ATSAMD21J17A target/thumbv6m-none-eabi/release/emon32-qfplib-performance
```

**Performance Documentation:**
- [`PERFORMANCE_TESTING_GUIDE.md`](./PERFORMANCE_TESTING_GUIDE.md) - Detailed hardware testing procedures
- [`PERFORMANCE_RESULTS_TEMPLATE.md`](./PERFORMANCE_RESULTS_TEMPLATE.md) - Template for documenting test results
- [`QFPLIB_INTEGRATION_COMPLETE.md`](./QFPLIB_INTEGRATION_COMPLETE.md) - qfplib integration technical details

**Expected qfplib Performance Improvements:**\n- Square root operations: 2-3x faster (critical for RMS calculations)\n- Division operations: 2-4x faster (power efficiency calculations)\n- Overall energy calculations: 2-3x improvement\n\n### Documentation Workflow\n\nTo document your performance test results:\n\n```bash\n# 1. Set up documentation files\n./setup_performance_docs.sh\n\n# 2. Run hardware tests (see PERFORMANCE_TESTING_GUIDE.md)\n# 3. Fill in results template with actual measurements\n# 4. Verify documentation completeness\n./verify_performance_docs.sh\n```

## Comparison with C Version

This Rust POC demonstrates:

✅ **Memory Safety**: No buffer overflows or null pointer dereferences  
✅ **Type Safety**: Compile-time prevention of many bugs  
✅ **Real-time**: Interrupt-driven architecture similar to C version  
✅ **Performance**: Zero-cost abstractions, efficient code generation  
✅ **Maintainability**: Clear module structure and error handling  

## Next Steps

To complete the migration:

1. Implement DMA-based ADC sampling
2. Add RFM69 radio communication
3. Port configuration system
4. Add USB CDC device support
5. Implement EEPROM wear leveling
6. Add comprehensive testing