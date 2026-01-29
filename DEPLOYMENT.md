# 🐳 Docker & Deployment - Complete!

## ✅ Implementation Complete

Full containerization with Docker and deployment configurations for Railway, Render, and Fly.io! Deploy your price tracker anywhere in minutes.

---

## 🎯 What's Included

### Docker Setup
✅ **Multi-stage Dockerfile**
- Stage 1: Rust builder with dependency caching
- Stage 2: Minimal runtime image (debian:bookworm-slim)
- Optimized build size (~100MB final image)
- Non-root user for security

✅ **Docker Compose**
- PostgreSQL database service
- Rust application service
- Health checks for both services
- Volume persistence for database
- Network isolation
- Environment variable support

✅ **Deployment Configs**
- Railway.json - One-click Railway deployment
- render.yaml - Render Blueprint deployment  
- fly.toml - Fly.io configuration
- Updated start.sh for production

---

## 🚀 Quick Start (Docker)

### Option 1: Docker Compose (Recommended)

```bash
# 1. Create .env file
cp .env.example .env
nano .env  # Add your JWT_SECRET and SMTP credentials

# 2. Start everything
docker-compose up -d

# 3. View logs
docker-compose logs -f app

# 4. Access app
open http://localhost:3000/app/
```

**That's it!** Database + app running in containers.

### Option 2: Docker Build Only

```bash
# Build image
docker build -t clothing-price-tracker .

# Run container (you need external PostgreSQL)
docker run -p 3000:3000 \
  -e DATABASE_URL="postgresql://user:pass@host:5432/db" \
  -e JWT_SECRET="your-secret" \
  clothing-price-tracker
```

---

## ☁️ Cloud Deployment

### Railway (Easiest - Free Tier Available)

1. **Install Railway CLI:**
   ```bash
   npm install -g @railway/cli
   railway login
   ```

2. **Deploy:**
   ```bash
   railway init
   railway up
   ```

3. **Add PostgreSQL:**
   ```bash
   railway add --database postgres
   ```

4. **Set environment variables:**
   ```bash
   railway variables set JWT_SECRET="your-secret-key"
   railway variables set SMTP_USERNAME="your-email@gmail.com"
   railway variables set SMTP_PASSWORD="your-app-password"
   ```

5. **Open app:**
   ```bash
   railway open
   ```

**Done!** Your app is live with database.

---

### Render (Blueprint Deployment)

1. **Fork repository** on GitHub

2. **Go to Render Dashboard:**
   - Click "New +" → "Blueprint"
   - Connect your GitHub repo
   - Select `render.yaml`

3. **Configure environment variables:**
   - DATABASE_URL (auto-generated)
   - JWT_SECRET (auto-generated)
   - SMTP_USERNAME, SMTP_PASSWORD, FROM_EMAIL

4. **Deploy:**
   - Click "Apply" - Render deploys automatically!

**Live in ~5 minutes!**

---

### Fly.io (Global Edge Deployment)

1. **Install Fly CLI:**
   ```bash
   curl -L https://fly.io/install.sh | sh
   fly auth login
   ```

2. **Launch app:**
   ```bash
   fly launch
   # Choose app name, region
   # Fly.io will detect Dockerfile automatically
   ```

3. **Create PostgreSQL:**
   ```bash
   fly postgres create
   fly postgres attach <postgres-app-name>
   ```

4. **Set secrets:**
   ```bash
   fly secrets set JWT_SECRET="your-secret-key"
   fly secrets set SMTP_USERNAME="your-email@gmail.com"
   fly secrets set SMTP_PASSWORD="your-app-password"
   fly secrets set FROM_EMAIL="your-email@gmail.com"
   ```

5. **Deploy:**
   ```bash
   fly deploy
   ```

6. **Open:**
   ```bash
   fly open
   ```

**Deployed globally!**

---

## 📦 Dockerfile Details

### Multi-Stage Build

**Stage 1: Builder**
```dockerfile
FROM rust:1.75-slim as builder
# - Install build dependencies
# - Cache Cargo dependencies
# - Build release binary
```

**Stage 2: Runtime**
```dockerfile
FROM debian:bookworm-slim
# - Minimal runtime dependencies only
# - Non-root user (appuser)
# - Health check endpoint
# - ~100MB final image
```

### Build Process

1. **Layer caching** - Dependencies built separately
2. **Release optimizations** - Compiled with --release
3. **Security** - Runs as non-root user
4. **Health checks** - Automatic container health monitoring

---

## 🔧 Docker Compose Services

### PostgreSQL Service
```yaml
postgres:
  image: postgres:16-alpine
  ports: 5432:5432
  volumes: postgres_data:/var/lib/postgresql/data
  healthcheck: pg_isready
```

### Application Service
```yaml
app:
  build: .
  ports: 3000:3000
  depends_on:
    postgres: service_healthy
  restart: unless-stopped
```

---

## 🌐 Environment Variables

### Required
```bash
DATABASE_URL=postgresql://user:pass@host:5432/db
JWT_SECRET=your-super-secret-jwt-key-min-32-chars
```

### Optional (Email)
```bash
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
SMTP_SERVER=smtp.gmail.com
SMTP_PORT=587
FROM_EMAIL=your-email@gmail.com
FROM_NAME=Price Tracker
```

### Application
```bash
PORT=3000
RUST_LOG=clothing_price_tracker=info
```

---

## 🛠️ Docker Commands

### Development
```bash
# Build image
docker build -t price-tracker .

# Run with compose
docker-compose up

# Run detached
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild after code changes
docker-compose up --build
```

### Production
```bash
# Build optimized image
docker build -t price-tracker:prod .

# Push to registry
docker tag price-tracker:prod your-registry/price-tracker:latest
docker push your-registry/price-tracker:latest

# Pull and run on server
docker pull your-registry/price-tracker:latest
docker run -d -p 3000:3000 --env-file .env price-tracker:latest
```

### Debugging
```bash
# Access container shell
docker exec -it price_tracker_app bash

# View container logs
docker logs price_tracker_app

# Inspect container
docker inspect price_tracker_app

# Database shell
docker exec -it price_tracker_db psql -U postgres -d price_tracker
```

---

## 📊 Deployment Comparison

| Platform | Free Tier | PostgreSQL | Auto-Deploy | Global CDN | Sleep Policy |
|----------|-----------|------------|-------------|------------|--------------|
| **Railway** | ✅ $5 credit | ✅ Included | ✅ Yes | ❌ No | ❌ No |
| **Render** | ✅ Yes | ✅ Free tier | ✅ Yes | ❌ No | ⚠️ 15min idle |
| **Fly.io** | ✅ Yes | ✅ Included | ✅ Yes | ✅ Yes | ✅ Auto-scale |

### Recommendations

**For development/testing:**
- Use **Docker Compose** locally

**For production (free):**
- **Railway** - Best DX, easy PostgreSQL
- **Fly.io** - Global performance, auto-scaling

**For production (paid):**
- **Railway** - Simplest, great support
- **Fly.io** - Best performance, edge locations

---

## 🔐 Security Best Practices

### Docker Security
✅ Non-root user (UID 1001)  
✅ Minimal base image (debian-slim)  
✅ No unnecessary packages  
✅ Health checks enabled  
✅ .dockerignore for secrets  

### Deployment Security
✅ Environment variables (not hardcoded)  
✅ Generated JWT secrets  
✅ HTTPS enforced  
✅ Database credentials isolated  
✅ No exposed debug info  

---

## 📈 Performance Optimizations

### Build Optimizations
- **Layer caching** - Fast rebuilds
- **Multi-stage** - Small final image
- **Release mode** - Optimized binary

### Runtime Optimizations
- **Health checks** - Auto-restart on failure
- **Connection pooling** - Database efficiency
- **Static file serving** - No external CDN needed

### Scaling
- **Horizontal scaling** - Add more containers
- **Auto-scaling** - Fly.io scales automatically
- **Database replication** - PostgreSQL read replicas

---

## 🧪 Testing Deployment

### Test Docker Build
```bash
# Build
docker build -t test .

# Should complete without errors
# Final image should be ~100MB
docker images test
```

### Test Docker Compose
```bash
# Start services
docker-compose up -d

# Check health
docker-compose ps

# Both services should be "healthy"
curl http://localhost:3000/

# Should return {"status":"healthy"}
```

### Test Production Build
```bash
# Build release
cargo build --release

# Binary should be ~15-20MB
ls -lh target/release/clothing_price_tracker

# Test run
PORT=3001 ./target/release/clothing_price_tracker
```

---

## 🐛 Troubleshooting

### Build Fails

**Problem:** Cargo build fails in Docker
```bash
# Check Rust version in Dockerfile
# Ensure dependencies are compatible
docker build --no-cache -t test .
```

**Problem:** Frontend not found
```bash
# Verify frontend/ directory exists
# Check COPY command in Dockerfile
ls -la frontend/
```

### Runtime Issues

**Problem:** Database connection failed
```bash
# Check DATABASE_URL format
# Ensure PostgreSQL is running
docker-compose logs postgres
```

**Problem:** Port already in use
```bash
# Change port in docker-compose.yml
ports:
  - "3001:3000"  # Use 3001 instead
```

### Deployment Issues

**Problem:** Environment variables not set
```bash
# Railway
railway variables

# Fly.io
fly secrets list

# Render
# Check dashboard → Environment tab
```

---

## 📁 New Files Created

- **[Dockerfile](Dockerfile)** - Multi-stage production build
- **[docker-compose.yml](docker-compose.yml)** - Local development stack
- **[.dockerignore](.dockerignore)** - Exclude unnecessary files
- **[railway.json](railway.json)** - Railway deployment config
- **[render.yaml](render.yaml)** - Render Blueprint spec
- **[fly.toml](fly.toml)** - Fly.io configuration
- **[start.sh](start.sh)** - Updated production start script

---

## 🎯 Quick Deploy Checklist

Before deploying:

- [ ] Set strong JWT_SECRET (min 32 chars)
- [ ] Configure SMTP credentials for email
- [ ] Set production DATABASE_URL
- [ ] Update CORS origins if needed
- [ ] Test locally with Docker Compose
- [ ] Build succeeds without errors
- [ ] Health check returns 200 OK

---

## 🎉 Summary

Docker & deployment is now **fully configured**!

- ✅ Multi-stage Dockerfile (~100MB image)
- ✅ Docker Compose for local dev
- ✅ Railway one-click deploy
- ✅ Render Blueprint deploy
- ✅ Fly.io global edge deploy
- ✅ Health checks & auto-restart
- ✅ PostgreSQL included
- ✅ Production-ready configs

**Your price tracker can now be deployed anywhere!** 🚀

Choose your platform and go live in minutes! 🌍
