# 🚀 Quick Start Guide

## Prerequisites
- Rust 1.75+ installed
- MongoDB 6.0+ running locally or accessible remotely

## Setup (3 steps)

### 1. Start MongoDB
```bash
# macOS (Homebrew)
brew services start mongodb-community

# Linux
sudo systemctl start mongod

# Docker
docker run -d -p 27017:27017 --name mongodb mongo:latest
```

### 2. Configure Environment
```bash
cp .env.example .env
# Edit .env if needed (default works for local MongoDB)
```

### 3. Run the Server
```bash
cargo run
```

Server starts on **http://localhost:3000**

---

## 📡 API Examples

### Create Price Alert
```bash
curl -X POST http://localhost:3000/alerts \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://www.myntra.com/tshirts/levis/product-123",
    "target_price": 799.0,
    "user_email": "your@email.com"
  }'
```

### List All Alerts
```bash
curl http://localhost:3000/alerts
```

### Delete Alert
```bash
curl -X DELETE http://localhost:3000/alerts/{alert_id}
```

### Trigger Manual Check
```bash
curl -X POST http://localhost:3000/alerts/check
```

---

## 🧪 Automated Testing
```bash
./test_api.sh
```

---

## 📊 How It Works

1. **Create Alert**: POST to `/alerts` with product URL and target price
2. **Background Worker**: Checks prices every 6 hours automatically
3. **Price Drop**: When current price ≤ target price, logs "🚨 ALARM!"
4. **Platform Detection**: Automatically detects Myntra/Flipkart/Ajio/Tata Cliq from URL

---

## 🔍 Monitoring Logs

Watch for price drops in real-time:
```bash
cargo run | grep ALARM
```

Expected output when price drops:
```
2026-01-22T10:30:00Z WARN 🚨 ALARM! Price drop detected for user@example.com: ₹749 <= ₹799
```

---

## ⚠️ Troubleshooting

### MongoDB Connection Error
```bash
# Verify MongoDB is running
brew services list | grep mongodb

# Or check with mongo shell
mongosh
```

### Scraping Fails (Empty Response)
The site might be using JavaScript rendering. Solutions:
1. Check if the URL is correct and accessible
2. For Myntra/Ajio (SPA sites), consider using headless browser (thirtyfour)
3. Update selectors in `src/scrapers/{platform}.rs` if site changed

### Rate Limiting
Increase delay in `src/worker.rs` line 95:
```rust
tokio::time::sleep(Duration::from_secs(5)).await; // Increase to 5 seconds
```

---

## 🎯 Development Commands

```bash
# Build (debug)
cargo build

# Build (optimized)
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Watch for changes (requires cargo-watch)
cargo watch -x run
```

---

## 📁 Project Structure

```
src/
├── main.rs              # Server entry point
├── api.rs               # REST endpoints
├── worker.rs            # Background price checker
├── db.rs                # MongoDB connection
├── models.rs            # Data structures
├── scraper_trait.rs     # PriceScraper trait
└── scrapers/
    ├── myntra.rs        # Myntra implementation
    ├── flipkart.rs      # Flipkart implementation
    ├── ajio.rs          # Ajio implementation
    └── tata_cliq.rs     # Tata Cliq implementation
```

---

## 💡 Tips

- Monitor logs for `ALARM` messages to see price drops
- The worker runs every 6 hours by default (configurable in `worker.rs`)
- Use `/alerts/check` endpoint to trigger immediate price check
- Price selectors may need updates if e-commerce sites change their HTML

---

## 🔐 Production Deployment

1. Set strong MongoDB credentials in `.env`
2. Use environment variables for all secrets
3. Enable MongoDB authentication
4. Consider using a reverse proxy (nginx)
5. Add rate limiting to API endpoints
6. Set up proper logging and monitoring

---

Built with ❤️ using Rust 🦀
