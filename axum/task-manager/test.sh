#!/bin/bash
PORT=3007
echo "Testing task-manager on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/tasks -H "Content-Type: application/json" -d '{"title": "Implement tests", "description": "Add curl scripts", "priority": "high"}' | jq .
curl -s http://127.0.0.1:$PORT/tasks | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
