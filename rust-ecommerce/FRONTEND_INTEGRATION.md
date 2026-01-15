# Frontend Integration Guide

## Overview

This guide helps frontend developers integrate with the new Rust API after migrating from the Node.js backend.

---

## API Base URL Changes

### Before (Node.js)

```javascript
const API_BASE_URL = 'http://localhost:5000/api';
```

### After (Rust)

```javascript
const API_BASE_URL = 'http://localhost:8080/api';
```

**Production:**
```javascript
const API_BASE_URL = 'https://api.yourdomain.com/api';
```

---

## Authentication Changes

### Token Storage

The authentication mechanism remains the same (JWT), but token handling is slightly different:

**Before (Node.js):**
```javascript
// Token format: plain JWT string
localStorage.setItem('token', response.data.token);
```

**After (Rust):**
```javascript
// Token format: same, but response structure may differ
localStorage.setItem('token', response.data.access_token);
localStorage.setItem('refreshToken', response.data.refresh_token);
```

### API Client Configuration

Update your axios/fetch configuration:

```javascript
// api/client.js
import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/api';

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 10000,
});

// Request interceptor to add auth token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor for token refresh
apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;

      try {
        const refreshToken = localStorage.getItem('refreshToken');
        const response = await axios.post(`${API_BASE_URL}/auth/refresh`, {
          refresh_token: refreshToken,
        });

        const { access_token } = response.data;
        localStorage.setItem('token', access_token);

        originalRequest.headers.Authorization = `Bearer ${access_token}`;
        return apiClient(originalRequest);
      } catch (refreshError) {
        // Refresh failed, redirect to login
        localStorage.removeItem('token');
        localStorage.removeItem('refreshToken');
        window.location.href = '/login';
        return Promise.reject(refreshError);
      }
    }

    return Promise.reject(error);
  }
);

export default apiClient;
```

---

## Endpoint Mapping

### Authentication Endpoints

| Feature | Node.js Endpoint | Rust Endpoint | Changes |
|---------|------------------|---------------|---------|
| Register | `POST /api/auth/register` | `POST /api/auth/register` | ✅ Same |
| Login | `POST /api/auth/login` | `POST /api/auth/login` | ✅ Same |
| Logout | `POST /api/auth/logout` | `POST /api/auth/logout` | ✅ Same |
| Refresh Token | `GET /api/auth/refresh` | `POST /api/auth/refresh` | ⚠️ Changed to POST |
| Forgot Password | `POST /api/auth/forgot-password` | `POST /api/auth/forgot-password` | ✅ Same |
| Reset Password | `PUT /api/auth/reset-password/:token` | `PUT /api/auth/reset-password/:token` | ✅ Same |

**Refresh Token Change:**

```javascript
// Before (Node.js)
const response = await axios.get('/api/auth/refresh', {
  headers: { Authorization: `Bearer ${refreshToken}` }
});

// After (Rust)
const response = await axios.post('/api/auth/refresh', {
  refresh_token: refreshToken
});
```

### User Endpoints

| Feature | Node.js Endpoint | Rust Endpoint | Changes |
|---------|------------------|---------------|---------|
| Get All Users | `GET /api/users` | `GET /api/users` | ✅ Same |
| Get User | `GET /api/users/:id` | `GET /api/users/:id` | ✅ Same |
| Update User | `PUT /api/users/:id` | `PUT /api/users/:id` | ✅ Same |
| Delete User | `DELETE /api/users/:id` | `DELETE /api/users/:id` | ✅ Same |
| Block User | `PUT /api/users/:id/block` | `PUT /api/users/:id/block` | ✅ Same |
| Unblock User | `PUT /api/users/:id/unblock` | `PUT /api/users/:id/unblock` | ✅ Same |
| Get Wishlist | `GET /api/user/wishlist` | `GET /api/users/wishlist` | ⚠️ Changed path |
| Add to Wishlist | `POST /api/user/wishlist/:productId` | `POST /api/users/wishlist/:productId` | ⚠️ Changed path |
| Remove from Wishlist | `DELETE /api/user/wishlist/:productId` | `DELETE /api/users/wishlist/:productId` | ⚠️ Changed path |

**Wishlist Path Change:**

```javascript
// Before (Node.js)
await apiClient.get('/user/wishlist');

// After (Rust)
await apiClient.get('/users/wishlist');
```

### Product Endpoints

| Feature | Node.js Endpoint | Rust Endpoint | Changes |
|---------|------------------|---------------|---------|
| Get All Products | `GET /api/product` | `GET /api/products` | ⚠️ Changed to plural |
| Create Product | `POST /api/product` | `POST /api/products` | ⚠️ Changed to plural |
| Get Product | `GET /api/product/:id` | `GET /api/products/:id` | ⚠️ Changed to plural |
| Update Product | `PUT /api/product/:id` | `PUT /api/products/:id` | ⚠️ Changed to plural |
| Delete Product | `DELETE /api/product/:id` | `DELETE /api/products/:id` | ⚠️ Changed to plural |
| Search Products | `GET /api/product/search?q=` | `GET /api/products?search=` | ⚠️ Changed query param |
| Add Rating | `POST /api/product/:id/rating` | `POST /api/products/:id/ratings` | ⚠️ Changed path |
| Get Ratings | `GET /api/product/:id/ratings` | `GET /api/products/:id/ratings` | ⚠️ Changed path |

**Product Path Changes:**

```javascript
// Before (Node.js)
await apiClient.get('/product');
await apiClient.get('/product/search?q=laptop');

// After (Rust)
await apiClient.get('/products');
await apiClient.get('/products?search=laptop');
```

### Category Endpoints

| Feature | Node.js Endpoint | Rust Endpoint | Changes |
|---------|------------------|---------------|---------|
| Get All Categories | `GET /api/category` | `GET /api/categories` | ⚠️ Changed to plural |
| Create Category | `POST /api/category` | `POST /api/categories` | ⚠️ Changed to plural |
| Get Category | `GET /api/category/:id` | `GET /api/categories/:id` | ⚠️ Changed to plural |
| Update Category | `PUT /api/category/:id` | `PUT /api/categories/:id` | ⚠️ Changed to plural |
| Delete Category | `DELETE /api/category/:id` | `DELETE /api/categories/:id` | ⚠️ Changed to plural |

### Cart Endpoints

| Feature | Node.js Endpoint | Rust Endpoint | Changes |
|---------|------------------|---------------|---------|
| Get Cart | `GET /api/cart` | `GET /api/cart` | ✅ Same |
| Add to Cart | `POST /api/cart/items` | `POST /api/cart/items` | ✅ Same |
| Update Cart Item | `PUT /api/cart/items/:id` | `PUT /api/cart/items/:id` | ✅ Same |
| Remove Cart Item | `DELETE /api/cart/items/:id` | `DELETE /api/cart/items/:id` | ✅ Same |
| Clear Cart | `DELETE /api/cart` | `DELETE /api/cart` | ✅ Same |
| Apply Coupon | `POST /api/cart/apply-coupon` | `POST /api/cart/apply-coupon` | ✅ Same |

### Order Endpoints

| Feature | Node.js Endpoint | Rust Endpoint | Changes |
|---------|------------------|---------------|---------|
| Create Order | `POST /api/order` | `POST /api/orders` | ⚠️ Changed to plural |
| Get Orders | `GET /api/orders` | `GET /api/orders` | ✅ Same |
| Get Order | `GET /api/orders/:id` | `GET /api/orders/:id` | ✅ Same |
| Update Order Status | `PUT /api/orders/:id/status` | `PUT /api/orders/:id/status` | ✅ Same |

---

## Response Format Changes

### Success Responses

**Before (Node.js):**
```json
{
  "success": true,
  "data": { ... }
}
```

**After (Rust):**
```json
{
  "status": "success",
  "data": { ... }
}
```

Or directly the data object (depending on endpoint):
```json
{ ... }
```

### Error Responses

**Before (Node.js):**
```json
{
  "success": false,
  "message": "Error message"
}
```

**After (Rust):**
```json
{
  "status": "error",
  "message": "Error message",
  "code": "ERROR_CODE"
}
```

### Pagination

**Before (Node.js):**
```json
{
  "data": [...],
  "page": 1,
  "total": 100,
  "pages": 10
}
```

**After (Rust):**
```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "page_size": 10,
    "total_items": 100,
    "total_pages": 10
  }
}
```

---

## Data Format Changes

### User Object

**Before (Node.js):**
```json
{
  "_id": "507f1f77bcf86cd799439011",
  "firstname": "John",
  "lastname": "Doe",
  "email": "john@example.com",
  "mobile": "1234567890",
  "role": "user"
}
```

**After (Rust):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "mobile": "1234567890",
  "role": "user"
}
```

**Key Changes:**
- `_id` → `id` (MongoDB ObjectId → PostgreSQL UUID)
- `firstname` → `first_name` (snake_case)
- `lastname` → `last_name` (snake_case)

### Product Object

**Before (Node.js):**
```json
{
  "_id": "507f1f77bcf86cd799439011",
  "title": "Product Name",
  "price": 99.99,
  "category": "507f1f77bcf86cd799439012",
  "totalrating": 4.5
}
```

**After (Rust):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Product Name",
  "price": 99.99,
  "category_id": "550e8400-e29b-41d4-a716-446655440001",
  "total_ratings": 4.5
}
```

**Key Changes:**
- `_id` → `id`
- `category` → `category_id`
- `totalrating` → `total_ratings`

### Date/Time Format

**Before (Node.js):**
```json
{
  "createdAt": "2026-01-15T10:30:00.000Z",
  "updatedAt": "2026-01-15T12:45:00.000Z"
}
```

**After (Rust):**
```json
{
  "created_at": "2026-01-15T10:30:00.000Z",
  "updated_at": "2026-01-15T12:45:00.000Z"
}
```

**Key Changes:**
- `createdAt` → `created_at`
- `updatedAt` → `updated_at`

---

## Migration Checklist for Frontend

### Configuration

- [ ] Update API base URL in environment variables
- [ ] Update `.env.development` and `.env.production`
- [ ] Change default port from 5000 to 8080 (if hardcoded)

### API Client

- [ ] Update axios/fetch configuration
- [ ] Implement token refresh logic with POST method
- [ ] Update error handling for new response format
- [ ] Update pagination handling

### Endpoints

- [ ] Change `/api/product` to `/api/products`
- [ ] Change `/api/category` to `/api/categories`
- [ ] Change `/api/order` to `/api/orders`
- [ ] Change `/api/user/wishlist` to `/api/users/wishlist`
- [ ] Change search query from `?q=` to `?search=`
- [ ] Change refresh token from GET to POST

### Data Mapping

- [ ] Update ID field from `_id` to `id`
- [ ] Update snake_case fields (firstname → first_name)
- [ ] Update date fields (createdAt → created_at)
- [ ] Update response structure parsing
- [ ] Handle UUID format instead of ObjectId

### Components

- [ ] Update user profile components
- [ ] Update product listing components
- [ ] Update cart components
- [ ] Update order components
- [ ] Update wishlist components

### Testing

- [ ] Test user authentication flow
- [ ] Test product listing and search
- [ ] Test cart operations
- [ ] Test order creation
- [ ] Test wishlist functionality
- [ ] Test error handling
- [ ] Test token refresh

---

## Environment Variables

### Development

Create `.env.development`:

```env
VITE_API_BASE_URL=http://localhost:8080/api
VITE_API_TIMEOUT=10000
```

### Production

Create `.env.production`:

```env
VITE_API_BASE_URL=https://api.yourdomain.com/api
VITE_API_TIMEOUT=10000
```

---

## Example Code Updates

### Before (Node.js API)

```javascript
// services/productService.js
import apiClient from './api/client';

export const getProducts = async (page = 1) => {
  const response = await apiClient.get(`/product?page=${page}`);
  return response.data;
};

export const searchProducts = async (query) => {
  const response = await apiClient.get(`/product/search?q=${query}`);
  return response.data;
};

export const getProduct = async (id) => {
  const response = await apiClient.get(`/product/${id}`);
  return response.data.data;
};
```

### After (Rust API)

```javascript
// services/productService.js
import apiClient from './api/client';

export const getProducts = async (page = 1) => {
  const response = await apiClient.get(`/products?page=${page}`);
  return response.data;
};

export const searchProducts = async (query) => {
  const response = await apiClient.get(`/products?search=${query}`);
  return response.data;
};

export const getProduct = async (id) => {
  const response = await apiClient.get(`/products/${id}`);
  return response.data; // Direct data object
};
```

---

## Troubleshooting

### CORS Issues

If you encounter CORS errors:

1. Check that the Rust API has proper CORS configuration
2. Verify the allowed origins in `.env`
3. Ensure credentials are properly sent

```javascript
// Enable credentials in axios
apiClient.defaults.withCredentials = true;
```

### 404 Errors

- Double-check endpoint paths (singular vs plural)
- Verify API base URL
- Check for trailing slashes

### Authentication Errors

- Ensure token is properly stored
- Verify Authorization header format: `Bearer <token>`
- Check token expiration handling

### Data Format Errors

- Verify field name changes (camelCase vs snake_case)
- Check for null/undefined values
- Validate UUID format

---

## Support

For issues or questions:
- Review API documentation: `/rust-ecommerce/API_DOCUMENTATION.md`
- Check backend logs for detailed error messages
- Test endpoints with Postman/Thunder Client
- Consult the Rust API team

---

## Summary

The migration to the Rust API requires primarily:

1. **URL updates**: Singular to plural endpoints (product → products)
2. **Field naming**: camelCase to snake_case (firstName → first_name)
3. **ID format**: MongoDB ObjectId to PostgreSQL UUID
4. **HTTP methods**: Some endpoints changed (GET to POST for refresh)
5. **Response structure**: Minor adjustments to JSON structure

Most of the business logic and user workflows remain unchanged. The frontend integration should be straightforward with these documented changes.
