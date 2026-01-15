use rust_ecommerce::{configure_routes, AppState};
use actix_web::{test, web, App};
use serde_json::json;

mod common;
use common::test_container::TestDatabase;

async fn create_test_user_with_token(app: &impl actix_web::dev::Service<
    actix_web::dev::ServiceRequest,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
>, email: &str, role: &str) -> (String, String) {
    // Set JWT secret
    std::env::set_var("JWT_SECRET", "test_secret_for_integration_tests");

    // Register user
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "first_name": "Test",
            "last_name": "User",
            "email": email,
            "mobile": format!("{}", rand::random::<u64>() % 10000000000),
            "password": "testPassword123"
        }))
        .to_request();

    let resp = test::call_service(app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    
    let user_id = body["user"]["id"].as_str().unwrap().to_string();
    let access_token = body["access_token"].as_str().unwrap().to_string();

    (user_id, access_token)
}

#[tokio::test]
async fn test_get_user_by_id() {
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

    let (user_id, access_token) = create_test_user_with_token(&app, "getuser@example.com", "user").await;

    // Get user by ID
    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}", user_id))
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["email"], "getuser@example.com");
}

#[tokio::test]
async fn test_update_user() {
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

    let (user_id, access_token) = create_test_user_with_token(&app, "update@example.com", "user").await;

    // Update user
    let req = test::TestRequest::put()
        .uri(&format!("/api/users/{}", user_id))
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .set_json(json!({
            "first_name": "Updated",
            "last_name": "Name",
            "mobile": "9999999999"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["first_name"], "Updated");
    assert_eq!(body["last_name"], "Name");
}

#[tokio::test]
async fn test_user_cannot_access_other_user() {
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

    let (user1_id, _) = create_test_user_with_token(&app, "user1@example.com", "user").await;
    let (_, user2_token) = create_test_user_with_token(&app, "user2@example.com", "user").await;

    // User 2 tries to access User 1's profile
    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}", user1_id))
        .insert_header(("Authorization", format!("Bearer {}", user2_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403); // Forbidden
}

#[tokio::test]
async fn test_unauthorized_access() {
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

    // Try to access protected endpoint without token
    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[tokio::test]
async fn test_invalid_token() {
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

    // Try to access with invalid token
    let req = test::TestRequest::get()
        .uri("/api/users")
        .insert_header(("Authorization", "Bearer invalid_token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401); // Unauthorized
}
