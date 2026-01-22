#!/bin/bash

# Test script for Clothing Price Tracker API
# Make sure the server is running before executing this script

BASE_URL="http://localhost:3000"

echo "🧪 Testing Clothing Price Tracker API"
echo "======================================"

# Health Check
echo -e "\n1️⃣ Health Check"
curl -s "$BASE_URL/" | jq '.'

# Create Alert - Myntra
echo -e "\n2️⃣ Creating Myntra Alert"
ALERT1=$(curl -s -X POST "$BASE_URL/alerts" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://www.myntra.com/tshirts/levis/example-product",
    "target_price": 799.0,
    "user_email": "test@example.com"
  }')
echo "$ALERT1" | jq '.'
ALERT1_ID=$(echo "$ALERT1" | jq -r '.id')

# Create Alert - Flipkart
echo -e "\n3️⃣ Creating Flipkart Alert"
curl -s -X POST "$BASE_URL/alerts" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://www.flipkart.com/clothing/example-product",
    "target_price": 599.0,
    "user_email": "test2@example.com"
  }' | jq '.'

# List All Alerts
echo -e "\n4️⃣ Listing All Alerts"
curl -s "$BASE_URL/alerts" | jq '.'

# Manual Price Check
echo -e "\n5️⃣ Triggering Manual Price Check"
curl -s -X POST "$BASE_URL/alerts/check" | jq '.'

# Delete Alert
echo -e "\n6️⃣ Deleting Alert: $ALERT1_ID"
curl -s -X DELETE "$BASE_URL/alerts/$ALERT1_ID" -w "\nHTTP Status: %{http_code}\n"

# List Alerts Again
echo -e "\n7️⃣ Listing Alerts After Deletion"
curl -s "$BASE_URL/alerts" | jq '.'

echo -e "\n✅ API Testing Complete!"
