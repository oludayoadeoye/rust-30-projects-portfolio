#!/bin/bash
PORT=8000
echo "Testing recipe-app on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/recipes -H "Content-Type: application/json" -d '{"name": "Pasta", "ingredients": "Flour, Egg", "instructions": "Mix and boil"}' | jq .
curl -s http://127.0.0.1:$PORT/recipes | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
