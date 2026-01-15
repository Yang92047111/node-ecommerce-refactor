use rust_ecommerce::{config::database, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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
    log::info!("Connecting to database...");
    let pool = database::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    log::info!("Running database migrations...");
    database::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    log::info!("Database setup complete");

    // Start the server
    run(pool, port).await
}
