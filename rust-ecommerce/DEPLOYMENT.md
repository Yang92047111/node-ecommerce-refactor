# Deployment Guide

## E-Commerce Platform - Rust Backend

This guide covers deploying the Rust-based e-commerce backend to production.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Database Setup](#database-setup)
4. [Building for Production](#building-for-production)
5. [Deployment Options](#deployment-options)
6. [Configuration](#configuration)
7. [Monitoring & Logging](#monitoring--logging)
8. [Security Considerations](#security-considerations)
9. [Backup & Recovery](#backup--recovery)
10. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### System Requirements

- **OS**: Linux (Ubuntu 20.04+ recommended), macOS, or Windows Server
- **RAM**: Minimum 2GB, Recommended 4GB+
- **CPU**: 2+ cores recommended
- **Storage**: 10GB+ available space
- **Rust**: 1.70.0 or later
- **PostgreSQL**: 14.0 or later

### Required Tools

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib

# macOS
brew install postgresql@14

# Verify installations
rustc --version
psql --version
```

---

## Environment Setup

### 1. Clone the Repository

```bash
git clone <repository-url>
cd rust-ecommerce
```

### 2. Create Environment File

Create a `.env` file in the project root:

```env
# Server Configuration
PORT=8080
RUST_LOG=info
ALLOWED_ORIGIN=https://yourdomain.com

# Database Configuration
DATABASE_URL=postgresql://username:password@localhost:5432/ecommerce_db
DB_MAX_CONNECTIONS=20
DB_MIN_CONNECTIONS=5

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION=3600
REFRESH_TOKEN_SECRET=your-super-secret-refresh-token-key
REFRESH_TOKEN_EXPIRATION=604800

# Application Settings
MAX_FILE_SIZE=5242880
```

### 3. Secure Environment Variables

```bash
# Set proper permissions
chmod 600 .env

# Never commit .env to version control
echo ".env" >> .gitignore
```

---

## Database Setup

### 1. Create Database

```bash
# Connect to PostgreSQL
sudo -u postgres psql

# Create database and user
CREATE DATABASE ecommerce_db;
CREATE USER ecommerce_user WITH ENCRYPTED PASSWORD 'strong_password_here';
GRANT ALL PRIVILEGES ON DATABASE ecommerce_db TO ecommerce_user;
\q
```

### 2. Configure PostgreSQL for Production

Edit `/etc/postgresql/14/main/postgresql.conf`:

```conf
# Connection Settings
max_connections = 100
shared_buffers = 256MB
effective_cache_size = 1GB
maintenance_work_mem = 64MB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
work_mem = 2621kB
min_wal_size = 1GB
max_wal_size = 4GB

# Logging
logging_collector = on
log_directory = 'log'
log_filename = 'postgresql-%Y-%m-%d_%H%M%S.log'
log_rotation_age = 1d
log_rotation_size = 100MB
log_min_duration_statement = 1000
log_line_prefix = '%m [%p] %q%u@%d '
```

Edit `/etc/postgresql/14/main/pg_hba.conf`:

```conf
# Allow connections from application server
host    ecommerce_db    ecommerce_user    10.0.0.0/24    md5
```

Restart PostgreSQL:

```bash
sudo systemctl restart postgresql
```

### 3. Run Migrations

```bash
# Build the application first
cargo build --release

# Run migrations
DATABASE_URL=postgresql://ecommerce_user:password@localhost:5432/ecommerce_db \
./target/release/rust_ecommerce migrate
```

---

## Building for Production

### 1. Optimize Build

```bash
# Build with release profile
cargo build --release

# The binary will be at: target/release/rust_ecommerce
```

### 2. Verify Build

```bash
# Test the binary
./target/release/rust_ecommerce --version

# Run health check
curl http://localhost:8080/api/health
```

---

## Deployment Options

### Option 1: Systemd Service (Linux)

Create `/etc/systemd/system/ecommerce-api.service`:

```ini
[Unit]
Description=E-Commerce API Service
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=ecommerce
Group=ecommerce
WorkingDirectory=/opt/ecommerce-api
Environment="RUST_LOG=info"
EnvironmentFile=/opt/ecommerce-api/.env
ExecStart=/opt/ecommerce-api/rust_ecommerce
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=ecommerce-api

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/ecommerce-api/logs

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
# Create user and directory
sudo useradd -r -s /bin/false ecommerce
sudo mkdir -p /opt/ecommerce-api
sudo cp target/release/rust_ecommerce /opt/ecommerce-api/
sudo cp .env /opt/ecommerce-api/
sudo chown -R ecommerce:ecommerce /opt/ecommerce-api

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable ecommerce-api
sudo systemctl start ecommerce-api

# Check status
sudo systemctl status ecommerce-api
```

### Option 2: Docker Deployment

Create `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust_ecommerce /usr/local/bin/
COPY --from=builder /app/migrations /migrations

ENV RUST_LOG=info
EXPOSE 8080

CMD ["rust_ecommerce"]
```

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:14-alpine
    environment:
      POSTGRES_DB: ecommerce_db
      POSTGRES_USER: ecommerce_user
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ecommerce_user"]
      interval: 10s
      timeout: 5s
      retries: 5

  api:
    build: .
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgresql://ecommerce_user:${DB_PASSWORD}@postgres:5432/ecommerce_db
      JWT_SECRET: ${JWT_SECRET}
      REFRESH_TOKEN_SECRET: ${REFRESH_TOKEN_SECRET}
    depends_on:
      postgres:
        condition: service_healthy
    restart: unless-stopped

volumes:
  postgres_data:
```

Deploy with Docker:

```bash
# Build and start
docker-compose up -d

# View logs
docker-compose logs -f api

# Stop
docker-compose down
```

### Option 3: Kubernetes Deployment

Create `k8s-deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ecommerce-api
  labels:
    app: ecommerce-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ecommerce-api
  template:
    metadata:
      labels:
        app: ecommerce-api
    spec:
      containers:
      - name: api
        image: your-registry/ecommerce-api:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: ecommerce-secrets
              key: database-url
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: ecommerce-secrets
              key: jwt-secret
        livenessProbe:
          httpGet:
            path: /api/monitoring/liveness
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/monitoring/readiness
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
---
apiVersion: v1
kind: Service
metadata:
  name: ecommerce-api
spec:
  selector:
    app: ecommerce-api
  ports:
    - protocol: TCP
      port: 80
      targetPort: 8080
  type: LoadBalancer
```

Deploy to Kubernetes:

```bash
# Create secrets
kubectl create secret generic ecommerce-secrets \
  --from-literal=database-url='postgresql://...' \
  --from-literal=jwt-secret='...'

# Deploy
kubectl apply -f k8s-deployment.yaml

# Check status
kubectl get pods
kubectl logs -f deployment/ecommerce-api
```

---

## Configuration

### Nginx Reverse Proxy

Create `/etc/nginx/sites-available/ecommerce-api`:

```nginx
upstream ecommerce_backend {
    server 127.0.0.1:8080;
}

server {
    listen 80;
    server_name api.yourdomain.com;

    # Redirect to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/api.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourdomain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    client_max_body_size 10M;

    location / {
        proxy_pass http://ecommerce_backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    location /api/monitoring/health {
        proxy_pass http://ecommerce_backend;
        access_log off;
    }
}
```

Enable the site:

```bash
sudo ln -s /etc/nginx/sites-available/ecommerce-api /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## Monitoring & Logging

### 1. Application Logs

```bash
# Systemd service logs
sudo journalctl -u ecommerce-api -f

# Docker logs
docker-compose logs -f api

# Kubernetes logs
kubectl logs -f deployment/ecommerce-api
```

### 2. Health Monitoring

Set up health check monitoring:

```bash
# Create monitoring script
cat > /usr/local/bin/health-check.sh << 'EOF'
#!/bin/bash
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/monitoring/health)
if [ $RESPONSE -ne 200 ]; then
    echo "Health check failed with status $RESPONSE"
    systemctl restart ecommerce-api
fi
EOF

chmod +x /usr/local/bin/health-check.sh

# Add to crontab (check every 5 minutes)
crontab -e
*/5 * * * * /usr/local/bin/health-check.sh
```

### 3. Database Monitoring

```bash
# Monitor database connections
psql -U ecommerce_user -d ecommerce_db -c "
  SELECT count(*) as total_connections,
         count(*) FILTER (WHERE state = 'active') as active,
         count(*) FILTER (WHERE state = 'idle') as idle
  FROM pg_stat_activity
  WHERE datname = 'ecommerce_db';
"
```

---

## Security Considerations

### 1. Firewall Configuration

```bash
# Ubuntu/Debian with UFW
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable

# Only allow database access from application server
sudo ufw allow from 10.0.0.0/24 to any port 5432
```

### 2. SSL/TLS Certificate

```bash
# Install Certbot
sudo apt-get install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d api.yourdomain.com

# Auto-renewal
sudo certbot renew --dry-run
```

### 3. Security Headers

Already configured in the application, but verify with Nginx:

```nginx
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header Referrer-Policy "strict-origin-when-cross-origin" always;
```

---

## Backup & Recovery

### 1. Database Backup

```bash
# Create backup script
cat > /usr/local/bin/backup-db.sh << 'EOF'
#!/bin/bash
BACKUP_DIR="/var/backups/postgresql"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR

pg_dump -U ecommerce_user ecommerce_db | gzip > \
  $BACKUP_DIR/ecommerce_db_$TIMESTAMP.sql.gz

# Keep only last 7 days
find $BACKUP_DIR -name "*.sql.gz" -mtime +7 -delete
EOF

chmod +x /usr/local/bin/backup-db.sh

# Schedule daily backups at 2 AM
crontab -e
0 2 * * * /usr/local/bin/backup-db.sh
```

### 2. Database Recovery

```bash
# Restore from backup
gunzip -c /var/backups/postgresql/ecommerce_db_20260115_020000.sql.gz | \
  psql -U ecommerce_user ecommerce_db
```

---

## Troubleshooting

### Common Issues

#### 1. Connection Pool Exhausted

**Symptoms**: "Failed to acquire connection from pool"

**Solution**:
```env
# Increase pool size in .env
DB_MAX_CONNECTIONS=50
```

#### 2. High Memory Usage

**Solution**:
```bash
# Monitor memory
free -h
ps aux --sort=-%mem | head

# Reduce connections if needed
```

#### 3. Slow Queries

**Solution**:
```sql
-- Enable slow query logging
ALTER DATABASE ecommerce_db SET log_min_duration_statement = 1000;

-- Check slow queries
SELECT query, calls, total_time, mean_time
FROM pg_stat_statements
ORDER BY mean_time DESC
LIMIT 10;
```

#### 4. Database Migration Failures

**Solution**:
```bash
# Check migration status
sqlx migrate info

# Revert last migration
sqlx migrate revert

# Re-run migrations
sqlx migrate run
```

### Getting Help

1. Check application logs
2. Check database logs: `/var/log/postgresql/`
3. Verify network connectivity
4. Test database connection
5. Review monitoring endpoints

---

## Performance Tuning

### 1. Database Indexing

```sql
-- Check missing indexes
SELECT schemaname, tablename, attname, n_distinct, correlation
FROM pg_stats
WHERE schemaname = 'public'
  AND n_distinct > 100
ORDER BY abs(correlation) DESC;
```

### 2. Connection Pool Tuning

Adjust based on load:
- High traffic: `DB_MAX_CONNECTIONS=50-100`
- Medium traffic: `DB_MAX_CONNECTIONS=20-30`
- Low traffic: `DB_MAX_CONNECTIONS=10-20`

### 3. Caching Strategy

Consider adding Redis for caching:
```bash
# Install Redis
sudo apt-get install redis-server

# Configure in application
REDIS_URL=redis://localhost:6379
```

---

## Conclusion

Your e-commerce API is now deployed and ready for production use. Remember to:

- Monitor logs and metrics regularly
- Keep backups up to date
- Update dependencies and security patches
- Scale horizontally as traffic grows
- Implement proper disaster recovery procedures

For support, refer to the API documentation and project repository.
