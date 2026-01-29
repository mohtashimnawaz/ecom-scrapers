#!/bin/bash
# Quick validation script - runs basic checks without full test suite
# Usage: ./test_quick.sh

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}Quick Test Validation${NC}"
echo ""

echo -e "${YELLOW}1. Checking code format...${NC}"
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Code formatting OK${NC}"
else
    echo -e "${RED}✗ Run: cargo fmt${NC}"
    exit 1
fi
echo ""

echo -e "${YELLOW}2. Running clippy...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings 2>&1 | grep -q "warning\|error"; then
    echo -e "${RED}✗ Clippy found issues${NC}"
    cargo clippy --all-targets --all-features
    exit 1
else
    echo -e "${GREEN}✓ Clippy passed${NC}"
fi
echo ""

echo -e "${YELLOW}3. Compiling...${NC}"
if cargo build --release; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi
echo ""

echo -e "${YELLOW}4. Running unit tests...${NC}"
if cargo test --lib; then
    echo -e "${GREEN}✓ Unit tests passed${NC}"
else
    echo -e "${RED}✗ Unit tests failed${NC}"
    exit 1
fi
echo ""

echo -e "${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}All quick checks passed! ✓${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Run './test.sh' for full test suite${NC}"
