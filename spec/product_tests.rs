# Integration Test Example: Product Tests
# tests/integration/product_tests.rs

use sqlx::PgPool;
use testcontainers::clients::Cli;
use uuid::Uuid;

mod common;
use common::{setup_test_db, cleanup_test_db, TestFixtures, fixtures::seed_test_data};

#[tokio::test]
async fn test_create_product_success() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    // Seed test data
    let test_data = seed_test_data(ctx.pool()).await;
    
    // TODO: Create product via API
    // let product_data = CreateProductDto {
    //     title: "New Laptop".to_string(),
    //     price: 999.99,
    //     description: "Test laptop".to_string(),
    //     quantity: 10,
    //     category_id: test_data.electronics_id,
    //     brand: Some("TestBrand".to_string()),
    // };
    
    // let response = app.create_product(product_data, admin_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_get_all_products() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    // Seed test data
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Query products from database
    let products = sqlx::query!(
        r#"SELECT id, title, price FROM products"#
    )
    .fetch_all(ctx.pool())
    .await
    .expect("Failed to fetch products");
    
    assert!(products.len() >= 2, "Should have at least 2 seeded products");
    
    // TODO: Test via API
    // let response = app.get_products().await;
    // assert!(response.is_ok());
    // assert_eq!(response.data.len(), 2);
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_get_product_by_id() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Fetch product by ID
    let product = sqlx::query!(
        r#"SELECT id, title, price, description FROM products WHERE id = $1"#,
        test_data.product1_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch product");
    
    assert_eq!(product.title, "Laptop Pro");
    assert_eq!(product.price, rust_decimal::Decimal::from_str("1299.99").unwrap());
    
    // TODO: Test via API
    // let response = app.get_product(test_data.product1_id).await;
    // assert!(response.is_ok());
    // assert_eq!(response.title, "Laptop Pro");
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_update_product() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Update product
    let new_price = rust_decimal::Decimal::from_str("1199.99").unwrap();
    sqlx::query!(
        r#"UPDATE products SET price = $1 WHERE id = $2"#,
        new_price,
        test_data.product1_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to update product");
    
    // Verify update
    let product = sqlx::query!(
        r#"SELECT price FROM products WHERE id = $1"#,
        test_data.product1_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch updated product");
    
    assert_eq!(product.price, new_price);
    
    // TODO: Test via API with admin authentication
    // let response = app.update_product(product_id, update_data, admin_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_delete_product() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Delete product
    sqlx::query!(
        r#"DELETE FROM products WHERE id = $1"#,
        test_data.product1_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to delete product");
    
    // Verify deletion
    let product = sqlx::query!(
        r#"SELECT id FROM products WHERE id = $1"#,
        test_data.product1_id
    )
    .fetch_optional(ctx.pool())
    .await
    .expect("Failed to check product");
    
    assert!(product.is_none(), "Product should be deleted");
    
    // TODO: Test via API
    // let response = app.delete_product(product_id, admin_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_product_search() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Search for products using PostgreSQL full-text search
    let search_query = "laptop";
    let products = sqlx::query!(
        r#"
        SELECT id, title, price 
        FROM products 
        WHERE to_tsvector('english', title || ' ' || description) @@ plainto_tsquery('english', $1)
        "#,
        search_query
    )
    .fetch_all(ctx.pool())
    .await
    .expect("Failed to search products");
    
    assert!(products.len() >= 1, "Should find at least one laptop");
    assert!(products.iter().any(|p| p.title.to_lowercase().contains("laptop")));
    
    // TODO: Test via API
    // let response = app.search_products(search_query).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_product_pagination() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create additional products for pagination test
    for i in 0..15 {
        let product_id = Uuid::new_v4();
        let title = format!("Product {}", i);
        let slug = format!("product-{}", i);
        
        sqlx::query!(
            r#"
            INSERT INTO products (id, title, slug, description, price, quantity)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            product_id,
            title,
            slug,
            "Test description",
            99.99,
            10
        )
        .execute(ctx.pool())
        .await
        .expect("Failed to create test product");
    }
    
    // Test pagination
    let limit = 10i64;
    let offset = 0i64;
    
    let products = sqlx::query!(
        r#"
        SELECT id, title, price 
        FROM products 
        ORDER BY created_at DESC 
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(ctx.pool())
    .await
    .expect("Failed to fetch paginated products");
    
    assert_eq!(products.len(), 10, "Should return 10 products per page");
    
    // TODO: Test via API
    // let response = app.get_products_paginated(1, 10).await;
    // assert_eq!(response.data.len(), 10);
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_add_product_rating() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Add rating
    let rating_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO ratings (id, product_id, user_id, rating, comment)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        rating_id,
        test_data.product1_id,
        test_data.user_id,
        5,
        "Excellent product!"
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add rating");
    
    // Verify rating
    let rating = sqlx::query!(
        r#"SELECT rating, comment FROM ratings WHERE id = $1"#,
        rating_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch rating");
    
    assert_eq!(rating.rating, 5);
    
    // Calculate average rating
    let avg_rating = sqlx::query!(
        r#"
        SELECT AVG(rating) as avg_rating 
        FROM ratings 
        WHERE product_id = $1
        "#,
        test_data.product1_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to calculate average rating");
    
    assert!(avg_rating.avg_rating.is_some());
    
    // TODO: Test via API
    // let response = app.add_rating(product_id, rating_data, user_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}
