#!/bin/bash

# Test Email Notifications Script
# Usage: ./test_email.sh your-email@example.com

EMAIL=${1:-"test@example.com"}

echo "🧪 Testing Email Notifications"
echo "================================"
echo "Sending test email to: $EMAIL"
echo

# Test endpoint
curl -X POST http://localhost:3000/email/test \
  -H "Content-Type: application/json" \
  -d "{\"email\": \"$EMAIL\"}" \
  | jq .

echo
echo "✅ Check your inbox at $EMAIL"
