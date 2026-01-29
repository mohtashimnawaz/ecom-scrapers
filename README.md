# Clothing Price Tracker

Professional-grade multi-user price tracking system for Indian e-commerce clothing platforms. Built with Rust, PostgreSQL, and modern web technologies.

## ✨ Features

### Core Features
- **🔐 User Authentication**: Secure JWT-based login/signup system
- **🧩 Browser Extension**: One-click tracking from product pages (Chrome/Firefox)
- **📊 Price History & Charts**: Interactive Chart.js visualizations
- **📧 Email Notifications**: HTML email alerts when prices drop
- **🎨 Modern Web UI**: Sleek dark-themed dashboard with real-time updates
- **📈 Live Statistics**: Best/average prices, savings tracking
- **🔔 Price Alerts**: User-specific alert management
- **🤖 Auto-monitoring**: Background checks every 6 hours
- **📱 Responsive**: Works on desktop, tablet, and mobile
- **⚡ Fast**: Built with Rust for maximum performance
- **🐳 Docker Ready**: Containerized deployment with Docker Compose

### Platform Support
- **Myntra** - JSON extraction from `window.__myntra_preloaded_state__`
- **Flipkart** - CSS selector scraping
- **Ajio** - JSON extraction from `window.__INITIAL_STATE__`
- **Tata Cliq** - CSS selector scraping

## 🚀 Quick Start

### Option 1: Docker Compose (Recommended)

```bash
# Clone and setup
git clone <repo>
cd ecom-scrapers
cp .env.example .env

# Edit .env with your credentials
nano .env

# Start with Docker
docker-compose up -d

# Access app
open http://localhost:3000/app/
```

### Option 2: Local Development

```bash
# Prerequisites: Rust, PostgreSQL

# Setup environment
cp .env.example .env
nano .env  # Add DATABASE_URL, JWT_SECRET

# Run
cargo run --release

# Access app
open http://localhost:3000/app/
```

## 🏗️ Architecture

### Tech Stack
- **Backend**: Axum 0.7 (high-performance async web framework)
- **Frontend**: Vanilla JavaScript + Chart.js + Modern CSS
- **Database**: PostgreSQL (Supabase or self-hosted)
- **ORM**: SQLx 0.7 (compile-time SQL verification)
- **Auth**: JWT with bcrypt password hashing
- **Email**: lettre (SMTP support)
- **HTTP Client**: Reqwest (with stealth headers)
- **HTML Parsing**: Scraper crate
- **Async Runtime**: Tokio
- **Containerization**: Docker + Docker Compose

### Components

```
src/
├── main.rs              # Entry point & server setup
├── models.rs            # Data models (User, PriceAlert, etc.)
├── db.rs                # Database operations with SQLx
├── auth.rs              # JWT authentication & password hashing
├── email.rs             # Email notification service
├── scraper_trait.rs     # PriceScraper trait
├── api.rs               # REST API endpoints
├── worker.rs            # Background price monitoring
└── scrapers/
    ├── myntra.rs        # Myntra scraper
    ├── flipkart.rs      # Flipkart scraper
    ├── ajio.rs          # Ajio scraper
    └── tata_cliq.rs     # Tata Cliq scraper

frontend/
├── index.html           # Main web interface
├── style.css            # Dark theme styling
└── app.js               # Frontend logic + auth
├── app.js               # Frontend logic & API calls
└── README.md            # Frontend docution
    └── tata_cliq.rs     # Tata Cliq scraper implementation
```

## 🚀 Quick Start

###**Quick start** (automated):
```bash
./start.sh
```

2. **Manual setup**:
```bash
# Start MongoDB
brew services start mongodb-community  # macOS
# OR
sudo systemctl start mongod  # Linux

# Configure environment
cp .env.example .env

# Build and run
cargo run --release
```

3. **Access the application**:
- 🎨 **Frontend**: http://localhost:3000/app
- 🔌 **API**: http://localhost:3000st](https://rustup.rs/))
- MongoDB 6.0+ ([Install MongoDB](https://www.mongodb.com/try/download/community))

### Installation

1. Clone and navigate to the project:
```bash
cd clothing_price_tracker
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your MongoDB connection string
```

3. Start MongoDB (if not running):
```bash
# macOS (Homebrew)
brew services start mongodb-community

# Linux (systemd)
sudo systemctl start mongod
```

4. Build and run:
```bash
cargo run --release
```

The server will start on `http://localhost:3000`

## 📡 API Usage

### Create Price Alert
```bash
curl -X POST http://localhost:3000/alerts \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://www.myntra.com/tshirts/levis/...",
    "target_price": 799.0,
    "user_email": "user@example.com"
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

### Manual Price Check
```bash
curl -X POST http://localhost:3000/alerts/check
```

## 🔍 How It Works

### 1. Scraping Strategy

Each platform has a custom scraper implementing the `PriceScraper` trait:

```rust
#[async_trait]
pub trait PriceScraper {
    async fn get_price(&self, url: &str) -> Result<f64>;
    fn platform_name(&self) -> &'static str;
    fn can_handle(&self, url: &str) -> bool;
}
```

**Flipkart & Tata Cliq**: Direct CSS selector extraction  
**Myntra & Ajio**: JSON extraction from `<script>` tags (SPA architecture)

### 2. Background Monitoring

- Tokio task runs every **6 hours**
- Fetches all active alerts from MongoDB
- Scrapes current prices using appropriate scraper
- Compares with target price
- Logs "ALARM" when price drops below target

### 3. Stealth Mode

All scrapers use realistic browser headers:
```rust
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/122.0.0.0
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MONGODB_URI` | MongoDB connection string | `mongodb://localhost:27017` |
| `DB_NAME` | Database name | `price_tracker` |
| `PORT` | Server port | `3000` |
| `RUST_LOG` | Logging level | `info` |

### Database Schema

```javascript
{
  "_id": ObjectId,
  "url": String,
  "target_price": Number,
  "last_price": Number?,
  "user_email": String,
  "platform": String,  // myntra | flipkart | ajio | tata_cliq
  "created_at": DateTime,
  "last_checked": DateTime,
  "is_active": Boolean
}
```

## 🛠️ Development

### Run in Debug Mode
```bash
cargo run
```

### Run Tests
```bash
cargo test
```

### Check for Issues
```bash
cargo clippy
```

### Format Code
```bash
cargo fmt
```

## ⚠️ Important Notes

### SPA Platforms (Myntra & Ajio)

These sites use client-side rendering. If the scrapers fail:

1. The HTML might be empty initially
2. Consider using headless browser:
```bash
# Add to Cargo.toml dependencies
thirtyfour = "0.32"
```

3. Update scraper to wait for content:
```rust
use thirtyfour::prelude::*;

async fn get_price_headless(url: &str) -> Result<f64> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto(url).await?;
    // Wait for price element and extract
}
```

### Rate Limiting

The worker includes a 2-second delay between scrapes to avoid rate limiting. Adjust in [src/worker.rs](src/worker.rs#L95).

### Selector Changes

E-commerce sites frequently update their HTML structure. If scraping fails:

1. Inspect the product page
2. Find the new price selector
3. Update the scraper in `src/scrapers/{platform}.rs`

## 📊 Monitoring

Check logs for price drops:
```bash
2026-01-22T10:30:00Z WARN 🚨 ALARM! Price drop detected for user@example.com: ₹749 <= ₹799 (Target)
```

## 🔐 Security

- Never commit `.env` file
- Use environment variables for sensitive data
- MongoDB connection should use authentication in production
- Consider rate limiting API endpoints

## 📝 TODO

- [ ] Email notifications (SMTP integration)
- [ ] Telegram/WhatsApp alerts
- [ ] Headless browser fallback for SPA sites
- [ ] Price history tracking
- [ ] Multiple price threshold alerts
- [ ] Web dashboard (React/Vue)

## 📄 License

MIT License - Feel free to use for personal or commercial projects.

## 🤝 Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Test thoroughly
4. Submit a pull request

---

Built with ❤️ using Rust 🦀
