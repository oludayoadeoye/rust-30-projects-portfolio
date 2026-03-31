#!/bin/bash
PORT=3006
echo "Testing ecommerce-platform on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/products -H "Content-Type: application/json" -d '{"name": "Rust Book", "price": 49.99, "stock": 100}' | jq .
curl -s http://127.0.0.1:$PORT/products | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
