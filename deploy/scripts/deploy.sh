#!/bin/bash

# Docker Deployment Script for E-commerce Application
# This script automates the deployment process

set -e

# Get the project root directory (two levels up from this script)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$PROJECT_ROOT"

echo "======================================"
echo "E-commerce Application Deployment"
echo "======================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if .env file exists
if [ ! -f .env ]; then
    echo -e "${YELLOW}Warning: .env file not found. Creating from deploy/config/.env.example...${NC}"
    cp deploy/config/.env.example .env
    echo -e "${YELLOW}Please update .env file with your actual configuration values.${NC}"
    read -p "Press enter to continue or Ctrl+C to exit and configure .env first..."
fi

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        echo -e "${RED}Error: Docker is not running. Please start Docker and try again.${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Docker is running${NC}"
}

# Function to stop existing containers
stop_containers() {
    echo ""
    echo "Stopping existing containers..."
    docker compose -f deploy/config/docker-compose.yml down
    echo -e "${GREEN}✓ Containers stopped${NC}"
}

# Function to build images
build_images() {
    echo ""
    echo "Building Docker images..."
    docker compose -f deploy/config/docker-compose.yml build --no-cache
    echo -e "${GREEN}✓ Images built successfully${NC}"
}

# Function to start containers
start_containers() {
    echo ""
    echo "Starting containers..."
    docker compose -f deploy/config/docker-compose.yml up -d
    echo -e "${GREEN}✓ Containers started${NC}"
}

# Function to check health status
check_health() {
    echo ""
    echo "Waiting for services to be healthy..."
    sleep 10
    
    # Check MongoDB
    echo -n "Checking MongoDB... "
    if docker compose -f deploy/config/docker-compose.yml exec -T mongodb mongosh --eval "db.runCommand('ping')" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠ (might still be starting)${NC}"
    fi
    
    # Check Elasticsearch
    echo -n "Checking Elasticsearch... "
    if curl -s http://localhost:9200/_cluster/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠ (might still be starting)${NC}"
    fi
    
    # Check Backend
    echo -n "Checking Backend API... "
    sleep 5
    if curl -s http://localhost:3000/ > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠ (might still be starting)${NC}"
    fi
    
    # Check Frontend
    echo -n "Checking Frontend... "
    if curl -s http://localhost/ > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠ (might still be starting)${NC}"
    fi
}

# Function to show logs
show_logs() {
    echo ""
    echo -e "${YELLOW}Showing recent logs (Ctrl+C to exit)...${NC}"
    docker compose logs --tail=50 -f
}

# Main deployment flow
main() {
    check_docker
    
    # Ask user what they want to do
    echo ""
    echo "Select deployment option:"
    echo "1) Full deployment (stop, build, start)"
    echo "2) Quick restart (stop and start with existing images)"
    echo "3) Build only"
    echo "4) Start only"
    echo "5) Stop only"
    echo "6) View logs"
    read -p "Enter option (1-6): " option
    
    case $option in
        1)
            stop_containers
            build_images
            start_containers
            check_health
            echo ""
            echo -e "${GREEN}======================================"
            echo "Deployment completed successfully!"
            echo "======================================"
            echo ""
            echo "Access points:"
            echo "  Frontend: http://localhost"
            echo "  Backend API: http://localhost:3000"
            echo "  API Docs: http://localhost:3000/api-docs"
            echo "  MongoDB: localhost:27017"
            echo "  Elasticsearch: http://localhost:9200"
            echo ""
            echo "View logs: docker compose logs -f"
            echo "Stop services: docker compose down"
            echo -e "${NC}"
            ;;
        2)
            stop_containers
            start_containers
            check_health
            ;;
        3)
            build_images
            ;;
        4)
            start_containers
            check_health
            ;;
        5)
            stop_containers
            ;;
        6)
            show_logs
            ;;
        *)
            echo -e "${RED}Invalid option${NC}"
            exit 1
            ;;
    esac
}

main
