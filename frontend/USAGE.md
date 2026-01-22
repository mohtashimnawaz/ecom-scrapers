# Frontend Preview & Usage Guide

## 🎨 UI Overview

### Dashboard Layout

The frontend features a modern, dark-themed interface with:

1. **Header Section**
   - Title: "🛍️ Clothing Price Tracker"
   - Subtitle: Platform support information

2. **Create Alert Form** (Top Card)
   - Product URL input (with platform auto-detection)
   - Target Price field (₹)
   - Email input
   - "Add Alert" button with hover effects

3. **Statistics Cards** (3-column grid)
   - Active Alerts count
   - Price Drops detected
   - Average Target Price

4. **Active Alerts Section**
   - Refresh button
   - "Check Prices Now" button
   - List of all active alerts with:
     * Platform badge (color-coded)
     * Product URL
     * Target Price vs Current Price comparison
     * Savings calculation
     * Delete button
     * Email notification recipient

5. **Footer**
   - Built with info
   - Background monitoring status

## 🎯 How to Use

### Creating Your First Alert

1. **Find a product** on any supported platform:
   - Myntra: `https://www.myntra.com/tshirts/brand/product-name`
   - Flipkart: `https://www.flipkart.com/clothing/product-name`
   - Ajio: `https://www.ajio.com/clothing/product-name`
   - Tata Cliq: `https://www.tatacliq.com/product-name`

2. **Copy the URL** from your browser

3. **Open the dashboard**: http://localhost:3000/app

4. **Fill the form**:
   ```
   Product URL: [paste URL]
   Target Price: [e.g., 799]
   Email: your@email.com
   ```

5. **Click "Add Alert"**

### Monitoring Prices

**Automatic Checks**:
- Backend checks every 6 hours automatically
- Dashboard auto-refreshes every 30 seconds

**Manual Checks**:
- Click "🔄 Refresh" to reload alerts
- Click "💰 Check Prices Now" to trigger immediate scraping

### Understanding Price Displays

**Color Coding**:
- 🟡 **Yellow** - Target Price
- 🟢 **Green** - Current Price (above target)
- 🔴 **Red** - PRICE DROP! (at or below target)

**Platform Badges**:
- 🎀 **Pink** - Myntra
- 🔵 **Blue** - Flipkart
- 🟤 **Brown** - Ajio
- 🟣 **Purple** - Tata Cliq

### Price Drop Alerts

When a price drops to or below your target:

1. Alert card turns red
2. "🚨 PRICE DROP DETECTED!" appears
3. Savings amount is highlighted
4. (Future: Email notification sent)

## 📱 Mobile Experience

The UI is fully responsive:

- **Desktop**: 3-column stats, wide form layout
- **Tablet**: 2-column stats, adjusted spacing
- **Mobile**: Single column, stacked elements

## 🎨 Color Scheme

```css
Primary:   #6366f1 (Indigo)
Success:   #10b981 (Green)
Danger:    #ef4444 (Red)
Warning:   #f59e0b (Amber)
Background: #0f172a (Dark Blue)
Cards:     #1e293b (Slate)
```

## ⚡ Performance

- **Instant loading**: Static files served directly
- **Fast updates**: Efficient API calls
- **Smooth animations**: CSS transitions
- **No frameworks**: Vanilla JS = smaller bundle

## 🔧 Customization

### Change Theme Colors

Edit `frontend/style.css`:

```css
:root {
    --primary: #6366f1;  /* Your color here */
}
```

### Modify Auto-refresh Time

Edit `frontend/app.js`:

```javascript
// Change 30000 (30 seconds) to your preference
autoRefreshInterval = setInterval(loadAlerts, 30000);
```

### Add Platform Logo

Update platform badges in `style.css`:

```css
.platform-myntra::before {
    content: '🎀 ';  /* Add emoji or icon */
}
```

## 🐛 Troubleshooting

### Frontend Not Loading

**Problem**: Blank page or 404
**Solution**: 
```bash
# Ensure frontend directory exists
ls -la frontend/

# Restart server
cargo run --release
```

### API Not Connecting

**Problem**: Toast shows "Failed to load alerts"
**Solution**:
```bash
# Check server is running
curl http://localhost:3000/

# Check MongoDB
brew services list | grep mongodb
```

### Prices Not Updating

**Problem**: "Not checked yet" on all alerts
**Solution**:
1. Click "💰 Check Prices Now" button
2. Wait 10-30 seconds for scraping
3. Click "🔄 Refresh"

### CORS Errors

**Problem**: Browser console shows CORS error
**Solution**: Already fixed - CORS is enabled in `api.rs`

## 📊 Example Screenshots

### Dashboard with Alerts
```
┌─────────────────────────────────────┐
│  🛍️ Clothing Price Tracker          │
│  Track prices across platforms      │
├─────────────────────────────────────┤
│  Create Price Alert                 │
│  URL: [myntra.com/tshirts/...]     │
│  Price: [999]  Email: [you@...]    │
│  [Add Alert]                        │
├─────────────────────────────────────┤
│  [5] Active  [2] Drops  [₹850] Avg │
├─────────────────────────────────────┤
│  🎀 Myntra                [Delete]  │
│  myntra.com/tshirts/levis...        │
│  Target: ₹799  Current: ₹749        │
│  🚨 PRICE DROP! Save ₹50            │
│  📧 test@example.com                │
└─────────────────────────────────────┘
```

## 🚀 Next Steps

1. **Email Notifications**: Integrate SMTP for alerts
2. **Price History**: Add charts showing price trends
3. **Export**: Download alerts as CSV
4. **Filters**: Sort/filter by platform, price, etc.
5. **Dark/Light Toggle**: Theme switcher
6. **PWA**: Make it installable on mobile

## 📝 License

Same as main project - see root README.md
