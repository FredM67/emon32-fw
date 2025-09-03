#!/bin/bash
#
# Binary Cleanup Script: Organize and clean up generated firmware binaries
# Keeps the latest, working binaries and archives old/experimental versions
#

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

BIN_DIR="bin"
ARCHIVE_DIR="$BIN_DIR/archive"
OLD_ARCHIVE_DIR="$BIN_DIR/old_builds"

echo -e "${YELLOW}🧹 Cleaning up firmware binaries...${NC}"
echo ""

# Create archive directories
mkdir -p "$ARCHIVE_DIR"
mkdir -p "$OLD_ARCHIVE_DIR"

# Current working binaries (keep these)
CURRENT_BINARIES=(
    "emon32-poc.uf2"
    "emon32-rtic.uf2"
    "emon32-performance-micromath.uf2"
    "emon32-performance-qfplib.uf2"
    "emon32-qfplib-debug.uf2"
    "emon32-qfplib-complex.uf2"
    "emon32-qfplib-sys-lto.uf2"
    "emon32-micromath-sys-lto.uf2"
    "emon32-uart-demo.uf2"
    "emon32-uart-hardware.uf2"
    "emon32-rtic-uart-demo.uf2"
    "emon32-rtic-uart-hardware.uf2"
)

# Keep the corresponding ELF files too
CURRENT_ELFS=(
    "emon32-poc.elf"
    "emon32-rtic.elf"
    "emon32-performance-micromath.elf"
    "emon32-performance-qfplib.elf"
    "emon32-qfplib-debug.elf"
    "emon32-qfplib-complex.elf"
    "emon32-qfplib-sys-lto.elf"
    "emon32-micromath-sys-lto.elf"
)

# Archive patterns for old/experimental versions
ARCHIVE_PATTERNS=(
    "*-v2.*"
    "*-v3.*"
    "*-v4.*"
    "*-final.*"
    "*-fixed.*"
    "*-complete.*"
    "*-debug.*"
    "*-bare-minimum.*"
    "*-ultra-minimal.*"
    "*-rtt-fix.*"
    "*hybrid*"
    "*-test.*"
    "*-qfplib-lto.*"
    "*-micromath-lto.*"
)

echo -e "${BLUE}Current working binaries (keeping):${NC}"
for binary in "${CURRENT_BINARIES[@]}"; do
    if [ -f "$BIN_DIR/$binary" ]; then
        echo -e "  ✅ $binary"
    else
        echo -e "  ⚠️  Missing: $binary"
    fi
done

echo ""
echo -e "${BLUE}Current working ELF files (keeping):${NC}"
for elf in "${CURRENT_ELFS[@]}"; do
    if [ -f "$BIN_DIR/$elf" ]; then
        echo -e "  ✅ $elf"
    else
        echo -e "  ⚠️  Missing: $elf"
    fi
done

echo ""
echo -e "${YELLOW}Archiving old/experimental binaries:${NC}"

# Archive old/experimental versions
archived_count=0
cd "$BIN_DIR"

# Archive files matching patterns
for pattern in "${ARCHIVE_PATTERNS[@]#*/}"; do  # Remove BIN_DIR prefix
    for file in $pattern; do
        if [ -f "$file" ]; then
            # Skip if this is one of our current binaries
            skip=false
            for current in "${CURRENT_BINARIES[@]}" "${CURRENT_ELFS[@]}"; do
                if [ "$file" = "$current" ]; then
                    skip=true
                    break
                fi
            done
            
            if [ "$skip" = false ]; then
                mv "$file" "old_builds/"
                echo -e "  📦 Archived: $file → old_builds/"
                archived_count=$((archived_count + 1))
            fi
        fi
    done
done

cd ..

# Archive remaining .bin files (we mainly use .uf2 now)
echo ""
echo -e "${YELLOW}Archiving .bin files (replaced by .uf2):${NC}"
bin_count=0
cd "$BIN_DIR"
for file in *.bin; do
    if [ -f "$file" ]; then
        mv "$file" "archive/"
        echo -e "  📦 Archived: $file → archive/"
        bin_count=$((bin_count + 1))
    fi
done
cd ..

# Create archive documentation
cat > "$OLD_ARCHIVE_DIR/README.md" << 'EOF'
# Archived Firmware Binaries

This directory contains old and experimental firmware binaries from the development process.

## Binary Types

- `*-v2.*`, `*-v3.*`, `*-v4.*` - Iterative development versions
- `*-final.*`, `*-fixed.*`, `*-complete.*` - Development milestones
- `*-debug.*`, `*-bare-minimum.*`, `*-ultra-minimal.*` - Debug/test variants
- `*-rtt-fix.*` - RTT debugging fixes
- `*hybrid*` - Hybrid math library experiments
- `*-test.*` - Test binaries
- `*-lto.*` - LTO optimization experiments (superseded by qfplib-sys)

## Current Working Binaries

The current, working binaries are in the parent `bin/` directory:

- `emon32-poc.uf2` - Basic proof-of-concept
- `emon32-rtic.uf2` - RTIC-based implementation
- `emon32-performance-micromath.uf2` - micromath performance test
- `emon32-performance-qfplib.uf2` - qfplib performance test
- `emon32-qfplib-debug.uf2` - qfplib integration debug test
- `emon32-qfplib-complex.uf2` - qfplib complex math test
- `emon32-qfplib-sys-lto.uf2` - Latest LTO-optimized qfplib-sys
- `emon32-micromath-sys-lto.uf2` - LTO-optimized micromath baseline
- `emon32-uart-*.uf2` - UART demo variants

## Recovery

If you need any of these archived binaries, they are preserved with their original names and can be copied back to the main bin/ directory.

Archived: $(date)
EOF

cat > "$ARCHIVE_DIR/README.md" << 'EOF'
# Archived .bin Files

This directory contains .bin format firmware files. These have been replaced by .uf2 files for easier Arduino Zero uploading.

## UF2 vs BIN

- `.uf2` files can be drag-and-dropped to EMONBOOT drive
- `.bin` files require manual flashing tools

## Current Usage

Use the .uf2 files in the parent `bin/` directory for all firmware uploads.

Archived: $(date)
EOF

echo ""
echo -e "${GREEN}✅ Binary cleanup complete!${NC}"
echo ""
echo -e "${YELLOW}Summary:${NC}"
echo -e "  ✅ Current binaries: ${#CURRENT_BINARIES[@]} .uf2 files"
echo -e "  ✅ Current ELF files: ${#CURRENT_ELFS[@]} files"
echo -e "  📦 Archived old builds: $archived_count files → old_builds/"
echo -e "  📦 Archived .bin files: $bin_count files → archive/"
echo ""
echo -e "${BLUE}Current firmware ready for use:${NC}"
echo -e "  🎯 Basic: emon32-poc.uf2, emon32-rtic.uf2"
echo -e "  🔬 Performance: emon32-performance-micromath.uf2, emon32-performance-qfplib.uf2"
echo -e "  🧪 Tests: emon32-qfplib-debug.uf2, emon32-qfplib-complex.uf2"
echo -e "  🚀 Latest: emon32-qfplib-sys-lto.uf2, emon32-micromath-sys-lto.uf2"
echo -e "  📡 UART: emon32-uart-demo.uf2, emon32-uart-hardware.uf2"
echo ""
echo -e "${GREEN}🎉 Clean, organized firmware ready for testing!${NC}"