#!/bin/bash
PORT=3001
echo "Testing weather-app on port $PORT..."
curl -s "http://127.0.0.1:$PORT/weather?city=London" | jq .
echo "Swagger check:"
curl -I http://127.0.0.1:$PORT/swagger-ui/
