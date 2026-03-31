#!/bin/bash
PORT=3009
echo "Testing blockchain-voting on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/mine -H "Content-Type: application/json" -d '{"data": "Vote for Rust"}' | jq .
curl -s http://127.0.0.1:$PORT/blocks | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
