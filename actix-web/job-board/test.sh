#!/bin/bash
PORT=8087
echo "Testing job-board on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/jobs -H "Content-Type: application/json" -d '{"title": "Rust Engineer", "company": "TechCorp", "description": "Build high-perf systems"}' | jq .
curl -s http://127.0.0.1:$PORT/jobs | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
