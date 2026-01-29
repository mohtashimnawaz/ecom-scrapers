# Browser Extension Installation Guide 🚀

## Prerequisites
- Chrome/Chromium browser OR Firefox
- Backend API running on `http://localhost:3000`

## Quick Start

### Step 1: Generate Icons (One-time setup)

**Option A: Using ImageMagick** (Recommended)
```bash
brew install imagemagick
cd browser-extension/icons
for size in 16 32 48 128; do
  convert icon128.svg -resize ${size}x${size} icon${size}.png
done
```

**Option B: Online Tool**
1. Go to https://redketchup.io/icon-converter
2. Upload `browser-extension/icons/icon128.svg`
3. Generate PNG icons: 16x16, 32x32, 48x48, 128x128
4. Save all PNGs in `browser-extension/icons/`

### Step 2: Install Extension

#### Chrome/Edge
1. Open browser and go to: `chrome://extensions/`
2. Enable **Developer mode** (toggle in top-right corner)
3. Click **Load unpacked**
4. Navigate to and select: `/path/to/ecom-scrapers/browser-extension/`
5. Extension appears in toolbar! 🎉

#### Firefox
1. Open Firefox and go to: `about:debugging#/runtime/this-firefox`
2. Click **Load Temporary Add-on**
3. Navigate to `browser-extension/` folder
4. Select `manifest.json`
5. Extension appears in toolbar! 🎉

### Step 3: Login
1. Click extension icon in toolbar
2. Enter your email and password
3. Click **Login** (or **Sign Up** if new user)

### Step 4: Track a Product
1. Visit any supported product page:
   - Myntra: https://www.myntra.com/...
   - Flipkart: https://www.flipkart.com/...
   - Ajio: https://www.ajio.com/...
   - Tata Cliq: https://www.tatacliq.com/...

2. Look for the purple **"Track This Price"** button
3. Click it and enter your target price
4. Done! You'll get email alerts when price drops 📧

## Features

### ✨ Auto-Detection
- Automatically detects product pages on supported platforms
- Extracts current price from page
- Injects tracking button seamlessly

### 📊 Quick Dashboard
- Click extension icon to see all alerts
- View current prices vs target prices
- Price drops highlighted in green
- Delete alerts with one click

### 🔔 Notifications
- Badge counter shows number of price drops
- Toast notifications for actions
- Email alerts when prices drop

### 🔐 Security
- Tokens stored securely in browser storage
- Auto-logout on token expiry
- All API calls use Bearer authentication

## Troubleshooting

### Button Not Appearing?
1. Refresh the product page
2. Check DevTools Console for errors
3. Verify the URL matches platform pattern
4. Platform may have changed HTML structure

### Can't Login?
1. Ensure backend is running: `cargo run`
2. Check API endpoint in popup.js (should be `http://localhost:3000`)
3. Clear extension storage: DevTools → Application → Clear storage
4. Try again

### Extension Not Loading?
1. Check for errors in `chrome://extensions/`
2. Verify all files exist in `browser-extension/` folder
3. Ensure manifest.json is valid JSON
4. Check browser console for errors

## API Endpoint Configuration

For production deployment, update API endpoint in:
- `content.js` line 3
- `background.js` line 4  
- `popup.js` line 3

Change from:
```javascript
const API_BASE = 'http://localhost:3000';
```

To:
```javascript
const API_BASE = 'https://your-api-domain.com';
```

## Uninstall

### Chrome
1. Go to `chrome://extensions/`
2. Click **Remove** on the extension

### Firefox
1. Go to `about:addons`
2. Click **Remove** next to the extension

---

**Happy Shopping! 🛍️**
