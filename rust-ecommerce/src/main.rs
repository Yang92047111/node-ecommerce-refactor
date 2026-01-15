use rust_ecommerce::{config::database, run};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize enhanced logging with tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_ecommerce=info,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting E-Commerce API Server");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Get port from environment or use default
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Create database connection pool
    tracing::info!("Connecting to database...");
    let pool = database::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!(
        "Database connection pool created with {} max connections",
        pool.options().get_max_connections()
    );

    // Run migrations
    tracing::info!("Running database migrations...");
    database::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Database setup complete");
    tracing::info!("Starting HTTP server on port {}", port);

    // Start the server
    run(pool, port).await
}
