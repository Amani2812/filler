#!/bin/bash

# Validation script to check if the build is correct
# Run this before testing against game_engine

echo "================================"
echo "Filler AI - Build Validation"
echo "================================"
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

# Check 1: Rust installation
echo "Check 1: Rust Installation"
echo "----------------------------"
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✓${NC} Cargo found: $CARGO_VERSION"
else
    echo -e "${RED}✗${NC} Cargo not found!"
    echo "Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    ((ERRORS++))
fi
echo ""

# Check 2: Source files exist
echo "Check 2: Source Files"
echo "----------------------------"
if [ -f "Cargo.toml" ]; then
    echo -e "${GREEN}✓${NC} Cargo.toml exists"
else
    echo -e "${RED}✗${NC} Cargo.toml not found!"
    ((ERRORS++))
fi

if [ -f "src/main.rs" ]; then
    echo -e "${GREEN}✓${NC} src/main.rs exists"
    LINES=$(wc -l < src/main.rs)
    echo "  Lines of code: $LINES"
else
    echo -e "${RED}✗${NC} src/main.rs not found!"
    ((ERRORS++))
fi
echo ""

# Check 3: Compilation
echo "Check 3: Compilation Test"
echo "----------------------------"
echo "Attempting to compile..."

if cargo build --release 2>&1 | tee /tmp/build_output.txt; then
    echo -e "${GREEN}✓${NC} Compilation successful!"
    
    # Check for warnings
    if grep -q "warning:" /tmp/build_output.txt; then
        echo -e "${YELLOW}⚠${NC} Compilation warnings found:"
        grep "warning:" /tmp/build_output.txt | head -5
        ((WARNINGS++))
    else
        echo -e "${GREEN}✓${NC} No compilation warnings"
    fi
else
    echo -e "${RED}✗${NC} Compilation failed!"
    echo "Check the errors above and fix them."
    ((ERRORS++))
fi
echo ""

# Check 4: Binary exists
echo "Check 4: Binary Output"
echo "----------------------------"
if [ -f "target/release/filler" ]; then
    echo -e "${GREEN}✓${NC} Binary created: target/release/filler"
    
    SIZE=$(du -h target/release/filler | cut -f1)
    echo "  Binary size: $SIZE"
    
    if [ -x "target/release/filler" ]; then
        echo -e "${GREEN}✓${NC} Binary is executable"
    else
        echo -e "${YELLOW}⚠${NC} Binary is not executable"
        echo "  Run: chmod +x target/release/filler"
        ((WARNINGS++))
    fi
else
    echo -e "${RED}✗${NC} Binary not found!"
    ((ERRORS++))
fi
echo ""

# Check 5: Clippy (optional but recommended)
echo "Check 5: Code Quality (Clippy)"
echo "----------------------------"
if command -v cargo-clippy &> /dev/null || cargo clippy --version &> /dev/null; then
    echo "Running clippy..."
    if cargo clippy --release 2>&1 | tee /tmp/clippy_output.txt; then
        if grep -q "warning:" /tmp/clippy_output.txt; then
            echo -e "${YELLOW}⚠${NC} Clippy found some suggestions:"
            grep "warning:" /tmp/clippy_output.txt | head -5
            ((WARNINGS++))
        else
            echo -e "${GREEN}✓${NC} No clippy warnings"
        fi
    fi
else
    echo -e "${YELLOW}⚠${NC} Clippy not installed (optional)"
    echo "  Install: rustup component add clippy"
fi
echo ""

# Check 6: Format check
echo "Check 6: Code Formatting"
echo "----------------------------"
if command -v rustfmt &> /dev/null || cargo fmt --version &> /dev/null; then
    echo "Checking code format..."
    if cargo fmt -- --check &> /dev/null; then
        echo -e "${GREEN}✓${NC} Code is properly formatted"
    else
        echo -e "${YELLOW}⚠${NC} Code formatting issues found"
        echo "  Run: cargo fmt"
        ((WARNINGS++))
    fi
else
    echo -e "${YELLOW}⚠${NC} rustfmt not installed (optional)"
fi
echo ""

# Check 7: Documentation files
echo "Check 7: Documentation"
echo "----------------------------"
DOCS=("README.md" "USAGE.md" "QUICK_START.md")
for doc in "${DOCS[@]}"; do
    if [ -f "$doc" ]; then
        echo -e "${GREEN}✓${NC} $doc exists"
    else
        echo -e "${YELLOW}⚠${NC} $doc not found"
        ((WARNINGS++))
    fi
done
echo ""

# Check 8: Test scripts
echo "Check 8: Test Scripts"
echo "----------------------------"
SCRIPTS=("build.sh" "test.sh" "test_input_parser.sh")
for script in "${SCRIPTS[@]}"; do
    if [ -f "$script" ]; then
        echo -e "${GREEN}✓${NC} $script exists"
        if [ -x "$script" ]; then
            echo "  (executable)"
        else
            echo -e "  ${YELLOW}⚠${NC} Not executable - run: chmod +x $script"
            ((WARNINGS++))
        fi
    else
        echo -e "${YELLOW}⚠${NC} $script not found"
        ((WARNINGS++))
    fi
done
echo ""

# Summary
echo "================================"
echo "Validation Summary"
echo "================================"
echo -e "Errors: ${RED}$ERRORS${NC}"
echo -e "Warnings: ${YELLOW}$WARNINGS${NC}"
echo ""

if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ Build validation PASSED!${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Copy binary: cp target/release/filler ./filler"
    echo "2. Test input parsing: ./test_input_parser.sh"
    echo "3. Run in Docker with game_engine"
    echo ""
    exit 0
else
    echo -e "${RED}✗ Build validation FAILED!${NC}"
    echo "Please fix the errors above before proceeding."
    echo ""
    exit 1
fi

# Cleanup
rm -f /tmp/build_output.txt /tmp/clippy_output.txt
