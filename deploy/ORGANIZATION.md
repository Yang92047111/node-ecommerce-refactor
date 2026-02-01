# Deployment Files Organization

This document explains how all deployment-related files have been organized in the `deploy/` directory for better maintainability and clarity.

## 📁 Directory Structure

```
deploy/
├── config/                          # Configuration files
│   ├── .dockerignore               # Backend Docker build exclusions
│   ├── .dockerignore.frontend      # Frontend Docker build exclusions
│   ├── .env.example                # Environment variables template
│   ├── docker-compose.yml          # Multi-container orchestration
│   ├── Dockerfile                  # Backend Docker image
│   ├── Dockerfile.frontend         # Frontend Docker image
│   └── nginx.conf                  # Nginx web server configuration
├── docs/                           # Documentation
│   ├── API_TESTING.md              # API endpoint testing guide
│   ├── DEPLOYMENT_SUMMARY.md       # Complete deployment summary
│   ├── DOCKER_DEPLOYMENT.md        # Detailed Docker instructions
│   ├── GET_STARTED.md              # Beginner's deployment guide
│   ├── QUICK_REFERENCE.md          # Quick command reference
│   └── TEST_IMPROVEMENTS.md        # Test suite enhancements
├── scripts/                        # Executable scripts
│   ├── deploy.sh                   # Interactive deployment script
│   ├── import-db.sh                # Database import utility
│   ├── test-api.sh                 # Bash API testing suite
│   └── test-endpoints.js           # Node.js API testing suite
├── ORGANIZATION.md                 # This file
└── README.md                       # Deployment overview
```

## 📦 What's Included

### Configuration Files (`config/`)
All Docker and configuration files needed for deployment:
- **Docker Compose**: Orchestrates all services (MongoDB, Elasticsearch, Backend, Frontend)
- **Dockerfiles**: Build instructions for backend and frontend containers
- **Nginx Config**: Reverse proxy and static file serving (also copied to frontend/ for Docker build)
- **Environment Template**: Example configuration with all required variables

**Note**: `nginx.conf` is kept in both `deploy/config/` and `frontend/` directories. The copy in `frontend/` is required for the Docker build process, while the one in `deploy/config/` serves as the source of truth.

### Documentation (`docs/`)
Comprehensive guides for different deployment scenarios:
- **GET_STARTED.md**: Perfect for beginners, step-by-step instructions
- **DOCKER_DEPLOYMENT.md**: In-depth Docker deployment details
- **API_TESTING.md**: How to test all API endpoints
- **QUICK_REFERENCE.md**: Handy commands and troubleshooting
- **DEPLOYMENT_SUMMARY.md**: Architecture and complete overview

### Scripts (`scripts/`)
Automated scripts for deployment and testing:
- **deploy.sh**: Interactive menu-driven deployment with health checks
- **import-db.sh**: Import MongoDB data from dump files
- **test-api.sh**: Comprehensive Bash-based API testing (20 tests)
- **test-endpoints.js**: Node.js-based API testing suite

## 🔄 File Paths Updated

All scripts and configuration files have been updated with correct relative paths:

### Docker Compose (`config/docker-compose.yml`)
```yaml
# Backend build context
build:
  context: ../..                    # Project root
  dockerfile: deploy/config/Dockerfile

# Frontend build context
build:
  context: ../../frontend
  dockerfile: ../deploy/config/Dockerfile.frontend

# Volume mounts
volumes:
  - ../../dump:/docker-entrypoint-initdb.d
  - ../../uploads:/app/uploads
```

### Deploy Script (`scripts/deploy.sh`)
```bash
# Automatically finds project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$PROJECT_ROOT"

# Uses correct compose file path
docker compose -f deploy/config/docker-compose.yml up -d
```

## 🚀 How to Use

### From Project Root
```bash
# Run deployment
./deploy/scripts/deploy.sh

# Run tests
./deploy/scripts/test-api.sh

# Import database
./deploy/scripts/import-db.sh

# Deploy manually
docker compose -f deploy/config/docker-compose.yml up -d
```

### From Any Directory
The deploy.sh script automatically navigates to the project root, so you can run it from anywhere:
```bash
cd /path/to/project
./deploy/scripts/deploy.sh
# Script automatically changes to project root
```

## 📝 Environment Setup

1. **Copy environment template:**
   ```bash
   cp deploy/config/.env.example .env
   ```

2. **Edit with your values:**
   ```bash
   nano .env  # or vim, code, etc.
   ```

3. **Required variables:**
   - `MONGO_ROOT_USERNAME` / `MONGO_ROOT_PASSWORD`
   - `JWT_SECRET` / `REFRESH_TOKEN_SECRET`
   - `CLOUDINARY_*` (optional, for image uploads)
   - `EMAIL_*` (optional, for email features)

## 🔍 Finding Documentation

All documentation is organized by purpose:

| Need to... | Read this |
|------------|-----------|
| Start from scratch | `docs/GET_STARTED.md` |
| Understand Docker setup | `docs/DOCKER_DEPLOYMENT.md` |
| Quick command lookup | `docs/QUICK_REFERENCE.md` |
| Test API endpoints | `docs/API_TESTING.md` |
| See architecture | `docs/DEPLOYMENT_SUMMARY.md` |

## 🛠️ Maintenance

### Adding New Scripts
1. Place in `deploy/scripts/`
2. Make executable: `chmod +x deploy/scripts/newscript.sh`
3. Update `deploy/README.md`
4. Update this file (ORGANIZATION.md)

### Adding Configuration
1. Place in `deploy/config/`
2. Update `docker-compose.yml` if needed
3. Document in relevant guides

### Adding Documentation
1. Place in `deploy/docs/`
2. Update `deploy/README.md` with link
3. Update main `README.md` if relevant

## ✅ Benefits of This Organization

1. **Clear Separation**: Deployment files are isolated from application code
2. **Easy Navigation**: Logical grouping makes files easy to find
3. **Better Git Management**: Can .gitignore or track deployment files separately
4. **Portable**: The entire deploy/ directory can be copied to other projects
5. **Documentation**: Everything deployment-related is in one place
6. **Maintainable**: Changes to deployment don't affect app structure

## 🔄 Migration Notes

If you have old paths in your scripts or documentation:

| Old Path | New Path |
|----------|----------|
| `./deploy.sh` | `./deploy/scripts/deploy.sh` |
| `./test-api.sh` | `./deploy/scripts/test-api.sh` |
| `./.env.example` | `./deploy/config/.env.example` |
| `./docker-compose.yml` | `./deploy/config/docker-compose.yml` |
| `./DOCKER_DEPLOYMENT.md` | `./deploy/docs/DOCKER_DEPLOYMENT.md` |

**Note**: The `.env` file stays in the project root for compatibility.

## 📚 Additional Resources

- [Main README](../../README.md) - Project overview
- [Deploy README](./README.md) - Deployment guide
- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Reference](https://docs.docker.com/compose/)

---

**Last Updated**: February 1, 2026
**Organization Version**: 1.0
