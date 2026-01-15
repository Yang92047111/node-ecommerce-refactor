# Specification and Test Templates

This directory contains the complete specification and test templates for refactoring the Node.js + MongoDB e-commerce platform to Rust + PostgreSQL + Testcontainers.

## Contents

### Planning Documents
- **spec.md** - Comprehensive refactoring specification including:
  - Current and target architecture
  - Database schema design (PostgreSQL)
  - Project structure
  - Migration phases (7 phases)
  - Testing strategy
  - Git flow guidelines

### Test Templates
- **test_common_mod.rs** - Common test utilities and testcontainer setup
- **test_container.rs** - Test context and helper functions
- **fixtures.rs** - Test fixtures and seed data generation
- **auth_tests.rs** - Authentication integration test examples
- **product_tests.rs** - Product management integration test examples
- **cart_order_tests.rs** - Cart and order workflow integration test examples

### Configuration Templates
- **Cargo.toml.template** - Complete Rust project dependencies
- **.env.template** - Production environment configuration
- **.env.test.template** - Test environment configuration

### Scripts
- **run_integration_tests.sh** - Script to set up and run integration tests

## How to Use

### 1. Review the Specification
Start by reading [spec.md](./spec.md) to understand:
- The overall architecture and design decisions
- The 7-phase migration plan
- Database schema changes
- API endpoint mappings

### 2. Set Up the Rust Project
When ready to start Phase 1:

```bash
# Create new Rust project
cargo new rust-ecommerce --bin
cd rust-ecommerce

# Copy Cargo.toml template
cp ../spec/Cargo.toml.template ./Cargo.toml

# Copy environment files
cp ../spec/.env.template ./.env
cp ../spec/.env.test.template ./.env.test

# Edit .env with your configuration
nano .env
```

### 3. Set Up Database
```bash
# Install PostgreSQL (macOS)
brew install postgresql@16
brew services start postgresql@16

# Create database
createdb ecommerce_db

# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Create migrations directory
mkdir migrations

# Copy the schema from spec.md into a migration file
# See spec.md "Database Schema (PostgreSQL)" section
```

### 4. Set Up Test Infrastructure
```bash
# Copy test files to your project
mkdir -p tests/common tests/integration

cp ../spec/test_common_mod.rs tests/common/mod.rs
cp ../spec/test_container.rs tests/common/test_container.rs
cp ../spec/fixtures.rs tests/common/fixtures.rs
cp ../spec/auth_tests.rs tests/integration/auth_tests.rs
cp ../spec/product_tests.rs tests/integration/product_tests.rs
cp ../spec/cart_order_tests.rs tests/integration/cart_order_tests.rs

# Make test script executable
cp ../spec/run_integration_tests.sh .
chmod +x run_integration_tests.sh

# Verify Docker is running
docker info

# Run test setup
./run_integration_tests.sh setup
```

### 5. Start Development
Follow the phases in spec.md:

**Phase 1: Foundation Setup**
- Set up project structure
- Configure database connection
- Create basic error handling
- Set up logging

**Phase 2: Authentication & User Management**
- Implement user models and repositories
- Build authentication system
- Create JWT utilities
- Write tests

**Phase 3-7**: Continue following the spec.md plan

### 6. Run Tests
```bash
# Run all integration tests
./run_integration_tests.sh all

# Run specific test
./run_integration_tests.sh test auth_tests

# Clean up test containers
./run_integration_tests.sh clean
```

## Phase Completion Workflow

At the end of each phase:

1. **Complete all tasks** listed in spec.md for that phase
2. **Ensure all tests pass**
3. **Create phase summary**: `spec/summary-phase<N>.md`
4. **Update spec.md** with status and progress
5. **Commit changes**:
   ```bash
   git add .
   git commit -m "feat(phase-<N>): complete <phase-name>
   
   - Task 1 completed
   - Task 2 completed
   - All tests passing
   
   Phase <N> completed. See summary-phase<N>.md"
   ```
6. **Merge to develop branch**

## Summary Template

When completing each phase, create a summary file:

**spec/summary-phase1.md**:
```markdown
# Phase 1 Summary: Foundation Setup

## Completion Date
YYYY-MM-DD

## Objectives
- [x] Objective 1
- [x] Objective 2

## What Was Accomplished
- Detailed description of work done
- Key decisions made
- Challenges encountered and solutions

## Test Results
- Tests written: X
- Tests passing: X
- Coverage: X%

## Next Steps
- What to do in Phase 2
- Any blockers or concerns

## Files Modified/Created
- List of key files
```

## Test Coverage Goals

Follow these coverage goals throughout the project:
- Overall: >80%
- Critical paths: >95%
- Repository layer: >90%
- Service layer: >85%
- Handler layer: >80%

## Integration Test Best Practices

1. **Use testcontainers** - Real PostgreSQL for integration tests
2. **Clean up** - Always clean database between tests
3. **Isolation** - Each test should be independent
4. **Realistic data** - Use the fixtures module for test data
5. **Test failures** - Test both success and failure paths
6. **Performance** - Keep tests fast (<5s each)

## Reference Architecture

```
┌─────────────┐
│   Frontend  │ (Vue.js - minimal changes)
│   (Vue 3)   │
└──────┬──────┘
       │ HTTP/REST
       ▼
┌─────────────────────────────────────┐
│        Actix-Web Server             │
│  ┌──────────────────────────────┐  │
│  │      Handlers/Routes         │  │
│  └──────────┬───────────────────┘  │
│             │                       │
│  ┌──────────▼───────────────────┐  │
│  │      Services (Business)     │  │
│  └──────────┬───────────────────┘  │
│             │                       │
│  ┌──────────▼───────────────────┐  │
│  │   Repositories (Data Access) │  │
│  └──────────┬───────────────────┘  │
└─────────────┼───────────────────────┘
              │ SQLx
              ▼
       ┌─────────────┐
       │ PostgreSQL  │
       └─────────────┘
```

## Questions or Issues?

If you encounter any issues or have questions about the specification:
1. Review the spec.md file
2. Check the test examples
3. Refer to the original Node.js implementation
4. Document decisions and update spec.md

## Next Steps

1. ✅ Review spec.md thoroughly
2. ⬜ Set up Rust project with Cargo.toml
3. ⬜ Configure database and migrations
4. ⬜ Copy test infrastructure
5. ⬜ Begin Phase 1: Foundation Setup
6. ⬜ Follow the 7-phase plan to completion

Good luck with the refactoring! 🦀
