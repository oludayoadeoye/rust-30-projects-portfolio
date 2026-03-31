#!/bin/bash
PORT=8000
echo "Testing quote-generator on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/quotes -H "Content-Type: application/json" -d '{"content": "Stay hungry, stay foolish", "author": "Steve Jobs"}' | jq .
curl -s http://127.0.0.1:$PORT/quotes | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
