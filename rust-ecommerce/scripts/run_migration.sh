#!/bin/bash

# MongoDB to PostgreSQL Migration Runner
# This script sets up the environment and runs the migration

set -e

echo "=========================================="
echo "MongoDB to PostgreSQL Migration"
echo "=========================================="
echo ""

# Check if .env file exists
if [ ! -f ../.env ]; then
    echo "Error: .env file not found in rust-ecommerce directory"
    echo "Please create a .env file with DATABASE_URL"
    exit 1
fi

# Load environment variables
source ../.env

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "Error: DATABASE_URL not set in .env file"
    exit 1
fi

# Set default dump directory if not provided
DUMP_DIR=${1:-"../../dump/ecommercedb"}

# Check if dump directory exists
if [ ! -d "$DUMP_DIR" ]; then
    echo "Error: Dump directory not found: $DUMP_DIR"
    echo "Usage: ./run_migration.sh [dump_directory]"
    exit 1
fi

echo "Configuration:"
echo "  Dump directory: $DUMP_DIR"
echo "  Database: $DATABASE_URL"
echo ""

# Check if Python dependencies are installed
echo "Checking Python dependencies..."
if ! python3 -c "import psycopg2" 2>/dev/null; then
    echo "Installing Python dependencies..."
    pip3 install -r requirements.txt
fi

echo ""
echo "Starting migration..."
echo ""

# Run the migration script
python3 migrate_mongo_to_postgres.py \
    --dump-dir "$DUMP_DIR" \
    --db-url "$DATABASE_URL" \
    --clear

echo ""
echo "=========================================="
echo "Migration completed!"
echo "=========================================="
