Node-ecommerce
E-Commerce App with REST API | MongoDB | Advanced Authentication

Overview
This is a Node.js e-commerce project that allows users to browse products, add them to their cart, and complete purchases.
Project is still under construction, supposed  to end by 30 March 2024.
Stay tuned for updates.

Features
- User Authentication: Users can sign up, log in, and log out securely.
- Product Catalog: Users can view a list of products with details such as name, price, and description.
- Shopping Cart: Users can add products to their shopping cart and manage the quantities.
- Checkout Process: Users can complete purchases by entering shipping and payment information.
- Order History: Users can view their past orders.
- Admin Panel: Administrators can add, edit, and delete products.
Technologies Used
- Node.js
- Express.js
- MongoDB (with Mongoose ODM)
- JWT for authentication

Getting Started

### 🐳 Docker Deployment (Recommended)

The easiest way to run this application is using Docker. All deployment files are organized in the `deploy/` directory:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/shazaaly/node-ecommerce
   cd node-ecommerce-main
   ```

2. **Setup environment:**
   ```bash
   cp deploy/config/.env.example .env
   # Edit .env with your configuration
   ```

3. **Deploy with one command:**
   ```bash
   chmod +x deploy/scripts/deploy.sh
   ./deploy/scripts/deploy.sh
   ```

4. **Access the application:**
   - Frontend: http://localhost
   - Backend API: http://localhost:3000
   - API Docs: http://localhost:3000/api-docs

5. **Test the endpoints:**
   ```bash
   ./deploy/scripts/test-api.sh
   # or
   node deploy/scripts/test-endpoints.js
   ```

📚 **Complete Deployment Documentation:**
- 📂 [Deploy Directory](./deploy/README.md) - Complete deployment guide and structure
- 🚀 [Get Started Guide](./deploy/docs/GET_STARTED.md) - Beginner's deployment guide
- 🐳 [Docker Deployment](./deploy/docs/DOCKER_DEPLOYMENT.md) - Detailed Docker instructions
- 🧪 [API Testing Guide](./deploy/docs/API_TESTING.md) - Comprehensive testing documentation
- ⚡ [Quick Reference](./deploy/docs/QUICK_REFERENCE.md) - Handy command reference

**Directory Structure:**
```
deploy/
├── config/          # Docker and configuration files
├── docs/            # All deployment documentation
├── scripts/         # Deployment and testing scripts
└── README.md        # Deployment overview
```

### 💻 Manual Installation

1. Install dependencies:
   ```bash
   npm install
   ```

2. Set up environment variables:
   - Create a `.env` file in the root directory
   - Define the following variables:
     ```
     PORT=3000
     MONGO_URL=mongodb://localhost:27017/ecommercedb
     JWT_SECRET=your_jwt_secret
     REFRESH_TOKEN_SECRET=your_refresh_token_secret
     ```

3. Run MongoDB and Elasticsearch locally

4. Run the application:
   ```bash
   npm start
   ```

5. Access the application at `http://localhost:3000`

Folder Structure
- `config/`: Configuration files (e.g., database connection, authentication strategies).
- `models/`: Mongoose models for database schema.
- `routes/`: Express routes for different parts of the application.
- `views/`: Handlebars templates for rendering HTML.
- `public/`: Static assets (e.g., CSS, JavaScript).
- `controllers/`: Controllers for handling business logic.
- `middlewares/`: Custom middleware functions.
- `utils/`: Utility functions.



The application is deployed on Heroku. Feel free to test the live endpoints:

- **Base URL**: `https://enigmatic-ravine-23031-1df92bd6df4a.herokuapp.com`

### Endpoints

| Method | Endpoint                | Description                             | Parameters          |
|--------|-------------------------|-----------------------------------------|---------------------|
| GET    | `/api/resource`         | Retrieves all resources.                | N/A                 |
| POST   | `/api/resource`         | Creates a new resource.                 | `{ "data": "value"}`|
| GET    | `/api/resource/{id}`    | Retrieves a resource by ID.             | N/A                 |
| PUT    | `/api/resource/{id}`    | Updates a resource by ID.               | `{ "data": "new value"}`|
| DELETE | `/api/resource/{id}`    | Deletes a resource by ID.               | N/A                 |

Using the Endpoints

To use these endpoints, please ensure you have the necessary authentication tokens as many operations are protected. The tokens are obtained after successfully logging in.

markdown

## Authentication

To perform operations such as creating categories, adding to cart, creating orders, or creating coupons, you must be authenticated. Use the `POST /api/auth/login` endpoint to log in and obtain your Bearer token.

Here's an example of using the `POST /api/category` endpoint to create a new category:

```http
POST /api/category
Authorization: Bearer <your-token>
Content-Type: application/json

{
  "name": "Electronics",
  "description": "A category for all electronic products"
}

On successful creation, you'll receive a response like:

json

{
  "id": "new-category-id",
  "name": "Electronics",
  "description": "A category for all electronic products"
}

Note: Replace `<your-token>` with your actual auth token obtained after login.

---

Be sure to replace any placeholder text like `<your-token>` with actual valid tokens or parameters fo
For more details on how to use these endpoints, please refer to the [API Documentation] - #will be added soon

POSTMAN Doc.
https://documenter.getpostman.com/view/10270559/2sA35G4hMc


Contributing
Contributions are welcome!

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
