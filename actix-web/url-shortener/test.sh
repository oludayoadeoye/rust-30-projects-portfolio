#!/bin/bash
PORT=8082
echo "Testing url-shortener on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/shorten -H "Content-Type: application/json" -d '{"url": "https://www.rust-lang.org"}' | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
