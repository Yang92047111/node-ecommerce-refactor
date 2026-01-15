# Monitoring and Backup Strategy

## Overview

This document outlines the monitoring and backup strategies for the production Rust e-commerce backend.

---

## Monitoring Strategy

### Application Monitoring

#### Health Checks

The application exposes health check endpoints:

```bash
# Basic health check
curl http://localhost:8080/api/health

# Database health check
curl http://localhost:8080/api/health/db
```

Set up automated health checks with a monitoring service:

```bash
# Example cron job for health monitoring
*/5 * * * * curl -f http://localhost:8080/api/health || echo "API health check failed" | mail -s "API Down" admin@example.com
```

#### Application Logs

Configure structured logging in production:

```env
# .env
RUST_LOG=info,rust_ecommerce=info,sqlx=warn
```

Log locations:
- Application logs: `/var/log/ecommerce-api/app.log`
- Error logs: `/var/log/ecommerce-api/error.log`
- Access logs: `/var/log/ecommerce-api/access.log`

**Log Rotation:**

Create `/etc/logrotate.d/ecommerce-api`:

```
/var/log/ecommerce-api/*.log {
    daily
    rotate 30
    compress
    delaycompress
    notifempty
    create 0644 ecommerce-api ecommerce-api
    sharedscripts
    postrotate
        systemctl reload ecommerce-api
    endscript
}
```

#### Metrics Collection

Expose Prometheus metrics endpoint:

```rust
// Already implemented in middleware
// GET /metrics
```

Configure Prometheus scraper in `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'ecommerce-api'
    scrape_interval: 15s
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
```

**Key Metrics to Monitor:**

- Request rate (requests per second)
- Response time (p50, p95, p99)
- Error rate (4xx, 5xx errors)
- Database connection pool usage
- Active connections
- Memory usage
- CPU usage

#### Grafana Dashboards

Create dashboards for:

1. **System Overview**
   - Request rate
   - Response times
   - Error rates
   - Active users

2. **Database Performance**
   - Query execution times
   - Connection pool utilization
   - Slow queries
   - Transaction rates

3. **Business Metrics**
   - Orders per hour
   - Revenue trends
   - Popular products
   - User registrations

### Database Monitoring

#### PostgreSQL Monitoring

Install and configure `pg_stat_statements`:

```sql
-- Enable pg_stat_statements
CREATE EXTENSION IF NOT EXISTS pg_stat_statements;

-- Check slow queries
SELECT 
    query,
    calls,
    total_time / 1000 as total_time_seconds,
    mean_time / 1000 as mean_time_seconds,
    max_time / 1000 as max_time_seconds
FROM pg_stat_statements
WHERE mean_time > 1000  -- queries taking more than 1 second
ORDER BY mean_time DESC
LIMIT 20;
```

#### Database Size Monitoring

```sql
-- Monitor database size
SELECT 
    pg_size_pretty(pg_database_size('ecommerce_db')) as db_size;

-- Monitor table sizes
SELECT 
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) as size
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;
```

#### Connection Monitoring

```sql
-- Check active connections
SELECT 
    datname,
    count(*) as connections,
    max_conn,
    round((count(*) * 100.0 / max_conn), 2) as percent_used
FROM pg_stat_activity, 
     (SELECT setting::int AS max_conn FROM pg_settings WHERE name='max_connections') mc
WHERE datname = 'ecommerce_db'
GROUP BY datname, max_conn;
```

### Alerting

Set up alerts for critical conditions:

#### Uptime Robot / StatusCake

Configure uptime monitoring:
- Check interval: 1 minute
- Alert on: 3 consecutive failures
- Notification channels: Email, Slack, SMS

#### PagerDuty / Opsgenie

Set up incident management:
- Critical: API down, database connection failures
- High: High error rates (>5%), slow response times (>2s)
- Medium: Elevated memory usage, connection pool warnings
- Low: Disk space warnings, log rotation issues

#### Alert Thresholds

```yaml
alerts:
  - name: HighErrorRate
    condition: error_rate > 5%
    duration: 5m
    severity: high
    
  - name: SlowResponseTime
    condition: p95_response_time > 2s
    duration: 10m
    severity: high
    
  - name: DatabaseConnectionPoolHigh
    condition: db_pool_usage > 90%
    duration: 5m
    severity: medium
    
  - name: HighMemoryUsage
    condition: memory_usage > 90%
    duration: 10m
    severity: medium
    
  - name: DiskSpaceWarning
    condition: disk_usage > 80%
    duration: 30m
    severity: low
```

---

## Backup Strategy

### Database Backups

#### Automated Daily Backups

Create `/opt/scripts/backup_postgres.sh`:

```bash
#!/bin/bash

# PostgreSQL Backup Script
# Run daily via cron

BACKUP_DIR="/var/backups/postgresql"
DB_NAME="ecommerce_db"
DB_USER="ecommerce_user"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="${BACKUP_DIR}/${DB_NAME}_${TIMESTAMP}.sql.gz"
RETENTION_DAYS=30

# Create backup directory if not exists
mkdir -p $BACKUP_DIR

# Perform backup
echo "Starting backup of $DB_NAME..."
pg_dump -U $DB_USER -h localhost $DB_NAME | gzip > $BACKUP_FILE

# Check if backup was successful
if [ $? -eq 0 ]; then
    echo "Backup successful: $BACKUP_FILE"
    
    # Calculate backup size
    SIZE=$(du -h $BACKUP_FILE | cut -f1)
    echo "Backup size: $SIZE"
    
    # Remove old backups
    find $BACKUP_DIR -name "${DB_NAME}_*.sql.gz" -mtime +$RETENTION_DAYS -delete
    echo "Old backups removed (older than $RETENTION_DAYS days)"
else
    echo "Backup failed!"
    exit 1
fi

# Optional: Upload to S3
# aws s3 cp $BACKUP_FILE s3://your-backup-bucket/postgres/

echo "Backup completed at $(date)"
```

Make executable and set up cron:

```bash
chmod +x /opt/scripts/backup_postgres.sh

# Add to cron (run at 2 AM daily)
crontab -e
0 2 * * * /opt/scripts/backup_postgres.sh >> /var/log/postgresql/backup.log 2>&1
```

#### Point-in-Time Recovery (PITR)

Enable WAL archiving in PostgreSQL:

```conf
# /etc/postgresql/14/main/postgresql.conf
wal_level = replica
archive_mode = on
archive_command = 'test ! -f /var/lib/postgresql/archive/%f && cp %p /var/lib/postgresql/archive/%f'
archive_timeout = 3600
```

Create archive directory:

```bash
mkdir -p /var/lib/postgresql/archive
chown postgres:postgres /var/lib/postgresql/archive
```

#### Weekly Full Backups

Create `/opt/scripts/backup_postgres_full.sh`:

```bash
#!/bin/bash

# Full PostgreSQL backup including WAL files

BACKUP_DIR="/var/backups/postgresql/full"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="full_backup_${TIMESTAMP}"

mkdir -p $BACKUP_DIR

echo "Starting full backup..."
pg_basebackup -U postgres -D ${BACKUP_DIR}/${BACKUP_NAME} -Ft -z -P

# Upload to S3 for off-site backup
# aws s3 sync ${BACKUP_DIR}/${BACKUP_NAME} s3://your-backup-bucket/postgres/full/${BACKUP_NAME}/

echo "Full backup completed: ${BACKUP_DIR}/${BACKUP_NAME}"
```

Set up weekly cron:

```bash
# Run every Sunday at 3 AM
0 3 * * 0 /opt/scripts/backup_postgres_full.sh >> /var/log/postgresql/backup_full.log 2>&1
```

### Application Backups

#### Configuration Backups

Back up configuration files:

```bash
#!/bin/bash
# /opt/scripts/backup_configs.sh

BACKUP_DIR="/var/backups/configs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
CONFIG_ARCHIVE="${BACKUP_DIR}/configs_${TIMESTAMP}.tar.gz"

mkdir -p $BACKUP_DIR

tar -czf $CONFIG_ARCHIVE \
    /etc/systemd/system/ecommerce-api.service \
    /etc/nginx/sites-available/ecommerce-api \
    /home/ecommerce-api/.env \
    /etc/postgresql/14/main/postgresql.conf

echo "Configuration backup created: $CONFIG_ARCHIVE"
```

#### Application Code Backups

Use Git tags for versioning:

```bash
# Tag releases
git tag -a v1.0.0 -m "Production release 1.0.0"
git push origin v1.0.0

# Back up to remote repository
git push origin main
git push --tags
```

### Backup Verification

#### Test Restores

Perform monthly test restores:

```bash
#!/bin/bash
# /opt/scripts/test_restore.sh

# Create test database
psql -U postgres -c "CREATE DATABASE ecommerce_db_test;"

# Restore from latest backup
LATEST_BACKUP=$(ls -t /var/backups/postgresql/*.sql.gz | head -1)
gunzip -c $LATEST_BACKUP | psql -U postgres ecommerce_db_test

# Verify data
psql -U postgres ecommerce_db_test -c "SELECT COUNT(*) FROM users;"
psql -U postgres ecommerce_db_test -c "SELECT COUNT(*) FROM products;"

# Cleanup
psql -U postgres -c "DROP DATABASE ecommerce_db_test;"

echo "Restore test completed successfully"
```

### Off-Site Backups

#### AWS S3 Backup

Configure AWS CLI and sync backups:

```bash
# Install AWS CLI
apt-get install awscli

# Configure credentials
aws configure

# Create S3 bucket
aws s3 mb s3://ecommerce-backups

# Sync backups daily
aws s3 sync /var/backups/postgresql/ s3://ecommerce-backups/postgresql/ --storage-class STANDARD_IA
```

#### Backup Encryption

Encrypt backups before uploading:

```bash
# Encrypt backup
gpg --symmetric --cipher-algo AES256 backup_file.sql.gz

# Upload encrypted backup
aws s3 cp backup_file.sql.gz.gpg s3://ecommerce-backups/

# Decrypt when needed
gpg --decrypt backup_file.sql.gz.gpg > backup_file.sql.gz
```

### Recovery Procedures

#### Database Recovery

1. **Standard Recovery** (from daily backup):

```bash
# Stop application
sudo systemctl stop ecommerce-api

# Drop and recreate database
psql -U postgres -c "DROP DATABASE ecommerce_db;"
psql -U postgres -c "CREATE DATABASE ecommerce_db;"

# Restore from backup
gunzip -c /var/backups/postgresql/ecommerce_db_20260115_020000.sql.gz | \
  psql -U postgres ecommerce_db

# Start application
sudo systemctl start ecommerce-api
```

2. **Point-in-Time Recovery**:

```bash
# Stop PostgreSQL
sudo systemctl stop postgresql

# Restore base backup
rm -rf /var/lib/postgresql/14/main
pg_basebackup -U postgres -D /var/lib/postgresql/14/main

# Configure recovery
cat > /var/lib/postgresql/14/main/recovery.conf << EOF
restore_command = 'cp /var/lib/postgresql/archive/%f %p'
recovery_target_time = '2026-01-15 14:30:00'
EOF

# Start PostgreSQL
sudo systemctl start postgresql
```

#### Application Recovery

```bash
# Pull latest stable version
git fetch origin
git checkout tags/v1.0.0

# Rebuild
cargo build --release

# Restart service
sudo systemctl restart ecommerce-api
```

### Backup Checklist

- [ ] Daily automated backups configured
- [ ] Weekly full backups scheduled
- [ ] Backup retention policy set (30 days)
- [ ] Off-site backups to S3 configured
- [ ] Backup encryption enabled
- [ ] Monthly restore tests scheduled
- [ ] Backup monitoring and alerts set up
- [ ] Recovery procedures documented
- [ ] Team trained on recovery procedures

---

## Disaster Recovery Plan

### Recovery Time Objective (RTO)

Target time to restore service: **4 hours**

### Recovery Point Objective (RPO)

Maximum acceptable data loss: **24 hours** (daily backups)

### DR Procedures

1. **Assess the situation**
   - Determine scope of failure
   - Notify stakeholders
   - Activate DR team

2. **Database Recovery**
   - Restore from latest backup
   - Verify data integrity
   - Apply WAL logs if needed

3. **Application Recovery**
   - Deploy application to standby server
   - Update DNS/load balancer
   - Verify functionality

4. **Post-Recovery**
   - Monitor for issues
   - Document incident
   - Update procedures
   - Conduct post-mortem

### Contact Information

**DR Team:**
- Primary: [Name] - [Phone] - [Email]
- Secondary: [Name] - [Phone] - [Email]
- Database Admin: [Name] - [Phone] - [Email]

**External Services:**
- Hosting Provider Support: [Contact]
- Database Support: [Contact]
- DNS Provider: [Contact]

---

## Conclusion

This monitoring and backup strategy ensures:

- Proactive issue detection through comprehensive monitoring
- Data safety through automated backups
- Quick recovery through tested procedures
- Minimal data loss with point-in-time recovery
- Business continuity through disaster recovery planning

Review and update this document quarterly or after any major infrastructure changes.
