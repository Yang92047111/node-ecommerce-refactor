# Rust E-Commerce Backend

A modern e-commerce backend built with Rust, Actix-web, PostgreSQL, and SQLx.

## Features

- **Async/Await**: Built on Tokio runtime for high-performance async operations
- **Type Safety**: Leveraging Rust's type system for compile-time correctness
- **Database**: PostgreSQL with SQLx for compile-time verified queries
- **Testing**: Integration tests with Testcontainers for real database testing
- **Error Handling**: Comprehensive error handling with custom error types
- **Logging**: Structured logging with env_logger

## Prerequisites

- Rust 1.70 or higher
- PostgreSQL 14 or higher
- Docker (for running tests with Testcontainers)

## Project Structure

```
rust-ecommerce/
├── Cargo.toml                  # Project dependencies
├── .env                        # Environment variables
├── .env.test                   # Test environment variables
├── migrations/                 # Database migrations
│   └── 20260115000000_initial_schema.sql
├── src/
│   ├── main.rs                # Application entry point
│   ├── lib.rs                 # Library root
│   ├── config/                # Configuration modules
│   │   ├── mod.rs
│   │   └── database.rs        # Database connection setup
│   ├── models/                # Database entity models
│   │   └── mod.rs
│   ├── dto/                   # Data Transfer Objects
│   │   └── mod.rs
│   ├── repositories/          # Database access layer
│   │   └── mod.rs
│   ├── services/              # Business logic layer
│   │   └── mod.rs
│   ├── handlers/              # HTTP request handlers
│   │   └── mod.rs
│   ├── middleware/            # HTTP middleware
│   │   └── mod.rs
│   ├── utils/                 # Utility functions
│   │   └── mod.rs
│   └── errors/                # Error types
│       ├── mod.rs
│       └── app_error.rs
└── tests/
    ├── common/                # Test utilities
    │   ├── mod.rs
    │   └── test_container.rs  # Testcontainers setup
    └── integration/           # Integration tests
        ├── mod.rs
        └── health_test.rs
```

## Getting Started

### 1. Install Dependencies

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Set Up Environment Variables

Copy the `.env` file and update it with your configuration:

```bash
cp .env .env.local
```

Edit `.env` with your PostgreSQL connection details:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/ecommerce
PORT=8080
RUST_LOG=info
```

### 3. Set Up PostgreSQL

Create a PostgreSQL database:

```sql
CREATE DATABASE ecommerce;
```

### 4. Run Migrations

The application will automatically run migrations on startup. Alternatively, you can use SQLx CLI:

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations manually
sqlx migrate run
```

### 5. Build the Project

```bash
cargo build
```

### 6. Run the Application

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080` (or the port specified in your `.env` file).

### 7. Test the Health Endpoint

```bash
curl http://127.0.0.1:8080/health
```

Expected response:
```json
{
  "status": "ok",
  "message": "Server is running"
}
```

## Running Tests

### Unit and Integration Tests

The project uses Testcontainers to spin up a real PostgreSQL database for integration tests.

```bash
# Run all tests
cargo test

# Run tests with logging output
RUST_LOG=debug cargo test -- --nocapture

# Run specific test
cargo test test_health_check_endpoint
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Development

### Database Migrations

To create a new migration:

```bash
sqlx migrate add <migration_name>
```

This will create a new SQL file in the `migrations/` directory.

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

### Linting

```bash
# Run clippy for linting
cargo clippy

# Run clippy with stricter warnings
cargo clippy -- -W clippy::pedantic
```

## Architecture

### Layered Architecture

The application follows a layered architecture pattern:

1. **Handlers Layer**: HTTP request/response handling
2. **Services Layer**: Business logic
3. **Repositories Layer**: Database operations
4. **Models Layer**: Domain entities

### Error Handling

Custom error types are defined in `src/errors/app_error.rs`:

- `DatabaseError`: Database operation errors
- `BadRequest`: Invalid request data
- `Unauthorized`: Authentication errors
- `Forbidden`: Authorization errors
- `NotFound`: Resource not found
- `Conflict`: Resource conflicts
- `ValidationError`: Input validation errors

### Logging

The application uses `env_logger` for logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
RUST_LOG=debug cargo run
RUST_LOG=info cargo run
RUST_LOG=warn cargo run
```

## Next Steps

Phase 1 is now complete. The foundation is set up with:

- ✅ Cargo project initialized
- ✅ Project structure created
- ✅ Database migrations defined
- ✅ PostgreSQL connection pool configured
- ✅ Error handling implemented
- ✅ Logging configured
- ✅ Testcontainers infrastructure set up
- ✅ Health check endpoint implemented
- ✅ Integration tests working

**Next Phase**: Phase 2 - Authentication & User Management
- User registration and login
- JWT token generation
- Password hashing with Argon2
- User CRUD operations
- Role-based authorization

## Contributing

1. Create a feature branch from `develop`
2. Make your changes
3. Write tests for your changes
4. Ensure all tests pass
5. Submit a pull request

## License

See LICENSE file for details.
