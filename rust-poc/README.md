# Rust Embedded Project for emon32

This is a proof-of-concept implementation of the emon32 energy monitoring firmware in Rust.

> **🎯 Project Status**: See [PROJECT_STATUS.md](./PROJECT_STATUS.md) for complete overview and current status.

## Features

- Real-time ADC sampling with timer interrupts
- Energy calculation with power, RMS, and energy accumulation
- UART debug output
- Event-driven architecture using RTIC
- Memory-safe embedded programming

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

Connect to the device via USB serial at 115200 baud to see energy monitoring data:

```
1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W
2000 ms: V1=231.1V P1=152.3W P2=73.8W P3=5.2W
...
```

## Code Structure

- `src/main.rs` - Main application with RTIC tasks
- `src/board/` - Board support package (pin definitions, constants)
- `src/energy/` - Energy calculation algorithms
- `memory.x` - Memory layout for SAMD21J17A
- `Cargo.toml` - Project dependencies and configuration

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