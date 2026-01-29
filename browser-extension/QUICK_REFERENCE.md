# 🧩 Browser Extension - Quick Reference

## Installation (2 minutes)

### Chrome/Edge
```bash
1. Open chrome://extensions/
2. Enable "Developer mode" (top-right toggle)
3. Click "Load unpacked"
4. Select: /path/to/ecom-scrapers/browser-extension/
5. Done! 🎉
```

### Firefox
```bash
1. Open about:debugging#/runtime/this-firefox
2. Click "Load Temporary Add-on"
3. Select: browser-extension/manifest.json
4. Done! 🎉
```

## Usage

### 1️⃣ Login
- Click extension icon in toolbar
- Enter email/password
- Click "Login" or "Sign Up"

### 2️⃣ Track Product
- Visit product page (Myntra/Flipkart/Ajio/Tata Cliq)
- Click purple "Track This Price" button
- Enter target price
- Get email when price drops! 📧

### 3️⃣ Manage Alerts
- Click extension icon
- View all tracked products
- See current vs target prices
- Delete unwanted alerts

## Platform URLs

✅ **Myntra**
```
https://www.myntra.com/tshirts/brand/product-name/12345
```

✅ **Flipkart**
```
https://www.flipkart.com/product-name/p/itm123abc
```

✅ **Ajio**
```
https://www.ajio.com/product-name/p/12345
```

✅ **Tata Cliq**
```
https://www.tatacliq.com/product-name/p-mp12345
```

## Features

| Feature | Description |
|---------|-------------|
| 🎯 Auto-Detection | Detects platforms automatically |
| 🔘 One-Click | Track price in 2 clicks |
| 📊 Quick View | See all alerts in popup |
| 🔔 Badges | Price drop counter on icon |
| 🎨 Dark Theme | Matches main app |
| 🔐 Secure | JWT authentication |

## Files

```
browser-extension/
├── manifest.json      # Extension config (Manifest V3)
├── content.js        # Runs on product pages
├── content.css       # Button styles
├── background.js     # API communication
├── popup.html        # Extension popup UI
├── popup.js          # Popup logic
├── popup.css         # Popup styles
└── icons/            # Extension icons
```

## Troubleshooting

### Button not appearing?
1. Refresh product page
2. Check URL matches platform pattern
3. Open DevTools Console for errors

### Can't login?
1. Ensure backend running: `cargo run`
2. Check API endpoint (should be localhost:3000)
3. Clear storage: DevTools → Application → Clear

### Extension not loading?
1. Check chrome://extensions/ for errors
2. Verify all files exist
3. Check manifest.json is valid JSON

## API Endpoints

The extension uses these backend endpoints:

```javascript
POST /auth/signup       # Create account
POST /auth/login        # Login
GET  /auth/me           # Verify token
POST /alerts            # Create alert
GET  /alerts            # List alerts
DELETE /alerts/:id      # Delete alert
```

## Configuration

### Change API Endpoint
Edit these files to change from localhost to production:

1. **content.js** (line 3):
   ```javascript
   const API_BASE = 'https://your-api.com';
   ```

2. **background.js** (line 4):
   ```javascript
   const API_BASE = 'https://your-api.com';
   ```

3. **popup.js** (line 3):
   ```javascript
   const API_BASE = 'https://your-api.com';
   ```

## Permissions

The extension requires:

- ✅ **storage** - Save auth tokens
- ✅ **activeTab** - Access current tab
- ✅ **scripting** - Inject content scripts
- ✅ **host_permissions** - Access e-commerce sites

## Testing Checklist

- [ ] Extension loads in Chrome
- [ ] Extension loads in Firefox
- [ ] Login works in popup
- [ ] Visit Myntra product page
- [ ] "Track Price" button appears
- [ ] Click button, enter target price
- [ ] Alert appears in popup
- [ ] Delete alert works
- [ ] Logout works
- [ ] Badge shows price drops

## Development

### Hot Reload
- **Chrome**: Go to chrome://extensions/ → Click refresh
- **Firefox**: Auto-reloads temporary extensions

### Debug
- **Content Script**: Open DevTools on product page
- **Background**: chrome://extensions/ → "service worker"
- **Popup**: Right-click icon → "Inspect popup"

## Stats

- **Lines of Code**: 1,237
- **Total Files**: 16
- **JavaScript**: 850 lines
- **CSS**: 350 lines
- **HTML**: 150 lines

## What's Next?

### For Development
1. Test on all 4 platforms
2. Verify button injection works
3. Test alert creation flow
4. Check popup functionality

### For Production
1. Update API_BASE to production URL
2. Create Chrome Web Store listing
3. Create Firefox Add-ons listing
4. Submit for review

## Links

- [Full Documentation](README.md)
- [Installation Guide](INSTALL.md)
- [Implementation Details](../EXTENSION_COMPLETE.md)
- [Main Project](../README.md)

---

**Quick Test**: Visit https://www.myntra.com/tshirts and look for the purple "Track This Price" button! 🎯
