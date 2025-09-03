#!/bin/bash

# Performance Documentation Verification Script
# Checks that performance test documentation is complete and consistent

# Note: Not using 'set -e' to allow proper error counting

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}📋 Performance Documentation Verification${NC}"
echo "==========================================="

ERRORS=0
WARNINGS=0

# Function to check file exists
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} Found: $1"
    else
        echo -e "${RED}✗${NC} Missing: $1"
        ((ERRORS++))
    fi
}

# Function to check for placeholders in file
check_placeholders() {
    local file="$1"
    local placeholder_count
    
    if [ -f "$file" ]; then
        placeholder_count=$(grep -c "\\[.*\\]" "$file" 2>/dev/null || true)
        if [ "$placeholder_count" -gt 0 ]; then
            echo -e "${YELLOW}⚠${NC} $file has $placeholder_count unfilled placeholders"
            grep -n "\\[.*\\]" "$file" | head -5
            ((WARNINGS++))
        else
            echo -e "${GREEN}✓${NC} $file placeholders filled"
        fi
    fi
}

# Function to check for RTT data files
check_rtt_data() {
    if [ -d "performance_data" ]; then
        echo -e "${GREEN}✓${NC} performance_data/ directory exists"
        
        if [ -f "performance_data/standard_math_output.txt" ]; then
            echo -e "${GREEN}✓${NC} Standard math RTT data found"
        else
            echo -e "${YELLOW}⚠${NC} Missing: performance_data/standard_math_output.txt"
            ((WARNINGS++))
        fi
        
        if [ -f "performance_data/qfplib_output.txt" ]; then
            echo -e "${GREEN}✓${NC} qfplib RTT data found"
        else
            echo -e "${YELLOW}⚠${NC} Missing: performance_data/qfplib_output.txt"
            ((WARNINGS++))
        fi
    else
        echo -e "${YELLOW}⚠${NC} performance_data/ directory not found"
        ((WARNINGS++))
    fi
}

echo ""
echo -e "${BLUE}Checking Core Documentation Files:${NC}"
check_file "PERFORMANCE_TESTING_GUIDE.md"
check_file "PERFORMANCE_RESULTS_TEMPLATE.md"
check_file "PERFORMANCE_DOCUMENTATION_GUIDE.md"
check_file "QFPLIB_INTEGRATION_COMPLETE.md"
check_file "WSL_SETUP_GUIDE.md"
check_file "FTDI_CONNECTION_GUIDE.md"

echo ""
echo -e "${BLUE}Checking Performance Test Binaries:${NC}"
check_file "bin/emon32-performance-standard.uf2"
check_file "bin/emon32-qfplib-performance.uf2"
check_file "build_qfplib_performance.sh"

echo ""
echo -e "${BLUE}Checking for Results Files:${NC}"
RESULTS_FILES=$(ls PERFORMANCE_RESULTS_*.md 2>/dev/null | grep -v TEMPLATE || true)
if [ -n "$RESULTS_FILES" ]; then
    for file in $RESULTS_FILES; do
        echo -e "${GREEN}✓${NC} Found results file: $file"
        check_placeholders "$file"
    done
else
    echo -e "${YELLOW}⚠${NC} No performance results files found (PERFORMANCE_RESULTS_*.md)"
    echo "    Run: ./setup_performance_docs.sh to create one"
    ((WARNINGS++))
fi

echo ""
echo -e "${BLUE}Checking RTT Data Files:${NC}"
check_rtt_data

echo ""
echo -e "${BLUE}Checking Project Documentation Updates:${NC}"

# Check if README mentions performance testing
if grep -q "Performance Testing" README.md; then
    echo -e "${GREEN}✓${NC} README.md includes performance testing section"
else
    echo -e "${YELLOW}⚠${NC} README.md missing performance testing section"
    ((WARNINGS++))
fi

# Check if PROGRESS_SUMMARY mentions performance docs
if grep -q "Performance Results Template" PROGRESS_SUMMARY.md; then
    echo -e "${GREEN}✓${NC} PROGRESS_SUMMARY.md updated with performance documentation"
else
    echo -e "${YELLOW}⚠${NC} PROGRESS_SUMMARY.md not updated with performance documentation"
    ((WARNINGS++))
fi

echo ""
echo -e "${BLUE}Build System Check:${NC}"

# Check if performance build script is executable
if [ -x "build_qfplib_performance.sh" ]; then
    echo -e "${GREEN}✓${NC} build_qfplib_performance.sh is executable"
else
    echo -e "${RED}✗${NC} build_qfplib_performance.sh is not executable"
    echo "    Fix with: chmod +x build_qfplib_performance.sh"
    ((ERRORS++))
fi

# Check if setup script is executable
if [ -x "setup_performance_docs.sh" ]; then
    echo -e "${GREEN}✓${NC} setup_performance_docs.sh is executable"
else
    echo -e "${RED}✗${NC} setup_performance_docs.sh is not executable"
    echo "    Fix with: chmod +x setup_performance_docs.sh"
    ((ERRORS++))
fi

echo ""
echo "=========================================="
echo -e "${BLUE}Verification Summary:${NC}"

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ All checks passed! Documentation is complete.${NC}"
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠ $WARNINGS warnings found. Documentation is mostly complete.${NC}"
    echo "   These are typically missing data files that will be filled after hardware testing."
else
    echo -e "${RED}❌ $ERRORS errors and $WARNINGS warnings found.${NC}"
    echo "   Fix the errors before proceeding with performance testing."
fi

if [ $WARNINGS -gt 0 ] || [ $ERRORS -gt 0 ]; then
    echo ""
    echo -e "${BLUE}Recommendations:${NC}"
    if [ $WARNINGS -gt 0 ]; then
        echo "• Run hardware tests and fill in the RTT data"
        echo "• Complete any unfilled placeholders in results files"
        echo "• Run: ./setup_performance_docs.sh to create results template"
    fi
    if [ $ERRORS -gt 0 ]; then
        echo "• Fix missing files and permissions"
        echo "• Ensure build scripts are properly set up"
    fi
fi

# Exit with 0 if only warnings, 1 if errors
if [ $ERRORS -eq 0 ]; then
    exit 0
else
    exit 1
fi