# 🚀 Quick Reference: Supabase Setup

## 30-Second Setup

```bash
# 1. Configure database
export DATABASE_URL="postgresql://postgres:@localhost:5432/price_tracker"

# 2. Run app
cd ~/Desktop/ecom-scrapers
cargo run --release

# 3. Open dashboard
open http://localhost:3000/app
```

## Local PostgreSQL

```bash
# Install
brew install postgresql@15

# Start
brew services start postgresql@15

# Create database
createdb price_tracker

# Set environment
echo 'DATABASE_URL=postgresql://postgres:@localhost:5432/price_tracker' >> .env
```

## Supabase Cloud

```
1. Go to supabase.com
2. Create new project
3. Copy DATABASE_URL from Settings > Database > Connection String
4. Paste into .env: DATABASE_URL=...
5. Run: cargo run --release
```

## Key Files

| File | Purpose |
|------|---------|
| `.env` | Database credentials (create from .env.example) |
| `src/db.rs` | PostgreSQL connection & queries |
| `src/models.rs` | Data structures |
| `Cargo.toml` | Dependencies (sqlx, uuid, etc) |

## Environment Variable

```bash
# Required
DATABASE_URL=postgresql://[user]:[password]@[host]:[port]/[database]

# Examples:
# Local:     postgresql://postgres:@localhost:5432/price_tracker
# Supabase:  postgresql://postgres:password@db.xxx.supabase.co:5432/postgres
```

## API Testing

```bash
# Create alert
curl -X POST http://localhost:3000/alerts \
  -H "Content-Type: application/json" \
  -d '{"url":"https://myntra.com/...","target_price":999,"user_email":"you@test.com"}'

# List alerts
curl http://localhost:3000/alerts

# Delete alert
curl -X DELETE http://localhost:3000/alerts/[uuid]

# Check prices now
curl -X POST http://localhost:3000/alerts/check
```

## Common Issues

| Issue | Solution |
|-------|----------|
| Connection refused | Start PostgreSQL: `brew services start postgresql@15` |
| Permission denied | Ensure database exists: `createdb price_tracker` |
| UUID not found | Run: `psql -d price_tracker -c 'CREATE EXTENSION IF NOT EXISTS "uuid-ossp";'` |
| Tables missing | App auto-creates on startup, check logs |

## Useful Links

- 📖 [Setup Guide](SUPABASE_SETUP.md)
- 🔍 [Technical Details](MIGRATION_DETAILS.md)
- 📊 [Migration Status](SUPABASE_MIGRATION_STATUS.md)
- 🎨 [Frontend Guide](frontend/USAGE.md)

## Dashboard

```
Frontend:  http://localhost:3000/app
API:       http://localhost:3000
Health:    http://localhost:3000/
```

---

**Ready? Start with:** `cargo run --release`
