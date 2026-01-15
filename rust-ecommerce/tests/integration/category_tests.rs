mod common;

use common::test_container::setup_test_env;
use serde_json::json;

#[tokio::test]
async fn test_category_crud() {
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

    assert_eq!(register_res.status(), 201);
    let register_body: serde_json::Value = register_res.json().await.unwrap();
    let admin_token = register_body["access_token"].as_str().unwrap();

    // Create category
    let category_payload = json!({
        "title": "Electronics"
    });

    let create_res = test_env
        .client
        .post(format!("{}/categories", base_url))
        .bearer_auth(admin_token)
        .json(&category_payload)
        .send()
        .await
        .expect("Failed to create category");

    assert_eq!(create_res.status(), 201);
    let create_body: serde_json::Value = create_res.json().await.unwrap();
    assert_eq!(create_body["title"], "Electronics");
    let category_id = create_body["id"].as_str().unwrap();

    // Get category
    let get_res = test_env
        .client
        .get(format!("{}/categories/{}", base_url, category_id))
        .send()
        .await
        .expect("Failed to get category");

    assert_eq!(get_res.status(), 200);
    let get_body: serde_json::Value = get_res.json().await.unwrap();
    assert_eq!(get_body["title"], "Electronics");

    // Get all categories
    let list_res = test_env
        .client
        .get(format!("{}/categories", base_url))
        .send()
        .await
        .expect("Failed to get categories");

    assert_eq!(list_res.status(), 200);
    let list_body: serde_json::Value = list_res.json().await.unwrap();
    assert!(list_body["categories"].as_array().unwrap().len() > 0);

    // Update category
    let update_payload = json!({
        "title": "Updated Electronics"
    });

    let update_res = test_env
        .client
        .put(format!("{}/categories/{}", base_url, category_id))
        .bearer_auth(admin_token)
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update category");

    assert_eq!(update_res.status(), 200);
    let update_body: serde_json::Value = update_res.json().await.unwrap();
    assert_eq!(update_body["title"], "Updated Electronics");

    // Delete category
    let delete_res = test_env
        .client
        .delete(format!("{}/categories/{}", base_url, category_id))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to delete category");

    assert_eq!(delete_res.status(), 200);

    // Verify deleted
    let verify_res = test_env
        .client
        .get(format!("{}/categories/{}", base_url, category_id))
        .send()
        .await
        .expect("Failed to verify deletion");

    assert_eq!(verify_res.status(), 404);
}

#[tokio::test]
async fn test_category_requires_admin() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register regular user
    let user_email = format!("user{}@test.com", uuid::Uuid::new_v4());
    let register_payload = json!({
        "first_name": "Regular",
        "last_name": "User",
        "email": user_email,
        "mobile": format!("+1555{:07}", rand::random::<u32>() % 10000000),
        "password": "User123!@#"
    });

    let register_res = test_env
        .client
        .post(format!("{}/auth/register", base_url))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to send register request");

    let register_body: serde_json::Value = register_res.json().await.unwrap();
    let user_token = register_body["access_token"].as_str().unwrap();

    // Try to create category as regular user
    let category_payload = json!({
        "title": "Electronics"
    });

    let create_res = test_env
        .client
        .post(format!("{}/categories", base_url))
        .bearer_auth(user_token)
        .json(&category_payload)
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_res.status(), 403);
}
