#!/bin/bash

# Performance Results Documentation Helper
# This script helps set up documentation for qfplib performance test results

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Performance Results Documentation Helper${NC}"
echo "=============================================="

# Get current date
DATE=$(date +%Y-%m-%d)

# Create results file
RESULTS_FILE="PERFORMANCE_RESULTS_${DATE}.md"

if [ -f "$RESULTS_FILE" ]; then
    echo -e "${YELLOW}Results file already exists: $RESULTS_FILE${NC}"
    read -p "Overwrite? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

echo -e "${YELLOW}Creating results file: $RESULTS_FILE${NC}"
cp PERFORMANCE_RESULTS_TEMPLATE.md "$RESULTS_FILE"

# Replace date placeholder
sed -i "s/\\[DATE\\]/$DATE/g" "$RESULTS_FILE"

# Create raw data directory
mkdir -p performance_data

echo -e "${GREEN}✓ Created performance results file: $RESULTS_FILE${NC}"
echo -e "${GREEN}✓ Created performance_data/ directory${NC}"

echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo "1. Deploy firmware to Arduino Zero:"
echo "   - emon32-performance-standard.uf2"
echo "   - emon32-qfplib-performance.uf2"
echo ""
echo "2. Collect RTT output:"
echo "   probe-rs rtt attach"
echo ""
echo "3. Save RTT output to:"
echo "   - performance_data/standard_math_output.txt"
echo "   - performance_data/qfplib_output.txt"
echo ""
echo "4. Fill out the results in: $RESULTS_FILE"
echo ""
echo "5. Run documentation verification:"
echo "   ./verify_performance_docs.sh"
echo ""
echo -e "${YELLOW}See PERFORMANCE_DOCUMENTATION_GUIDE.md for detailed instructions${NC}"