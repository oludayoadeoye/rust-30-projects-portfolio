#!/bin/bash
PORT=3002
echo "Testing calculator on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/calculate -H "Content-Type: application/json" -d '{"op": "add", "a": 10, "b": 5}' | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
