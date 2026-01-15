use actix_web::{test, web, App};
use rust_ecommerce::{configure_routes, AppState};
use testcontainers::clients::Cli;

mod common;
use common::test_container::TestDatabase;

#[actix_web::test]
async fn test_health_check_endpoint() {
    // Set up test database
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    // Create test app with database pool
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: test_db.pool.clone(),
            }))
            .configure(configure_routes),
    )
    .await;

    // Make request to health check endpoint
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert response
    assert!(resp.status().is_success());
    assert_eq!(resp.status().as_u16(), 200);

    // Parse and verify response body
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["status"], "ok");
    assert_eq!(json["message"], "Server is running");
}

#[actix_web::test]
async fn test_database_connection() {
    // Set up test database
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;

    // Test simple query
    let result = sqlx::query("SELECT 1 as test_value")
        .fetch_one(&test_db.pool)
        .await;

    assert!(result.is_ok());
}
