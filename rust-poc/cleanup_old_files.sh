#!/bin/bash
#
# Cleanup script for .old, .backup, and similar temporary files
# Moves them to appropriate archive directories
#

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🧹 Cleaning up .old and backup files...${NC}"
echo ""

# Create archive directory if needed
mkdir -p build_scripts_archive

# File patterns to clean up
BACKUP_PATTERNS=(
    "*.old"
    "*.bak" 
    "*.orig"
    "*.save"
    "*.backup"
)

# Special cases that go to specific directories
BUILD_SCRIPT_PATTERNS=(
    "build*.old"
    "Cargo.toml.old"
    "*.rs.old"
)

cleaned_count=0

echo -e "${BLUE}Looking for backup/old files...${NC}"

# Handle build-related .old files → build_scripts_archive
for pattern in "${BUILD_SCRIPT_PATTERNS[@]}"; do
    for file in $pattern; do
        if [ -f "$file" ]; then
            mv "$file" "build_scripts_archive/"
            echo -e "  📦 Moved build file: $file → build_scripts_archive/"
            cleaned_count=$((cleaned_count + 1))
        fi
    done
done

# Handle general backup files
for pattern in "${BACKUP_PATTERNS[@]}"; do
    for file in $pattern; do
        if [ -f "$file" ] && [[ ! "$file" =~ ^build ]]; then
            # Create a general backup archive if needed
            mkdir -p old_files_archive
            mv "$file" "old_files_archive/"
            echo -e "  📦 Moved backup file: $file → old_files_archive/"
            cleaned_count=$((cleaned_count + 1))
        fi
    done
done

# Look for .old files in subdirectories
echo ""
echo -e "${BLUE}Checking subdirectories...${NC}"

find . -name "*.old" -type f | while read -r file; do
    if [ -f "$file" ]; then
        dir=$(dirname "$file")
        filename=$(basename "$file")
        
        # Move to appropriate archive based on location
        if [[ "$dir" == *"src"* ]] || [[ "$filename" == *".rs.old" ]]; then
            mkdir -p src_archive
            mv "$file" "src_archive/"
            echo -e "  📦 Moved source file: $file → src_archive/"
        else
            mkdir -p old_files_archive
            mv "$file" "old_files_archive/"
            echo -e "  📦 Moved file: $file → old_files_archive/"
        fi
        cleaned_count=$((cleaned_count + 1))
    fi
done

echo ""
if [ $cleaned_count -eq 0 ]; then
    echo -e "${GREEN}✅ No .old or backup files found - project is clean!${NC}"
else
    echo -e "${GREEN}✅ Cleaned up $cleaned_count backup/old files${NC}"
    echo ""
    echo -e "${BLUE}Archive directories created:${NC}"
    for dir in build_scripts_archive old_files_archive src_archive; do
        if [ -d "$dir" ]; then
            file_count=$(find "$dir" -type f | wc -l)
            echo -e "  📁 $dir/ - $file_count files"
        fi
    done
fi

echo ""
echo -e "${GREEN}🎉 Backup file cleanup complete!${NC}"