#!/bin/sh

echo "Starting React development server..."
echo "API URL: $REACT_APP_API_URL"
echo "Host: $HOST"
echo "Port: $PORT"
echo "WDS_SOCKET_HOST: $WDS_SOCKET_HOST"
echo "WDS_SOCKET_PORT: $WDS_SOCKET_PORT"

# Wait a moment for any initialization
sleep 2

# Start the React dev server
# Use exec to ensure the process runs as PID 1 and receives signals properly
exec npm start

