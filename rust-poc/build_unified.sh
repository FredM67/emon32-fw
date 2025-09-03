#!/bin/bash
#
# Unified Build System for emon32 Rust POC
# Consolidates all build functionality into a single, maintainable script
#

set -e

# Configuration
TARGET="thumbv6m-none-eabi"
UF2_SCRIPT="../scripts/bin_to_uf2.py"
LINKER="../linker/samd21j17.ld"
BIN_DIR="bin"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build function - common cargo build and UF2 conversion
build_binary() {
    local binary_name="$1"
    local features="$2"
    local profile="${3:-release}"
    local output_name="${4:-$binary_name}"
    
    echo -e "${BLUE}Building $binary_name with features: $features, profile: $profile${NC}"
    
    if [ "$profile" = "release" ]; then
        cargo build --bin "$binary_name" --target "$TARGET" --features "$features" --release
        local target_path="target/$TARGET/release/$binary_name"
    else
        cargo build --bin "$binary_name" --target "$TARGET" --features "$features" --profile "$profile"
        local target_path="target/$TARGET/$profile/$binary_name"
    fi
    
    # Copy ELF
    cp "$target_path" "$BIN_DIR/${output_name}.elf"
    
    # Convert to UF2
    python3 "$UF2_SCRIPT" "$BIN_DIR/${output_name}.elf" "$BIN_DIR/${output_name}.uf2" --linker "$LINKER"
    
    echo -e "${GREEN}  ✅ Created $BIN_DIR/${output_name}.uf2${NC}"
}

# Build function for development binaries (no UF2 conversion needed)
build_dev_binary() {
    local binary_name="$1"
    local features="$2"
    local profile="${3:-release}"
    
    echo -e "${BLUE}Building $binary_name (dev mode)${NC}"
    
    if [ "$profile" = "release" ]; then
        cargo build --bin "$binary_name" --features "$features" --release
    else
        cargo build --bin "$binary_name" --features "$features" --profile "$profile"
    fi
}

# Show usage information
show_usage() {
    echo -e "${YELLOW}emon32 Unified Build System${NC}"
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  basic           Build basic POC binaries (poc, rtic)"
    echo "  debug           Build debug variants"
    echo "  uart            Build UART demo variants"
    echo "  performance     Build performance test suite"
    echo "  lto             Build LTO-optimized performance tests"
    echo "  qfplib-sys      Build with qfplib-sys crate (latest)"
    echo "  comparison      Build comparison test suite"
    echo "  all             Build everything (warning: takes time!)"
    echo "  clean           Clean all build artifacts"
    echo ""
    echo "Options:"
    echo "  --clean         Clean before building"
    echo "  --host          Build host tests (for development)"
    echo "  --yes           Skip confirmation prompts (for automation)"
    echo ""
    echo "Examples:"
    echo "  $0 basic                    # Build POC and RTIC binaries"
    echo "  $0 performance --clean      # Clean build performance tests"
    echo "  $0 qfplib-sys              # Build latest qfplib-sys tests"
    echo "  $0 all --yes               # Build everything without prompts"
    echo ""
}

# Clean build artifacts
clean_build() {
    echo -e "${YELLOW}🧹 Cleaning build artifacts...${NC}"
    cargo clean
    echo -e "${GREEN}✅ Clean complete${NC}"
}

# Build basic POC binaries
build_basic() {
    echo -e "${YELLOW}🔧 Building Basic POC Binaries${NC}"
    echo "================================"
    
    build_binary "emon32-poc" "" "release" "emon32-poc"
    build_binary "emon32-rtic" "" "release" "emon32-rtic"
    
    echo -e "${GREEN}✅ Basic binaries ready in $BIN_DIR/${NC}"
}

# Build debug variants
build_debug() {
    echo -e "${YELLOW}🔧 Building Debug Variants${NC}"
    echo "=========================="
    
    build_binary "emon32-debug" "" "release" "emon32-debug"
    build_binary "emon32-rtic-debug" "" "release" "emon32-rtic-debug"
    
    echo -e "${GREEN}✅ Debug binaries ready in $BIN_DIR/${NC}"
}

# Build UART demo variants
build_uart() {
    echo -e "${YELLOW}🔧 Building UART Demo Variants${NC}"
    echo "=============================="
    
    build_binary "emon32-uart" "rtt" "release" "emon32-uart-demo"
    build_binary "emon32-rtic-uart" "rtt" "release" "emon32-rtic-uart-demo"
    build_binary "emon32-uart-hardware" "" "release" "emon32-uart-hardware"
    build_binary "emon32-rtic-uart-hardware" "" "release" "emon32-rtic-uart-hardware"
    
    echo -e "${GREEN}✅ UART demo binaries ready in $BIN_DIR/${NC}"
}

# Build performance test suite
build_performance() {
    echo -e "${YELLOW}🔧 Building Performance Test Suite${NC}"
    echo "=================================="
    
    build_binary "emon32-performance" "rtt" "release" "emon32-performance-micromath"
    build_binary "emon32-qfplib-performance" "rtt,qfplib" "release" "emon32-performance-qfplib"
    build_binary "emon32-qfplib-debug" "rtt,qfplib" "release" "emon32-qfplib-debug"
    build_binary "emon32-qfplib-complex" "rtt,qfplib" "release" "emon32-qfplib-complex"
    
    echo -e "${GREEN}✅ Performance test binaries ready in $BIN_DIR/${NC}"
}

# Build LTO-optimized performance tests
build_lto() {
    echo -e "${YELLOW}🔧 Building LTO-Optimized Performance Tests${NC}"
    echo "==========================================="
    
    # Clean for fresh LTO compilation
    echo "Cleaning for fresh LTO compilation..."
    cargo clean
    
    build_binary "emon32-qfplib-performance" "rtt,qfplib" "lto-max" "emon32-qfplib-lto"
    build_binary "emon32-performance" "rtt" "lto-max" "emon32-micromath-lto"
    build_binary "emon32-hybrid-performance" "rtt,qfplib" "lto-max" "emon32-hybrid-lto"
    
    echo -e "${GREEN}✅ LTO-optimized binaries ready in $BIN_DIR/${NC}"
    echo -e "${BLUE}📋 LTO Testing: Upload each .uf2 file to compare LTO effectiveness${NC}"
}

# Build with qfplib-sys crate (latest architecture)
build_qfplib_sys() {
    echo -e "${YELLOW}🔧 Building qfplib-sys LTO Performance Tests${NC}"
    echo "============================================"
    
    # Clean for fresh LTO compilation with new crate structure
    echo "Cleaning for fresh qfplib-sys compilation..."
    cargo clean
    
    # Set environment for maximum LTO optimization
    export QFPLIB_LTO_LEVEL=aggressive
    
    build_binary "emon32-qfplib-performance" "rtt,qfplib" "lto-max" "emon32-qfplib-sys-lto"
    build_binary "emon32-performance" "rtt" "lto-max" "emon32-micromath-sys-lto"
    
    # Build size-optimized variant
    export QFPLIB_LTO_LEVEL=size
    if grep -q \"lto-size\" Cargo.toml; then
        build_binary "emon32-qfplib-performance" "rtt,qfplib" "lto-size" "emon32-qfplib-sys-size"
    else
        echo -e "${YELLOW}⚠️  lto-size profile not found, skipping size-optimized build${NC}"
    fi
    
    echo -e "${GREEN}✅ qfplib-sys binaries ready in $BIN_DIR/${NC}"
    echo -e "${BLUE}📋 These use the latest modular qfplib-sys crate with zero-cost FFI${NC}"
}

# Build comparison test suite
build_comparison() {
    echo -e "${YELLOW}🔧 Building Comparison Test Suite${NC}"
    echo "================================="
    
    build_binary "emon32-qfplib-debug" "rtt,qfplib" "release" "emon32-qfplib-debug"
    build_binary "emon32-performance" "rtt" "release" "emon32-performance-micromath"
    build_binary "emon32-qfplib-performance" "rtt,qfplib" "release" "emon32-performance-qfplib"
    build_binary "emon32-qfplib-complex" "rtt,qfplib" "release" "emon32-qfplib-complex"
    
    # Also create .bin files for some tests
    arm-none-eabi-objcopy -O binary "$BIN_DIR/emon32-performance-qfplib.elf" "$BIN_DIR/emon32-performance-qfplib.bin"
    
    echo -e "${GREEN}✅ Comparison test binaries ready in $BIN_DIR/${NC}"
    echo -e "${BLUE}📋 Test sequence: debug → micromath → qfplib → complex${NC}"
}

# Build host tests (for development)
build_host() {
    echo -e "${YELLOW}🔧 Building Host Tests${NC}"
    echo "====================="
    
    cargo build --bin test_host
    cargo test
    
    echo -e "${GREEN}✅ Host tests complete${NC}"
}

# Build everything
build_all() {
    echo -e "${RED}⚠️  Building ALL binaries - this will take significant time!${NC}"
    if [ "$AUTO_YES" != "true" ]; then
        read -p "Continue? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Cancelled."
            exit 0
        fi
    else
        echo "Auto-proceeding with --yes option..."
    fi
    
    build_basic
    echo ""
    build_debug
    echo ""
    build_uart
    echo ""
    build_performance
    echo ""
    build_lto
    echo ""
    build_qfplib_sys
    echo ""
    
    echo -e "${GREEN}🎉 ALL BINARIES COMPLETE!${NC}"
    echo -e "${BLUE}📁 Check $BIN_DIR/ for all .uf2 files${NC}"
}

# Create bin directory if it doesn't exist
mkdir -p "$BIN_DIR"

# Parse command line arguments
CLEAN_FIRST=false
BUILD_HOST=false
AUTO_YES=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN_FIRST=true
            shift
            ;;
        --host)
            BUILD_HOST=true
            shift
            ;;
        --yes)
            AUTO_YES=true
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        basic|debug|uart|performance|lto|qfplib-sys|comparison|all|clean)
            COMMAND="$1"
            shift
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            show_usage
            exit 1
            ;;
    esac
done

# Execute the requested command
case "${COMMAND:-}" in
    basic)
        build_basic
        ;;
    debug)
        build_debug
        ;;
    uart)
        build_uart
        ;;
    performance)
        build_performance
        ;;
    lto)
        build_lto
        ;;
    qfplib-sys)
        build_qfplib_sys
        ;;
    comparison)
        build_comparison
        ;;
    all)
        build_all
        ;;
    clean)
        clean_build
        ;;
    "")
        echo -e "${YELLOW}No command specified.${NC}"
        show_usage
        exit 0
        ;;
    *)
        echo -e "${RED}Error: Unknown command $COMMAND${NC}"
        show_usage
        exit 1
        ;;
esac

# Build host tests if requested
if [ "$BUILD_HOST" = true ]; then
    echo ""
    build_host
fi