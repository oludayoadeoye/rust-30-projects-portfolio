#!/bin/bash
PORT=8000
echo "Testing contact-form on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/contact -H "Content-Type: application/json" -d '{"name": "John", "email": "john@example.com", "subject": "Hello", "message": "Hi there"}' | jq .
curl -s http://127.0.0.1:$PORT/contact | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
