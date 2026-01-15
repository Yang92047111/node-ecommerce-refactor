mod common;

use common::test_container::setup_test_env;
use serde_json::json;

#[tokio::test]
async fn test_order_workflow() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin user
    let admin_email = format!("admin{}@test.com", uuid::Uuid::new_v4());
    let admin_register = json!({
        "first_name": "Admin",
        "last_name": "User",
        "email": admin_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "Admin123!@#"
    });

    let admin_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&admin_register)
        .send()
        .await
        .expect("Failed to register admin");

    let admin_body: serde_json::Value = admin_res.json().await.unwrap();
    let admin_token = admin_body["access_token"].as_str().unwrap();

    // Create category and product
    let category_res = test_env
        .client
        .post(format!("{}/categories", base_url))
        .bearer_auth(admin_token)
        .json(&json!({"title": "Electronics"}))
        .send()
        .await
        .expect("Failed to create category");

    let category_body: serde_json::Value = category_res.json().await.unwrap();
    let category_id = category_body["id"].as_str().unwrap();

    let product_res = test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&json!({
            "title": "iPhone 15 Pro",
            "description": "Latest Apple smartphone",
            "price": "999.99",
            "quantity": 50,
            "brand": "Apple",
            "category_id": category_id,
            "images": ["https://example.com/iphone.jpg"]
        }))
        .send()
        .await
        .expect("Failed to create product");

    let product_body: serde_json::Value = product_res.json().await.unwrap();
    let product_id = product_body["id"].as_str().unwrap();

    // Register regular user
    let user_email = format!("user{}@test.com", uuid::Uuid::new_v4());
    let user_register = json!({
        "first_name": "John",
        "last_name": "Doe",
        "email": user_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "User123!@#"
    });

    let user_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&user_register)
        .send()
        .await
        .expect("Failed to register user");

    let user_body: serde_json::Value = user_res.json().await.unwrap();
    let user_token = user_body["access_token"].as_str().unwrap();

    // Add items to cart
    test_env
        .client
        .post(format!("{}/cart/items", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "product_id": product_id,
            "quantity": 2
        }))
        .send()
        .await
        .expect("Failed to add item to cart");

    // Create order from cart
    let create_order_res = test_env
        .client
        .post(format!("{}/orders", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "payment_method": "Credit Card",
            "shipping_price": "10.00",
            "shipping_address_street": "123 Main St",
            "shipping_address_city": "New York"
        }))
        .send()
        .await
        .expect("Failed to create order");

    assert_eq!(create_order_res.status(), 201);
    let order_body: serde_json::Value = create_order_res.json().await.unwrap();
    assert_eq!(order_body["status"], "Pending");
    assert_eq!(order_body["items"].as_array().unwrap().len(), 1);
    assert_eq!(order_body["items"][0]["quantity"], 2);
    let order_id = order_body["id"].as_str().unwrap();

    // Verify cart is cleared
    let cart_res = test_env
        .client
        .get(format!("{}/cart", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to get cart");

    let cart_body: serde_json::Value = cart_res.json().await.unwrap();
    assert_eq!(cart_body["items"].as_array().unwrap().len(), 0);

    // Get order by ID
    let get_order_res = test_env
        .client
        .get(format!("{}/orders/{}", base_url, order_id))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to get order");

    assert_eq!(get_order_res.status(), 200);
    let get_order_body: serde_json::Value = get_order_res.json().await.unwrap();
    assert_eq!(get_order_body["id"], order_id);
    assert_eq!(get_order_body["status"], "Pending");

    // Get user orders
    let list_orders_res = test_env
        .client
        .get(format!("{}/orders", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to list orders");

    assert_eq!(list_orders_res.status(), 200);
    let list_body: serde_json::Value = list_orders_res.json().await.unwrap();
    assert!(list_body["orders"].as_array().unwrap().len() > 0);
    assert_eq!(list_body["total"], 1);

    // Update order status (admin only)
    let update_status_res = test_env
        .client
        .put(format!("{}/orders/{}/status", base_url, order_id))
        .bearer_auth(admin_token)
        .json(&json!({
            "status": "Processing"
        }))
        .send()
        .await
        .expect("Failed to update order status");

    assert_eq!(update_status_res.status(), 200);
    let update_body: serde_json::Value = update_status_res.json().await.unwrap();
    assert_eq!(update_body["status"], "Processing");

    // Verify updated status
    let verify_res = test_env
        .client
        .get(format!("{}/orders/{}", base_url, order_id))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to verify order");

    let verify_body: serde_json::Value = verify_res.json().await.unwrap();
    assert_eq!(verify_body["status"], "Processing");
}

#[tokio::test]
async fn test_order_validation() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register user
    let user_email = format!("user{}@test.com", uuid::Uuid::new_v4());
    let user_register = json!({
        "first_name": "John",
        "last_name": "Doe",
        "email": user_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "User123!@#"
    });

    let user_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&user_register)
        .send()
        .await
        .expect("Failed to register user");

    let user_body: serde_json::Value = user_res.json().await.unwrap();
    let user_token = user_body["access_token"].as_str().unwrap();

    // Try to create order with empty cart
    let empty_cart_res = test_env
        .client
        .post(format!("{}/orders", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "payment_method": "Credit Card",
            "shipping_price": "10.00",
            "shipping_address_street": "123 Main St",
            "shipping_address_city": "New York"
        }))
        .send()
        .await
        .expect("Failed to send order request");

    assert_eq!(empty_cart_res.status(), 400);
}

#[tokio::test]
async fn test_admin_order_management() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin user
    let admin_email = format!("admin{}@test.com", uuid::Uuid::new_v4());
    let admin_register = json!({
        "first_name": "Admin",
        "last_name": "User",
        "email": admin_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "Admin123!@#"
    });

    let admin_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&admin_register)
        .send()
        .await
        .expect("Failed to register admin");

    let admin_body: serde_json::Value = admin_res.json().await.unwrap();
    let admin_token = admin_body["access_token"].as_str().unwrap();

    // Get all orders (admin)
    let all_orders_res = test_env
        .client
        .get(format!("{}/admin/orders", base_url))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to get all orders");

    assert_eq!(all_orders_res.status(), 200);
    let all_orders_body: serde_json::Value = all_orders_res.json().await.unwrap();
    assert!(all_orders_body["orders"].is_array());
}

#[tokio::test]
async fn test_order_status_transitions() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin and create product
    let admin_email = format!("admin{}@test.com", uuid::Uuid::new_v4());
    let admin_register = json!({
        "first_name": "Admin",
        "last_name": "User",
        "email": admin_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "Admin123!@#"
    });

    let admin_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&admin_register)
        .send()
        .await
        .expect("Failed to register admin");

    let admin_body: serde_json::Value = admin_res.json().await.unwrap();
    let admin_token = admin_body["access_token"].as_str().unwrap();

    let category_res = test_env
        .client
        .post(format!("{}/categories", base_url))
        .bearer_auth(admin_token)
        .json(&json!({"title": "Test Category"}))
        .send()
        .await
        .expect("Failed to create category");

    let category_body: serde_json::Value = category_res.json().await.unwrap();
    let category_id = category_body["id"].as_str().unwrap();

    let product_res = test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&json!({
            "title": "Test Product",
            "description": "Test description",
            "price": "100.00",
            "quantity": 10,
            "category_id": category_id
        }))
        .send()
        .await
        .expect("Failed to create product");

    let product_body: serde_json::Value = product_res.json().await.unwrap();
    let product_id = product_body["id"].as_str().unwrap();

    // Register user and create order
    let user_email = format!("user{}@test.com", uuid::Uuid::new_v4());
    let user_register = json!({
        "first_name": "John",
        "last_name": "Doe",
        "email": user_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "User123!@#"
    });

    let user_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&user_register)
        .send()
        .await
        .expect("Failed to register user");

    let user_body: serde_json::Value = user_res.json().await.unwrap();
    let user_token = user_body["access_token"].as_str().unwrap();

    test_env
        .client
        .post(format!("{}/cart/items", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "product_id": product_id,
            "quantity": 1
        }))
        .send()
        .await
        .expect("Failed to add to cart");

    let order_res = test_env
        .client
        .post(format!("{}/orders", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "payment_method": "Credit Card",
            "shipping_price": "5.00",
            "shipping_address_street": "123 Test St",
            "shipping_address_city": "Test City"
        }))
        .send()
        .await
        .expect("Failed to create order");

    let order_body: serde_json::Value = order_res.json().await.unwrap();
    let order_id = order_body["id"].as_str().unwrap();

    // Test status transitions
    let statuses = vec!["Processing", "Shipped", "Delivered"];
    for status in statuses {
        let update_res = test_env
            .client
            .put(format!("{}/orders/{}/status", base_url, order_id))
            .bearer_auth(admin_token)
            .json(&json!({"status": status}))
            .send()
            .await
            .expect("Failed to update status");

        assert_eq!(update_res.status(), 200);
        let update_body: serde_json::Value = update_res.json().await.unwrap();
        assert_eq!(update_body["status"], status);
    }
}
