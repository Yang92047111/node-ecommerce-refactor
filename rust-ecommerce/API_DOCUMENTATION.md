# API Documentation

## E-Commerce Platform REST API

**Base URL**: `http://localhost:8080/api`  
**Version**: 0.1.0  
**Authentication**: JWT Bearer Token

---

## Table of Contents

1. [Authentication](#authentication)
2. [Users](#users)
3. [Products](#products)
4. [Categories](#categories)
5. [Cart](#cart)
6. [Orders](#orders)
7. [Coupons](#coupons)
8. [Blogs](#blogs)
9. [Monitoring](#monitoring)
10. [Rate Limiting](#rate-limiting)
11. [Error Handling](#error-handling)

---

## Authentication

### Register User
**POST** `/api/auth/register`

Register a new user account.

**Request Body**:
```json
{
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "mobile": "+1234567890",
  "password": "SecurePassword123!"
}
```

**Response** (201 Created):
```json
{
  "id": "uuid",
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "mobile": "+1234567890",
  "role": "user",
  "token": "eyJhbGc...",
  "refresh_token": "eyJhbGc..."
}
```

### Login
**POST** `/api/auth/login`

Authenticate and receive access tokens.

**Request Body**:
```json
{
  "email": "john@example.com",
  "password": "SecurePassword123!"
}
```

**Response** (200 OK):
```json
{
  "user": {
    "id": "uuid",
    "first_name": "John",
    "last_name": "Doe",
    "email": "john@example.com",
    "role": "user"
  },
  "token": "eyJhbGc...",
  "refresh_token": "eyJhbGc..."
}
```

### Refresh Token
**POST** `/api/auth/refresh`

Get a new access token using refresh token.

**Request Body**:
```json
{
  "refresh_token": "eyJhbGc..."
}
```

**Response** (200 OK):
```json
{
  "token": "eyJhbGc...",
  "refresh_token": "eyJhbGc..."
}
```

### Forgot Password
**POST** `/api/auth/forgot-password`

Request password reset token.

**Request Body**:
```json
{
  "email": "john@example.com"
}
```

**Response** (200 OK):
```json
{
  "message": "Password reset token sent",
  "reset_token": "generated_token"
}
```

### Reset Password
**PUT** `/api/auth/reset-password/:token`

Reset password using token.

**Request Body**:
```json
{
  "password": "NewSecurePassword123!"
}
```

**Response** (200 OK):
```json
{
  "message": "Password reset successful"
}
```

---

## Users

All user endpoints require authentication unless specified.

### Get All Users (Admin Only)
**GET** `/api/users`

**Query Parameters**:
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 10)

**Response** (200 OK):
```json
{
  "users": [
    {
      "id": "uuid",
      "first_name": "John",
      "last_name": "Doe",
      "email": "john@example.com",
      "mobile": "+1234567890",
      "role": "user",
      "is_blocked": false,
      "created_at": "2026-01-15T00:00:00Z"
    }
  ],
  "total": 100,
  "page": 1,
  "limit": 10
}
```

### Get User by ID
**GET** `/api/users/:id`

**Response** (200 OK):
```json
{
  "id": "uuid",
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "mobile": "+1234567890",
  "role": "user",
  "is_blocked": false
}
```

### Update User
**PUT** `/api/users/:id`

**Request Body**:
```json
{
  "first_name": "John",
  "last_name": "Doe",
  "mobile": "+1234567890"
}
```

### Block User (Admin Only)
**PUT** `/api/users/:id/block`

### Unblock User (Admin Only)
**PUT** `/api/users/:id/unblock`

### Delete User (Admin Only)
**DELETE** `/api/users/:id`

---

## Products

### Get All Products
**GET** `/api/products`

**Query Parameters**:
- `page` (optional): Page number
- `limit` (optional): Items per page
- `search` (optional): Search query
- `category` (optional): Filter by category ID
- `min_price` (optional): Minimum price
- `max_price` (optional): Maximum price

**Response** (200 OK):
```json
{
  "products": [
    {
      "id": "uuid",
      "title": "Product Name",
      "slug": "product-name",
      "description": "Product description",
      "price": 99.99,
      "quantity": 100,
      "brand": "Brand Name",
      "category_id": "uuid",
      "sold": 10,
      "discount": 10.0,
      "images": ["url1", "url2"],
      "total_ratings": 4.5,
      "created_at": "2026-01-15T00:00:00Z"
    }
  ],
  "total": 50,
  "page": 1,
  "limit": 10
}
```

### Create Product (Admin Only)
**POST** `/api/products`

**Request Body**:
```json
{
  "title": "Product Name",
  "description": "Product description",
  "price": 99.99,
  "quantity": 100,
  "brand": "Brand Name",
  "category_id": "uuid",
  "images": ["url1", "url2"]
}
```

### Get Product by ID
**GET** `/api/products/:id`

### Update Product (Admin Only)
**PUT** `/api/products/:id`

### Delete Product (Admin Only)
**DELETE** `/api/products/:id`

### Add Product Rating
**POST** `/api/products/:id/ratings`

**Request Body**:
```json
{
  "rating": 5,
  "comment": "Great product!"
}
```

### Get Product Ratings
**GET** `/api/products/:id/ratings`

---

## Categories

### Get All Categories
**GET** `/api/categories`

**Response** (200 OK):
```json
[
  {
    "id": "uuid",
    "title": "Electronics",
    "created_at": "2026-01-15T00:00:00Z"
  }
]
```

### Create Category (Admin Only)
**POST** `/api/categories`

**Request Body**:
```json
{
  "title": "Electronics"
}
```

### Get Category by ID
**GET** `/api/categories/:id`

### Update Category (Admin Only)
**PUT** `/api/categories/:id`

### Delete Category (Admin Only)
**DELETE** `/api/categories/:id`

---

## Cart

All cart endpoints require authentication.

### Get Cart
**GET** `/api/cart`

**Response** (200 OK):
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "items": [
    {
      "id": "uuid",
      "product": {
        "id": "uuid",
        "title": "Product Name",
        "price": 99.99
      },
      "quantity": 2
    }
  ],
  "total": 199.98
}
```

### Add Item to Cart
**POST** `/api/cart/items`

**Request Body**:
```json
{
  "product_id": "uuid",
  "quantity": 2
}
```

### Update Cart Item
**PUT** `/api/cart/items/:id`

**Request Body**:
```json
{
  "quantity": 3
}
```

### Remove Cart Item
**DELETE** `/api/cart/items/:id`

### Clear Cart
**DELETE** `/api/cart`

---

## Orders

All order endpoints require authentication.

### Create Order
**POST** `/api/orders`

**Request Body**:
```json
{
  "payment_method": "credit_card",
  "shipping_address": {
    "street": "123 Main St",
    "city": "New York"
  },
  "coupon_code": "SAVE10"
}
```

**Response** (201 Created):
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "status": "Pending",
  "payment_method": "credit_card",
  "shipping_price": 10.00,
  "total_price": 209.98,
  "items": [
    {
      "product_id": "uuid",
      "quantity": 2,
      "price": 99.99
    }
  ],
  "created_at": "2026-01-15T00:00:00Z"
}
```

### Get User Orders
**GET** `/api/orders`

### Get Order by ID
**GET** `/api/orders/:id`

### Update Order Status (Admin Only)
**PUT** `/api/orders/:id/status`

**Request Body**:
```json
{
  "status": "Shipped"
}
```

### Get All Orders (Admin Only)
**GET** `/api/admin/orders`

---

## Coupons

### Validate Coupon
**POST** `/api/coupons/validate`

**Request Body**:
```json
{
  "code": "SAVE10"
}
```

**Response** (200 OK):
```json
{
  "id": "uuid",
  "code": "SAVE10",
  "discount_percentage": 10.0,
  "expiry_date": "2026-12-31T23:59:59Z",
  "is_active": true
}
```

### Get All Coupons (Admin Only)
**GET** `/api/coupons`

### Create Coupon (Admin Only)
**POST** `/api/coupons`

**Request Body**:
```json
{
  "code": "SAVE10",
  "discount_percentage": 10.0,
  "expiry_date": "2026-12-31T23:59:59Z"
}
```

### Update Coupon (Admin Only)
**PUT** `/api/coupons/:id`

### Delete Coupon (Admin Only)
**DELETE** `/api/coupons/:id`

---

## Blogs

### Get All Blogs
**GET** `/api/blogs`

**Query Parameters**:
- `page` (optional): Page number
- `limit` (optional): Items per page

**Response** (200 OK):
```json
{
  "blogs": [
    {
      "id": "uuid",
      "title": "Blog Title",
      "description": "Short description",
      "content": "Full content...",
      "author_id": "uuid",
      "category_id": "uuid",
      "images": ["url1"],
      "views_count": 100,
      "created_at": "2026-01-15T00:00:00Z"
    }
  ],
  "total": 20,
  "page": 1
}
```

### Create Blog (Admin Only)
**POST** `/api/blogs`

**Request Body**:
```json
{
  "title": "Blog Title",
  "description": "Short description",
  "content": "Full content...",
  "category_id": "uuid",
  "images": ["url1"]
}
```

### Get Blog by ID
**GET** `/api/blogs/:id`

### Update Blog (Admin Only)
**PUT** `/api/blogs/:id`

### Delete Blog (Admin Only)
**DELETE** `/api/blogs/:id`

---

## Monitoring

### Health Check
**GET** `/api/monitoring/health`

Returns overall system health status.

**Response** (200 OK):
```json
{
  "status": "healthy",
  "timestamp": 1705276800,
  "version": "0.1.0",
  "database": {
    "status": "healthy",
    "connections": {
      "active": 5,
      "idle": 15,
      "max": 20
    }
  },
  "uptime": 3600
}
```

### Liveness Probe
**GET** `/api/monitoring/liveness`

Simple check to verify the application is running.

**Response** (200 OK):
```json
{
  "status": "alive",
  "timestamp": 1705276800
}
```

### Readiness Probe
**GET** `/api/monitoring/readiness`

Checks if the application is ready to serve traffic.

**Response** (200 OK):
```json
{
  "ready": true,
  "checks": [
    {
      "name": "database",
      "status": "ready",
      "message": null
    }
  ]
}
```

---

## Rate Limiting

The API implements rate limiting to prevent abuse:

- **Authentication endpoints**: 5 requests per minute
- **General API endpoints**: 100 requests per minute
- **Admin endpoints**: 50 requests per minute

When rate limit is exceeded, the API returns:

**Response** (429 Too Many Requests):
```json
{
  "error": "Too many requests",
  "retry_after": 60
}
```

---

## Error Handling

The API uses standard HTTP status codes and returns errors in JSON format:

**Error Response Format**:
```json
{
  "error": "Error message",
  "details": "Additional details (optional)"
}
```

### Common Status Codes

- `200 OK`: Request successful
- `201 Created`: Resource created successfully
- `400 Bad Request`: Invalid input data
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource already exists
- `422 Unprocessable Entity`: Validation error
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server error
- `503 Service Unavailable`: Service temporarily unavailable

---

## Authentication

Most endpoints require authentication using JWT Bearer tokens.

**Header Format**:
```
Authorization: Bearer <your_jwt_token>
```

**Token Expiration**:
- Access Token: 1 hour
- Refresh Token: 7 days

Use the refresh token endpoint to obtain new tokens before expiration.

---

## Best Practices

1. **Always use HTTPS in production**
2. **Store tokens securely** (e.g., HttpOnly cookies or secure storage)
3. **Handle rate limiting** with exponential backoff
4. **Validate input data** before sending requests
5. **Implement proper error handling** in your client
6. **Use refresh tokens** to maintain session without re-authentication
7. **Monitor API health** using monitoring endpoints

---

## Support

For issues or questions, please refer to the project documentation or contact the development team.
