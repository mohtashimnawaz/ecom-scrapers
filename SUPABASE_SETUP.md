# 🚀 Supabase Migration Guide

Your Clothing Price Tracker has been successfully migrated from **MongoDB to Supabase (PostgreSQL)**!

## ✨ What's Changed

### Database Migration
- ✅ **MongoDB** → **PostgreSQL (via Supabase)**
- ✅ **Replaced**: `mongodb` crate → `sqlx` + `postgrest`
- ✅ **Model changes**: BSON → Native Rust types with sqlx::FromRow
- ✅ **ID generation**: MongoDB ObjectId → UUID v4

### Code Updates
- ✅ [src/db.rs](src/db.rs) - Complete rewrite for PostgreSQL
- ✅ [src/models.rs](src/models.rs) - Updated with UUID and sqlx derives
- ✅ [src/worker.rs](src/worker.rs) - Fixed for new database layer
- ✅ [src/api.rs](src/api.rs) - Updated to use Database struct
- ✅ [Cargo.toml](Cargo.toml) - Added sqlx, uuid, postgrest
- ✅ [.env.example](.env.example) - Updated for Supabase credentials

## 🛠️ Setup Instructions

### Option 1: Local PostgreSQL Development

```bash
# 1. Install PostgreSQL (macOS)
brew install postgresql@15

# 2. Start PostgreSQL service
brew services start postgresql@15

# 3. Create database and user
createuser postgres -s
createdb price_tracker

# 4. Update .env
DATABASE_URL=postgresql://postgres:@localhost:5432/price_tracker

# 5. Run the app
cargo run --release
```

### Option 2: Supabase Cloud (Recommended for Production)

#### Step 1: Create Supabase Project

1. Go to [supabase.com](https://supabase.com)
2. Click **"New Project"**
3. Fill in details:
   - Organization: (create or select)
   - Project name: `clothing-price-tracker`
   - Database password: (generate strong password)
   - Region: (closest to you)
4. Click **"Create new project"** (wait ~2 min)

#### Step 2: Get Connection Details

1. In Supabase dashboard, go to **Settings → Database**
2. Find **Connection String** section
3. Copy the **URI** (PostgreSQL)
4. Replace password in the string with your database password
5. Should look like:
   ```
   postgresql://postgres:YOUR_PASSWORD@db.XXXXX.supabase.co:5432/postgres
   ```

#### Step 3: Create Environment File

```bash
# Copy example
cp .env.example .env

# Edit .env with your Supabase connection string
nano .env
```

Add these to `.env`:

```dotenv
DATABASE_URL=postgresql://postgres:YOUR_PASSWORD@db.XXXXX.supabase.co:5432/postgres
PORT=3000
RUST_LOG=clothing_price_tracker=info
```

#### Step 4: Run Application

```bash
# Build and run
cargo run --release
```

The application will **automatically create the tables** on first run!

## 📊 Database Schema

### Table: `price_alerts`

```sql
CREATE TABLE price_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url TEXT NOT NULL,
    target_price DOUBLE PRECISION NOT NULL,
    last_price DOUBLE PRECISION,
    user_email TEXT NOT NULL,
    platform TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_checked TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Index for fast queries
CREATE INDEX idx_is_active ON price_alerts(is_active);
```

## 🔄 API Compatibility

✅ **All existing API endpoints work the same**:

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/` | GET | Health check |
| `/alerts` | GET | List all alerts |
| `/alerts` | POST | Create new alert |
| `/alerts/:id` | DELETE | Delete alert |
| `/alerts/check` | POST | Trigger price check |
| `/app` | GET | Frontend dashboard |

## 📈 Performance Comparison

| Aspect | MongoDB | PostgreSQL |
|--------|---------|-----------|
| Connection Pool | Native | Configurable (5 connections) |
| Queries | Document-based | SQL-based |
| Transactions | Limited | Full ACID support |
| Indexing | Automatic | Manual (we added is_active index) |
| Cloud Option | MongoDB Atlas | Supabase (free tier) |

## 🔐 Security Best Practices

### Supabase Security

1. **Strong password**: Generate with Supabase
2. **Network restrictions**: Use IP whitelist in Supabase
3. **Row-level security**: Available in Supabase (optional)
4. **API keys**: Use anon/service role keys carefully

### Local Development

1. Store `.env` in `.gitignore` (already done)
2. Never commit passwords
3. Use different passwords for dev/prod
4. Rotate passwords regularly

## 🐛 Troubleshooting

### Connection Timeout

**Error**: `connection timed out`

**Solution**:
```bash
# Test connection
psql DATABASE_URL

# Verify DATABASE_URL format
echo $DATABASE_URL
```

### Table Creation Failed

**Error**: `CREATE TABLE IF NOT EXISTS failed`

**Solution**:
```bash
# Check permissions - ensure user is superuser
# For Supabase: Use postgres user (automatically superuser)

# For local PostgreSQL:
psql -U postgres -c "ALTER ROLE postgres WITH SUPERUSER;"
```

### UUID Generation Issues

**Error**: `could not find function gen_random_uuid`

**Solution**:
```bash
# In Supabase SQL Editor, run:
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

# Or for local PostgreSQL:
psql -U postgres -d price_tracker -c 'CREATE EXTENSION IF NOT EXISTS "uuid-ossp";'
```

### Connection Pool Exhausted

**Error**: `all pool connections in use`

**Solution**: Increase pool size in [src/db.rs](src/db.rs#L13):
```rust
.max_connections(10)  // Increase from 5
```

## 🚀 Deployment

### Deploy to Heroku

```bash
# 1. Create Heroku app
heroku create clothing-price-tracker

# 2. Add Supabase PostgreSQL
heroku config:set DATABASE_URL=postgresql://...

# 3. Add buildpack
heroku buildpacks:add https://github.com/emk/heroku-buildpack-rust.git

# 4. Deploy
git push heroku main
```

### Deploy to Railway

```bash
# 1. Connect GitHub
# 2. Link PostgreSQL database
# 3. Set DATABASE_URL environment variable
# 4. Deploy automatically on push
```

## 📝 Migration Notes

### What Stayed the Same
- ✅ Scraper implementations (Myntra, Flipkart, Ajio, Tata Cliq)
- ✅ API endpoints and responses
- ✅ Background worker (6-hour checks)
- ✅ Frontend (HTML/CSS/JS)
- ✅ Error handling and logging

### What Changed
- ✅ Database layer (new db.rs)
- ✅ Model derives (added sqlx::FromRow)
- ✅ Connection management (PgPool instead of Client)
- ✅ Query syntax (SQL instead of BSON)
- ✅ ID type (UUID instead of ObjectId)

## 🔄 Rollback (if needed)

To go back to MongoDB:

```bash
# In Cargo.toml, revert to:
mongodb = "2.8"

# Restore old src files from git:
git checkout HEAD -- src/db.rs src/models.rs src/worker.rs

# Restore .env.example
cp backup/.env.example.mongodb .env.example
```

## 📚 Useful Commands

### View Data in Supabase

```bash
# SQL Editor in Supabase dashboard:
SELECT * FROM price_alerts ORDER BY created_at DESC;

# Show active alerts:
SELECT * FROM price_alerts WHERE is_active = TRUE;

# Count by platform:
SELECT platform, COUNT(*) FROM price_alerts GROUP BY platform;
```

### Local PostgreSQL Commands

```bash
# Connect to database
psql -U postgres -d price_tracker

# List tables
\dt

# Show table schema
\d price_alerts

# Exit
\q
```

## ✅ Verification Checklist

After setup, verify everything works:

- [ ] PostgreSQL/Supabase connection successful
- [ ] Tables created automatically
- [ ] Frontend loads at http://localhost:3000/app
- [ ] Create alert works (POST /alerts)
- [ ] List alerts works (GET /alerts)
- [ ] Delete alert works (DELETE /alerts/:id)
- [ ] Manual price check works (POST /alerts/check)
- [ ] Background worker running (check logs)

## 📊 Build Status

```
✅ Code compiles successfully
✅ All warnings are non-critical
✅ Database layer migrated
✅ API updated
✅ Worker updated
✅ Models updated
✅ Environment configured
```

## 🎉 Success!

Your application is now using **Supabase PostgreSQL** for scalable, cloud-based data storage!

### Next Steps

1. **Set up Supabase account** (or use local PostgreSQL)
2. **Configure DATABASE_URL** in .env
3. **Run the app**: `cargo run --release`
4. **Open dashboard**: http://localhost:3000/app
5. **Start tracking prices!** 🎯

### Support

- Supabase Docs: [supabase.com/docs](https://supabase.com/docs)
- SQLx Docs: [sqlx.rs](https://sqlx.rs)
- PostgreSQL Docs: [postgresql.org/docs](https://www.postgresql.org/docs/)
