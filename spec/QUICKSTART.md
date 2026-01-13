# Quick Start Guide: Rust E-commerce Migration

## 🚀 Getting Started in 5 Minutes

### Prerequisites
```bash
# Check you have these installed
rust --version      # Should be 1.75+
cargo --version
docker --version
psql --version      # PostgreSQL client
```

### Step 1: Create Project (2 min)
```bash
# Create new Rust project
cargo new rust-ecommerce --bin
cd rust-ecommerce

# Copy templates
cp ../spec/Cargo.toml.template ./Cargo.toml
cp ../spec/.env.template ./.env
cp ../spec/.env.test.template ./.env.test

# Edit .env with your database credentials
nano .env
```

### Step 2: Set Up Database (2 min)
```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Create database
createdb ecommerce_db

# Create migrations directory
mkdir -p migrations

# Copy schema from spec.md to migrations/001_initial_schema.sql
# See spec/spec.md "Database Schema (PostgreSQL)" section

# Run migrations
sqlx migrate run
```

### Step 3: Set Up Tests (1 min)
```bash
# Copy test files
mkdir -p tests/{common,integration}
cp ../spec/test_*.rs tests/common/
cp ../spec/*_tests.rs tests/integration/
cp ../spec/fixtures.rs tests/common/
cp ../spec/run_integration_tests.sh .
chmod +x run_integration_tests.sh

# Verify Docker is running
docker info

# Test setup
./run_integration_tests.sh setup
```

## 📋 Daily Workflow

### Start Development
```bash
# Terminal 1: Run PostgreSQL
docker run -d -p 5432:5432 \
  -e POSTGRES_USER=ecommerce_user \
  -e POSTGRES_PASSWORD=ecommerce_password \
  -e POSTGRES_DB=ecommerce_db \
  postgres:16-alpine

# Terminal 2: Build and run
cargo watch -x run
```

### Run Tests
```bash
# All tests
cargo test

# Integration tests only
./run_integration_tests.sh all

# Specific test
./run_integration_tests.sh test auth_tests

# With output
cargo test -- --nocapture
```

### Check Code Quality
```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check without building
cargo check
```

## 📚 Project Structure Quick Reference

```
rust-ecommerce/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── config/              # DB config, env
│   ├── models/              # Domain models
│   ├── dto/                 # Request/response DTOs
│   ├── repositories/        # Data access
│   ├── services/            # Business logic
│   ├── handlers/            # HTTP handlers
│   ├── middleware/          # Auth, errors
│   └── utils/               # Helpers
├── tests/
│   ├── common/              # Test utilities
│   └── integration/         # Integration tests
├── migrations/              # SQL migrations
└── Cargo.toml              # Dependencies
```

## 🎯 Current Phase: Phase 1 - Foundation Setup

### Tasks Checklist
- [ ] Project initialized with Cargo
- [ ] Database connection configured
- [ ] Migrations created and run
- [ ] Error handling framework
- [ ] Logging configured
- [ ] Test infrastructure working
- [ ] First integration test passing

### What to Build First
1. **Config module** (`src/config/database.rs`)
   - Database connection pool
   - Environment variables

2. **Error handling** (`src/errors/app_error.rs`)
   - Custom error types
   - Error responses

3. **Basic server** (`src/main.rs`)
   - Actix-web setup
   - Health check endpoint

4. **First test** (`tests/integration/health_tests.rs`)
   - Test server startup
   - Test database connection

## 🔧 Common Commands

```bash
# Build
cargo build                 # Debug build
cargo build --release       # Release build

# Run
cargo run                   # Run with default config
cargo run -- --port 8080    # Run with args

# Test
cargo test                  # All tests
cargo test auth             # Tests matching "auth"
cargo test --test auth_tests # Specific test file

# Database
sqlx migrate add <name>     # Create new migration
sqlx migrate run            # Run migrations
sqlx migrate revert         # Revert last migration

# Quality
cargo fmt                   # Format code
cargo clippy                # Lint
cargo check                 # Type check
cargo audit                 # Security audit

# Docs
cargo doc --open            # Generate and open docs
```

## 🐛 Troubleshooting

### "Database connection failed"
```bash
# Check PostgreSQL is running
psql -h localhost -U ecommerce_user -d ecommerce_db

# Check DATABASE_URL in .env
echo $DATABASE_URL

# Test connection
sqlx database create
sqlx migrate run
```

### "Docker not found" (tests)
```bash
# Start Docker Desktop (macOS)
open -a Docker

# Or install Docker
brew install --cask docker
```

### "Compilation errors"
```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

## 📖 Key Files to Read

1. **spec/spec.md** - Complete specification (read first!)
2. **spec/README.md** - Detailed usage guide
3. **spec/summary-phase0.md** - Planning phase summary
4. **spec/test_*.rs** - Test examples
5. **Cargo.toml** - Dependencies and config

## 🎓 Learning Resources

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Actix-web
- [Actix-web Docs](https://actix.rs/)
- [Actix Examples](https://github.com/actix/examples)

### SQLx
- [SQLx Docs](https://docs.rs/sqlx/)
- [SQLx GitHub](https://github.com/launchbadge/sqlx)

### PostgreSQL
- [PostgreSQL Tutorial](https://www.postgresqltutorial.com/)
- [SQL Style Guide](https://www.sqlstyle.guide/)

## ⏱️ Time Estimates

| Phase | Task | Time |
|-------|------|------|
| 1 | Foundation Setup | 3-5 days |
| 2 | Auth & Users | 5-7 days |
| 3 | Products & Categories | 5-7 days |
| 4 | Cart & Orders | 5-7 days |
| 5 | Coupons & Features | 3-5 days |
| 6 | Performance & Docs | 3-5 days |
| 7 | Migration & Deploy | 3-5 days |
| **Total** | | **25-38 days** |

## 🎯 Success Criteria

### Phase 1 Complete When:
- [x] Server starts without errors
- [x] Can connect to PostgreSQL
- [x] Health check endpoint responds
- [x] At least 1 integration test passes
- [x] Logging is working
- [x] Error handling is consistent
- [x] Documentation is updated

### Overall Project Complete When:
- [x] All API endpoints migrated
- [x] All tests passing (>80% coverage)
- [x] Data migrated from MongoDB
- [x] Frontend working with new API
- [x] Deployed to production
- [x] Performance meets requirements
- [x] Documentation complete

## 💡 Tips

1. **Start small**: Get health check working first
2. **Test-driven**: Write test before implementation
3. **Commit often**: Small, focused commits
4. **Read errors**: Rust errors are helpful, read them carefully
5. **Use clippy**: It teaches Rust best practices
6. **Ask for help**: Rust community is very helpful

## 📞 Getting Help

- **Rust Forums**: [users.rust-lang.org](https://users.rust-lang.org/)
- **Discord**: [Rust Discord](https://discord.gg/rust-lang)
- **Stack Overflow**: Tag `rust`, `actix-web`, `sqlx`
- **This Project**: Check spec/ directory documentation

## ✅ Quick Wins

Get these working in first hour:
1. `cargo new` - Project created ✓
2. `cargo build` - Compiles ✓
3. `cargo test` - Tests run ✓
4. Database connects ✓
5. Server starts ✓

You're ready to start Phase 1! 🦀
