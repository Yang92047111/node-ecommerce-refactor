use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Create an optimized database connection pool
/// 
/// Configuration is based on best practices:
/// - max_connections: Set to 20 for production use (should be tuned based on load)
/// - min_connections: Keep 5 connections warm for quick response
/// - acquire_timeout: 30 seconds to prevent long waits
/// - idle_timeout: Close idle connections after 10 minutes
/// - max_lifetime: Recycle connections after 30 minutes
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    let min_connections = std::env::var("DB_MIN_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Some(Duration::from_secs(600))) // 10 minutes
        .max_lifetime(Some(Duration::from_secs(1800))) // 30 minutes
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
