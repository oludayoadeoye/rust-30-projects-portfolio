#!/bin/bash
PORT=8000
echo "Testing bmi-calculator on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/bmi -H "Content-Type: application/json" -d '{"weight": 70.0, "height": 1.75}' | jq .
curl -s http://127.0.0.1:$PORT/bmi | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
