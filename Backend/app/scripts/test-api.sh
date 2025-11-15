#!/bin/bash

# API Testing script for Bisca
echo "🧪 Testing Bisca API..."

BASE_URL="http://localhost:3000"

# Test health endpoint
echo "1. Testing health endpoint..."
curl -s "$BASE_URL/health" | jq '.'

echo -e "\n2. Testing root endpoint..."
curl -s "$BASE_URL/" | jq '.'

echo -e "\n3. Testing get users..."
curl -s "$BASE_URL/api/users" | jq '.'

echo -e "\n4. Testing get specific user..."
curl -s "$BASE_URL/api/users/1" | jq '.'

echo -e "\n5. Testing get user aura..."
curl -s "$BASE_URL/api/users/1/aura" | jq '.'

echo -e "\n6. Testing get ratings..."
curl -s "$BASE_URL/api/ratings" | jq '.'

echo -e "\n7. Testing create rating..."
curl -s -X POST "$BASE_URL/api/ratings" \
  -H "Content-Type: application/json" \
  -d '{"to_user_id": "2", "rating_value": 1}' | jq '.'

echo -e "\n8. Testing get user ratings..."
curl -s "$BASE_URL/api/ratings/user/1" | jq '.'

echo -e "\n✅ API testing complete!"
