# Browser Extension - Clothing Price Tracker 🧩

A powerful Chrome/Firefox extension that enables one-click price tracking from any supported e-commerce product page.

## ✨ Features

- **🎯 Auto-Detection**: Automatically detects Myntra, Flipkart, Ajio, and Tata Cliq product pages
- **🔘 One-Click Tracking**: "Track This Price" button injected directly on product pages
- **📊 Quick Dashboard**: Extension popup for instant alert management
- **🔐 Secure Authentication**: Login/signup directly from extension
- **🔔 Price Drop Badges**: Visual notification badges for price drops
- **⚡ Real-time Sync**: Syncs with backend API instantly

## 🚀 Installation

### Chrome

1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode" (toggle in top-right)
3. Click "Load unpacked"
4. Select the `browser-extension/` folder
5. Extension is now installed! 🎉

### Firefox

1. Open Firefox and navigate to `about:debugging#/runtime/this-firefox`
2. Click "Load Temporary Add-on"
3. Navigate to `browser-extension/` folder
4. Select `manifest.json`
5. Extension is now installed! 🎉

## 📖 Usage

### 1. Login/Signup
- Click the extension icon in your browser toolbar
- Login with your existing account or create a new one
- Your session persists across browser restarts

### 2. Track a Product
- Visit any product page on:
  - Myntra: https://www.myntra.com/...
  - Flipkart: https://www.flipkart.com/...
  - Ajio: https://www.ajio.com/...
  - Tata Cliq: https://www.tatacliq.com/...
- Look for the **"Track This Price"** button (automatically injected)
- Click the button
- Enter your target price
- Done! You'll be notified when the price drops

### 3. Manage Alerts
- Click the extension icon
- View all your active alerts
- See current price vs target price
- Price drops highlighted in green
- Delete alerts with one click
- Click "Dashboard" to open the full web app

### 4. Notifications
- Badge on extension icon shows number of price drops
- Visual indicators in popup for alerts that hit target price
- Toast notifications for successful actions

## 🏗️ Extension Architecture

### Files Structure
```
browser-extension/
├── manifest.json         # Extension configuration (Manifest V3)
├── background.js         # Service worker for API calls
├── content.js           # Injected into product pages
├── content.css          # Styles for injected button
├── popup.html           # Extension popup interface
├── popup.js             # Popup logic
├── popup.css            # Popup styles
└── icons/               # Extension icons (16, 32, 48, 128)
```

### Platform Detection
The extension automatically detects product pages using URL patterns:

- **Myntra**: `/myntra.com\/([^\/]+)\/([^\/]+)\/(\d+)/`
- **Flipkart**: `/flipkart.com\/.*\/p\//`
- **Ajio**: `/ajio.com\/.*\/p\//`
- **Tata Cliq**: `/tatacliq.com\/.*\/p-/`

### Price Extraction
Each platform has custom selectors for price extraction:

- **Myntra**: `.pdp-price`
- **Flipkart**: `._30jeq3`, `._25b18c`
- **Ajio**: `.prod-sp`
- **Tata Cliq**: `.ProductDescription__price`

## ⚙️ Configuration

### API Endpoint
Default: `http://localhost:3000`

To change the API endpoint, update `API_BASE` in:
- [content.js](browser-extension/content.js#L3)
- [background.js](browser-extension/background.js#L4)
- [popup.js](browser-extension/popup.js#L3)

For production:
```javascript
const API_BASE = 'https://your-api-domain.com';
```

### Permissions
The extension requires:
- **storage**: Store auth tokens and user data
- **activeTab**: Access current tab for injection
- **scripting**: Inject content scripts dynamically
- **host_permissions**: Access to e-commerce sites

## 🎨 UI/UX Features

### Injected Button
- Gradient background (purple/indigo)
- Smooth animations on hover
- Loading states during API calls
- Success state with checkmark
- Auto-resets after 3 seconds

### Popup Interface
- Dark theme matching main app
- Compact 380px width
- Quick stats (alert count, price drops)
- Scrollable alert list
- Delete confirmation dialogs
- Toast notifications for feedback

### Notifications
- In-page toast for tracking confirmations
- Badge counter for price drops
- Visual highlights for dropped prices

## 🔧 Development

### Hot Reload
Chrome requires manual refresh after code changes:
1. Go to `chrome://extensions/`
2. Click refresh icon on the extension card

Firefox auto-reloads temporary extensions.

### Debugging
- **Content Script**: Open DevTools on product page, check Console
- **Background Script**: Go to `chrome://extensions/`, click "service worker"
- **Popup**: Right-click extension icon → "Inspect popup"

### Testing
1. Start the backend: `cargo run`
2. Visit a product page
3. Check if button appears
4. Test tracking flow
5. Verify in popup and web dashboard

## 📱 Browser Compatibility

### Chrome/Edge
- ✅ Manifest V3 fully supported
- ✅ Service worker background script
- ✅ All features working

### Firefox
- ✅ Manifest V3 compatible
- ✅ Background scripts supported
- ✅ All features working
- ⚠️ Note: Use temporary add-on for development

### Safari
- ⚠️ Requires conversion to Safari Web Extension
- Not currently supported

## 🐛 Troubleshooting

### Button Not Appearing
1. Check if URL matches platform pattern
2. Verify content script loaded (DevTools Console)
3. Check if price element exists on page
4. Platform may have changed their HTML structure

### API Calls Failing
1. Ensure backend is running on `localhost:3000`
2. Check CORS settings in backend
3. Verify auth token in storage (DevTools → Application → Storage)
4. Check network tab for error details

### Authentication Issues
1. Clear extension storage: DevTools → Application → Storage → Clear
2. Re-login through popup
3. Verify backend auth endpoints working

## 🔐 Security

- Auth tokens stored in chrome.storage.local (encrypted by browser)
- HTTPS recommended for production API
- Content Security Policy enforced
- No eval() or inline scripts
- Host permissions limited to required domains

## 📊 Analytics

Track extension usage (optional):
- Number of alerts created
- Most tracked platforms
- Conversion rates
- Price drop success rates

## 🚀 Distribution

### Chrome Web Store
1. Create ZIP of `browser-extension/` folder
2. Upload to Chrome Web Store Developer Dashboard
3. Fill in store listing details
4. Submit for review

### Firefox Add-ons
1. Create ZIP of `browser-extension/` folder
2. Upload to Firefox Add-on Developer Hub
3. Fill in listing details
4. Submit for review

## 📈 Future Enhancements

- [ ] Bulk delete alerts
- [ ] Sort/filter alerts by platform
- [ ] Export alerts to CSV
- [ ] Price history charts in popup
- [ ] Desktop notifications
- [ ] Wishlist sync
- [ ] Coupon code detection
- [ ] Compare prices across platforms

## 🤝 Contributing

Found a bug or have a feature request? Please open an issue!

---

**Made with ❤️ for smart shoppers**
