#!/bin/bash

# Quick Start Script for Clothing Price Tracker
# This script starts MongoDB, builds the Rust backend, and opens the frontend

set -e

echo "🚀 Starting Clothing Price Tracker"
echo "=================================="

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if MongoDB is running
echo -e "\n${YELLOW}1. Checking MongoDB...${NC}"
if ! pgrep -x "mongod" > /dev/null; then
    echo "Starting MongoDB..."
    brew services start mongodb-community 2>/dev/null || \
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
