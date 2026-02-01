# API Testing Guide

This guide explains how to test the E-commerce API endpoints and simulate user flows.

## 🧪 Available Test Scripts

### 1. Node.js Test Script (test-endpoints.js)

**Best for**: Comprehensive automated testing with detailed output

**Features**:
- ✅ Tests 12 different categories of endpoints
- ✅ Simulates complete user journey
- ✅ Automatic token management
- ✅ Detailed pass/fail reporting
- ✅ Color-coded terminal output
- ✅ Statistics and metrics

**Usage**:

```bash
# Run all tests
node test-endpoints.js

# Run with verbose output
VERBOSE=true node test-endpoints.js

# Test against different API
API_URL=http://production-api.com node test-endpoints.js
```

### 2. Bash Test Script (test-api.sh)

**Best for**: Quick manual testing and CI/CD pipelines

**Features**:
- ✅ Simple bash-based testing
- ✅ No Node.js dependencies
- ✅ Easy to customize
- ✅ Good for shell scripting

**Usage**:

```bash
# Make executable (first time only)
chmod +x test-api.sh

# Run tests
./test-api.sh

# Test against different API
API_URL=http://production-api.com ./test-api.sh
```

## 📋 Test Coverage

The test scripts cover the following user flow:

### 1. Authentication Flow
- ✓ User registration
- ✓ User login
- ✓ Token refresh
- ✓ User logout

### 2. Category Management
- ✓ List all categories
- ✓ Create new category
- ✓ Get single category
- ✓ Update category
- ✓ Delete category

### 3. Product Management
- ✓ List all products
- ✓ Create new product
- ✓ Get single product
- ✓ Update product
- ✓ Delete product
- ✓ Rate product
- ✓ Add to wishlist

### 4. Cart Operations
- ✓ Add product to cart
- ✓ View cart
- ✓ Update cart item quantity
- ✓ Remove from cart
- ✓ Clear cart

### 5. Order Management
- ✓ Create order
- ✓ View user orders
- ✓ Get order details
- ✓ Update order status

### 6. Coupon System
- ✓ List available coupons
- ✓ Create coupon
- ✓ Apply coupon to cart
- ✓ Delete coupon

### 7. Blog System
- ✓ List all blogs
- ✓ Create blog post
- ✓ Get single blog
- ✓ Update blog
- ✓ Delete blog
- ✓ Like/Unlike blog

### 8. Search Functionality
- ✓ Search products
- ✓ Filter by category
- ✓ Filter by price range
- ✓ Sort results

### 9. User Profile
- ✓ Get user profile
- ✓ Update profile
- ✓ Change password
- ✓ Get wishlist
- ✓ Get order history

### 10. Email System
- ✓ Send verification email
- ✓ Password reset request
- ✓ Order confirmation email

## 🎯 Manual Testing with cURL

### Authentication

#### Register New User
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "firstName": "John",
    "lastName": "Doe",
    "email": "john.doe@example.com",
    "mobile": "1234567890",
    "password": "SecurePass123"
  }'
```

#### Login
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "password": "SecurePass123"
  }'
```

**Save the token from response:**
```bash
export TOKEN="your-jwt-token-here"
```

### Product Operations

#### Get All Products
```bash
curl -X GET http://localhost:3000/api/product/
```

#### Get Single Product
```bash
curl -X GET http://localhost:3000/api/product/{productId}
```

#### Create Product (Admin)
```bash
curl -X POST http://localhost:3000/api/product/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "title": "iPhone 15 Pro",
    "description": "Latest iPhone model",
    "price": 999.99,
    "quantity": 50,
    "brand": "Apple",
    "category": "Electronics"
  }'
```

#### Update Product
```bash
curl -X PUT http://localhost:3000/api/product/{productId} \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "price": 899.99,
    "quantity": 45
  }'
```

#### Delete Product
```bash
curl -X DELETE http://localhost:3000/api/product/{productId} \
  -H "Authorization: Bearer $TOKEN"
```

### Cart Operations

#### Add to Cart
```bash
curl -X POST http://localhost:3000/api/cart/add \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "productId": "product-id-here",
    "quantity": 2,
    "color": "Black",
    "price": 999.99
  }'
```

#### View Cart
```bash
curl -X GET http://localhost:3000/api/cart/ \
  -H "Authorization: Bearer $TOKEN"
```

#### Update Cart Item
```bash
curl -X PUT http://localhost:3000/api/cart/update \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "cartItemId": "cart-item-id",
    "quantity": 3
  }'
```

#### Remove from Cart
```bash
curl -X DELETE http://localhost:3000/api/cart/{cartItemId} \
  -H "Authorization: Bearer $TOKEN"
```

### Order Operations

#### Create Order
```bash
curl -X POST http://localhost:3000/api/order/create \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "shippingInfo": {
      "firstName": "John",
      "lastName": "Doe",
      "address": "123 Main St",
      "city": "New York",
      "state": "NY",
      "country": "USA",
      "pincode": "10001"
    },
    "paymentInfo": {
      "method": "COD"
    }
  }'
```

#### Get User Orders
```bash
curl -X GET http://localhost:3000/api/order/user-orders \
  -H "Authorization: Bearer $TOKEN"
```

#### Get Order Details
```bash
curl -X GET http://localhost:3000/api/order/{orderId} \
  -H "Authorization: Bearer $TOKEN"
```

### Category Operations

#### Get All Categories
```bash
curl -X GET http://localhost:3000/api/category/
```

#### Create Category
```bash
curl -X POST http://localhost:3000/api/category/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "title": "Electronics",
    "description": "Electronic devices and gadgets"
  }'
```

### Search Operations

#### Search Products
```bash
curl -X GET "http://localhost:3000/api/search/products?q=iphone"
```

#### Search with Filters
```bash
curl -X GET "http://localhost:3000/api/search/products?q=phone&category=Electronics&minPrice=500&maxPrice=1500"
```

### Coupon Operations

#### Get All Coupons
```bash
curl -X GET http://localhost:3000/api/coupon/
```

#### Create Coupon
```bash
curl -X POST http://localhost:3000/api/coupon/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "SUMMER2024",
    "expiry": "2024-12-31",
    "discount": 20
  }'
```

#### Apply Coupon
```bash
curl -X POST http://localhost:3000/api/cart/apply-coupon \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "couponCode": "SUMMER2024"
  }'
```

### Blog Operations

#### Get All Blogs
```bash
curl -X GET http://localhost:3000/api/blog/
```

#### Create Blog Post
```bash
curl -X POST http://localhost:3000/api/blog/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "title": "10 Tips for Online Shopping",
    "description": "Learn how to shop smartly online",
    "category": "Shopping Tips",
    "author": "John Doe"
  }'
```

#### Like Blog
```bash
curl -X PUT http://localhost:3000/api/blog/like/{blogId} \
  -H "Authorization: Bearer $TOKEN"
```

### User Profile

#### Get Profile
```bash
curl -X GET http://localhost:3000/api/users/profile \
  -H "Authorization: Bearer $TOKEN"
```

#### Update Profile
```bash
curl -X PUT http://localhost:3000/api/users/profile \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "firstName": "John",
    "lastName": "Smith",
    "mobile": "9876543210"
  }'
```

#### Change Password
```bash
curl -X PUT http://localhost:3000/api/users/change-password \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "currentPassword": "OldPass123",
    "newPassword": "NewSecurePass456"
  }'
```

## 🔍 Testing with Postman

### Import Collection

1. Open Postman
2. Click **Import**
3. Use the API endpoints documented above
4. Set environment variables:
   - `baseUrl`: http://localhost:3000
   - `token`: (will be set after login)

### Automated Collection Run

1. Create a collection with all endpoints
2. Add tests to each request:

```javascript
// Example test script
pm.test("Status code is 200", function () {
    pm.response.to.have.status(200);
});

pm.test("Response has data", function () {
    var jsonData = pm.response.json();
    pm.expect(jsonData).to.have.property('data');
});

// Save token from login
if (pm.response.json().token) {
    pm.environment.set("token", pm.response.json().token);
}
```

3. Run collection with **Collection Runner**

## 📊 Load Testing

### Using Apache Bench

```bash
# Install Apache Bench (macOS)
brew install httpd

# Test product listing (100 requests, 10 concurrent)
ab -n 100 -c 10 http://localhost:3000/api/product/

# Test with POST request
ab -n 100 -c 10 -p data.json -T application/json \
  http://localhost:3000/api/auth/login
```

### Using Artillery

```bash
# Install Artillery
npm install -g artillery

# Create test config (artillery.yml)
cat > artillery.yml << EOF
config:
  target: "http://localhost:3000"
  phases:
    - duration: 60
      arrivalRate: 10
scenarios:
  - name: "Product Browsing"
    flow:
      - get:
          url: "/api/product/"
      - get:
          url: "/api/category/"
EOF

# Run load test
artillery run artillery.yml
```

## 🐛 Debugging Failed Tests

### Enable Debug Output

```bash
# For Node.js script
DEBUG=* node test-endpoints.js

# For bash script
bash -x test-api.sh
```

### Check API Logs

```bash
# Docker logs
docker compose logs -f backend

# Check for errors
docker compose logs backend | grep -i error
```

### Common Issues

#### 1. Connection Refused
```bash
# Check if API is running
curl http://localhost:3000/

# Check Docker containers
docker compose ps
```

#### 2. 401 Unauthorized
```bash
# Token might be expired or invalid
# Re-run login test and get new token
```

#### 3. 404 Not Found
```bash
# Check if endpoint exists
# Verify API routes in backend code
```

#### 4. 500 Internal Server Error
```bash
# Check backend logs
docker compose logs backend

# Check database connection
docker compose logs mongodb
```

## ✅ Best Practices

1. **Always test locally first** before deploying
2. **Use environment variables** for different environments
3. **Store tokens securely** (never commit to git)
4. **Run tests regularly** in CI/CD pipeline
5. **Test error scenarios** not just happy paths
6. **Monitor response times** for performance
7. **Test with realistic data** volumes
8. **Validate response schemas**
9. **Test concurrent requests**
10. **Document test cases** and expected results

## 🎯 CI/CD Integration

### GitHub Actions

```yaml
name: API Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Start services
        run: docker compose up -d
      
      - name: Wait for API
        run: |
          timeout 60 bash -c 'until curl -f http://localhost:3000/; do sleep 2; done'
      
      - name: Run tests
        run: node test-endpoints.js
      
      - name: Cleanup
        run: docker compose down
```

## 📝 Test Report Example

```
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║     E-COMMERCE API ENDPOINT TEST SUITE                   ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝

Testing API at: http://localhost:3000

============================================================
  TEST RESULTS
============================================================

  Total Tests:     25
  Passed:          25
  Failed:          0
  Pass Rate:       100.0%
  Duration:        8.45s

============================================================

✓ All tests passed!
```

## 🆘 Need Help?

- Check [DOCKER_DEPLOYMENT.md](./DOCKER_DEPLOYMENT.md) for deployment issues
- Review API logs: `docker compose logs backend`
- Verify database: `docker compose exec mongodb mongosh`
- Test connectivity: `curl -v http://localhost:3000/`
