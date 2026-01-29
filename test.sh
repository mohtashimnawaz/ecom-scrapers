#!/bin/bash

# Test runner script for Clothing Price Tracker
# Usage: ./test.sh [OPTIONS]
# Options:
#   --unit        Run only unit tests
#   --integration Run only integration tests
#   --coverage    Run tests with coverage
#   --watch       Run tests in watch mode
#   --verbose     Run with verbose output

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Environment setup
export DATABASE_URL="${DATABASE_URL:-postgresql://postgres:postgres@localhost/price_tracker_test}"
export JWT_SECRET="${JWT_SECRET:-test_secret_key_for_local_testing}"
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

echo -e "${GREEN}═══════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Clothing Price Tracker - Test Suite${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════${NC}"
echo ""

# Check if PostgreSQL is running
check_postgres() {
    echo -e "${YELLOW}Checking PostgreSQL connection...${NC}"
    if psql -d "$DATABASE_URL" -c "SELECT 1;" &> /dev/null; then
        echo -e "${GREEN}✓ PostgreSQL is running${NC}"
    else
        echo -e "${RED}✗ PostgreSQL connection failed${NC}"
        echo -e "${YELLOW}Please start PostgreSQL:${NC}"
        echo "  brew services start postgresql@15  # macOS"
        echo "  sudo systemctl start postgresql    # Linux"
        exit 1
    fi
    echo ""
}

# Create test database if it doesn't exist
setup_test_db() {
    echo -e "${YELLOW}Setting up test database...${NC}"
    
    # Extract database name from URL
    DB_NAME=$(echo "$DATABASE_URL" | sed -n 's|.*/\([^?]*\).*|\1|p')
    
    # Create database if it doesn't exist
    if ! psql -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
        echo "Creating database: $DB_NAME"
        createdb "$DB_NAME"
        echo -e "${GREEN}✓ Database created${NC}"
    else
        echo -e "${GREEN}✓ Database exists${NC}"
    fi
    
    # Run migrations
    echo "Running migrations..."
    if [ -d "migrations" ]; then
        sqlx migrate run --database-url "$DATABASE_URL"
        echo -e "${GREEN}✓ Migrations applied${NC}"
    fi
    echo ""
}

# Run unit tests
run_unit_tests() {
    echo -e "${YELLOW}Running unit tests...${NC}"
    echo ""
    
    if [ "$VERBOSE" = true ]; then
        cargo test --lib -- --nocapture
    else
        cargo test --lib
    fi
    
    echo ""
    echo -e "${GREEN}✓ Unit tests completed${NC}"
    echo ""
}

# Run integration tests
run_integration_tests() {
    echo -e "${YELLOW}Running integration tests...${NC}"
    echo ""
    
    if [ "$VERBOSE" = true ]; then
        cargo test --test '*' -- --nocapture
    else
        cargo test --test '*'
    fi
    
    echo ""
    echo -e "${GREEN}✓ Integration tests completed${NC}"
    echo ""
}

# Run all tests
run_all_tests() {
    echo -e "${YELLOW}Running all tests...${NC}"
    echo ""
    
    if [ "$VERBOSE" = true ]; then
        cargo test --all-features -- --nocapture
    else
        cargo test --all-features
    fi
    
    echo ""
    echo -e "${GREEN}✓ All tests completed${NC}"
    echo ""
}

# Run tests with coverage
run_with_coverage() {
    echo -e "${YELLOW}Running tests with coverage...${NC}"
    echo ""
    
    # Check if tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo -e "${YELLOW}Installing cargo-tarpaulin...${NC}"
        cargo install cargo-tarpaulin
    fi
    
    cargo tarpaulin --all-features --workspace --timeout 120 --out Xml --out Html
    
    echo ""
    echo -e "${GREEN}✓ Coverage report generated${NC}"
    echo -e "${YELLOW}Open coverage report: ${NC}tarpaulin-report.html"
    echo ""
}

# Watch mode
run_watch_mode() {
    echo -e "${YELLOW}Running tests in watch mode...${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
    echo ""
    
    # Check if cargo-watch is installed
    if ! command -v cargo-watch &> /dev/null; then
        echo -e "${YELLOW}Installing cargo-watch...${NC}"
        cargo install cargo-watch
    fi
    
    cargo watch -x test
}

# Display test summary
show_summary() {
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  Test Summary${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════${NC}"
    
    # Count tests
    echo -e "${YELLOW}Test counts:${NC}"
    cargo test --all-features -- --list 2>/dev/null | grep -c ': test' || echo "Run tests to see count"
    
    echo ""
    echo -e "${YELLOW}Test modules:${NC}"
    echo "  • Authentication (src/auth.rs)"
    echo "  • Scrapers (src/scrapers/*.rs)"
    echo "  • Database (src/db.rs)"
    echo "  • API Integration (tests/api_tests.rs)"
    
    echo ""
    echo -e "${YELLOW}Platforms tested:${NC}"
    echo "  • Myntra"
    echo "  • Flipkart"
    echo "  • Ajio"
    echo "  • Tata Cliq"
    
    echo ""
    echo -e "${GREEN}All tests passed! ✓${NC}"
    echo ""
}

# Parse command line arguments
UNIT_ONLY=false
INTEGRATION_ONLY=false
COVERAGE=false
WATCH=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --unit)
            UNIT_ONLY=true
            shift
            ;;
        --integration)
            INTEGRATION_ONLY=true
            shift
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --watch)
            WATCH=true
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            echo "Usage: ./test.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --unit        Run only unit tests"
            echo "  --integration Run only integration tests"
            echo "  --coverage    Run tests with coverage report"
            echo "  --watch       Run tests in watch mode"
            echo "  --verbose     Run with verbose output"
            echo "  --help        Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Main execution
check_postgres
setup_test_db

if [ "$WATCH" = true ]; then
    run_watch_mode
elif [ "$COVERAGE" = true ]; then
    run_with_coverage
elif [ "$UNIT_ONLY" = true ]; then
    run_unit_tests
elif [ "$INTEGRATION_ONLY" = true ]; then
    run_integration_tests
else
    run_all_tests
fi

show_summary
