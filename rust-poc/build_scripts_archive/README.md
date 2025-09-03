# Archived Build Scripts

This directory contains build scripts that have been replaced by the unified build system.

## Replaced by `build_unified.sh`

The following scripts have been consolidated into a single, maintainable build system:

- `build.sh` → `build_unified.sh basic` (legacy basic build script)
- `build_and_test_comparison.sh` → `build_unified.sh comparison`
- `build_lto_performance.sh` → `build_unified.sh lto`
- `build_qfplib_sys.sh` → `build_unified.sh qfplib-sys`
- `build_qfplib.sh` → Functionality merged into unified system
- `build_qfplib_performance.sh` → `build_unified.sh performance`
- `build_uart_demo.sh` → `build_unified.sh uart`
- `build_uart_hardware.sh` → Functionality merged into unified system
- `build_debug.sh` → `build_unified.sh debug`
- `build_all.sh` → `build_unified.sh all`

## Replaced by `qfplib-sys` crate

- `build.rs.old` → Replaced by dedicated `qfplib-sys/build.rs` crate

## Migration Guide

### Old Command → New Command

### Legacy `build.sh` vs Modern `build_unified.sh basic`

The old `build.sh` script provided basic functionality:
- Built only `emon32-poc` binary
- Generated .bin file (not UF2)
- Basic error checking
- Size information

The new `build_unified.sh basic` provides:
- Builds both `emon32-poc` and `emon32-rtic` binaries
- Generates UF2 files for Arduino Zero bootloader
- Comprehensive error handling and colored output
- Consistent interface with all other build targets
- Built-in help and documentation

```bash
# Old way
./build.sh

# New way  
./build_unified.sh basic
```
# New way
./build_unified.sh comparison

# Old way
./build_lto_performance.sh
# New way
./build_unified.sh lto

# Old way
./build_qfplib_sys.sh
# New way
./build_unified.sh qfplib-sys

# Old way
./build_debug.sh
# New way
./build_unified.sh debug

# Old way
./build_all.sh
# New way
./build_unified.sh all
```

### Advantages of Unified System

1. **Single Maintenance Point**: Only one script to update for build changes
2. **Consistent Interface**: All builds use the same command structure
3. **Better Documentation**: Built-in help and usage examples
4. **Reduced Duplication**: Eliminates ~80% of duplicated code
5. **Extensible**: Easy to add new build targets without creating new files

### Recovery

If you need to restore any of these scripts, they are preserved in this archive directory with their original functionality intact.

Created: $(date)
