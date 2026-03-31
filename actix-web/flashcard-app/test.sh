#!/bin/bash
PORT=8081
echo "Testing flashcard-app on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/decks -H "Content-Type: application/json" -d '{"name": "Rust Basics", "description": "Core concepts"}' | jq .
curl -s http://127.0.0.1:$PORT/decks | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
