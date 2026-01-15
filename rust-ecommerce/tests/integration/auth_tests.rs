use rust_ecommerce::{configure_routes, AppState};
use actix_web::{test, web, App};
use serde_json::json;

mod common;
use common::test_container::TestDatabase;

#[tokio::test]
async fn test_register_user() {
    // Setup test database
    let docker = testcontainers::clients::Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    // Test user registration
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "first_name": "John",
            "last_name": "Doe",
            "email": "john.doe@example.com",
            "mobile": "1234567890",
            "password": "securePassword123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["user"]["email"], "john.doe@example.com");
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let docker = testcontainers::clients::Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    let user_data = json!({
        "first_name": "Jane",
        "last_name": "Smith",
        "email": "jane.smith@example.com",
        "mobile": "9876543210",
        "password": "password123"
    });

    // Register first time - should succeed
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&user_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    // Register again with same email - should fail
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&user_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 409); // Conflict
}

#[tokio::test]
async fn test_login_success() {
    let docker = testcontainers::clients::Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    // Register a user first
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "first_name": "Test",
            "last_name": "User",
            "email": "test@example.com",
            "mobile": "1111111111",
            "password": "testPassword123"
        }))
        .to_request();
    let _ = test::call_service(&app, register_req).await;

    // Now login
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "email": "test@example.com",
            "password": "testPassword123"
        }))
        .to_request();

    let resp = test::call_service(&app, login_req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["user"]["email"], "test@example.com");
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());
}

#[tokio::test]
async fn test_login_wrong_password() {
    let docker = testcontainers::clients::Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    // Register a user
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "first_name": "Test",
            "last_name": "User",
            "email": "wrong@example.com",
            "mobile": "2222222222",
            "password": "correctPassword"
        }))
        .to_request();
    let _ = test::call_service(&app, register_req).await;

    // Try to login with wrong password
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "email": "wrong@example.com",
            "password": "wrongPassword"
        }))
        .to_request();

    let resp = test::call_service(&app, login_req).await;
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[tokio::test]
async fn test_refresh_token() {
    let docker = testcontainers::clients::Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    // Set JWT secret for testing
    std::env::set_var("JWT_SECRET", "test_secret_for_integration_tests");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    // Register and login to get tokens
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "first_name": "Refresh",
            "last_name": "Test",
            "email": "refresh@example.com",
            "mobile": "3333333333",
            "password": "password123"
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    let register_body: serde_json::Value = test::read_body_json(register_resp).await;
    let refresh_token = register_body["refresh_token"].as_str().unwrap();

    // Use refresh token to get new tokens
    let refresh_req = test::TestRequest::post()
        .uri("/api/auth/refresh")
        .set_json(json!({
            "refresh_token": refresh_token
        }))
        .to_request();

    let resp = test::call_service(&app, refresh_req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());
}

#[tokio::test]
async fn test_forgot_password() {
    let docker = testcontainers::clients::Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    // Register a user
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "first_name": "Forgot",
            "last_name": "Password",
            "email": "forgot@example.com",
            "mobile": "4444444444",
            "password": "oldPassword123"
        }))
        .to_request();
    let _ = test::call_service(&app, register_req).await;

    // Request password reset
    let forgot_req = test::TestRequest::post()
        .uri("/api/auth/forgot-password")
        .set_json(json!({
            "email": "forgot@example.com"
        }))
        .to_request();

    let resp = test::call_service(&app, forgot_req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["reset_token"].is_string());
}

#[tokio::test]
async fn test_reset_password() {
    let docker = testcontainers::clients::Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    // Register a user
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "first_name": "Reset",
            "last_name": "Password",
            "email": "reset@example.com",
            "mobile": "5555555555",
            "password": "oldPassword123"
        }))
        .to_request();
    let _ = test::call_service(&app, register_req).await;

    // Get reset token
    let forgot_req = test::TestRequest::post()
        .uri("/api/auth/forgot-password")
        .set_json(json!({
            "email": "reset@example.com"
        }))
        .to_request();
    let forgot_resp = test::call_service(&app, forgot_req).await;
    let forgot_body: serde_json::Value = test::read_body_json(forgot_resp).await;
    let reset_token = forgot_body["reset_token"].as_str().unwrap();

    // Reset password
    let reset_req = test::TestRequest::put()
        .uri(&format!("/api/auth/reset-password/{}", reset_token))
        .set_json(json!({
            "password": "newPassword123"
        }))
        .to_request();

    let resp = test::call_service(&app, reset_req).await;
    assert_eq!(resp.status(), 200);

    // Try to login with new password
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "email": "reset@example.com",
            "password": "newPassword123"
        }))
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), 200);
}
