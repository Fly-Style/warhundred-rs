#!/bin/bash
set -e

# Run database migrations
echo "Running database migrations..."
cd /app
diesel migration run

# Run the main binary
echo "Starting application..."
exec /app/main "$@"
