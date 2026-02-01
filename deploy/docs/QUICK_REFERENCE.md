# Quick Reference Guide

## 🚀 Quick Start Commands

```bash
# 1. Setup environment
cp .env.example .env
# Edit .env with your values

# 2. Deploy everything
./deploy.sh

# 3. Test the API
node test-endpoints.js
```

## 📦 Docker Commands

### Deployment
```bash
# Full deployment
./deploy.sh                    # Interactive menu

# Manual deployment
docker compose up -d           # Start in background
docker compose up              # Start with logs
docker compose down            # Stop all services
docker compose down -v         # Stop and remove volumes
```

### Build & Restart
```bash
docker compose build           # Build all images
docker compose build backend   # Build specific service
docker compose restart         # Restart all
docker compose restart backend # Restart specific service
```

### Logs & Monitoring
```bash
docker compose logs -f         # Follow all logs
docker compose logs backend    # View backend logs
docker compose ps              # List containers
docker compose top             # Show processes
```

### Shell Access
```bash
docker compose exec backend sh         # Backend shell
docker compose exec mongodb mongosh    # MongoDB shell
docker compose exec frontend sh        # Frontend shell
```

## 🧪 Testing Commands

### Automated Tests
```bash
# Node.js comprehensive test
node test-endpoints.js

# With verbose output
VERBOSE=true node test-endpoints.js

# Bash simple test
./test-api.sh

# Custom API URL
API_URL=http://api.example.com node test-endpoints.js
```

### Manual Testing
```bash
# Health check
curl http://localhost:3000/

# Get products
curl http://localhost:3000/api/product/

# Login
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Pass123"}'
```

## 🌐 Access Points

| Service | URL | Description |
|---------|-----|-------------|
| Frontend | http://localhost | Vue.js application |
| Backend API | http://localhost:3000 | REST API |
| API Docs | http://localhost:3000/api-docs | Swagger documentation |
| MongoDB | localhost:27017 | Database |
| Elasticsearch | http://localhost:9200 | Search engine |

## 🗄️ Database Commands

### MongoDB
```bash
# Access MongoDB shell
docker compose exec mongodb mongosh

# Inside mongosh:
use ecommercedb
db.products.find()
db.users.find()
show collections

# Import data
./import-db.sh

# Backup
docker compose exec mongodb mongodump --out /data/backup

# Restore
docker compose exec mongodb mongorestore /data/backup
```

### Elasticsearch
```bash
# Check cluster health
curl http://localhost:9200/_cluster/health

# List indices
curl http://localhost:9200/_cat/indices

# Search products
curl http://localhost:9200/products/_search?q=phone
```

## 🔧 Troubleshooting

### Service Won't Start
```bash
# Check logs
docker compose logs [service-name]

# Restart service
docker compose restart [service-name]

# Rebuild service
docker compose up -d --build [service-name]
```

### Port Conflicts
```bash
# Find process using port
lsof -i :3000
lsof -i :27017
lsof -i :9200
lsof -i :80

# Kill process
kill -9 [PID]
```

### Clean Slate
```bash
# Remove everything
docker compose down -v
docker system prune -a

# Rebuild from scratch
./deploy.sh
```

### Check Health
```bash
# All services status
docker compose ps

# Health checks
curl http://localhost:3000/
curl http://localhost:9200/_cluster/health
docker compose exec mongodb mongosh --eval "db.adminCommand('ping')"
```

## 📝 Environment Variables

### Required
```bash
NODE_ENV=production
MONGO_URL=mongodb://...
JWT_SECRET=your-secret
REFRESH_TOKEN_SECRET=your-refresh-secret
```

### Optional
```bash
CLOUDINARY_CLOUD_NAME=your_cloud
CLOUDINARY_API_KEY=your_key
CLOUDINARY_API_SECRET=your_secret
EMAIL_USER=your@email.com
EMAIL_PASSWORD=your_app_password
```

## 🔐 Common API Endpoints

### Authentication
- POST `/api/auth/register` - Register user
- POST `/api/auth/login` - Login
- POST `/api/auth/logout` - Logout
- POST `/api/auth/refresh` - Refresh token

### Products
- GET `/api/product/` - List products
- GET `/api/product/:id` - Get product
- POST `/api/product/` - Create product 🔒
- PUT `/api/product/:id` - Update product 🔒
- DELETE `/api/product/:id` - Delete product 🔒

### Cart
- GET `/api/cart/` - Get cart 🔒
- POST `/api/cart/add` - Add to cart 🔒
- PUT `/api/cart/update` - Update cart 🔒
- DELETE `/api/cart/:id` - Remove from cart 🔒

### Orders
- GET `/api/order/user-orders` - Get orders 🔒
- GET `/api/order/:id` - Get order details 🔒
- POST `/api/order/create` - Create order 🔒

### Categories
- GET `/api/category/` - List categories
- GET `/api/category/:id` - Get category
- POST `/api/category/` - Create category 🔒

### Search
- GET `/api/search/products?q=query` - Search products

🔒 = Requires authentication token

## 📊 Performance Tips

### Monitor Resources
```bash
# Docker stats
docker stats

# Container resource usage
docker compose exec backend top

# Check memory
docker compose exec mongodb free -m
```

### Optimize Database
```bash
# MongoDB
docker compose exec mongodb mongosh
db.products.createIndex({title: "text"})
db.products.createIndex({category: 1})

# Elasticsearch
curl -X PUT http://localhost:9200/products/_settings \
  -H "Content-Type: application/json" \
  -d '{"index":{"number_of_replicas":0}}'
```

## 🆘 Quick Fixes

### Backend API Not Responding
```bash
docker compose restart backend
docker compose logs backend
```

### Database Connection Failed
```bash
docker compose restart mongodb
docker compose exec mongodb mongosh --eval "db.adminCommand('ping')"
```

### Frontend Not Loading
```bash
docker compose restart frontend
curl http://localhost/health
```

### Out of Memory
```bash
# Increase Docker memory in Docker Desktop settings
# Or reduce ES memory in docker compose.yml:
# ES_JAVA_OPTS=-Xms256m -Xmx256m
```

## 📚 Documentation

- **Full Deployment Guide**: [DOCKER_DEPLOYMENT.md](./DOCKER_DEPLOYMENT.md)
- **API Testing Guide**: [API_TESTING.md](./API_TESTING.md)
- **Project README**: [README.md](./README.md)
- **Swagger API Docs**: http://localhost:3000/api-docs

## 🎯 Common Workflows

### Development Workflow
```bash
# 1. Start services
docker compose up -d

# 2. Watch logs
docker compose logs -f backend

# 3. Make code changes (hot reload enabled)

# 4. Test changes
node test-endpoints.js

# 5. Restart if needed
docker compose restart backend
```

### Production Deployment
```bash
# 1. Configure environment
cp .env.example .env
# Edit .env with production values

# 2. Deploy
./deploy.sh

# 3. Verify
docker compose ps
node test-endpoints.js

# 4. Monitor
docker compose logs -f
```

### Backup & Restore
```bash
# Backup
docker compose exec mongodb mongodump --out /data/backup
docker cp ecommerce-mongodb:/data/backup ./backup-$(date +%Y%m%d)

# Restore
docker cp ./backup ecommerce-mongodb:/data/restore
docker compose exec mongodb mongorestore /data/restore
```

---

**Need more help?** See the detailed guides:
- [DOCKER_DEPLOYMENT.md](./DOCKER_DEPLOYMENT.md) - Complete deployment guide
- [API_TESTING.md](./API_TESTING.md) - Comprehensive testing guide
