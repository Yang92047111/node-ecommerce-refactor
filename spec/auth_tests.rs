# Integration Test Example: Authentication Tests
# tests/integration/auth_tests.rs

use sqlx::PgPool;
use testcontainers::clients::Cli;

mod common;
use common::{setup_test_db, cleanup_test_db, TestFixtures};

#[tokio::test]
async fn test_user_registration_success() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    // Test registration logic here
    // This is a template - actual implementation would use your app's API
    
    let email = TestFixtures::random_email();
    let password = "SecurePassword123!";
    
    // TODO: Call your registration API endpoint
    // let response = app.register_user(email, password).await;
    // assert!(response.is_ok());
    
    // Verify user was created in database
    let user = sqlx::query!(
        r#"SELECT id, email, role FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(ctx.pool())
    .await
    .expect("Failed to query user");
    
    // TODO: Uncomment when implementing
    // assert!(user.is_some());
    // let user = user.unwrap();
    // assert_eq!(user.email, email);
    // assert_eq!(user.role, "user");
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_user_login_success() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    // Create a test user first
    let email = "test@example.com";
    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$..."; // TODO: Hash actual password
    
    let user_id = sqlx::query!(
        r#"
        INSERT INTO users (first_name, last_name, email, mobile, password_hash, role)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        "Test",
        "User",
        email,
        "+1234567890",
        password_hash,
        "user"
    )
    .fetch_one(ctx.pool())
    .await
    .expect("Failed to create test user");
    
    // TODO: Test login
    // let response = app.login(email, "password123").await;
    // assert!(response.is_ok());
    // assert!(response.access_token.is_some());
    // assert!(response.refresh_token.is_some());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_user_login_invalid_credentials() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    // TODO: Test login with invalid credentials
    // let response = app.login("nonexistent@example.com", "wrongpassword").await;
    // assert!(response.is_err());
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_refresh_token_flow() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    // Create user and login
    // Get refresh token
    // Use refresh token to get new access token
    // TODO: Implement refresh token flow test
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_password_reset_flow() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    // Create test user
    // Request password reset
    // Verify reset token is created
    // Reset password with token
    // Verify new password works
    // TODO: Implement password reset test
    
    cleanup_test_db(ctx.pool()).await;
}

#[tokio::test]
async fn test_duplicate_email_registration() {
    let docker = Cli::default();
    let (ctx, _container) = setup_test_db(&docker).await;
    
    let email = "duplicate@example.com";
    
    // Create first user
    sqlx::query!(
        r#"
        INSERT INTO users (first_name, last_name, email, mobile, password_hash, role)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        "First",
        "User",
        email,
        "+1111111111",
        "hashed_password",
        "user"
    )
    .execute(ctx.pool())
    .await
    .expect("Failed to create first user");
    
    // Try to register with same email
    // TODO: This should fail
    // let response = app.register_user(email, "password123").await;
    // assert!(response.is_err());
    
    cleanup_test_db(ctx.pool()).await;
}
