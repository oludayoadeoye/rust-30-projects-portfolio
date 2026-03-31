#!/bin/bash
PORT=8000
echo "Testing home-automation on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/devices -H "Content-Type: application/json" -d '{"name": "Living Room Light", "device_type": "Switch", "state": "OFF"}' | jq .
curl -s http://127.0.0.1:$PORT/devices | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
