# Common Test Utilities and Testcontainer Setup
# tests/common/mod.rs

use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};
use testcontainers::{clients::Cli, Container, GenericImage};
use std::sync::Arc;

pub mod test_container;
pub mod fixtures;

pub use test_container::TestContext;
pub use fixtures::TestFixtures;

/// Setup a test database with testcontainer
pub async fn setup_test_db(docker: &Cli) -> (TestContext, Container<'_, GenericImage>) {
    // Start PostgreSQL container
    let postgres_image = GenericImage::new("postgres", "16-alpine")
        .with_env_var("POSTGRES_DB", "test_ecommerce")
        .with_env_var("POSTGRES_USER", "test_user")
        .with_env_var("POSTGRES_PASSWORD", "test_password")
        .with_wait_for(testcontainers::core::WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ));

    let container = docker.run(postgres_image);
    let port = container.get_host_port_ipv4(5432);

    // Build connection string
    let database_url = format!(
        "postgres://test_user:test_password@localhost:{}/test_ecommerce",
        port
    );

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let context = TestContext::new(pool, database_url);

    (context, container)
}

/// Clean up test database
pub async fn cleanup_test_db(pool: &PgPool) {
    let tables = vec![
        "order_items",
        "orders",
        "cart_items",
        "carts",
        "ratings",
        "wishlist",
        "products",
        "categories",
        "coupons",
        "blogs",
        "users",
    ];

    for table in tables {
        sqlx::query(&format!("TRUNCATE TABLE {} CASCADE", table))
            .execute(pool)
            .await
            .expect(&format!("Failed to truncate {}", table));
    }
}
