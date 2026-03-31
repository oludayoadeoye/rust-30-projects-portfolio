#!/bin/bash
PORT=8000
echo "Testing library-management on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/books -H "Content-Type: application/json" -d '{"title": "The Rust Programming Language", "author": "Steve Klabnik", "isbn": "978-1593278281", "total_copies": 5}' | jq .
curl -s http://127.0.0.1:$PORT/books | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
