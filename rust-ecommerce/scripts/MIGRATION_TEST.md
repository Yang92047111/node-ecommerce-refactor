# Data Migration Test Guide

This document describes how to test the MongoDB to PostgreSQL migration.

## Prerequisites

1. PostgreSQL server running
2. Python 3.x installed
3. MongoDB dump files in `dump/ecommercedb/`
4. `.env` file configured with `DATABASE_URL`

## Test Migration Steps

### 1. Prepare Test Database

Create a test database for migration:

```bash
# Using psql
psql -U postgres
CREATE DATABASE ecommerce_test;
\q

# Or using Docker
docker run --name postgres-test \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=ecommerce_test \
  -p 5432:5432 \
  -d postgres:15
```

### 2. Run Database Migrations

Apply the schema to the test database:

```bash
cd rust-ecommerce

# Set test database URL
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/ecommerce_test"

# Run migrations using sqlx
sqlx migrate run --database-url $DATABASE_URL
```

### 3. Install Python Dependencies

```bash
cd scripts
pip3 install -r requirements.txt
```

### 4. Run Migration Script

```bash
# Using the shell script (recommended)
./run_migration.sh ../../dump/ecommercedb

# Or manually
python3 migrate_mongo_to_postgres.py \
    --dump-dir ../../dump/ecommercedb \
    --db-url "$DATABASE_URL" \
    --clear
```

### 5. Verify Migration

Connect to the database and verify data:

```sql
-- Check record counts
SELECT 'users' as table_name, COUNT(*) as count FROM users
UNION ALL
SELECT 'categories', COUNT(*) FROM categories
UNION ALL
SELECT 'products', COUNT(*) FROM products
UNION ALL
SELECT 'carts', COUNT(*) FROM carts
UNION ALL
SELECT 'cart_items', COUNT(*) FROM cart_items
UNION ALL
SELECT 'orders', COUNT(*) FROM orders
UNION ALL
SELECT 'order_items', COUNT(*) FROM order_items
UNION ALL
SELECT 'coupons', COUNT(*) FROM coupons
UNION ALL
SELECT 'blogs', COUNT(*) FROM blogs;

-- Verify relationships
SELECT u.email, COUNT(o.id) as order_count
FROM users u
LEFT JOIN orders o ON u.id = o.user_id
GROUP BY u.email
ORDER BY order_count DESC
LIMIT 10;

-- Check product categories
SELECT c.title, COUNT(p.id) as product_count
FROM categories c
LEFT JOIN products p ON c.id = p.category_id
GROUP BY c.title
ORDER BY product_count DESC;

-- Verify cart items
SELECT u.email, COUNT(ci.id) as cart_item_count
FROM users u
JOIN carts c ON u.id = c.user_id
LEFT JOIN cart_items ci ON c.id = ci.cart_id
GROUP BY u.email
ORDER BY cart_item_count DESC;
```

## Expected Results

After successful migration, you should see:

- All users migrated with valid email addresses
- All categories with unique titles
- All products linked to categories
- All carts linked to users
- All cart items linked to products
- All orders with order items
- All coupons with valid expiry dates
- All blogs linked to authors

## Troubleshooting

### Missing Foreign Key References

If you see warnings about missing users/products/categories:
- This is normal if the MongoDB data has orphaned references
- The script will skip records with invalid foreign keys
- Review the migration logs for specific warnings

### Duplicate Key Errors

If you encounter duplicate key errors:
- Use the `--clear` flag to truncate tables before migration
- Check that the MongoDB dump files are not corrupted

### Connection Errors

If the script cannot connect to PostgreSQL:
- Verify `DATABASE_URL` is correct
- Ensure PostgreSQL is running
- Check firewall settings

### Data Type Mismatches

If you see data type errors:
- Check the MongoDB data for unexpected formats
- The script handles most common type conversions
- You may need to adjust the script for custom data structures

## Validation Checklist

- [ ] All users migrated successfully
- [ ] Email addresses are valid and unique
- [ ] All categories created
- [ ] All products have valid slugs
- [ ] Product images stored as JSON
- [ ] Cart relationships maintained
- [ ] Order totals calculated correctly
- [ ] Coupons have valid expiry dates
- [ ] Blog authors exist in users table
- [ ] Timestamps preserved from MongoDB
- [ ] No orphaned foreign key references

## Performance Considerations

- Migration time depends on data volume
- Expect ~100-1000 records per second
- Use `--clear` flag for clean migrations
- Consider batch processing for very large datasets

## Next Steps

After successful test migration:

1. Review migrated data quality
2. Run integration tests against migrated data
3. Test application with migrated database
4. Plan production migration window
5. Prepare rollback strategy
