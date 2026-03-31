#!/bin/bash
PORT=8089
echo "Testing smart-inventory on port $PORT..."
curl -s -X POST http://127.0.0.1:$PORT/products -H "Content-Type: application/json" -d '{"sku": "SKU-123", "name": "Rust Tool", "quantity": 100, "price": "19.99"}' | jq .
curl -s http://127.0.0.1:$PORT/products | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
