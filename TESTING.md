# Testing Guide

## Overview
Comprehensive testing setup for the Clothing Price Tracker with unit tests, integration tests, and CI/CD automation.

## Test Structure

### Unit Tests
- **Auth Module** (`src/auth.rs`): JWT token generation/verification, password hashing
- **Scraper Modules** (`src/scrapers/*.rs`): Price extraction with mocked HTTP responses
- **Database Module** (`src/db.rs`): Database operations (inline tests)

### Integration Tests
- **API Tests** (`tests/api_tests.rs`): End-to-end API endpoint testing
- Auth flow (signup, login, protected routes)
- Alert CRUD operations
- User isolation and data security

## Running Tests Locally

### Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL (macOS)
brew install postgresql@15
brew services start postgresql@15

# Create test database
createdb price_tracker_test
```

### Environment Setup
```bash
# Copy .env.example to .env
cp .env.example .env

# Set required variables
export DATABASE_URL="postgresql://postgres:postgres@localhost/price_tracker_test"
export JWT_SECRET="test_secret_key_change_in_production"
```

### Run All Tests
```bash
# Quick test script
./test.sh

# Or manually:
cargo test --all-features

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_token_generation_and_verification

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'
```

### Test Coverage by Module

#### 1. Authentication Tests (8 tests)
- ✅ Claims creation with correct expiry
- ✅ Token generation and verification
- ✅ Invalid token rejection
- ✅ Token with wrong secret fails
- ✅ Password hashing (bcrypt format)
- ✅ Password verification (correct/incorrect)
- ✅ Different passwords produce different hashes
- ✅ Same password produces different hashes (salt)

#### 2. Scraper Tests (6 tests per platform = 24 tests)
**Myntra Scraper:**
- ✅ URL pattern matching (`can_handle`)
- ✅ Platform name identification
- ✅ Price extraction from `__myntra_preloaded_state__`
- ✅ Price extraction from `pdpData` fallback
- ✅ Error handling for missing price

**Flipkart Scraper:**
- ✅ URL pattern matching
- ✅ Platform name identification
- ✅ Price parsing (₹1,299 → 1299.0)
- ✅ Price extraction from `.Nx9W0j` selector
- ✅ Fallback selector testing (`._30jeq3`)
- ✅ Error handling for missing price

**Ajio Scraper:**
- Similar test coverage for Ajio-specific selectors

**Tata Cliq Scraper:**
- Similar test coverage for Tata Cliq-specific selectors

#### 3. Integration Tests (7 tests)
- ✅ Health check endpoint
- ✅ User signup and login flow
- ✅ Login with wrong password (401)
- ✅ Protected route without auth (401)
- ✅ Create and list alerts (with auth)
- ✅ Delete alert (with auth)
- ✅ User data isolation

## CI/CD Pipeline

### GitHub Actions Workflows

#### CI Workflow (`.github/workflows/ci.yml`)
Runs on every push and pull request:

1. **Test Suite**
   - PostgreSQL service container
   - Cargo fmt (code formatting)
   - Cargo clippy (linting)
   - Unit tests (`cargo test --lib`)
   - Integration tests (`cargo test --test '*'`)
   - Full test suite with coverage

2. **Build**
   - Release build compilation
   - Artifact upload

3. **Security Audit**
   - `cargo audit` for dependency vulnerabilities

4. **Docker Build**
   - Multi-stage Dockerfile validation
   - Build caching with GitHub Actions cache

#### Deploy Workflow (`.github/workflows/deploy.yml`)
Runs on push to `main` branch or version tags:

1. **Docker Image**
   - Build and push to GitHub Container Registry (ghcr.io)
   - Semantic versioning tags
   - SHA-based tags

2. **Railway Deployment**
   - Automatic deployment via Railway CLI
   - Requires `RAILWAY_TOKEN` secret

3. **Render Deployment**
   - Webhook trigger
   - Requires `RENDER_DEPLOY_HOOK_URL` secret

4. **Fly.io Deployment**
   - Automated deployment via flyctl
   - Requires `FLY_API_TOKEN` secret

### Required Secrets
Configure in GitHub repository settings:

```bash
# GitHub Container Registry (automatic)
GITHUB_TOKEN

# Railway
RAILWAY_TOKEN

# Render
RENDER_DEPLOY_HOOK_URL

# Fly.io
FLY_API_TOKEN
```

## Test Dependencies

### Dev Dependencies in Cargo.toml
```toml
[dev-dependencies]
mockito = "1.2"           # HTTP mocking
wiremock = "0.6"          # Server mocking
tokio-test = "0.4"        # Async test utilities
serial_test = "3.0"       # Sequential test execution
assert_matches = "1.5"    # Pattern matching assertions
sqlx = { ... "migrate" }  # Database migrations in tests
```

## Writing New Tests

### Unit Test Example
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function("input");
        assert_eq!(result, "expected");
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Test Example
```rust
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_api_endpoint() {
    let pool = setup_test_db().await;
    let db = Database::new(pool.clone());
    let app = create_app(db);
    
    let response = app
        .oneshot(Request::builder().uri("/endpoint").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    cleanup_test_db(&pool).await;
}
```

### Scraper Test with Mockito
```rust
#[tokio::test]
async fn test_price_extraction() {
    let mut server = Server::new_async().await;
    
    let mock_html = r#"<div class="price">₹1,299</div>"#;
    
    let _m = server.mock("GET", "/product")
        .with_status(200)
        .with_body(mock_html)
        .create_async()
        .await;
    
    let scraper = MyScraper::new();
    let price = scraper.get_price(&server.url()).await.unwrap();
    
    assert_eq!(price, 1299.0);
}
```

## Test Database Management

### Setup Test Database
```bash
# Create database
createdb price_tracker_test

# Run migrations
sqlx migrate run --database-url postgresql://postgres:postgres@localhost/price_tracker_test
```

### Reset Test Database
```bash
# Drop and recreate
dropdb price_tracker_test
createdb price_tracker_test
sqlx migrate run --database-url postgresql://postgres:postgres@localhost/price_tracker_test
```

### Cleanup in Tests
Integration tests use `#[serial]` attribute to run sequentially and include cleanup:

```rust
async fn cleanup_test_db(pool: &PgPool) {
    sqlx::query("DELETE FROM price_alerts").execute(pool).await.ok();
    sqlx::query("DELETE FROM users").execute(pool).await.ok();
}
```

## Performance Testing

### Load Testing (Optional)
```bash
# Install k6
brew install k6

# Run load test (create script first)
k6 run load_test.js
```

### Benchmarking (Optional)
```bash
# Install criterion
cargo install cargo-criterion

# Run benchmarks
cargo criterion
```

## Debugging Tests

### Enable Logging
```bash
# Set log level
RUST_LOG=debug cargo test -- --nocapture

# Specific module
RUST_LOG=clothing_price_tracker::scrapers=trace cargo test
```

### Run Single Test with Backtrace
```bash
RUST_BACKTRACE=1 cargo test test_name -- --nocapture
```

### Debug Integration Test
```rust
// Add debug prints
dbg!(&response);
dbg!(&body_bytes);

// Or use tracing
tracing::info!("Response status: {:?}", response.status());
```

## Continuous Integration Best Practices

1. **Fast Feedback**: Unit tests run first (faster), then integration tests
2. **Parallel Execution**: Independent tests run in parallel
3. **Serial Database Tests**: Use `#[serial]` for tests that modify database
4. **Isolated Environments**: Each test cleans up after itself
5. **Deterministic Tests**: No reliance on external services in CI
6. **Cache Dependencies**: GitHub Actions caches Cargo registry and builds

## Test Metrics

### Current Coverage
- **Unit Tests**: 32+ tests across auth, scrapers, models
- **Integration Tests**: 7 end-to-end API tests
- **Total**: 39+ automated tests
- **Platforms Covered**: All 4 (Myntra, Flipkart, Ajio, Tata Cliq)

### CI Pipeline Performance
- **Test Suite**: ~2-3 minutes
- **Build**: ~1-2 minutes
- **Security Audit**: ~30 seconds
- **Docker Build**: ~3-5 minutes
- **Total CI Time**: ~7-11 minutes

## Next Steps

1. **Increase Coverage**: Add tests for worker module and email notifications
2. **E2E Tests**: Selenium/Playwright tests for browser extension
3. **Performance Tests**: Load testing with k6 or Artillery
4. **Mutation Testing**: `cargo-mutants` for test quality
5. **Code Coverage**: `cargo-tarpaulin` or `cargo-llvm-cov`

## Troubleshooting

### Tests Fail with Database Connection Error
```bash
# Check PostgreSQL is running
brew services list

# Restart PostgreSQL
brew services restart postgresql@15

# Verify connection
psql -d price_tracker_test
```

### Tests Fail with "table does not exist"
```bash
# Run migrations
sqlx migrate run --database-url postgresql://postgres:postgres@localhost/price_tracker_test
```

### CI Fails but Local Tests Pass
- Check environment variables in `.github/workflows/ci.yml`
- Verify PostgreSQL version matches (15)
- Check for hardcoded localhost references

### Mockito Server Hangs
- Ensure you're using `Server::new_async().await`
- Check mock expectations match actual requests
- Verify mock is created before scraper call

## Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Mockito Documentation](https://docs.rs/mockito/)
- [GitHub Actions for Rust](https://github.com/actions-rs)
