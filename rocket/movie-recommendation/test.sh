#!/bin/bash
PORT=8000
echo "Testing movie-recommendation on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/movies -H "Content-Type: application/json" -d '{"title": "Inception", "genre": "Sci-Fi", "release_year": 2010, "description": "Dreams within dreams"}' | jq .
curl -s http://127.0.0.1:$PORT/movies | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
