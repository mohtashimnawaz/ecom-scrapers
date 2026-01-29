#!/bin/bash

# Browser Extension Testing Script
# Tests all extension functionality

echo "🧩 Browser Extension Testing Script"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if extension directory exists
if [ ! -d "browser-extension" ]; then
    echo -e "${RED}✗ browser-extension/ directory not found${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Extension directory found${NC}"

# Check required files
echo ""
echo "📋 Checking required files..."
echo ""

required_files=(
    "manifest.json"
    "content.js"
    "content.css"
    "background.js"
    "popup.html"
    "popup.js"
    "popup.css"
    "README.md"
    "INSTALL.md"
)

missing_files=0

for file in "${required_files[@]}"; do
    if [ -f "browser-extension/$file" ]; then
        echo -e "${GREEN}✓${NC} $file"
    else
        echo -e "${RED}✗${NC} $file (missing)"
        missing_files=$((missing_files + 1))
    fi
done

# Check icons
echo ""
echo "🎨 Checking icons..."
echo ""

icon_sizes=(16 32 48 128)
missing_icons=0

for size in "${icon_sizes[@]}"; do
    if [ -f "browser-extension/icons/icon${size}.png" ]; then
        echo -e "${GREEN}✓${NC} icon${size}.png"
    else
        echo -e "${YELLOW}⚠${NC} icon${size}.png (missing - needs generation)"
        missing_icons=$((missing_icons + 1))
    fi
done

# Check manifest.json syntax
echo ""
echo "🔍 Validating manifest.json..."
echo ""

if command -v jq &> /dev/null; then
    if jq empty browser-extension/manifest.json 2>/dev/null; then
        echo -e "${GREEN}✓ manifest.json is valid JSON${NC}"
    else
        echo -e "${RED}✗ manifest.json has syntax errors${NC}"
        missing_files=$((missing_files + 1))
    fi
else
    echo -e "${YELLOW}⚠ jq not installed, skipping JSON validation${NC}"
    echo "  Install with: brew install jq"
fi

# Check file sizes
echo ""
echo "📊 File statistics..."
echo ""

total_lines=$(find browser-extension -type f \( -name "*.js" -o -name "*.html" -o -name "*.css" -o -name "*.json" \) | xargs wc -l | tail -1 | awk '{print $1}')
echo "Total lines of code: $total_lines"

file_count=$(find browser-extension -type f | wc -l | tr -d ' ')
echo "Total files: $file_count"

# Check if backend is running
echo ""
echo "🔌 Checking backend API..."
echo ""

if curl -s http://localhost:3000/health > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Backend is running on http://localhost:3000${NC}"
else
    echo -e "${YELLOW}⚠ Backend not running${NC}"
    echo "  Start with: cargo run"
fi

# Summary
echo ""
echo "=================================="
echo "📋 Summary"
echo "=================================="
echo ""

if [ $missing_files -eq 0 ]; then
    echo -e "${GREEN}✓ All required files present${NC}"
else
    echo -e "${RED}✗ $missing_files required file(s) missing${NC}"
fi

if [ $missing_icons -eq 0 ]; then
    echo -e "${GREEN}✓ All icon sizes generated${NC}"
else
    echo -e "${YELLOW}⚠ $missing_icons icon(s) need generation${NC}"
    echo ""
    echo "Generate icons with:"
    echo "  brew install imagemagick"
    echo "  cd browser-extension/icons"
    echo "  for size in 16 32 48 128; do"
    echo "    convert icon128.svg -resize \${size}x\${size} icon\${size}.png"
    echo "  done"
fi

echo ""
echo "=================================="
echo "🚀 Next Steps"
echo "=================================="
echo ""
echo "1. Install Extension:"
echo "   Chrome: chrome://extensions/ → Load unpacked → Select browser-extension/"
echo "   Firefox: about:debugging → Load Temporary Add-on → Select manifest.json"
echo ""
echo "2. Test on Product Pages:"
echo "   • https://www.myntra.com/..."
echo "   • https://www.flipkart.com/..."
echo "   • https://www.ajio.com/..."
echo "   • https://www.tatacliq.com/..."
echo ""
echo "3. Look for 'Track This Price' button on product pages"
echo ""
echo "4. Click extension icon to manage alerts"
echo ""
echo "=================================="
echo ""

# Exit code
if [ $missing_files -gt 0 ]; then
    exit 1
else
    exit 0
fi
