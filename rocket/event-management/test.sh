#!/bin/bash
PORT=8000
echo "Testing event-management on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/events -H "Content-Type: application/json" -d '{"name": "Rust Conf", "description": "Annual Rust Conference", "event_date": "2024-09-01T10:00:00Z"}' | jq .
curl -s http://127.0.0.1:$PORT/events | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
