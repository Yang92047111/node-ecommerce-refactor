#!/bin/bash

# MongoDB Data Import Script
# Imports the database dump into MongoDB container

set -e

echo "Importing MongoDB database dump..."

# Wait for MongoDB to be ready
echo "Waiting for MongoDB to be ready..."
sleep 10

# Restore the database
docker compose exec -T mongodb mongorestore \
    --username admin \
    --password password123 \
    --authenticationDatabase admin \
    --db ecommercedb \
    /docker-entrypoint-initdb.d/ecommercedb

echo "Database import completed!"
