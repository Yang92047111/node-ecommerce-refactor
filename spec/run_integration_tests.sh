#!/bin/bash
# Integration Test Setup Script for Rust + PostgreSQL + Testcontainers
# This script sets up the test environment and runs integration tests

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Integration Test Setup${NC}"
echo -e "${GREEN}========================================${NC}"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Error: Docker is not running. Please start Docker first.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Docker is running${NC}"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo is not installed. Please install Rust first.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Rust is installed${NC}"

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo -e "${YELLOW}Installing sqlx-cli...${NC}"
    cargo install sqlx-cli --no-default-features --features postgres
fi

echo -e "${GREEN}✓ sqlx-cli is installed${NC}"

# Set test environment variables
export DATABASE_URL="postgres://test_user:test_password@localhost:5432/test_ecommerce_db"
export RUST_LOG=debug
export RUST_BACKTRACE=1

echo -e "${GREEN}✓ Environment variables set${NC}"

# Function to run integration tests
run_integration_tests() {
    echo -e "${YELLOW}Running integration tests...${NC}"
    cargo test --test '*' -- --test-threads=1 --nocapture
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ All integration tests passed!${NC}"
    else
        echo -e "${RED}✗ Some integration tests failed${NC}"
        exit 1
    fi
}

# Function to run specific test module
run_specific_test() {
    local test_name=$1
    echo -e "${YELLOW}Running test: ${test_name}...${NC}"
    cargo test --test "${test_name}" -- --test-threads=1 --nocapture
}

# Function to clean up test containers
cleanup() {
    echo -e "${YELLOW}Cleaning up Docker containers...${NC}"
    docker ps -a | grep testcontainers | awk '{print $1}' | xargs -r docker rm -f
    echo -e "${GREEN}✓ Cleanup complete${NC}"
}

# Parse command line arguments
case "$1" in
    "all")
        run_integration_tests
        ;;
    "clean")
        cleanup
        ;;
    "test")
        if [ -z "$2" ]; then
            echo -e "${RED}Please specify a test name${NC}"
            exit 1
        fi
        run_specific_test "$2"
        ;;
    "setup")
        echo -e "${GREEN}✓ Test environment is ready${NC}"
        ;;
    *)
        echo "Usage: $0 {all|clean|test <test_name>|setup}"
        echo ""
        echo "Commands:"
        echo "  all             - Run all integration tests"
        echo "  clean           - Clean up test containers"
        echo "  test <name>     - Run a specific test module"
        echo "  setup           - Verify setup only"
        exit 1
        ;;
esac

exit 0
