# Test Context and Container Management
# tests/common/test_container.rs

use sqlx::PgPool;
use std::sync::Arc;

/// Test context holds shared test resources
#[derive(Clone)]
pub struct TestContext {
    pub pool: PgPool,
    pub database_url: String,
}

impl TestContext {
    pub fn new(pool: PgPool, database_url: String) -> Self {
        Self {
            pool,
            database_url,
        }
    }

    /// Get a reference to the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the database URL
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}

/// Helper to create test users
pub async fn create_test_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let user_id = uuid::Uuid::new_v4();
    
    sqlx::query!(
        r#"
        INSERT INTO users (id, first_name, last_name, email, mobile, password_hash, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        user_id,
        "Test",
        "User",
        email,
        "+1234567890",
        password_hash,
        "user"
    )
    .execute(pool)
    .await?;

    Ok(user_id)
}

/// Helper to create test admin
pub async fn create_test_admin(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let admin_id = uuid::Uuid::new_v4();
    
    sqlx::query!(
        r#"
        INSERT INTO users (id, first_name, last_name, email, mobile, password_hash, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        admin_id,
        "Admin",
        "User",
        email,
        "+9876543210",
        password_hash,
        "admin"
    )
    .execute(pool)
    .await?;

    Ok(admin_id)
}

/// Helper to create test category
pub async fn create_test_category(
    pool: &PgPool,
    title: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let category_id = uuid::Uuid::new_v4();
    
    sqlx::query!(
        r#"
        INSERT INTO categories (id, title)
        VALUES ($1, $2)
        "#,
        category_id,
        title
    )
    .execute(pool)
    .await?;

    Ok(category_id)
}

/// Helper to create test product
pub async fn create_test_product(
    pool: &PgPool,
    title: &str,
    price: f64,
    category_id: Option<uuid::Uuid>,
) -> Result<uuid::Uuid, sqlx::Error> {
    let product_id = uuid::Uuid::new_v4();
    let slug = title.to_lowercase().replace(" ", "-");
    
    sqlx::query!(
        r#"
        INSERT INTO products (id, title, slug, description, price, quantity, category_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        product_id,
        title,
        slug,
        "Test product description",
        price,
        100,
        category_id
    )
    .execute(pool)
    .await?;

    Ok(product_id)
}
