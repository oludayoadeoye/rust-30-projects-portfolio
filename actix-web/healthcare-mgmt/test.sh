#!/bin/bash
PORT=8088
echo "Testing healthcare-mgmt on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/patients -H "Content-Type: application/json" -d '{"name": "Alice", "date_of_birth": "1990-01-01"}' | jq .
curl -s http://127.0.0.1:$PORT/patients | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
