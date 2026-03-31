#!/bin/bash
PORT=8084
echo "Testing simple-chatbot on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/chat -H "Content-Type: application/json" -d '{"message": "Hello"}' | jq .
curl -s http://127.0.0.1:$PORT/history | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
