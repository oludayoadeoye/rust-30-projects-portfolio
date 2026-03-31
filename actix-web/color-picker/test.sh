#!/bin/bash
PORT=8080
echo "Testing color-picker on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/palettes -H "Content-Type: application/json" -d '{"name": "Sunset", "colors": ["#FF5733", "#FFC300"]}' | jq .
curl -s http://127.0.0.1:$PORT/palettes | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
