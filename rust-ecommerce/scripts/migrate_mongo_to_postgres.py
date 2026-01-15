#!/usr/bin/env python3
"""
MongoDB to PostgreSQL Migration Script

This script reads BSON dump files from MongoDB and migrates the data to PostgreSQL.
It handles data transformation, UUID generation, and maintains referential integrity.

Usage:
    python migrate_mongo_to_postgres.py --dump-dir ../dump/ecommercedb --db-url postgresql://user:pass@localhost/dbname
"""

import argparse
import bson
import psycopg2
from psycopg2.extras import execute_values
from datetime import datetime
import uuid
import sys
import os
from typing import Dict, List, Any

class MongoToPostgresMigrator:
    def __init__(self, dump_dir: str, db_url: str):
        self.dump_dir = dump_dir
        self.db_url = db_url
        self.conn = None
        self.cursor = None
        
        # ID mapping for maintaining relationships
        self.user_id_map = {}
        self.product_id_map = {}
        self.category_id_map = {}
        self.cart_id_map = {}
        self.order_id_map = {}
        self.blog_id_map = {}
        
    def connect(self):
        """Connect to PostgreSQL database"""
        try:
            self.conn = psycopg2.connect(self.db_url)
            self.cursor = self.conn.cursor()
            print("✓ Connected to PostgreSQL database")
        except Exception as e:
            print(f"✗ Failed to connect to database: {e}")
            sys.exit(1)
    
    def close(self):
        """Close database connection"""
        if self.cursor:
            self.cursor.close()
        if self.conn:
            self.conn.close()
    
    def read_bson_file(self, filename: str) -> List[Dict[str, Any]]:
        """Read and parse BSON file"""
        filepath = os.path.join(self.dump_dir, filename)
        if not os.path.exists(filepath):
            print(f"⚠ Warning: {filename} not found, skipping...")
            return []
        
        documents = []
        with open(filepath, 'rb') as f:
            while True:
                try:
                    doc = bson.decode_file_iter(f).__next__()
                    documents.append(doc)
                except StopIteration:
                    break
                except Exception as e:
                    print(f"⚠ Warning: Error reading {filename}: {e}")
                    break
        
        print(f"✓ Read {len(documents)} documents from {filename}")
        return documents
    
    def clear_tables(self):
        """Clear all tables in correct order (respecting foreign keys)"""
        print("\n🔄 Clearing existing tables...")
        tables = [
            'ratings', 'wishlist', 'order_items', 'orders', 'cart_items', 'carts',
            'blogs', 'products', 'coupons', 'categories', 'users'
        ]
        
        try:
            for table in tables:
                self.cursor.execute(f"TRUNCATE TABLE {table} CASCADE")
            self.conn.commit()
            print("✓ Tables cleared successfully")
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to clear tables: {e}")
            raise
    
    def migrate_users(self):
        """Migrate users from MongoDB to PostgreSQL"""
        print("\n📥 Migrating users...")
        users = self.read_bson_file('users.bson')
        
        if not users:
            print("⚠ No users to migrate")
            return
        
        user_data = []
        for user in users:
            new_id = str(uuid.uuid4())
            old_id = str(user['_id'])
            self.user_id_map[old_id] = new_id
            
            user_data.append((
                new_id,
                user.get('firstname', ''),
                user.get('lastname', ''),
                user.get('email', ''),
                user.get('mobile', ''),
                user.get('password', ''),  # Password hash
                user.get('passwordChangedAt'),
                user.get('passwordResetToken'),
                user.get('passwordResetExpires'),
                user.get('role', 'user'),
                user.get('isBlocked', False),
                user.get('createdAt', datetime.now()),
                user.get('updatedAt', datetime.now())
            ))
        
        try:
            query = """
                INSERT INTO users (
                    id, first_name, last_name, email, mobile, password_hash,
                    password_changed_at, password_reset_token, password_reset_expires,
                    role, is_blocked, created_at, updated_at
                ) VALUES %s
            """
            execute_values(self.cursor, query, user_data)
            self.conn.commit()
            print(f"✓ Migrated {len(user_data)} users")
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to migrate users: {e}")
            raise
    
    def migrate_categories(self):
        """Migrate categories from MongoDB to PostgreSQL"""
        print("\n📥 Migrating categories...")
        categories = self.read_bson_file('categories.bson')
        
        if not categories:
            print("⚠ No categories to migrate")
            return
        
        category_data = []
        for category in categories:
            new_id = str(uuid.uuid4())
            old_id = str(category['_id'])
            self.category_id_map[old_id] = new_id
            
            category_data.append((
                new_id,
                category.get('title', ''),
                category.get('createdAt', datetime.now()),
                category.get('updatedAt', datetime.now())
            ))
        
        try:
            query = """
                INSERT INTO categories (id, title, created_at, updated_at)
                VALUES %s
            """
            execute_values(self.cursor, query, category_data)
            self.conn.commit()
            print(f"✓ Migrated {len(category_data)} categories")
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to migrate categories: {e}")
            raise
    
    def migrate_products(self):
        """Migrate products from MongoDB to PostgreSQL"""
        print("\n📥 Migrating products...")
        products = self.read_bson_file('products.bson')
        
        if not products:
            print("⚠ No products to migrate")
            return
        
        product_data = []
        for product in products:
            new_id = str(uuid.uuid4())
            old_id = str(product['_id'])
            self.product_id_map[old_id] = new_id
            
            # Map category ID
            category_id = None
            if product.get('category'):
                old_category_id = str(product['category'])
                category_id = self.category_id_map.get(old_category_id)
            
            # Handle images
            images = product.get('images', [])
            if isinstance(images, list):
                import json
                images_json = json.dumps([img.get('url', '') if isinstance(img, dict) else str(img) for img in images])
            else:
                images_json = '[]'
            
            product_data.append((
                new_id,
                product.get('title', ''),
                product.get('slug', ''),
                product.get('description', ''),
                float(product.get('price', 0)),
                int(product.get('quantity', 0)),
                product.get('brand', ''),
                category_id,
                int(product.get('sold', 0)),
                float(product.get('discount', 0)),
                images_json,
                float(product.get('totalrating', 0)),
                product.get('createdAt', datetime.now()),
                product.get('updatedAt', datetime.now())
            ))
        
        try:
            query = """
                INSERT INTO products (
                    id, title, slug, description, price, quantity, brand,
                    category_id, sold, discount, images, total_ratings,
                    created_at, updated_at
                ) VALUES %s
            """
            execute_values(self.cursor, query, product_data)
            self.conn.commit()
            print(f"✓ Migrated {len(product_data)} products")
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to migrate products: {e}")
            raise
    
    def migrate_carts(self):
        """Migrate carts from MongoDB to PostgreSQL"""
        print("\n📥 Migrating carts...")
        carts = self.read_bson_file('carts.bson')
        
        if not carts:
            print("⚠ No carts to migrate")
            return
        
        cart_data = []
        cart_item_data = []
        
        for cart in carts:
            new_cart_id = str(uuid.uuid4())
            old_cart_id = str(cart['_id'])
            self.cart_id_map[old_cart_id] = new_cart_id
            
            # Map user ID
            user_id = None
            if cart.get('userId'):
                old_user_id = str(cart['userId'])
                user_id = self.user_id_map.get(old_user_id)
            
            if not user_id:
                print(f"⚠ Skipping cart {old_cart_id}: user not found")
                continue
            
            cart_data.append((
                new_cart_id,
                user_id,
                cart.get('createdAt', datetime.now()),
                cart.get('updatedAt', datetime.now())
            ))
            
            # Migrate cart items
            products = cart.get('products', [])
            for item in products:
                product_id = None
                if item.get('product'):
                    old_product_id = str(item['product'])
                    product_id = self.product_id_map.get(old_product_id)
                
                if not product_id:
                    continue
                
                cart_item_data.append((
                    str(uuid.uuid4()),
                    new_cart_id,
                    product_id,
                    int(item.get('quantity', 1)),
                    datetime.now(),
                    datetime.now()
                ))
        
        try:
            if cart_data:
                query = """
                    INSERT INTO carts (id, user_id, created_at, updated_at)
                    VALUES %s
                """
                execute_values(self.cursor, query, cart_data)
                print(f"✓ Migrated {len(cart_data)} carts")
            
            if cart_item_data:
                query = """
                    INSERT INTO cart_items (id, cart_id, product_id, quantity, created_at, updated_at)
                    VALUES %s
                """
                execute_values(self.cursor, query, cart_item_data)
                print(f"✓ Migrated {len(cart_item_data)} cart items")
            
            self.conn.commit()
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to migrate carts: {e}")
            raise
    
    def migrate_orders(self):
        """Migrate orders from MongoDB to PostgreSQL"""
        print("\n📥 Migrating orders...")
        orders = self.read_bson_file('orders.bson')
        
        if not orders:
            print("⚠ No orders to migrate")
            return
        
        order_data = []
        order_item_data = []
        
        for order in orders:
            new_order_id = str(uuid.uuid4())
            old_order_id = str(order['_id'])
            self.order_id_map[old_order_id] = new_order_id
            
            # Map user ID
            user_id = None
            if order.get('userId'):
                old_user_id = str(order['userId'])
                user_id = self.user_id_map.get(old_user_id)
            
            if not user_id:
                print(f"⚠ Skipping order {old_order_id}: user not found")
                continue
            
            order_data.append((
                new_order_id,
                user_id,
                order.get('orderStatus', 'Pending'),
                order.get('paymentMethod', 'COD'),
                float(order.get('shippingPrice', 0)),
                float(order.get('totalPrice', 0)),
                order.get('shippingInfo', {}).get('address', ''),
                order.get('shippingInfo', {}).get('city', ''),
                order.get('createdAt', datetime.now()),
                order.get('updatedAt', datetime.now())
            ))
            
            # Migrate order items
            products = order.get('products', [])
            for item in products:
                product_id = None
                if item.get('product'):
                    old_product_id = str(item['product'])
                    product_id = self.product_id_map.get(old_product_id)
                
                if not product_id:
                    continue
                
                order_item_data.append((
                    str(uuid.uuid4()),
                    new_order_id,
                    product_id,
                    int(item.get('quantity', 1)),
                    float(item.get('price', 0)),
                    datetime.now()
                ))
        
        try:
            if order_data:
                query = """
                    INSERT INTO orders (
                        id, user_id, status, payment_method, shipping_price, total_price,
                        shipping_address_street, shipping_address_city, created_at, updated_at
                    ) VALUES %s
                """
                execute_values(self.cursor, query, order_data)
                print(f"✓ Migrated {len(order_data)} orders")
            
            if order_item_data:
                query = """
                    INSERT INTO order_items (id, order_id, product_id, quantity, price, created_at)
                    VALUES %s
                """
                execute_values(self.cursor, query, order_item_data)
                print(f"✓ Migrated {len(order_item_data)} order items")
            
            self.conn.commit()
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to migrate orders: {e}")
            raise
    
    def migrate_coupons(self):
        """Migrate coupons from MongoDB to PostgreSQL"""
        print("\n📥 Migrating coupons...")
        coupons = self.read_bson_file('coupons.bson')
        
        if not coupons:
            print("⚠ No coupons to migrate")
            return
        
        coupon_data = []
        for coupon in coupons:
            new_id = str(uuid.uuid4())
            
            coupon_data.append((
                new_id,
                coupon.get('name', ''),
                float(coupon.get('discount', 0)),
                coupon.get('expiry', datetime.now()),
                True,  # is_active
                coupon.get('createdAt', datetime.now()),
                coupon.get('updatedAt', datetime.now())
            ))
        
        try:
            query = """
                INSERT INTO coupons (
                    id, code, discount_percentage, expiry_date, is_active,
                    created_at, updated_at
                ) VALUES %s
            """
            execute_values(self.cursor, query, coupon_data)
            self.conn.commit()
            print(f"✓ Migrated {len(coupon_data)} coupons")
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to migrate coupons: {e}")
            raise
    
    def migrate_blogs(self):
        """Migrate blogs from MongoDB to PostgreSQL"""
        print("\n📥 Migrating blogs...")
        blogs = self.read_bson_file('blogs.bson')
        
        if not blogs:
            print("⚠ No blogs to migrate")
            return
        
        blog_data = []
        for blog in blogs:
            new_id = str(uuid.uuid4())
            old_id = str(blog['_id'])
            self.blog_id_map[old_id] = new_id
            
            # Map author ID (use first user if not specified)
            author_id = None
            if blog.get('author'):
                old_author_id = str(blog['author'])
                author_id = self.user_id_map.get(old_author_id)
            
            if not author_id and self.user_id_map:
                # Use first user as default author
                author_id = list(self.user_id_map.values())[0]
            
            if not author_id:
                print(f"⚠ Skipping blog {old_id}: no author found")
                continue
            
            # Map category ID
            category_id = None
            if blog.get('category'):
                old_category_id = str(blog['category'])
                category_id = self.category_id_map.get(old_category_id)
            
            # Handle images
            images = blog.get('images', [])
            if isinstance(images, list):
                import json
                images_json = json.dumps([img.get('url', '') if isinstance(img, dict) else str(img) for img in images])
            else:
                images_json = '[]'
            
            blog_data.append((
                new_id,
                blog.get('title', ''),
                blog.get('description', ''),
                blog.get('content', ''),
                author_id,
                category_id,
                images_json,
                int(blog.get('numViews', 0)),
                blog.get('createdAt', datetime.now()),
                blog.get('updatedAt', datetime.now())
            ))
        
        try:
            query = """
                INSERT INTO blogs (
                    id, title, description, content, author_id, category_id,
                    images, views_count, created_at, updated_at
                ) VALUES %s
            """
            execute_values(self.cursor, query, blog_data)
            self.conn.commit()
            print(f"✓ Migrated {len(blog_data)} blogs")
        except Exception as e:
            self.conn.rollback()
            print(f"✗ Failed to migrate blogs: {e}")
            raise
    
    def run_migration(self, clear_existing: bool = False):
        """Run the complete migration"""
        print("=" * 60)
        print("MongoDB to PostgreSQL Migration")
        print("=" * 60)
        
        self.connect()
        
        try:
            if clear_existing:
                self.clear_tables()
            
            # Migrate in order to maintain referential integrity
            self.migrate_users()
            self.migrate_categories()
            self.migrate_products()
            self.migrate_carts()
            self.migrate_orders()
            self.migrate_coupons()
            self.migrate_blogs()
            
            print("\n" + "=" * 60)
            print("✓ Migration completed successfully!")
            print("=" * 60)
            print(f"\nMigrated:")
            print(f"  • {len(self.user_id_map)} users")
            print(f"  • {len(self.category_id_map)} categories")
            print(f"  • {len(self.product_id_map)} products")
            print(f"  • {len(self.cart_id_map)} carts")
            print(f"  • {len(self.order_id_map)} orders")
            print(f"  • {len(self.blog_id_map)} blogs")
            
        except Exception as e:
            print(f"\n✗ Migration failed: {e}")
            sys.exit(1)
        finally:
            self.close()

def main():
    parser = argparse.ArgumentParser(description='Migrate MongoDB dump to PostgreSQL')
    parser.add_argument('--dump-dir', required=True, help='Path to MongoDB dump directory')
    parser.add_argument('--db-url', required=True, help='PostgreSQL connection URL')
    parser.add_argument('--clear', action='store_true', help='Clear existing data before migration')
    
    args = parser.parse_args()
    
    migrator = MongoToPostgresMigrator(args.dump_dir, args.db_url)
    migrator.run_migration(clear_existing=args.clear)

if __name__ == '__main__':
    main()
