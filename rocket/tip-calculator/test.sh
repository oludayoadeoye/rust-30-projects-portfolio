#!/bin/bash
PORT=8000
echo "Testing tip-calculator on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/tip -H "Content-Type: application/json" -d '{"bill_amount": 50.0, "tip_percentage": 15}' | jq .
curl -s http://127.0.0.1:$PORT/tip | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
