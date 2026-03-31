#!/bin/bash
PORT=3000
echo "Testing todo-list on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/todos -H "Content-Type: application/json" -d '{"title": "Test Task", "completed": false}' | jq .
curl -s http://127.0.0.1:$PORT/todos | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
