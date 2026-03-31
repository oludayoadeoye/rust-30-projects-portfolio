#!/bin/bash
PORT=8000
echo "Testing finance-manager on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/accounts -H "Content-Type: application/json" -d '{"name": "Savings", "account_type": "Bank"}' | jq .
curl -s http://127.0.0.1:$PORT/accounts | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
