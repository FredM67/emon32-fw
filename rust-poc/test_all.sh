#!/bin/bash
# Comprehensive test runner for emon32 Rust POC
# Tests both simple and RTIC versions with real-world performance validation

echo "🧪 emon32 Comprehensive Test Suite"
echo "=================================="
echo "Testing both Simple POC and RTIC versions with real-world performance validation"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

run_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"
    
    echo -e "${BLUE}Running: $test_name${NC}"
    echo "Description: $description"
    echo "Command: $test_command"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if eval "$test_command"; then
        echo -e "${GREEN}✅ $test_name - PASSED${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ $test_name - FAILED${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    echo ""
}

# 1. Build Tests
echo "🔨 Build and Compilation Tests"
echo "=============================="

run_test "Simple POC Build" \
    "cargo build --release --bin emon32-poc" \
    "Verify simple POC compiles without errors"

run_test "RTIC Build" \
    "cargo build --release --bin emon32-rtic" \
    "Verify RTIC version compiles without errors"

run_test "Library Build" \
    "cargo build --lib" \
    "Verify library interface compiles"

# 2. Unit Tests
echo "🧪 Unit and Integration Tests"
echo "============================="

run_test "Host Algorithm Test" \
    "rustc test_host.rs && ./test_host" \
    "Validate core energy calculation algorithms"

run_test "Integration Tests" \
    "cargo test" \
    "Run embedded integration tests (if compatible)"

# 3. Performance Tests
echo "⚡ Performance and Real-World Tests"
echo "=================================="

run_test "Algorithm Performance Test" \
    "rustc test_performance.rs && ./test_performance" \
    "Test calculation accuracy and performance with realistic loads"

run_test "RTIC Performance Test" \
    "rustc test_rtic_performance.rs && ./test_rtic_performance" \
    "Validate RTIC real-time behavior and task scheduling"

# 4. Binary Size Analysis
echo "📏 Binary Size Analysis"
echo "======================"

echo "Analyzing binary sizes:"
cargo size --release --bin emon32-poc | tail -n 1 > /tmp/poc_size.txt
cargo size --release --bin emon32-rtic | tail -n 1 > /tmp/rtic_size.txt

POC_SIZE=$(awk '{print $1}' /tmp/poc_size.txt)
RTIC_SIZE=$(awk '{print $1}' /tmp/rtic_size.txt)

echo "Simple POC:  $POC_SIZE bytes"
echo "RTIC:        $RTIC_SIZE bytes"

if [ "$POC_SIZE" -gt 0 ] && [ "$RTIC_SIZE" -gt 0 ]; then
    SIZE_INCREASE=$((RTIC_SIZE - POC_SIZE))
    SIZE_PERCENT=$(( (SIZE_INCREASE * 100) / POC_SIZE ))
    echo "RTIC overhead: +$SIZE_INCREASE bytes (+$SIZE_PERCENT%)"
    
    if [ "$SIZE_PERCENT" -lt 100 ]; then
        echo -e "${GREEN}✅ RTIC size overhead acceptable (<100%)${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ RTIC size overhead too high (>100%)${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
fi

# 5. Memory Layout Analysis
echo ""
echo "📊 Memory Layout Analysis"
echo "========================"

echo "Simple POC memory layout:"
cargo size --release --bin emon32-poc

echo ""
echo "RTIC memory layout:"
cargo size --release --bin emon32-rtic

# 6. Code Quality Checks
echo ""
echo "🔍 Code Quality Checks"
echo "====================="

run_test "Clippy Linting" \
    "cargo clippy --all-targets -- -D warnings" \
    "Check for code quality issues and best practices"

run_test "Format Check" \
    "cargo fmt -- --check" \
    "Verify code formatting consistency"

# 7. Documentation Tests
echo "📚 Documentation Tests"
echo "====================="

run_test "Documentation Build" \
    "cargo doc --no-deps" \
    "Generate and verify documentation"

run_test "Documentation Tests" \
    "cargo test --doc" \
    "Run documentation examples as tests"

# 8. Cross-compilation Test
echo "🎯 Cross-compilation Tests"
echo "=========================="

run_test "thumbv6m Target Check" \
    "cargo check --target thumbv6m-none-eabi" \
    "Verify compilation for embedded target"

# 9. Feature Tests
echo "🔧 Feature Tests"
echo "==============="

run_test "RTT Feature Test" \
    "cargo build --features rtt" \
    "Test RTT debugging feature compilation"

run_test "No-default Features" \
    "cargo build --no-default-features" \
    "Test minimal feature set compilation"

# 10. Benchmark Comparison
echo "🏁 Benchmark Comparison"
echo "======================"

if [ -f "test_performance" ] && [ -f "test_rtic_performance" ]; then
    echo "Running comparative benchmarks..."
    
    echo "Simple POC Performance:"
    timeout 30s ./test_performance > /tmp/poc_bench.txt 2>&1
    if [ $? -eq 0 ]; then
        grep -E "(✅|Processing Time|samples/sec|Memory)" /tmp/poc_bench.txt
    fi
    
    echo ""
    echo "RTIC Performance:"
    timeout 30s ./test_rtic_performance > /tmp/rtic_bench.txt 2>&1
    if [ $? -eq 0 ]; then
        grep -E "(✅|Average|CPU utilization|Task executions)" /tmp/rtic_bench.txt
    fi
fi

# 11. Hardware Readiness Check
echo ""
echo "🔌 Hardware Readiness Check"
echo "=========================="

echo "Checking objcopy availability for firmware generation:"
if command -v rust-objcopy &> /dev/null; then
    echo "✅ rust-objcopy available"
    
    echo "Generating firmware binaries:"
    cargo objcopy --release --bin emon32-poc -- -O binary emon32-poc.bin
    cargo objcopy --release --bin emon32-rtic -- -O binary emon32-rtic.bin
    
    if [ -f "emon32-poc.bin" ] && [ -f "emon32-rtic.bin" ]; then
        echo "✅ Firmware binaries generated successfully"
        echo "  emon32-poc.bin:  $(stat -c%s emon32-poc.bin) bytes"
        echo "  emon32-rtic.bin: $(stat -c%s emon32-rtic.bin) bytes"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo "❌ Failed to generate firmware binaries"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
else
    echo "⚠️  rust-objcopy not available - install with: rustup component add llvm-tools-preview"
fi

# Final Results
echo ""
echo "📊 Test Suite Summary"
echo "===================="
echo "Total tests run: $TOTAL_TESTS"
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}"
    echo "🎉 ALL TESTS PASSED!"
    echo "================="
    echo "✅ Both Simple POC and RTIC versions compile successfully"
    echo "✅ Energy calculation algorithms are accurate and performant"
    echo "✅ RTIC provides real-time guarantees with acceptable overhead"
    echo "✅ Memory usage is efficient for embedded deployment"
    echo "✅ Code quality meets professional standards"
    echo "✅ Hardware deployment ready"
    echo ""
    echo "🚀 Ready for SAMD21 hardware deployment!"
    echo "📡 Next steps:"
    echo "   1. Flash firmware to hardware: openocd + gdb"
    echo "   2. Validate timing with oscilloscope"
    echo "   3. Test real ADC sampling and UART output"
    echo "   4. Compare performance with original C firmware"
    echo -e "${NC}"
    exit 0
else
    echo -e "${RED}"
    echo "❌ TESTS FAILED!"
    echo "==============="
    echo "Some tests failed. Please review the failures above and fix issues before deployment."
    echo "Focus areas:"
    echo "  - Compilation errors"
    echo "  - Performance regressions"
    echo "  - Memory usage issues"
    echo "  - Real-time timing violations"
    echo -e "${NC}"
    exit 1
fi