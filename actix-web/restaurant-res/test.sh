#!/bin/bash
PORT=8085
echo "Testing restaurant-res on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/tables -H "Content-Type: application/json" -d '{"table_number": 1, "capacity": 4}' | jq .
curl -s http://127.0.0.1:$PORT/tables | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
