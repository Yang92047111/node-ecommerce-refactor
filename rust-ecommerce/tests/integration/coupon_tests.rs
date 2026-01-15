mod common;

use chrono::{Duration, Utc};
use common::test_container::setup_test_env;
use serde_json::json;

#[tokio::test]
async fn test_coupon_crud() {
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

    // Create coupon with future expiry date
    let expiry_date = Utc::now() + Duration::days(30);
    let coupon_payload = json!({
        "code": "SAVE20",
        "discount_percentage": 20.0,
        "expiry_date": expiry_date.to_rfc3339()
    });

    let create_res = test_env
        .client
        .post(format!("{}/coupons", base_url))
        .bearer_auth(admin_token)
        .json(&coupon_payload)
        .send()
        .await
        .expect("Failed to create coupon");

    assert_eq!(create_res.status(), 201);
    let create_body: serde_json::Value = create_res.json().await.unwrap();
    assert_eq!(create_body["code"], "SAVE20");
    assert_eq!(create_body["discount_percentage"], 20.0);
    assert_eq!(create_body["is_active"], true);
    let coupon_id = create_body["id"].as_str().unwrap();

    // Get coupon by ID
    let get_res = test_env
        .client
        .get(format!("{}/coupons/{}", base_url, coupon_id))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to get coupon");

    assert_eq!(get_res.status(), 200);
    let get_body: serde_json::Value = get_res.json().await.unwrap();
    assert_eq!(get_body["code"], "SAVE20");

    // Get all coupons
    let list_res = test_env
        .client
        .get(format!("{}/coupons", base_url))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to get coupons");

    assert_eq!(list_res.status(), 200);
    let list_body: serde_json::Value = list_res.json().await.unwrap();
    assert!(list_body["coupons"].as_array().unwrap().len() > 0);

    // Update coupon
    let update_payload = json!({
        "discount_percentage": 25.0,
        "is_active": true
    });

    let update_res = test_env
        .client
        .put(format!("{}/coupons/{}", base_url, coupon_id))
        .bearer_auth(admin_token)
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to update coupon");

    assert_eq!(update_res.status(), 200);
    let update_body: serde_json::Value = update_res.json().await.unwrap();
    assert_eq!(update_body["discount_percentage"], 25.0);

    // Delete coupon
    let delete_res = test_env
        .client
        .delete(format!("{}/coupons/{}", base_url, coupon_id))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to delete coupon");

    assert_eq!(delete_res.status(), 200);

    // Verify deleted
    let verify_res = test_env
        .client
        .get(format!("{}/coupons/{}", base_url, coupon_id))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to verify deletion");

    assert_eq!(verify_res.status(), 404);
}

#[tokio::test]
async fn test_coupon_validation() {
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

    // Create valid coupon
    let expiry_date = Utc::now() + Duration::days(30);
    let coupon_payload = json!({
        "code": "VALIDCODE",
        "discount_percentage": 15.0,
        "expiry_date": expiry_date.to_rfc3339()
    });

    test_env
        .client
        .post(format!("{}/coupons", base_url))
        .bearer_auth(admin_token)
        .json(&coupon_payload)
        .send()
        .await
        .expect("Failed to create coupon");

    // Validate valid coupon
    let validate_payload = json!({
        "code": "VALIDCODE"
    });

    let validate_res = test_env
        .client
        .post(format!("{}/coupons/validate", base_url))
        .bearer_auth(admin_token)
        .json(&validate_payload)
        .send()
        .await
        .expect("Failed to validate coupon");

    assert_eq!(validate_res.status(), 200);
    let validate_body: serde_json::Value = validate_res.json().await.unwrap();
    assert_eq!(validate_body["valid"], true);

    // Validate non-existent coupon
    let validate_invalid_payload = json!({
        "code": "NOTEXIST"
    });

    let validate_invalid_res = test_env
        .client
        .post(format!("{}/coupons/validate", base_url))
        .bearer_auth(admin_token)
        .json(&validate_invalid_payload)
        .send()
        .await
        .expect("Failed to validate coupon");

    assert_eq!(validate_invalid_res.status(), 200);
    let validate_invalid_body: serde_json::Value = validate_invalid_res.json().await.unwrap();
    assert_eq!(validate_invalid_body["valid"], false);
}

#[tokio::test]
async fn test_coupon_expired() {
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

    // Create expired coupon
    let expiry_date = Utc::now() - Duration::days(1);
    let coupon_payload = json!({
        "code": "EXPIRED",
        "discount_percentage": 10.0,
        "expiry_date": expiry_date.to_rfc3339()
    });

    // Should fail to create with past expiry date
    let create_res = test_env
        .client
        .post(format!("{}/coupons", base_url))
        .bearer_auth(admin_token)
        .json(&coupon_payload)
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_res.status(), 400);
}

#[tokio::test]
async fn test_coupon_requires_admin() {
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

    // Try to create coupon as regular user
    let expiry_date = Utc::now() + Duration::days(30);
    let coupon_payload = json!({
        "code": "SAVE10",
        "discount_percentage": 10.0,
        "expiry_date": expiry_date.to_rfc3339()
    });

    let create_res = test_env
        .client
        .post(format!("{}/coupons", base_url))
        .bearer_auth(user_token)
        .json(&coupon_payload)
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_res.status(), 403);

    // Try to get all coupons as regular user
    let list_res = test_env
        .client
        .get(format!("{}/coupons", base_url))
        .bearer_auth(user_token)
        .send()
        .await
        .expect("Failed to send list request");

    assert_eq!(list_res.status(), 403);
}

#[tokio::test]
async fn test_get_active_coupons() {
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

    // Create active coupon
    let expiry_date = Utc::now() + Duration::days(30);
    let active_coupon_payload = json!({
        "code": "ACTIVE20",
        "discount_percentage": 20.0,
        "expiry_date": expiry_date.to_rfc3339()
    });

    test_env
        .client
        .post(format!("{}/coupons", base_url))
        .bearer_auth(admin_token)
        .json(&active_coupon_payload)
        .send()
        .await
        .expect("Failed to create active coupon");

    // Create inactive coupon
    let inactive_coupon_payload = json!({
        "code": "INACTIVE10",
        "discount_percentage": 10.0,
        "expiry_date": expiry_date.to_rfc3339()
    });

    let create_inactive_res = test_env
        .client
        .post(format!("{}/coupons", base_url))
        .bearer_auth(admin_token)
        .json(&inactive_coupon_payload)
        .send()
        .await
        .expect("Failed to create inactive coupon");

    let inactive_body: serde_json::Value = create_inactive_res.json().await.unwrap();
    let inactive_id = inactive_body["id"].as_str().unwrap();

    // Deactivate the second coupon
    let update_payload = json!({
        "is_active": false
    });

    test_env
        .client
        .put(format!("{}/coupons/{}", base_url, inactive_id))
        .bearer_auth(admin_token)
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to deactivate coupon");

    // Get active coupons
    let active_res = test_env
        .client
        .get(format!("{}/coupons/active", base_url))
        .bearer_auth(admin_token)
        .send()
        .await
        .expect("Failed to get active coupons");

    assert_eq!(active_res.status(), 200);
    let active_body: serde_json::Value = active_res.json().await.unwrap();
    let active_coupons = active_body["coupons"].as_array().unwrap();

    // Should only have active coupon
    assert!(active_coupons.iter().all(|c| c["is_active"] == true));
}
