#!/usr/bin/env node

/**
 * E-commerce API Endpoint Test Script
 * Simulates complete user flow from registration to order placement
 * 
 * Usage: node test-endpoints.js
 */

const https = require('https');
const http = require('http');

// Configuration
const BASE_URL = process.env.API_URL || 'http://localhost:3000';
const VERBOSE = process.env.VERBOSE === 'true';

// ANSI color codes for terminal output
const colors = {
    reset: '\x1b[0m',
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m',
    magenta: '\x1b[35m'
};

// Test state to store data between requests
const testState = {
    accessToken: '',
    refreshToken: '',
    userId: '',
    productId: '',
    categoryId: '',
    blogId: '',
    couponId: '',
    cartId: '',
    orderId: '',
    testEmail: 'testuser@ecommerce-test.com', // Use consistent email for reusable test user
    testPassword: 'Test@123456',
    testMobile: '9999999999' // Unique mobile number for test user
};

// Statistics
const stats = {
    total: 0,
    passed: 0,
    failed: 0,
    startTime: Date.now()
};

/**
 * Make HTTP request
 */
function makeRequest(method, path, data = null, headers = {}) {
    return new Promise((resolve, reject) => {
        const url = new URL(path, BASE_URL);
        const isHttps = url.protocol === 'https:';
        const lib = isHttps ? https : http;
        
        const options = {
            hostname: url.hostname,
            port: url.port || (isHttps ? 443 : 80),
            path: url.pathname + url.search,
            method: method,
            headers: {
                'Content-Type': 'application/json',
                ...headers
            }
        };

        if (data && method !== 'GET') {
            const payload = JSON.stringify(data);
            options.headers['Content-Length'] = Buffer.byteLength(payload);
        }

        const req = lib.request(options, (res) => {
            let body = '';
            
            res.on('data', (chunk) => {
                body += chunk;
            });
            
            res.on('end', () => {
                try {
                    const response = {
                        statusCode: res.statusCode,
                        headers: res.headers,
                        body: body ? JSON.parse(body) : null
                    };
                    resolve(response);
                } catch (e) {
                    resolve({
                        statusCode: res.statusCode,
                        headers: res.headers,
                        body: body
                    });
                }
            });
        });

        req.on('error', (error) => {
            reject(error);
        });

        if (data && method !== 'GET') {
            req.write(JSON.stringify(data));
        }

        req.end();
    });
}

/**
 * Test runner function
 */
async function runTest(name, testFn, expectedStatus = 200) {
    stats.total++;
    process.stdout.write(`${colors.cyan}[TEST ${stats.total}]${colors.reset} ${name} ... `);
    
    try {
        const result = await testFn();
        
        if (result.statusCode === expectedStatus) {
            console.log(`${colors.green}✓ PASSED${colors.reset}`);
            if (VERBOSE && result.body) {
                console.log(`  Response:`, JSON.stringify(result.body, null, 2));
            }
            stats.passed++;
            return result;
        } else {
            console.log(`${colors.red}✗ FAILED${colors.reset}`);
            console.log(`  Expected status: ${expectedStatus}, Got: ${result.statusCode}`);
            if (result.body) {
                console.log(`  Response:`, JSON.stringify(result.body, null, 2));
            }
            stats.failed++;
            return result;
        }
    } catch (error) {
        console.log(`${colors.red}✗ ERROR${colors.reset}`);
        console.log(`  ${error.message}`);
        stats.failed++;
        throw error;
    }
}

/**
 * Sleep utility
 */
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Print section header
 */
function printSection(title) {
    console.log(`\n${colors.magenta}${'='.repeat(60)}`);
    console.log(`  ${title}`);
    console.log(`${'='.repeat(60)}${colors.reset}\n`);
}

/**
 * Test scenarios
 */
async function runTests() {
    console.log(`${colors.blue}
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║     E-COMMERCE API ENDPOINT TEST SUITE                   ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
${colors.reset}`);
    
    console.log(`Testing API at: ${colors.yellow}${BASE_URL}${colors.reset}\n`);

    try {
        // ============================================================
        // 1. Health Check & Setup
        // ============================================================
        printSection('1. HEALTH CHECK & SYSTEM STATUS');
        
        await runTest('Server Health Check', async () => {
            return await makeRequest('GET', '/');
        });

        // ============================================================
        // 2. User Authentication Flow
        // ============================================================
        printSection('2. USER AUTHENTICATION FLOW');
        
        // Try to login first
        console.log(`${colors.cyan}Attempting to login with existing user...${colors.reset}`);
        let loginResponse = await makeRequest('POST', '/api/auth/login', {
            email: testState.testEmail,
            password: testState.testPassword
        });

        // If login fails (user doesn't exist), register new user
        if (loginResponse.statusCode !== 200) {
            console.log(`${colors.yellow}User doesn't exist, registering new user...${colors.reset}\n`);
            
            await runTest('User Registration', async () => {
                const response = await makeRequest('POST', '/api/auth/register', {
                    firstName: 'Test',
                    lastName: 'User',
                    email: testState.testEmail,
                    mobile: testState.testMobile,
                    password: testState.testPassword
                });
                
                if (response.body && response.body._id) {
                    testState.userId = response.body._id;
                }
                return response;
            }, 201);

            await sleep(1000);
            
            // Login after registration
            loginResponse = await makeRequest('POST', '/api/auth/login', {
                email: testState.testEmail,
                password: testState.testPassword
            });
        } else {
            console.log(`${colors.green}✓ User already exists, using existing account${colors.reset}\n`);
        }

        await runTest('User Login', async () => {
            if (loginResponse.body) {
                testState.accessToken = loginResponse.body.token || loginResponse.body.accessToken;
                testState.refreshToken = loginResponse.body.refreshToken;
                if (!testState.userId && loginResponse.body.user) {
                    testState.userId = loginResponse.body.user._id;
                }
            }
            return loginResponse;
        });

        // ============================================================
        // 3. Category Management
        // ============================================================
        printSection('3. CATEGORY MANAGEMENT');
        
        await runTest('Get All Categories', async () => {
            return await makeRequest('GET', '/api/category/all', null, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        await runTest('Create New Category', async () => {
            const response = await makeRequest('POST', '/api/category/create', {
                title: `Test Category ${Date.now()}`,
                description: 'Test category description'
            }, {
                'Authorization': `Bearer ${testState.accessToken}`
            }, 201);
            
            if (response.body && response.body._id) {
                testState.categoryId = response.body._id;
            }
            return response;
        }, 201);

        if (testState.categoryId) {
            await runTest('Get Single Category', async () => {
                return await makeRequest('GET', `/api/category/${testState.categoryId}`);
            });
        }

        // ============================================================
        // 4. Product Management
        // ============================================================
        printSection('4. PRODUCT MANAGEMENT');
        
        await runTest('Get All Products', async () => {
            return await makeRequest('GET', '/api/product/all', null, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        if (testState.categoryId) {
            await runTest('Create New Product', async () => {
                const response = await makeRequest('POST', '/api/product/create', {
                    title: `Test Product ${Date.now()}`,
                    description: 'Test product description',
                    price: 99.99,
                    quantity: 100,
                    brand: 'Test Brand',
                    category: testState.categoryId
                }, {
                    'Authorization': `Bearer ${testState.accessToken}`
                }, 201);
                
                if (response.body && response.body._id) {
                    testState.productId = response.body._id;
                }
                return response;
            }, 201);
        }

        if (testState.productId) {
            await runTest('Get Single Product', async () => {
                return await makeRequest('GET', `/api/product/${testState.productId}`);
            });

            await runTest('Update Product', async () => {
                return await makeRequest('PUT', `/api/product/${testState.productId}`, {
                    title: `Updated Test Product ${Date.now()}`,
                    price: 89.99
                }, {
                    'Authorization': `Bearer ${testState.accessToken}`
                });
            });
        }

        // ============================================================
        // 5. Cart Management
        // ============================================================
        printSection('5. CART MANAGEMENT');
        
        if (testState.productId) {
            await runTest('Add Product to Cart', async () => {
                const response = await makeRequest('POST', '/api/cart/add', {
                    productId: testState.productId,
                    quantity: 2,
                    color: 'Black',
                    price: 99.99
                }, {
                    'Authorization': `Bearer ${testState.accessToken}`
                });
                
                if (response.body && response.body._id) {
                    testState.cartId = response.body._id;
                }
                return response;
            });
        }

        await runTest('Get User Cart', async () => {
            return await makeRequest('GET', '/api/cart/', {}, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        // ============================================================
        // 6. Coupon Management
        // ============================================================
        printSection('6. COUPON MANAGEMENT');
        
        // Note: No GET route for coupons exists in the backend

        if (testState.categoryId) {
            await runTest('Create New Coupon', async () => {
            const expiryDate = new Date();
            expiryDate.setDate(expiryDate.getDate() + 30);
            
            const response = await makeRequest('POST', '/api/coupon/create', {
                name: `TEST${Date.now()}`,
                expiry: expiryDate.toISOString(),
                discount: 10
            }, {
                'Authorization': `Bearer ${testState.accessToken}`
            }, 201);
            
            if (response.body && response.body._id) {
                testState.couponId = response.body._id;
            }
            return response;
        }, 201);
        }

        // ============================================================
        // 7. Blog Management
        // ============================================================
        printSection('7. BLOG MANAGEMENT');
        
        await runTest('Get All Blogs', async () => {
            return await makeRequest('GET', '/api/blog/all', {}, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        await runTest('Create New Blog', async () => {
            const response = await makeRequest('POST', '/api/blog/', {
                title: `Test Blog ${Date.now()}`,
                description: 'Test blog description',
                category: 'Technology',
                author: 'Test Author'
            }, {
                'Authorization': `Bearer ${testState.accessToken}`
            }, 201);
            
            if (response.body && response.body._id) {
                testState.blogId = response.body._id;
            }
            return response;
        }, 201);

        if (testState.blogId) {
            await runTest('Get Single Blog', async () => {
                return await makeRequest('GET', `/api/blog/${testState.blogId}`, {}, {
                    'Authorization': `Bearer ${testState.accessToken}`
                });
            });
        }

        // ============================================================
        // 8. Order Management
        // ============================================================
        printSection('8. ORDER MANAGEMENT');
        
        if (testState.cartId) {
            await runTest('Create Order from Cart', async () => {
                const response = await makeRequest('POST', '/api/order/create', {
                    shippingInfo: {
                        firstName: 'Test',
                        lastName: 'User',
                        address: '123 Test St',
                        city: 'Test City',
                        state: 'TS',
                        country: 'Test Country',
                        pincode: '12345'
                    },
                    paymentInfo: {
                        method: 'COD'
                    }
                }, {
                    'Authorization': `Bearer ${testState.accessToken}`
                });
                
                if (response.body && response.body._id) {
                    testState.orderId = response.body._id;
                }
                return response;
            });
        }

        await runTest('Get User Orders', async () => {
            return await makeRequest('GET', '/api/order/user-orders', {}, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        // ============================================================
        // 9. Search Functionality
        // ============================================================
        printSection('9. SEARCH FUNCTIONALITY');
        
        await runTest('Search Products', async () => {
            return await makeRequest('POST', '/api/search/', {
                searchQuery: 'test'
            }, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        // ============================================================
        // 10. User Profile ManaWishlist', async () => {
            return await makeRequest('GET', '/api/user/wishlist', {}, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        // Remove the Update User Profile test as it doesn't exist in routes     lastName: 'User Updated'
            }, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

        // ============================================================
        // 11. Refresh Token Flow
        // ============================================================
        printSection('11. TOKEN REFRESH FLOW');
        
        if (testState.refreshToken) {
            await runTest('Refresh Access Token', async () => {
                return await makeRequest('GET', '/api/auth/refresh');
            });
        }

        // ============================================================
        // 12. Logout
        // ============================================================
        printSection('12. USER LOGOUT');
        
        await runTest('User Logout', async () => {
            return await makeRequest('POST', '/api/auth/logout', {}, {
                'Authorization': `Bearer ${testState.accessToken}`
            });
        });

    } catch (error) {
        console.error(`\n${colors.red}Fatal error during test execution:${colors.reset}`, error.message);
    }

    // ============================================================
    // Print Results
    // ============================================================
    const duration = ((Date.now() - stats.startTime) / 1000).toFixed(2);
    const passRate = ((stats.passed / stats.total) * 100).toFixed(1);
    
    console.log(`\n${colors.blue}${'='.repeat(60)}`);
    console.log(`  TEST RESULTS`);
    console.log(`${'='.repeat(60)}${colors.reset}\n`);
    
    console.log(`  Total Tests:     ${stats.total}`);
    console.log(`  ${colors.green}Passed:          ${stats.passed}${colors.reset}`);
    console.log(`  ${colors.red}Failed:          ${stats.failed}${colors.reset}`);
    console.log(`  Pass Rate:       ${passRate}%`);
    console.log(`  Duration:        ${duration}s`);
    
    console.log(`\n${colors.blue}${'='.repeat(60)}${colors.reset}\n`);

    if (stats.failed === 0) {
        console.log(`${colors.green}✓ All tests passed!${colors.reset}\n`);
        process.exit(0);
    } else {
        console.log(`${colors.red}✗ Some tests failed. Please check the output above.${colors.reset}\n`);
        process.exit(1);
    }
}

// Run tests
console.log('Starting test suite...\n');
runTests().catch((error) => {
    console.error(`${colors.red}Unhandled error:${colors.reset}`, error);
    process.exit(1);
});
