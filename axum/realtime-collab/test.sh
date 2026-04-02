#!/bin/bash
PORT=3008
echo "Testing realtime-collab on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/docs -H "Content-Type: application/json" -d '{"name": "Spec v1", "content": "Initial draft"}' | jq .
curl -s http://127.0.0.1:$PORT/docs | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
