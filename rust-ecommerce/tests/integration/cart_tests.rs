mod common;

use common::test_container::setup_test_env;
use serde_json::json;

#[tokio::test]
async fn test_cart_workflow() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin user for creating products
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

    // Create category
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

    // Create products
    let product1_res = test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&json!({
            "title": "iPhone 15",
            "description": "Latest Apple smartphone",
            "price": "999.99",
            "quantity": 50,
            "brand": "Apple",
            "category_id": category_id,
            "images": ["https://example.com/iphone.jpg"]
        }))
        .send()
        .await
        .expect("Failed to create product 1");

    let product1_body: serde_json::Value = product1_res.json().await.unwrap();
    let product1_id = product1_body["id"].as_str().unwrap();

    let product2_res = test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&json!({
            "title": "MacBook Pro",
            "description": "Powerful laptop",
            "price": "2499.99",
            "quantity": 30,
            "brand": "Apple",
            "category_id": category_id,
            "images": ["https://example.com/macbook.jpg"]
        }))
        .send()
        .await
        .expect("Failed to create product 2");

    let product2_body: serde_json::Value = product2_res.json().await.unwrap();
    let product2_id = product2_body["id"].as_str().unwrap();

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

    // Get empty cart
    let get_cart_res = test_env
        .client
        .get(format!("{}/cart", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to get cart");

    assert_eq!(get_cart_res.status(), 200);
    let cart_body: serde_json::Value = get_cart_res.json().await.unwrap();
    assert_eq!(cart_body["items"].as_array().unwrap().len(), 0);
    assert_eq!(cart_body["total_items"], 0);

    // Add first item to cart
    let add_item1_res = test_env
        .client
        .post(format!("{}/cart/items", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "product_id": product1_id,
            "quantity": 2
        }))
        .send()
        .await
        .expect("Failed to add item to cart");

    assert_eq!(add_item1_res.status(), 201);
    let add_item1_body: serde_json::Value = add_item1_res.json().await.unwrap();
    assert_eq!(add_item1_body["items"].as_array().unwrap().len(), 1);
    assert_eq!(add_item1_body["total_items"], 2);
    let item1_id = add_item1_body["items"][0]["id"].as_str().unwrap();

    // Add second item to cart
    let add_item2_res = test_env
        .client
        .post(format!("{}/cart/items", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "product_id": product2_id,
            "quantity": 1
        }))
        .send()
        .await
        .expect("Failed to add second item to cart");

    assert_eq!(add_item2_res.status(), 201);
    let add_item2_body: serde_json::Value = add_item2_res.json().await.unwrap();
    assert_eq!(add_item2_body["items"].as_array().unwrap().len(), 2);
    assert_eq!(add_item2_body["total_items"], 3);

    // Update item quantity
    let update_res = test_env
        .client
        .put(format!("{}/cart/items/{}", base_url, item1_id))
        .bearer_auth(user_token)
        .json(&json!({
            "quantity": 3
        }))
        .send()
        .await
        .expect("Failed to update cart item");

    assert_eq!(update_res.status(), 200);
    let update_body: serde_json::Value = update_res.json().await.unwrap();
    assert_eq!(update_body["total_items"], 4);

    // Remove item from cart
    let remove_res = test_env
        .client
        .delete(format!("{}/cart/items/{}", base_url, item1_id))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to remove cart item");

    assert_eq!(remove_res.status(), 200);
    let remove_body: serde_json::Value = remove_res.json().await.unwrap();
    assert_eq!(remove_body["items"].as_array().unwrap().len(), 1);
    assert_eq!(remove_body["total_items"], 1);

    // Clear cart
    let clear_res = test_env
        .client
        .delete(format!("{}/cart", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to clear cart");

    assert_eq!(clear_res.status(), 200);

    // Verify cart is empty
    let final_cart_res = test_env
        .client
        .get(format!("{}/cart", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to get final cart");

    let final_cart_body: serde_json::Value = final_cart_res.json().await.unwrap();
    assert_eq!(final_cart_body["items"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_cart_validation() {
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

    // Try to add item with invalid quantity (0)
    let invalid_res = test_env
        .client
        .post(format!("{}/cart/items", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "product_id": uuid::Uuid::new_v4().to_string(),
            "quantity": 0
        }))
        .send()
        .await
        .expect("Failed to send invalid request");

    assert_eq!(invalid_res.status(), 400);

    // Try to add non-existent product
    let nonexistent_res = test_env
        .client
        .post(format!("{}/cart/items", base_url))
        .bearer_auth(user_token)
        .json(&json!({
            "product_id": uuid::Uuid::new_v4().to_string(),
            "quantity": 1
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(nonexistent_res.status(), 404);
}
