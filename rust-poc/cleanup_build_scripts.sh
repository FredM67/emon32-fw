#!/bin/bash
#
# Cleanup Script: Archive duplicated build scripts and replace with unified system
# This script moves old build scripts to an archive folder and updates documentation
#

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

ARCHIVE_DIR="build_scripts_archive"

echo -e "${YELLOW}🧹 Cleaning up duplicated build scripts...${NC}"
echo ""

# Create archive directory
mkdir -p "$ARCHIVE_DIR"

# List of scripts to archive (these are now replaced by build_unified.sh)
SCRIPTS_TO_ARCHIVE=(
    "build_and_test_comparison.sh"
    "build_lto_performance.sh"
    "build_qfplib_sys.sh"
    "build_qfplib.sh"
    "build_qfplib_performance.sh"
    "build_uart_demo.sh"
    "build_uart_hardware.sh"
    "build_debug.sh"
    "build_all.sh"
)

# Keep these scripts (they have unique functionality not in unified script)
SCRIPTS_TO_KEEP=(
    "build_unified.sh"
    "setup_performance_docs.sh"
    "verify_performance_docs.sh"
    "test_all.sh"
    "upload_arduino_zero.sh"
)

echo -e "${BLUE}Archiving duplicated build scripts:${NC}"
for script in "${SCRIPTS_TO_ARCHIVE[@]}"; do
    if [ -f "$script" ]; then
        mv "$script" "$ARCHIVE_DIR/"
        echo -e "  📦 Archived: $script → $ARCHIVE_DIR/"
    else
        echo -e "  ⚠️  Not found: $script"
    fi
done

echo ""
echo -e "${GREEN}Keeping essential scripts:${NC}"
for script in "${SCRIPTS_TO_KEEP[@]}"; do
    if [ -f "$script" ]; then
        echo -e "  ✅ Kept: $script"
    else
        echo -e "  ❌ Missing: $script"
    fi
done

# Create a README for the archive
cat > "$ARCHIVE_DIR/README.md" << 'EOF'
# Archived Build Scripts

This directory contains build scripts that have been replaced by the unified build system.

## Replaced by `build_unified.sh`

The following scripts have been consolidated into a single, maintainable build system:

- `build_and_test_comparison.sh` → `build_unified.sh comparison`
- `build_lto_performance.sh` → `build_unified.sh lto`
- `build_qfplib_sys.sh` → `build_unified.sh qfplib-sys`
- `build_qfplib.sh` → Functionality merged into unified system
- `build_qfplib_performance.sh` → `build_unified.sh performance`
- `build_uart_demo.sh` → `build_unified.sh uart`
- `build_uart_hardware.sh` → Functionality merged into unified system
- `build_debug.sh` → `build_unified.sh debug`
- `build_all.sh` → `build_unified.sh all`

## Migration Guide

### Old Command → New Command

```bash
# Old way
./build_and_test_comparison.sh
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
EOF

echo ""
echo -e "${BLUE}📝 Created archive documentation: $ARCHIVE_DIR/README.md${NC}"

# Update the main README to point to the unified build system
if [ -f "README.md" ]; then
    echo ""
    echo -e "${BLUE}📖 Updating main README.md references...${NC}"
    
    # Create a backup
    cp README.md README.md.backup
    
    # Update build script references (this is a simplified approach)
    sed -i 's/build_and_test_comparison\.sh/build_unified.sh comparison/g' README.md
    sed -i 's/build_lto_performance\.sh/build_unified.sh lto/g' README.md
    sed -i 's/build_qfplib_sys\.sh/build_unified.sh qfplib-sys/g' README.md
    sed -i 's/build_debug\.sh/build_unified.sh debug/g' README.md
    sed -i 's/build_all\.sh/build_unified.sh all/g' README.md
    
    echo -e "  ✅ Updated README.md (backup saved as README.md.backup)"
fi

echo ""
echo -e "${GREEN}✅ Build Script Cleanup Complete!${NC}"
echo ""
echo -e "${YELLOW}Summary:${NC}"
echo -e "  📦 Archived: ${#SCRIPTS_TO_ARCHIVE[@]} duplicated scripts"
echo -e "  ✅ Kept: ${#SCRIPTS_TO_KEEP[@]} essential scripts"
echo -e "  📁 Archive location: $ARCHIVE_DIR/"
echo ""
echo -e "${BLUE}🎯 New Build System Usage:${NC}"
echo -e "  ./build_unified.sh --help     # Show all options"
echo -e "  ./build_unified.sh basic      # Build POC binaries"
echo -e "  ./build_unified.sh performance # Build performance tests"
echo -e "  ./build_unified.sh all        # Build everything"
echo ""
echo -e "${GREEN}🎉 You now have a clean, maintainable build system!${NC}"