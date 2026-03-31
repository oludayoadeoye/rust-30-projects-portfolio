#!/bin/bash
PORT=3005
echo "Testing chat-app on port $PORT..."
echo "(WebSocket required for full test, checking health and Swagger)"
curl -I http://127.0.0.1:$PORT/swagger-ui/
