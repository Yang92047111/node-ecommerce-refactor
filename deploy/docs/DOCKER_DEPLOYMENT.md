# Docker Deployment Guide

This guide explains how to deploy the E-commerce application using Docker containers.

## 📋 Prerequisites

- Docker (version 20.10 or higher)
- Docker Compose (version 2.0 or higher)
- At least 4GB of available RAM
- Ports 80, 3000, 9200, and 27017 available on your system

## 🏗️ Architecture

The application consists of 4 main services:

1. **MongoDB** - Database (Port 27017)
2. **Elasticsearch** - Search engine (Port 9200)
3. **Backend API** - Node.js/Express (Port 3000)
4. **Frontend** - Vue.js with Nginx (Port 80)

## 🚀 Quick Start

### 1. Clone and Setup

```bash
cd /Users/yang/Desktop/Career-Haking/node-ecommerce-main
```

### 2. Configure Environment

Copy the example environment file and update with your values:

```bash
cp .env.example .env
```

Edit `.env` file and update these critical values:
- `JWT_SECRET` - Your secure JWT secret
- `REFRESH_TOKEN_SECRET` - Your secure refresh token secret
- `CLOUDINARY_*` - Your Cloudinary credentials (for image uploads)
- `EMAIL_*` - Your email configuration (for notifications)

### 3. Deploy with One Command

Make the deployment script executable and run it:

```bash
chmod +x deploy.sh
./deploy.sh
```

Select option **1** for full deployment.

### 4. Alternative: Manual Deployment

```bash
# Build images
docker compose build

# Start all services
docker compose up -d

# Check service status
docker compose ps

# View logs
docker compose logs -f
```

## 📊 Import Sample Data (Optional)

If you want to import the sample database:

```bash
chmod +x import-db.sh
./import-db.sh
```

## 🌐 Access Points

After deployment, access the application at:

- **Frontend**: http://localhost
- **Backend API**: http://localhost:3000
- **API Documentation**: http://localhost:3000/api-docs
- **MongoDB**: localhost:27017
- **Elasticsearch**: http://localhost:9200

## 🧪 Testing the Deployment

### Option 1: Node.js Test Script (Recommended)

Run the comprehensive test suite:

```bash
node test-endpoints.js
```

For verbose output:

```bash
VERBOSE=true node test-endpoints.js
```

### Option 2: Bash Test Script

```bash
chmod +x test-api.sh
./test-api.sh
```

### Option 3: Manual Testing with curl

```bash
# Health check
curl http://localhost:3000/

# Get products
curl http://localhost:3000/api/product/

# Register user
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "firstName": "John",
    "lastName": "Doe",
    "email": "john@example.com",
    "mobile": "1234567890",
    "password": "SecurePass123"
  }'
```

## 🔧 Common Commands

### View Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f backend
docker compose logs -f mongodb
```

### Restart Services

```bash
# Restart all
docker compose restart

# Restart specific service
docker compose restart backend
```

### Stop Services

```bash
# Stop all containers
docker compose down

# Stop and remove volumes (⚠️ deletes data)
docker compose down -v
```

### Rebuild After Code Changes

```bash
# Rebuild specific service
docker compose build backend

# Rebuild and restart
docker compose up -d --build backend
```

### Execute Commands in Container

```bash
# Access MongoDB shell
docker compose exec mongodb mongosh

# Access backend container shell
docker compose exec backend sh

# Check Node.js version in backend
docker compose exec backend node --version
```

## 🐛 Troubleshooting

### Port Already in Use

If you get "port already allocated" error:

```bash
# Find process using port 3000
lsof -i :3000

# Kill the process
kill -9 <PID>

# Or change port in docker compose.yml
```

### MongoDB Connection Issues

```bash
# Check MongoDB logs
docker compose logs mongodb

# Restart MongoDB
docker compose restart mongodb

# Verify MongoDB is accessible
docker compose exec mongodb mongosh --eval "db.adminCommand('ping')"
```

### Backend Not Starting

```bash
# Check backend logs
docker compose logs backend

# Verify environment variables
docker compose exec backend env | grep MONGO

# Restart backend
docker compose restart backend
```

### Elasticsearch Memory Issues

If Elasticsearch fails to start due to memory:

```bash
# Increase Docker memory limit in Docker Desktop
# Settings > Resources > Memory (increase to at least 4GB)

# Or reduce Elasticsearch memory in docker compose.yml:
# ES_JAVA_OPTS=-Xms256m -Xmx256m
```

### Image Build Failures

```bash
# Clear Docker cache and rebuild
docker compose build --no-cache

# Remove all unused images
docker system prune -a
```

## 🔐 Security Considerations

### For Production Deployment:

1. **Change all default passwords** in `.env`
2. **Use strong JWT secrets** (at least 32 characters)
3. **Enable MongoDB authentication** (already configured)
4. **Use HTTPS** with a reverse proxy (Nginx/Traefik)
5. **Set proper CORS policies** in backend
6. **Enable Elasticsearch security** if exposed
7. **Use Docker secrets** for sensitive data
8. **Regular updates** of base images

### Example Production .env:

```bash
NODE_ENV=production
JWT_SECRET=$(openssl rand -base64 32)
REFRESH_TOKEN_SECRET=$(openssl rand -base64 32)
MONGO_ROOT_PASSWORD=$(openssl rand -base64 24)
```

## 📈 Scaling

### Horizontal Scaling

To run multiple backend instances:

```bash
docker compose up -d --scale backend=3
```

Add a load balancer (Nginx/HAProxy) in front.

### Resource Limits

Add resource limits in docker compose.yml:

```yaml
services:
  backend:
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

## 🔄 CI/CD Integration

### GitHub Actions Example

```yaml
name: Deploy
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Deploy
        run: |
          docker compose build
          docker compose up -d
```

## 📝 Backup and Restore

### Backup MongoDB Data

```bash
# Create backup
docker compose exec -T mongodb mongodump \
  --username admin \
  --password password123 \
  --authenticationDatabase admin \
  --out /data/backup

# Copy backup to host
docker cp ecommerce-mongodb:/data/backup ./backup
```

### Restore MongoDB Data

```bash
# Copy backup to container
docker cp ./backup ecommerce-mongodb:/data/backup

# Restore
docker compose exec -T mongodb mongorestore \
  --username admin \
  --password password123 \
  --authenticationDatabase admin \
  /data/backup
```

## 🆘 Getting Help

If you encounter issues:

1. Check logs: `docker compose logs -f`
2. Verify all services are healthy: `docker compose ps`
3. Check environment variables: `cat .env`
4. Ensure ports are available: `netstat -an | grep LISTEN`
5. Review this documentation

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [MongoDB Docker Image](https://hub.docker.com/_/mongo)
- [Elasticsearch Docker Image](https://www.elastic.co/guide/en/elasticsearch/reference/current/docker.html)
- [Node.js Best Practices](https://github.com/goldbergyoni/nodebestpractices)

## 🎯 Next Steps

After successful deployment:

1. Configure Cloudinary for image uploads
2. Set up email notifications
3. Customize frontend branding
4. Configure production domain
5. Set up SSL/TLS certificates
6. Configure monitoring and logging
7. Set up automated backups
8. Implement rate limiting
9. Add health monitoring
10. Configure CDN for static assets
