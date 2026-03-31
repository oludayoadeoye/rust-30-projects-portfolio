#!/bin/bash
PORT=3004
echo "Testing notes-app on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/notes -H "Content-Type: application/json" -d '{"content": "Drafting my first note"}' | jq .
curl -s http://127.0.0.1:$PORT/notes | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
