#!/bin/bash
PORT=8083
echo "Testing lang-learning-app on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/vocab -H "Content-Type: application/json" -d '{"word": "Ferris", "translation": "Crab", "language": "Rust"}' | jq .
curl -s http://127.0.0.1:$PORT/vocab | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
