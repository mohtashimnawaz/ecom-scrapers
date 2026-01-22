# 🎨 Frontend - Clothing Price Tracker

Modern, responsive web interface for tracking clothing prices across Indian e-commerce platforms.

## Features

- **Real-time Dashboard**: View all active price alerts with live updates
- **Price Tracking**: Monitor prices from Myntra, Flipkart, Ajio, and Tata Cliq
- **Visual Alerts**: Instant notifications when prices drop below target
- **Statistics**: Track total alerts, price drops, and average savings
- **Auto-refresh**: Automatic updates every 30 seconds
- **Responsive Design**: Works seamlessly on desktop, tablet, and mobile

## Tech Stack

- **Pure JavaScript**: No frameworks required - vanilla JS for maximum performance
- **Modern CSS**: CSS Grid, Flexbox, and custom properties
- **Dark Theme**: Easy on the eyes with a sleek dark UI
- **REST API**: Communicates with Rust backend via fetch API

## Usage

### Access the Frontend

Once the server is running:

```bash
# Start the server
cargo run --release

# Open in browser
open http://localhost:3000/app
```

### Create a Price Alert

1. Enter the product URL from any supported platform
2. Set your target price
3. Add your email address
4. Click "Add Alert"

### Monitor Prices

- View all alerts in the dashboard
- Check current vs target prices
- See price drop notifications
- Manually trigger price checks
- Delete alerts when no longer needed

## API Integration

The frontend connects to these backend endpoints:

- `GET /alerts` - Fetch all alerts
- `POST /alerts` - Create new alert
- `DELETE /alerts/:id` - Remove alert
- `POST /alerts/check` - Trigger manual price check

## Customization

### Change Colors

Edit `style.css` CSS variables:

```css
:root {
    --primary: #6366f1;      /* Primary accent color */
    --success: #10b981;      /* Success/current price */
    --danger: #ef4444;       /* Danger/price drop */
    --warning: #f59e0b;      /* Warning/target price */
}
```

### Modify Auto-refresh Interval

In `app.js`:

```javascript
// Change from 30 seconds to desired interval
autoRefreshInterval = setInterval(loadAlerts, 30000); // milliseconds
```

## File Structure

```
frontend/
├── index.html    # Main HTML structure
├── style.css     # Styling and dark theme
└── app.js        # Application logic and API calls
```

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile browsers (iOS Safari, Chrome Mobile)

## Development

To modify the frontend:

1. Edit files in `frontend/` directory
2. Refresh browser (no build step needed)
3. Changes are instant

## Production Deployment

The frontend is served as static files by the Axum backend. No separate deployment needed - just run the Rust server.

## Troubleshooting

**Frontend not loading?**
- Ensure server is running on port 3000
- Check browser console for errors
- Verify `frontend/` directory exists

**API calls failing?**
- Check CORS configuration in backend
- Verify MongoDB is running
- Check server logs for errors

**Auto-refresh not working?**
- Check browser console for network errors
- Verify server is responding
- Try manual refresh button
