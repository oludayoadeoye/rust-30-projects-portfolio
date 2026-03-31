#!/bin/bash
PORT=3003
echo "Testing currency-converter on port $PORT..."
curl -s "http://127.0.0.1:$PORT/convert?from=USD&to=EUR&amount=100" | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
