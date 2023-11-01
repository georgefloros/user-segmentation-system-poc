#!/bin/bash

# Make an HTTP request to the /health endpoint
response=$(curl -f http://localhost:9000/minio/health/live)

# Check the HTTP response code
if [ $response -eq 200 ]; then
  exit 0  # Success
else
  exit 1  # Failure
fi
