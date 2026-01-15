# Test Fixtures for Integration Tests
# tests/common/fixtures.rs

use fake::{faker::internet::en::*, faker::name::en::*, Fake};
use sqlx::PgPool;
use uuid::Uuid;

pub struct TestFixtures;

impl TestFixtures {
    /// Generate a random email
    pub fn random_email() -> String {
        SafeEmail().fake()
    }

    /// Generate a random name
    pub fn random_name() -> String {
        Name().fake()
    }

    /// Generate a random mobile number
    pub fn random_mobile() -> String {
        format!("+1{}", (1000000000..9999999999_u64).fake::<u64>())
    }

    /// Generate a random product title
    pub fn random_product_title() -> String {
        let products = vec![
            "Laptop", "Phone", "Tablet", "Monitor", "Keyboard",
            "Mouse", "Headphones", "Speaker", "Camera", "Printer",
        ];
        let brands = vec![
            "Apple", "Samsung", "Dell", "HP", "Lenovo",
            "Sony", "LG", "Asus", "Microsoft", "Google",
        ];
        
        format!("{} {}", 
            brands[(0..brands.len()).fake::<usize>()],
            products[(0..products.len()).fake::<usize>()])
    }

    /// Generate random price
    pub fn random_price() -> f64 {
        (10.0..2000.0).fake()
    }
}

/// Seed database with test data
pub async fn seed_test_data(pool: &PgPool) -> TestData {
    // Create admin user
    let admin_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO users (id, first_name, last_name, email, mobile, password_hash, role)
        VALUES ($1, 'Admin', 'User', 'admin@test.com', '+11111111111', 'hashed_password', 'admin')
        "#,
        admin_id
    )
    .execute(pool)
    .await
    .expect("Failed to create admin user");

    // Create regular user
    let user_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO users (id, first_name, last_name, email, mobile, password_hash, role)
        VALUES ($1, 'Test', 'User', 'user@test.com', '+12222222222', 'hashed_password', 'user')
        "#,
        user_id
    )
    .execute(pool)
    .await
    .expect("Failed to create regular user");

    // Create categories
    let electronics_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO categories (id, title)
        VALUES ($1, 'Electronics')
        "#,
        electronics_id
    )
    .execute(pool)
    .await
    .expect("Failed to create Electronics category");

    let clothing_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO categories (id, title)
        VALUES ($1, 'Clothing')
        "#,
        clothing_id
    )
    .execute(pool)
    .await
    .expect("Failed to create Clothing category");

    // Create products
    let product1_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO products (id, title, slug, description, price, quantity, category_id)
        VALUES ($1, 'Laptop Pro', 'laptop-pro', 'High-end laptop', 1299.99, 50, $2)
        "#,
        product1_id,
        electronics_id
    )
    .execute(pool)
    .await
    .expect("Failed to create product 1");

    let product2_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO products (id, title, slug, description, price, quantity, category_id)
        VALUES ($1, 'Smart Phone', 'smart-phone', 'Latest smartphone', 899.99, 100, $2)
        "#,
        product2_id,
        electronics_id
    )
    .execute(pool)
    .await
    .expect("Failed to create product 2");

    // Create coupon
    let coupon_id = Uuid::new_v4();
    let expiry_date = chrono::Utc::now() + chrono::Duration::days(30);
    sqlx::query!(
        r#"
        INSERT INTO coupons (id, code, discount_percentage, expiry_date, is_active)
        VALUES ($1, 'SAVE10', 10.0, $2, true)
        "#,
        coupon_id,
        expiry_date
    )
    .execute(pool)
    .await
    .expect("Failed to create coupon");

    TestData {
        admin_id,
        user_id,
        electronics_id,
        clothing_id,
        product1_id,
        product2_id,
        coupon_id,
    }
}

/// Container for test data IDs
pub struct TestData {
    pub admin_id: Uuid,
    pub user_id: Uuid,
    pub electronics_id: Uuid,
    pub clothing_id: Uuid,
    pub product1_id: Uuid,
    pub product2_id: Uuid,
    pub coupon_id: Uuid,
}
