# Integration Test Example: Cart and Order Tests
# tests/integration/cart_order_tests.rs

use sqlx::PgPool;
use testcontainers::clients::Cli;
use uuid::Uuid;

mod common;
use common::{setup_test_db, cleanup_test_db, fixtures::seed_test_data};

#[tokio::test]
async fn test_add_item_to_cart() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create cart for user
    let cart_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO carts (id, user_id)
        VALUES ($1, $2)
        "#,
        cart_id,
        test_data.user_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create cart");
    
    // Add item to cart
    let cart_item_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO cart_items (id, cart_id, product_id, quantity)
        VALUES ($1, $2, $3, $4)
        "#,
        cart_item_id,
        cart_id,
        test_data.product1_id,
        2
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add item to cart");
    
    // Verify cart item
    let cart_item = sqlx::query!(
        r#"SELECT quantity FROM cart_items WHERE id = $1"#,
        cart_item_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch cart item");
    
    assert_eq!(cart_item.quantity, 2);
    
    // TODO: Test via API
    // let response = app.add_to_cart(product_id, quantity, user_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_update_cart_item_quantity() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create cart and add item
    let cart_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO carts (id, user_id) VALUES ($1, $2)"#,
        cart_id,
        test_data.user_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create cart");
    
    let cart_item_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO cart_items (id, cart_id, product_id, quantity)
        VALUES ($1, $2, $3, $4)
        "#,
        cart_item_id,
        cart_id,
        test_data.product1_id,
        1
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add item");
    
    // Update quantity
    sqlx::query!(
        r#"UPDATE cart_items SET quantity = $1 WHERE id = $2"#,
        5,
        cart_item_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to update quantity");
    
    // Verify update
    let cart_item = sqlx::query!(
        r#"SELECT quantity FROM cart_items WHERE id = $1"#,
        cart_item_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch cart item");
    
    assert_eq!(cart_item.quantity, 5);
    
    // TODO: Test via API
    // let response = app.update_cart_item(cart_item_id, 5, user_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_remove_item_from_cart() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create cart and add item
    let cart_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO carts (id, user_id) VALUES ($1, $2)"#,
        cart_id,
        test_data.user_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create cart");
    
    let cart_item_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO cart_items (id, cart_id, product_id, quantity)
        VALUES ($1, $2, $3, $4)
        "#,
        cart_item_id,
        cart_id,
        test_data.product1_id,
        1
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add item");
    
    // Remove item
    sqlx::query!(
        r#"DELETE FROM cart_items WHERE id = $1"#,
        cart_item_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to remove item");
    
    // Verify removal
    let cart_item = sqlx::query!(
        r#"SELECT id FROM cart_items WHERE id = $1"#,
        cart_item_id
    )
    .fetch_optional(ctx.pool())
    .await
    .expect("Failed to check cart item");
    
    assert!(cart_item.is_none());
    
    // TODO: Test via API
    // let response = app.remove_from_cart(cart_item_id, user_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_calculate_cart_total() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create cart with multiple items
    let cart_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO carts (id, user_id) VALUES ($1, $2)"#,
        cart_id,
        test_data.user_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create cart");
    
    // Add product 1 (quantity: 2, price: 1299.99)
    sqlx::query!(
        r#"
        INSERT INTO cart_items (id, cart_id, product_id, quantity)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        cart_id,
        test_data.product1_id,
        2
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add item 1");
    
    // Add product 2 (quantity: 1, price: 899.99)
    sqlx::query!(
        r#"
        INSERT INTO cart_items (id, cart_id, product_id, quantity)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        cart_id,
        test_data.product2_id,
        1
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add item 2");
    
    // Calculate total
    let total = sqlx::query!(
        r#"
        SELECT SUM(ci.quantity * p.price) as total
        FROM cart_items ci
        JOIN products p ON ci.product_id = p.id
        WHERE ci.cart_id = $1
        "#,
        cart_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to calculate total");
    
    // Expected: (2 * 1299.99) + (1 * 899.99) = 3499.97
    assert!(total.total.is_some());
    let total_value = total.total.unwrap();
    assert_eq!(total_value, rust_decimal::Decimal::from_str("3499.97").unwrap());
    
    // TODO: Test via API
    // let response = app.get_cart_total(user_token).await;
    // assert_eq!(response.total, 3499.97);
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_create_order_from_cart() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create cart with items
    let cart_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO carts (id, user_id) VALUES ($1, $2)"#,
        cart_id,
        test_data.user_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create cart");
    
    sqlx::query!(
        r#"
        INSERT INTO cart_items (id, cart_id, product_id, quantity)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        cart_id,
        test_data.product1_id,
        1
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add item");
    
    // Get product price
    let product = sqlx::query!(
        r#"SELECT price FROM products WHERE id = $1"#,
        test_data.product1_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch product");
    
    // Create order
    let order_id = Uuid::new_v4();
    let shipping_price = rust_decimal::Decimal::from_str("10.00").unwrap();
    let total_price = product.price + shipping_price;
    
    sqlx::query!(
        r#"
        INSERT INTO orders (id, user_id, status, payment_method, shipping_price, total_price, shipping_address_street, shipping_address_city)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        order_id,
        test_data.user_id,
        "Pending",
        "Credit Card",
        shipping_price,
        total_price,
        "123 Main St",
        "New York"
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create order");
    
    // Add order item
    sqlx::query!(
        r#"
        INSERT INTO order_items (id, order_id, product_id, quantity, price)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        order_id,
        test_data.product1_id,
        1,
        product.price
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to add order item");
    
    // Verify order
    let order = sqlx::query!(
        r#"SELECT status, total_price FROM orders WHERE id = $1"#,
        order_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch order");
    
    assert_eq!(order.status, "Pending");
    assert_eq!(order.total_price, total_price);
    
    // TODO: Test via API
    // let response = app.create_order(order_data, user_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_update_order_status() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create order
    let order_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO orders (id, user_id, status, payment_method, shipping_price, total_price, shipping_address_street, shipping_address_city)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        order_id,
        test_data.user_id,
        "Pending",
        "Credit Card",
        10.00,
        1309.99,
        "123 Main St",
        "New York"
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create order");
    
    // Update status to Processing
    sqlx::query!(
        r#"UPDATE orders SET status = $1 WHERE id = $2"#,
        "Processing",
        order_id
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to update order status");
    
    // Verify update
    let order = sqlx::query!(
        r#"SELECT status FROM orders WHERE id = $1"#,
        order_id
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to fetch order");
    
    assert_eq!(order.status, "Processing");
    
    // TODO: Test via API (admin only)
    // let response = app.update_order_status(order_id, "Shipped", admin_token).await;
    // assert!(response.is_ok());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_get_user_order_history() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let test_data = seed_test_data(ctx.pool()).await;
    
    // Create multiple orders
    for i in 0..3 {
        let order_id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO orders (id, user_id, status, payment_method, shipping_price, total_price, shipping_address_street, shipping_address_city)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            order_id,
            test_data.user_id,
            "Delivered",
            "Credit Card",
            10.00,
            100.00 + i as f64,
            "123 Main St",
            "New York"
        )
        .execute(ctx.pool())
        .await
        .expect("Failed to create order");
    }
    
    // Get order history
    let orders = sqlx::query!(
        r#"
        SELECT id, status, total_price 
        FROM orders 
        WHERE user_id = $1 
        ORDER BY created_at DESC
        "#,
        test_data.user_id
    )
    .fetch_all(ctx.pool())
    .await
    .expect("Failed to fetch orders");
    
    assert_eq!(orders.len(), 3);
    
    // TODO: Test via API
    // let response = app.get_order_history(user_token).await;
    // assert_eq!(response.data.len(), 3);
    
    cleanup_test_db(ctx.pool()).await;
}
