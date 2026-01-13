# E-Commerce Platform Refactoring: Node.js + MongoDB → Rust + PostgreSQL + Testcontainers

## Project Overview

### Current State
- **Backend**: Node.js + Express.js
- **Database**: MongoDB + Mongoose ODM
- **Search**: Elasticsearch
- **Auth**: JWT with refresh tokens
- **Storage**: Cloudinary for images
- **Frontend**: Vue.js 3 + Vuetify

### Target State
- **Backend**: Rust + Actix-web
- **Database**: PostgreSQL + SQLx
- **Search**: PostgreSQL full-text search (initial) or Meilisearch (future)
- **Testing**: Testcontainers-rs for integration tests
- **Auth**: JWT with refresh tokens (maintained)
- **Storage**: S3-compatible storage or local with future migration path
- **Frontend**: Vue.js 3 (maintained, only API integration changes)

## System Architecture

### Domain Model

#### Core Entities
1. **User**
   - id (UUID)
   - first_name, last_name
   - email (unique)
   - mobile (unique)
   - password_hash
   - password_changed_at
   - password_reset_token, password_reset_expires
   - role (enum: User, Admin)
   - is_blocked (boolean)
   - wishlist (user_id → product_id relation)
   - created_at, updated_at

2. **Product**
   - id (UUID)
   - title, slug (indexed)
   - description, price, quantity
   - brand, category_id (FK)
   - sold (counter)
   - discount (percentage)
   - images (JSON array)
   - total_ratings (numeric)
   - ratings (relation to Rating entity)
   - created_at, updated_at

3. **Category**
   - id (UUID)
   - title (unique, indexed)
   - created_at, updated_at

4. **Cart**
   - id (UUID)
   - user_id (FK, unique)
   - created_at, updated_at

5. **CartItem**
   - id (UUID)
   - cart_id (FK)
   - product_id (FK)
   - quantity
   - created_at, updated_at

6. **Order**
   - id (UUID)
   - user_id (FK)
   - status (enum: Pending, Processing, Shipped, Delivered, Cancelled)
   - payment_method
   - shipping_price, total_price
   - shipping_address_street, shipping_address_city
   - created_at, updated_at

7. **OrderItem**
   - id (UUID)
   - order_id (FK)
   - product_id (FK)
   - quantity, price (snapshot at order time)
   - created_at

8. **Coupon**
   - id (UUID)
   - code (unique)
   - discount_percentage
   - expiry_date
   - is_active
   - created_at, updated_at

9. **Blog**
   - id (UUID)
   - title, description
   - content (text)
   - author_id (FK to User)
   - category_id (FK)
   - images (JSON array)
   - views_count
   - created_at, updated_at

10. **Rating**
    - id (UUID)
    - product_id (FK)
    - user_id (FK)
    - rating (1-5)
    - comment (text)
    - created_at

### Database Schema (PostgreSQL)

```sql
-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    mobile VARCHAR(20) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    password_changed_at TIMESTAMP,
    password_reset_token VARCHAR(255),
    password_reset_expires TIMESTAMP,
    role VARCHAR(20) NOT NULL DEFAULT 'user' CHECK (role IN ('user', 'admin')),
    is_blocked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Categories table
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Products table
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    brand VARCHAR(255),
    category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
    sold INTEGER DEFAULT 0,
    discount DECIMAL(5, 2) DEFAULT 0,
    images JSONB DEFAULT '[]',
    total_ratings DECIMAL(3, 2) DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Ratings table
CREATE TABLE ratings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(product_id, user_id)
);

-- Wishlist (many-to-many)
CREATE TABLE wishlist (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (user_id, product_id)
);

-- Carts table
CREATE TABLE carts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Cart items table
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    cart_id UUID NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1 CHECK (quantity > 0),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(cart_id, product_id)
);

-- Orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' 
        CHECK (status IN ('Pending', 'Processing', 'Shipped', 'Delivered', 'Cancelled')),
    payment_method VARCHAR(50) NOT NULL,
    shipping_price DECIMAL(10, 2) NOT NULL,
    total_price DECIMAL(10, 2) NOT NULL,
    shipping_address_street VARCHAR(500) NOT NULL,
    shipping_address_city VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Order items table
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Coupons table
CREATE TABLE coupons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) UNIQUE NOT NULL,
    discount_percentage DECIMAL(5, 2) NOT NULL CHECK (discount_percentage >= 0 AND discount_percentage <= 100),
    expiry_date TIMESTAMP NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Blogs table
CREATE TABLE blogs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    content TEXT NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
    images JSONB DEFAULT '[]',
    views_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_products_title ON products USING gin(to_tsvector('english', title));
CREATE INDEX idx_cart_items_cart ON cart_items(cart_id);
CREATE INDEX idx_order_items_order ON order_items(order_id);
CREATE INDEX idx_orders_user ON orders(user_id);
CREATE INDEX idx_ratings_product ON ratings(product_id);
CREATE INDEX idx_blogs_author ON blogs(author_id);
CREATE INDEX idx_coupons_code ON coupons(code);
CREATE INDEX idx_wishlist_user ON wishlist(user_id);

-- Triggers for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_products_updated_at BEFORE UPDATE ON products 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_carts_updated_at BEFORE UPDATE ON carts 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_cart_items_updated_at BEFORE UPDATE ON cart_items 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_coupons_updated_at BEFORE UPDATE ON coupons 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_blogs_updated_at BEFORE UPDATE ON blogs 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

### Rust Project Structure

```
rust-ecommerce/
├── Cargo.toml
├── .env
├── .env.test
├── migrations/
│   └── [timestamp]_initial_schema.sql
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── database.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── product.rs
│   │   ├── category.rs
│   │   ├── cart.rs
│   │   ├── order.rs
│   │   ├── coupon.rs
│   │   ├── blog.rs
│   │   └── rating.rs
│   ├── dto/
│   │   ├── mod.rs
│   │   ├── user_dto.rs
│   │   ├── product_dto.rs
│   │   ├── auth_dto.rs
│   │   └── ...
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   ├── product_repository.rs
│   │   └── ...
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── user_service.rs
│   │   ├── product_service.rs
│   │   └── ...
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth_handler.rs
│   │   ├── user_handler.rs
│   │   ├── product_handler.rs
│   │   └── ...
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── error_handler.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── jwt.rs
│   │   ├── password.rs
│   │   └── slug.rs
│   └── errors/
│       ├── mod.rs
│       └── app_error.rs
├── tests/
│   ├── common/
│   │   ├── mod.rs
│   │   └── test_container.rs
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── auth_tests.rs
│   │   ├── user_tests.rs
│   │   ├── product_tests.rs
│   │   ├── cart_tests.rs
│   │   └── order_tests.rs
│   └── fixtures/
│       ├── mod.rs
│       └── test_data.rs
└── scripts/
    ├── init_db.sh
    └── seed_data.sql
```

### Technology Stack

#### Core Dependencies
- **actix-web** (4.x) - Web framework
- **sqlx** (0.7.x) - Async PostgreSQL driver with compile-time query verification
- **tokio** (1.x) - Async runtime
- **serde** (1.x) - Serialization
- **uuid** (1.x) - UUID generation
- **chrono** (0.4.x) - Date/time handling

#### Authentication & Security
- **jsonwebtoken** (9.x) - JWT implementation
- **argon2** (0.5.x) - Password hashing
- **validator** (0.16.x) - Data validation

#### Testing
- **testcontainers** (0.15.x) - Container-based integration tests
- **reqwest** (0.11.x) - HTTP client for testing
- **fake** (2.x) - Test data generation

#### Utilities
- **dotenv** (0.15.x) - Environment variables
- **env_logger** (0.11.x) - Logging
- **anyhow** (1.x) - Error handling
- **thiserror** (1.x) - Custom error types

## Migration Phases

### Phase 1: Foundation Setup
**Status**: Not Started  
**Estimated Duration**: 3-5 days

#### Tasks
- [ ] Initialize Rust project with Cargo
- [ ] Set up project structure (models, handlers, services, repositories)
- [ ] Configure PostgreSQL connection pool with SQLx
- [ ] Create initial database migrations
- [ ] Set up testcontainers infrastructure
- [ ] Implement basic error handling
- [ ] Create logging configuration
- [ ] Write initial integration test framework
- [ ] Document development environment setup

**Deliverables**:
- Working Cargo project
- Database connection established
- Test infrastructure functional
- summary-phase1.md

### Phase 2: Authentication & User Management
**Status**: Not Started  
**Estimated Duration**: 5-7 days

#### Tasks
- [ ] Implement User model and repository
- [ ] Create password hashing utilities (Argon2)
- [ ] Implement JWT generation and validation
- [ ] Build refresh token mechanism
- [ ] Create user registration endpoint
- [ ] Create login/logout endpoints
- [ ] Implement password reset flow
- [ ] Create authentication middleware
- [ ] Write integration tests for auth flow
- [ ] Implement role-based authorization (User/Admin)
- [ ] Create user CRUD endpoints
- [ ] Write comprehensive auth tests

**API Endpoints**:
- POST /api/auth/register
- POST /api/auth/login
- POST /api/auth/logout
- POST /api/auth/refresh
- POST /api/auth/forgot-password
- PUT /api/auth/reset-password/:token
- GET /api/users (admin)
- GET /api/users/:id
- PUT /api/users/:id
- DELETE /api/users/:id
- PUT /api/users/:id/block (admin)
- PUT /api/users/:id/unblock (admin)

**Deliverables**:
- Fully functional authentication system
- User management endpoints
- Integration tests with >80% coverage
- summary-phase2.md

### Phase 3: Product & Category Management
**Status**: Not Started  
**Estimated Duration**: 5-7 days

#### Tasks
- [ ] Implement Category model and repository
- [ ] Implement Product model and repository
- [ ] Create slug generation utility
- [ ] Build category CRUD endpoints
- [ ] Build product CRUD endpoints
- [ ] Implement product search with PostgreSQL full-text search
- [ ] Add pagination support
- [ ] Implement image URL handling (prepare for storage service)
- [ ] Create product rating system
- [ ] Add wishlist functionality
- [ ] Write integration tests for products
- [ ] Write integration tests for categories
- [ ] Add validation for product data

**API Endpoints**:
- GET /api/categories
- POST /api/categories (admin)
- GET /api/categories/:id
- PUT /api/categories/:id (admin)
- DELETE /api/categories/:id (admin)
- GET /api/products
- POST /api/products (admin)
- GET /api/products/:id
- PUT /api/products/:id (admin)
- DELETE /api/products/:id (admin)
- GET /api/products/search?q=:query
- POST /api/products/:id/ratings
- POST /api/users/wishlist/:productId
- DELETE /api/users/wishlist/:productId

**Deliverables**:
- Product and category management complete
- Search functionality working
- Rating system implemented
- Integration tests with >80% coverage
- summary-phase3.md

### Phase 4: Shopping Cart & Orders
**Status**: Not Started  
**Estimated Duration**: 5-7 days

#### Tasks
- [ ] Implement Cart and CartItem models
- [ ] Create cart repository with CRUD operations
- [ ] Build cart management endpoints
- [ ] Implement cart total calculation
- [ ] Implement Order and OrderItem models
- [ ] Create order repository
- [ ] Build order creation endpoint
- [ ] Implement order status management
- [ ] Add order history endpoint
- [ ] Create admin order management endpoints
- [ ] Write integration tests for cart operations
- [ ] Write integration tests for order workflow
- [ ] Add validation for cart and order operations

**API Endpoints**:
- GET /api/cart
- POST /api/cart/items
- PUT /api/cart/items/:id
- DELETE /api/cart/items/:id
- DELETE /api/cart
- POST /api/orders
- GET /api/orders
- GET /api/orders/:id
- PUT /api/orders/:id/status (admin)
- GET /api/orders/user/:userId (admin)

**Deliverables**:
- Complete cart system
- Order management functional
- Integration tests with >80% coverage
- summary-phase4.md

### Phase 5: Coupons & Additional Features
**Status**: Not Started  
**Estimated Duration**: 3-5 days

#### Tasks
- [ ] Implement Coupon model and repository
- [ ] Create coupon CRUD endpoints (admin)
- [ ] Implement coupon validation logic
- [ ] Add coupon application to cart/order
- [ ] Implement Blog model and repository
- [ ] Create blog CRUD endpoints
- [ ] Add blog view counter
- [ ] Write integration tests for coupons
- [ ] Write integration tests for blogs
- [ ] Add comprehensive validation

**API Endpoints**:
- GET /api/coupons (admin)
- POST /api/coupons (admin)
- GET /api/coupons/:code
- PUT /api/coupons/:id (admin)
- DELETE /api/coupons/:id (admin)
- POST /api/cart/apply-coupon
- GET /api/blogs
- POST /api/blogs (admin)
- GET /api/blogs/:id
- PUT /api/blogs/:id (admin)
- DELETE /api/blogs/:id (admin)

**Deliverables**:
- Coupon system functional
- Blog management complete
- Integration tests with >80% coverage
- summary-phase5.md

### Phase 6: Performance, Security & Documentation
**Status**: Not Started  
**Estimated Duration**: 3-5 days

#### Tasks
- [ ] Add database query optimization
- [ ] Implement rate limiting
- [ ] Add CORS configuration
- [ ] Implement request validation middleware
- [ ] Add comprehensive logging
- [ ] Create OpenAPI/Swagger documentation
- [ ] Write performance benchmarks
- [ ] Security audit (SQL injection, XSS, etc.)
- [ ] Add database connection pooling optimization
- [ ] Create deployment documentation
- [ ] Write API documentation
- [ ] Add monitoring/observability setup

**Deliverables**:
- Optimized application
- Complete API documentation
- Security hardened
- Deployment guide
- summary-phase6.md

### Phase 7: Data Migration & Deployment
**Status**: Not Started  
**Estimated Duration**: 3-5 days

#### Tasks
- [ ] Create MongoDB to PostgreSQL migration script
- [ ] Test data migration with sample data
- [ ] Set up production PostgreSQL instance
- [ ] Configure production environment
- [ ] Deploy application
- [ ] Perform smoke tests
- [ ] Set up monitoring
- [ ] Create backup strategy
- [ ] Document migration process
- [ ] Update frontend API endpoints

**Deliverables**:
- Data successfully migrated
- Application deployed
- Monitoring active
- summary-phase7.md
- Final project summary

## Testing Strategy

### Unit Tests
- Test individual functions and methods
- Mock external dependencies
- Focus on business logic validation

### Integration Tests with Testcontainers
- Spin up real PostgreSQL container
- Test complete API workflows
- Test database operations
- Test authentication flows
- Test complex business logic

### Test Coverage Goals
- Overall coverage: >80%
- Critical paths: >95%
- Repository layer: >90%
- Service layer: >85%
- Handler layer: >80%

### Test Organization
```rust
// tests/common/test_container.rs
pub struct TestContext {
    pub pool: PgPool,
    pub app: App,
}

pub async fn setup_test_db() -> TestContext {
    // Spin up PostgreSQL container
    // Run migrations
    // Return test context
}
```

## Git Flow Strategy

### Branch Strategy
- `main` - Production-ready code
- `develop` - Integration branch
- `phase-*` - Feature branches for each phase
- `hotfix-*` - Critical bug fixes

### Commit Convention
```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Adding tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance tasks

Example:
```
feat(auth): implement JWT authentication

- Add JWT token generation
- Add token validation middleware
- Add refresh token mechanism

Refs: #12
```

### Phase Completion Checklist
At the end of each phase:
1. [ ] All tasks completed
2. [ ] All tests passing
3. [ ] Code reviewed
4. [ ] Documentation updated
5. [ ] summary-phase*.md created
6. [ ] spec.md updated with status
7. [ ] Commit and push changes
8. [ ] Merge to develop branch

## Current Status

**Last Updated**: 2026-01-13  
**Current Phase**: Phase 0 - Planning  
**Overall Progress**: 0%

### Progress Tracking
- [ ] Phase 1: Foundation Setup (0%)
- [ ] Phase 2: Authentication & User Management (0%)
- [ ] Phase 3: Product & Category Management (0%)
- [ ] Phase 4: Shopping Cart & Orders (0%)
- [ ] Phase 5: Coupons & Additional Features (0%)
- [ ] Phase 6: Performance, Security & Documentation (0%)
- [ ] Phase 7: Data Migration & Deployment (0%)

## API Comparison Matrix

| Feature | Node.js Endpoint | Rust Endpoint | Status |
|---------|-----------------|---------------|--------|
| Register | POST /api/auth/register | POST /api/auth/register | Not Started |
| Login | POST /api/auth/login | POST /api/auth/login | Not Started |
| Refresh Token | GET /api/auth/refresh | POST /api/auth/refresh | Not Started |
| Get Products | GET /api/product | GET /api/products | Not Started |
| Create Product | POST /api/product | POST /api/products | Not Started |
| Get Cart | GET /api/cart | GET /api/cart | Not Started |
| Create Order | POST /api/order | POST /api/orders | Not Started |
| ... | ... | ... | ... |

## Notes & Decisions

### Database Design Decisions
1. Using UUID for primary keys (better for distributed systems)
2. Storing images as JSON array (preparing for S3/CDN migration)
3. Denormalizing order items (snapshot prices at order time)
4. Using PostgreSQL full-text search initially (can migrate to dedicated search later)
5. Implementing soft deletes where needed vs hard deletes

### Security Considerations
1. Using Argon2 for password hashing (more secure than bcrypt)
2. JWT with short expiration + refresh token pattern
3. HTTPS only in production
4. Rate limiting on authentication endpoints
5. SQL injection prevention via SQLx parameterized queries
6. Input validation on all endpoints

### Performance Considerations
1. Connection pooling with SQLx
2. Database indexes on frequently queried fields
3. Pagination for list endpoints
4. Caching strategy (future: Redis)
5. Async/await throughout for non-blocking I/O

### Future Enhancements
1. Migrate to Meilisearch for better search
2. Add Redis for caching
3. Implement file upload to S3
4. Add email service integration
5. Implement real-time notifications
6. Add GraphQL API option
7. Implement payment gateway integration
8. Add inventory management
9. Implement product reviews moderation
10. Add analytics and reporting

## References
- Actix-web documentation: https://actix.rs/
- SQLx documentation: https://github.com/launchbadge/sqlx
- Testcontainers-rs: https://github.com/testcontainers/testcontainers-rs
- Rust API guidelines: https://rust-lang.github.io/api-guidelines/
