mod common;

use common::test_container::setup_test_env;
use serde_json::json;

#[tokio::test]
async fn test_product_crud() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin user
    let admin_email = format!("admin{}@test.com", uuid::Uuid::new_v4());
    let register_payload = json!({
        "first_name": "Admin",
        "last_name": "User",
        "email": admin_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "Admin123!@#"
    });

    let register_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to send register request");

    let register_body: serde_json::Value = register_res.json().await.unwrap();
    let admin_token = register_body["access_token"].as_str().unwrap();

    // Create category first
    let category_payload = json!({
        "title": "Electronics"
    });

    let category_res = test_env
        .client
        .post(format!("{}/categories", base_url))
        .bearer_auth(admin_token)
        .json(&category_payload)
        .send()
        .await
        .expect("Failed to create category");

    let category_body: serde_json::Value = category_res.json().await.unwrap();
    let category_id = category_body["id"].as_str().unwrap();

    // Create product
    let product_payload = json!({
        "title": "iPhone 15 Pro",
        "description": "Latest Apple smartphone",
        "price": "999.99",
        "quantity": 50,
        "brand": "Apple",
        "category_id": category_id,
        "images": ["https://example.com/image1.jpg"]
    });

    let create_res = test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&product_payload)
        .send()
        .await
        .expect("Failed to create product");

    assert_eq!(create_res.status(), 201);
    let create_body: serde_json::Value = create_res.json().await.unwrap();
    assert_eq!(create_body["title"], "iPhone 15 Pro");
    assert!(create_body["slug"].as_str().unwrap().starts_with("iphone-15-pro"));
    let product_id = create_body["id"].as_str().unwrap();

    // Get product
    let get_res = test_env
        .client
        .get(format!("{}/products/{}", base_url, product_id))
        .send()
        .await
        .expect("Failed to get product");

    assert_eq!(get_res.status(), 200);
    let get_body: serde_json::Value = get_res.json().await.unwrap();
    assert_eq!(get_body["title"], "iPhone 15 Pro");

    // Get all products
    let list_res = test_env
        .client
        .get(format!("{}/products", base_url))
        .send()
        .await
        .expect("Failed to get products");

    assert_eq!(list_res.status(), 200);
    let list_body: serde_json::Value = list_res.json().await.unwrap();
    assert!(list_body["products"].as_array().unwrap().len() > 0);

    // Update product
    let update_payload = json!({
        "title": "iPhone 15 Pro Max",
        "price": "1199.99",
        "quantity": 30
    });

    let update_res = test_env
        .client
        .put(format!("{}/products/{}", base_url, product_id))
        .bearer_auth(admin_token)
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update product");

    assert_eq!(update_res.status(), 200);
    let update_body: serde_json::Value = update_res.json().await.unwrap();
    assert_eq!(update_body["title"], "iPhone 15 Pro Max");

    // Delete product
    let delete_res = test_env
        .client
        .delete(format!("{}/products/{}", base_url, product_id))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to delete product");

    assert_eq!(delete_res.status(), 200);
}

#[tokio::test]
async fn test_product_search() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin user
    let admin_email = format!("admin{}@test.com", uuid::Uuid::new_v4());
    let register_payload = json!({
        "first_name": "Admin",
        "last_name": "User",
        "email": admin_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "Admin123!@#"
    });

    let register_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to send register request");

    let register_body: serde_json::Value = register_res.json().await.unwrap();
    let admin_token = register_body["access_token"].as_str().unwrap();

    // Create products
    let product1_payload = json!({
        "title": "Laptop Computer",
        "description": "High-performance laptop",
        "price": "1299.99",
        "quantity": 20
    });

    let product2_payload = json!({
        "title": "Desktop Computer",
        "description": "Powerful desktop PC",
        "price": "1599.99",
        "quantity": 15
    });

    test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&product1_payload)
        .send()
        .await
        .expect("Failed to create product1");

    test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&product2_payload)
        .send()
        .await
        .expect("Failed to create product2");

    // Search products
    let search_res = test_env
        .client
        .get(format!("{}/products?search=computer", base_url))
        .send()
        .await
        .expect("Failed to search products");

    assert_eq!(search_res.status(), 200);
    let search_body: serde_json::Value = search_res.json().await.unwrap();
    let products = search_body["products"].as_array().unwrap();
    assert!(products.len() >= 2);
}

#[tokio::test]
async fn test_product_rating() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin user
    let admin_email = format!("admin{}@test.com", uuid::Uuid::new_v4());
    let register_payload = json!({
        "first_name": "Admin",
        "last_name": "User",
        "email": admin_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "Admin123!@#"
    });

    let register_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to send register request");

    let register_body: serde_json::Value = register_res.json().await.unwrap();
    let admin_token = register_body["access_token"].as_str().unwrap();

    // Create product
    let product_payload = json!({
        "title": "Test Product",
        "description": "Product for rating",
        "price": "99.99",
        "quantity": 10
    });

    let create_res = test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&product_payload)
        .send()
        .await
        .expect("Failed to create product");

    let create_body: serde_json::Value = create_res.json().await.unwrap();
    let product_id = create_body["id"].as_str().unwrap();

    // Add rating
    let rating_payload = json!({
        "rating": 5,
        "comment": "Excellent product!"
    });

    let rating_res = test_env
        .client
        .post(format!("{}/products/{}/ratings", base_url, product_id))
        .bearer_auth(admin_token)
        .json(&rating_payload)
        .send()
        .await
        .expect("Failed to add rating");

    assert_eq!(rating_res.status(), 201);
    let rating_body: serde_json::Value = rating_res.json().await.unwrap();
    assert_eq!(rating_body["rating"], 5);
    assert_eq!(rating_body["comment"], "Excellent product!");

    // Get ratings
    let get_ratings_res = test_env
        .client
        .get(format!("{}/products/{}/ratings", base_url, product_id))
        .send()
        .await
        .expect("Failed to get ratings");

    assert_eq!(get_ratings_res.status(), 200);
    let ratings_body: serde_json::Value = get_ratings_res.json().await.unwrap();
    assert_eq!(ratings_body["ratings"].as_array().unwrap().len(), 1);
    assert_eq!(ratings_body["average"].as_str().unwrap(), "5.00");
}

#[tokio::test]
async fn test_wishlist() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register users
    let admin_email = format!("admin{}@test.com", uuid::Uuid::new_v4());
    let user_email = format!("user{}@test.com", uuid::Uuid::new_v4());

    let admin_register = json!({
        "first_name": "Admin",
        "last_name": "User",
        "email": admin_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "Admin123!@#"
    });

    let user_register = json!({
        "first_name": "Regular",
        "last_name": "User",
        "email": user_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "User123!@#"
    });

    let admin_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&admin_register)
        .send()
        .await
        .unwrap();
    let admin_body: serde_json::Value = admin_res.json().await.unwrap();
    let admin_token = admin_body["access_token"].as_str().unwrap();

    let user_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&user_register)
        .send()
        .await
        .unwrap();
    let user_body: serde_json::Value = user_res.json().await.unwrap();
    let user_token = user_body["access_token"].as_str().unwrap();

    // Create product
    let product_payload = json!({
        "title": "Wishlist Product",
        "description": "Product for wishlist test",
        "price": "49.99",
        "quantity": 100
    });

    let product_res = test_env
        .client
        .post(format!("{}/products", base_url))
        .bearer_auth(admin_token)
        .json(&product_payload)
        .send()
        .await
        .unwrap();

    let product_body: serde_json::Value = product_res.json().await.unwrap();
    let product_id = product_body["id"].as_str().unwrap();

    // Add to wishlist
    let add_res = test_env
        .client
        .post(format!("{}/users/wishlist/{}", base_url, product_id))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to add to wishlist");

    assert_eq!(add_res.status(), 200);

    // Get wishlist
    let get_wishlist_res = test_env
        .client
        .get(format!("{}/users/wishlist", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to get wishlist");

    assert_eq!(get_wishlist_res.status(), 200);
    let wishlist_body: serde_json::Value = get_wishlist_res.json().await.unwrap();
    assert_eq!(wishlist_body["products"].as_array().unwrap().len(), 1);

    // Remove from wishlist
    let remove_res = test_env
        .client
        .delete(format!("{}/users/wishlist/{}", base_url, product_id))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to remove from wishlist");

    assert_eq!(remove_res.status(), 200);

    // Verify empty wishlist
    let verify_res = test_env
        .client
        .get(format!("{}/users/wishlist", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to get wishlist");

    let verify_body: serde_json::Value = verify_res.json().await.unwrap();
    assert_eq!(verify_body["products"].as_array().unwrap().len(), 0);
}
