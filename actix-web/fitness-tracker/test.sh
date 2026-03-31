#!/bin/bash
PORT=8086
echo "Testing fitness-tracker on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/workouts -H "Content-Type: application/json" -d '{"name": "Upper Body", "notes": "Felt strong"}' | jq .
curl -s http://127.0.0.1:$PORT/workouts | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
