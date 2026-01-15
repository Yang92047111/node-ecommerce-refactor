mod common;

use common::test_container::setup_test_env;
use serde_json::json;

#[tokio::test]
async fn test_blog_crud() {
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

    // Create blog
    let blog_payload = json!({
        "title": "My First Blog Post",
        "description": "This is a test blog post",
        "content": "This is the content of my first blog post. It contains important information.",
        "images": ["https://example.com/image1.jpg"]
    });

    let create_res = test_env
        .client
        .post(format!("{}/blogs", base_url))
        .bearer_auth(admin_token)
        .json(&blog_payload)
        .send()
        .await
        .expect("Failed to create blog");

    assert_eq!(create_res.status(), 201);
    let create_body: serde_json::Value = create_res.json().await.unwrap();
    assert_eq!(create_body["title"], "My First Blog Post");
    assert_eq!(create_body["views_count"], 0);
    let blog_id = create_body["id"].as_str().unwrap();

    // Get blog (this should increment views)
    let get_res = test_env
        .client
        .get(format!("{}/blogs/{}", base_url, blog_id))
        .send()
        .await
        .expect("Failed to get blog");

    assert_eq!(get_res.status(), 200);
    let get_body: serde_json::Value = get_res.json().await.unwrap();
    assert_eq!(get_body["title"], "My First Blog Post");

    // Get blog again to verify view count incremented
    let get_res2 = test_env
        .client
        .get(format!("{}/blogs/{}", base_url, blog_id))
        .send()
        .await
        .expect("Failed to get blog");

    assert_eq!(get_res2.status(), 200);

    // Get all blogs
    let list_res = test_env
        .client
        .get(format!("{}/blogs", base_url))
        .send()
        .await
        .expect("Failed to get blogs");

    assert_eq!(list_res.status(), 200);
    let list_body: serde_json::Value = list_res.json().await.unwrap();
    assert!(list_body["blogs"].as_array().unwrap().len() > 0);
    assert!(list_body["total"].as_i64().unwrap() > 0);

    // Update blog
    let update_payload = json!({
        "title": "Updated Blog Post",
        "content": "This is the updated content."
    });

    let update_res = test_env
        .client
        .put(format!("{}/blogs/{}", base_url, blog_id))
        .bearer_auth(admin_token)
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update blog");

    assert_eq!(update_res.status(), 200);
    let update_body: serde_json::Value = update_res.json().await.unwrap();
    assert_eq!(update_body["title"], "Updated Blog Post");

    // Delete blog
    let delete_res = test_env
        .client
        .delete(format!("{}/blogs/{}", base_url, blog_id))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to delete blog");

    assert_eq!(delete_res.status(), 200);

    // Verify deleted
    let verify_res = test_env
        .client
        .get(format!("{}/blogs/{}", base_url, blog_id))
        .send()
        .await
        .expect("Failed to verify deletion");

    assert_eq!(verify_res.status(), 404);
}

#[tokio::test]
async fn test_blog_pagination() {
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

    // Create multiple blogs
    for i in 1..=5 {
        let blog_payload = json!({
            "title": format!("Blog Post {}", i),
            "content": format!("Content for blog post {}", i)
        });

        test_env
            .client
            .post(format!("{}/blogs", base_url))
            .bearer_auth(admin_token)
            .json(&blog_payload)
            .send()
            .await
            .expect("Failed to create blog");
    }

    // Get first page with page size 2
    let list_res = test_env
        .client
        .get(format!("{}/blogs?page=1&page_size=2", base_url))
        .send()
        .await
        .expect("Failed to get blogs");

    assert_eq!(list_res.status(), 200);
    let list_body: serde_json::Value = list_res.json().await.unwrap();
    assert_eq!(list_body["blogs"].as_array().unwrap().len(), 2);
    assert_eq!(list_body["page"], 1);
    assert_eq!(list_body["page_size"], 2);
    assert!(list_body["total"].as_i64().unwrap() >= 5);
}

#[tokio::test]
async fn test_blog_requires_admin_to_create() {
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

    // Try to create blog as regular user
    let blog_payload = json!({
        "title": "Unauthorized Blog",
        "content": "This should not be allowed"
    });

    let create_res = test_env
        .client
        .post(format!("{}/blogs", base_url))
        .bearer_auth(user_token)
        .json(&blog_payload)
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_res.status(), 403);
}

#[tokio::test]
async fn test_blog_author_can_update_own_blog() {
    let test_env = setup_test_env().await;
    let base_url = format!("http://127.0.0.1:{}/api", test_env.port);

    // Register admin user (author)
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

    // Create blog
    let blog_payload = json!({
        "title": "Original Title",
        "content": "Original content"
    });

    let create_res = test_env
        .client
        .post(format!("{}/blogs", base_url))
        .bearer_auth(admin_token)
        .json(&blog_payload)
        .send()
        .await
        .expect("Failed to create blog");

    let create_body: serde_json::Value = create_res.json().await.unwrap();
    let blog_id = create_body["id"].as_str().unwrap();

    // Author updates their own blog
    let update_payload = json!({
        "title": "Updated by Author"
    });

    let update_res = test_env
        .client
        .put(format!("{}/blogs/{}", base_url, blog_id))
        .bearer_auth(admin_token)
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update blog");

    assert_eq!(update_res.status(), 200);
    let update_body: serde_json::Value = update_res.json().await.unwrap();
    assert_eq!(update_body["title"], "Updated by Author");
}

#[tokio::test]
async fn test_blog_validation() {
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

    // Try to create blog without content
    let invalid_payload = json!({
        "title": "No Content Blog",
        "content": ""
    });

    let create_res = test_env
        .client
        .post(format!("{}/blogs", base_url))
        .bearer_auth(admin_token)
        .json(&invalid_payload)
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_res.status(), 400);

    // Try to create blog without title
    let invalid_payload2 = json!({
        "content": "Some content"
    });

    let create_res2 = test_env
        .client
        .post(format!("{}/blogs", base_url))
        .bearer_auth(admin_token)
        .json(&invalid_payload2)
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_res2.status(), 400);
}

#[tokio::test]
async fn test_blog_public_access() {
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

    // Create blog
    let blog_payload = json!({
        "title": "Public Blog",
        "content": "This blog should be accessible to everyone"
    });

    let create_res = test_env
        .client
        .post(format!("{}/blogs", base_url))
        .bearer_auth(admin_token)
        .json(&blog_payload)
        .send()
        .await
        .expect("Failed to create blog");

    let create_body: serde_json::Value = create_res.json().await.unwrap();
    let blog_id = create_body["id"].as_str().unwrap();

    // Access blog without authentication
    let get_res = test_env
        .client
        .get(format!("{}/blogs/{}", base_url, blog_id))
        .send()
        .await
        .expect("Failed to get blog");

    assert_eq!(get_res.status(), 200);

    // List blogs without authentication
    let list_res = test_env
        .client
        .get(format!("{}/blogs", base_url))
        .send()
        .await
        .expect("Failed to get blogs");

    assert_eq!(list_res.status(), 200);
}
