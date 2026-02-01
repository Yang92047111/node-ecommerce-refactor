# Deployment Files

This directory contains all deployment-related files, scripts, and documentation for the e-commerce application.

## 📁 Directory Structure

```
deploy/
├── config/          # Docker and configuration files
├── docs/            # Deployment documentation
├── scripts/         # Deployment and testing scripts
└── README.md        # This file
```

## 🐳 Configuration Files (`config/`)

### Docker Files
- **`Dockerfile`** - Backend Node.js application Docker configuration
- **`Dockerfile.frontend`** - Frontend Vue.js application Docker configuration
- **`docker-compose.yml`** - Multi-container orchestration (MongoDB, Elasticsearch, Backend, Frontend)
- **`.dockerignore`** - Backend Docker build exclusions
- **`.dockerignore.frontend`** - Frontend Docker build exclusions
- **`nginx.conf`** - Nginx configuration for frontend (reverse proxy, static files)
- **`.env.example`** - Environment variables template

### Quick Setup
```bash
# Copy environment template
cp config/.env.example ../.env

# Update .env with your values, then deploy
cd .. && docker compose -f deploy/config/docker-compose.yml up -d
```

## 📚 Documentation (`docs/`)

### Getting Started
1. **`GET_STARTED.md`** - Complete beginner's guide to deployment
2. **`DOCKER_DEPLOYMENT.md`** - Detailed Docker deployment instructions
3. **`QUICK_REFERENCE.md`** - Quick command reference and troubleshooting

### Testing & Validation
4. **`API_TESTING.md`** - API endpoint testing guide with examples
5. **`TEST_IMPROVEMENTS.md`** - Test suite enhancements and best practices

### Summary
6. **`DEPLOYMENT_SUMMARY.md`** - Complete deployment summary and architecture overview

### Reading Order (Recommended)
1. Start with `GET_STARTED.md` for initial setup
2. Review `DOCKER_DEPLOYMENT.md` for detailed deployment
3. Use `QUICK_REFERENCE.md` during operations
4. Follow `API_TESTING.md` to validate your deployment

## 🚀 Scripts (`scripts/`)

### Deployment Scripts
- **`deploy.sh`** - Interactive deployment script with health checks
  ```bash
  cd .. && ./deploy/scripts/deploy.sh
  ```

### Database Scripts
- **`import-db.sh`** - MongoDB database import from dump files
  ```bash
  ./deploy/scripts/import-db.sh
  ```

### Testing Scripts
- **`test-api.sh`** - Comprehensive Bash-based API testing (20 test cases)
  ```bash
  ./deploy/scripts/test-api.sh
  ```

- **`test-endpoints.js`** - Node.js-based API testing suite
  ```bash
  node deploy/scripts/test-endpoints.js
  ```

## 🎯 Quick Start Guide

### 1. Prerequisites
- Docker & Docker Compose V2
- Git
- 4GB+ RAM available

### 2. Initial Setup
```bash
# Clone repository (if not done)
git clone <repository-url>
cd node-ecommerce-main

# Copy environment template
cp deploy/config/.env.example .env

# Edit .env with your configuration
nano .env
```

### 3. Deploy Application
```bash
# Option 1: Interactive deployment (recommended)
./deploy/scripts/deploy.sh

# Option 2: Manual deployment
docker compose -f deploy/config/docker-compose.yml up -d
```

### 4. Verify Deployment
```bash
# Check container health
docker compose -f deploy/config/docker-compose.yml ps

# Check logs
docker compose -f deploy/config/docker-compose.yml logs -f

# Run tests
./deploy/scripts/test-api.sh
```

### 5. Access Application
- **Frontend**: http://localhost
- **Backend API**: http://localhost:3000
- **MongoDB**: localhost:27017
- **Elasticsearch**: http://localhost:9200

## 📊 Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                      Frontend (Nginx)                    │
│                      Port 80                             │
│                      Vue.js 3 + Vite                     │
└────────────────┬────────────────────────────────────────┘
                 │
                 │ Reverse Proxy /api/* → backend:3000
                 │
┌────────────────▼────────────────────────────────────────┐
│                      Backend (Node.js)                   │
│                      Port 3000                           │
│                      Express + JWT Auth                  │
└──────┬──────────────────────────────────┬───────────────┘
       │                                  │
       │                                  │
┌──────▼──────────────┐         ┌────────▼──────────────┐
│    MongoDB 7.0      │         │  Elasticsearch 8.13   │
│    Port 27017       │         │  Port 9200/9300       │
│    Auth Enabled     │         │  Single Node          │
└─────────────────────┘         └───────────────────────┘
```

## 🔧 Common Operations

### View Logs
```bash
# All services
docker compose -f deploy/config/docker-compose.yml logs -f

# Specific service
docker compose -f deploy/config/docker-compose.yml logs -f backend
```

### Restart Services
```bash
# All services
docker compose -f deploy/config/docker-compose.yml restart

# Specific service
docker compose -f deploy/config/docker-compose.yml restart backend
```

### Stop Services
```bash
docker compose -f deploy/config/docker-compose.yml down

# Stop and remove volumes
docker compose -f deploy/config/docker-compose.yml down -v
```

### Rebuild Images
```bash
docker compose -f deploy/config/docker-compose.yml build --no-cache
docker compose -f deploy/config/docker-compose.yml up -d
```

## 🧪 Testing

### Run All Tests
```bash
# Bash version (no Node.js required)
./deploy/scripts/test-api.sh

# Node.js version
node deploy/scripts/test-endpoints.js
```

### Test Results
- **20 test cases** covering full user flow
- **100% pass rate** expected on healthy deployment
- Tests include: Auth, Categories, Products, Cart, Blogs, User Profile

## 📝 Environment Variables

Key environment variables in `.env`:

```env
# MongoDB
MONGO_URL=mongodb://admin:password123@mongodb:27017/ecommercedb?authSource=admin

# JWT Secrets
JWT_SECRET=your-jwt-secret-key
REFRESH_TOKEN_SECRET=your-refresh-token-secret

# Email Configuration (optional)
EMAIL_HOST=smtp.gmail.com
EMAIL_PORT=587
EMAIL_USER=your-email@gmail.com
EMAIL_PASS=your-app-password

# Cloudinary (optional, for image uploads)
CLOUDINARY_CLOUD_NAME=your-cloud-name
CLOUDINARY_API_KEY=your-api-key
CLOUDINARY_API_SECRET=your-api-secret
```

## 🆘 Troubleshooting

### Container Issues
```bash
# Check container status
docker compose -f deploy/config/docker-compose.yml ps

# Check container logs
docker compose -f deploy/config/docker-compose.yml logs backend

# Restart problematic container
docker compose -f deploy/config/docker-compose.yml restart backend
```

### Database Issues
```bash
# Access MongoDB shell
docker compose -f deploy/config/docker-compose.yml exec mongodb mongosh -u admin -p password123

# Import database dump
./deploy/scripts/import-db.sh
```

### Network Issues
```bash
# Check Docker networks
docker network ls

# Inspect network
docker network inspect node-ecommerce-main_default
```

## 📖 Additional Resources

- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Nginx Documentation](https://nginx.org/en/docs/)
- [MongoDB Documentation](https://docs.mongodb.com/)
- [Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html)

## 🤝 Contributing

When adding new deployment features:
1. Update relevant documentation in `docs/`
2. Add scripts to `scripts/` with proper permissions
3. Update configuration files in `config/`
4. Update this README with new information

## 📄 License

See LICENSE file in the root directory.
