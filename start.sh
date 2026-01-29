#!/bin/bash

# Production Start Script for Clothing Price Tracker
# Checks database connection and starts the application

set -e

echo "🚀 Starting Clothing Price Tracker"
echo "=================================="

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Load environment variables
if [ -f .env ]; then
    echo -e "${YELLOW}Loading environment variables...${NC}"
    export $(cat .env | grep -v '^#' | xargs)
else
    echo -e "${RED}Warning: .env file not found${NC}"
fi

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo -e "${RED}ERROR: DATABASE_URL not set${NC}"
    echo "Please set DATABASE_URL in .env file"
    exit 1
fi

# Wait for database to be ready
echo -e "\n${YELLOW}Checking database connection...${NC}"
max_attempts=30
attempt=0

while [ $attempt -lt $max_attempts ]; do
    if cargo run --bin check_db 2>/dev/null || true; then
        echo -e "${GREEN}✓ Database connected${NC}"
        break
    fi
    
    attempt=$((attempt + 1))
    if [ $attempt -eq $max_attempts ]; then
        echo -e "${RED}ERROR: Could not connect to database after $max_attempts attempts${NC}"
        exit 1
    fi
    
    echo "Waiting for database... (attempt $attempt/$max_attempts)"
    sleep 2
done

# Build and run
echo -e "\n${YELLOW}Building application...${NC}"
cargo build --release

echo -e "\n${GREEN}✓ Build complete${NC}"
echo -e "\n${YELLOW}Starting server...${NC}"
echo -e "${GREEN}Server will be available at: http://localhost:${PORT:-3000}${NC}"
echo -e "${GREEN}Frontend available at: http://localhost:${PORT:-3000}/app/${NC}"
echo ""

cargo run --release
    sudo systemctl start mongod 2>/dev/null || \
    docker start mongodb 2>/dev/null || \
    echo "Please start MongoDB manually"
else
    echo -e "${GREEN}✓ MongoDB is running${NC}"
fi

# Check for .env file
echo -e "\n${YELLOW}2. Checking environment configuration...${NC}"
if [ ! -f .env ]; then
    echo "Creating .env from .env.example..."
    cp .env.example .env
    echo -e "${GREEN}✓ Created .env file${NC}"
else
    echo -e "${GREEN}✓ .env file exists${NC}"
fi

# Build the project
echo -e "\n${YELLOW}3. Building Rust backend...${NC}"
cargo build --release
echo -e "${GREEN}✓ Build complete${NC}"

# Start the server in background
echo -e "\n${YELLOW}4. Starting server...${NC}"
cargo run --release &
SERVER_PID=$!

# Wait for server to start
echo "Waiting for server to start..."
sleep 3

# Open frontend in browser
echo -e "\n${YELLOW}5. Opening frontend...${NC}"
if command -v open &> /dev/null; then
    open http://localhost:3000/app
elif command -v xdg-open &> /dev/null; then
    xdg-open http://localhost:3000/app
else
    echo "Please open http://localhost:3000/app in your browser"
fi

echo -e "\n${GREEN}================================${NC}"
echo -e "${GREEN}✓ Server is running!${NC}"
echo -e "${GREEN}  Frontend: http://localhost:3000/app${NC}"
echo -e "${GREEN}  API:      http://localhost:3000${NC}"
echo -e "${GREEN}================================${NC}"
echo -e "\nPress Ctrl+C to stop the server"

# Wait for Ctrl+C
trap "kill $SERVER_PID 2>/dev/null; exit" INT
wait $SERVER_PID
