pub mod config;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .configure(handlers::monitoring_handler::configure_routes)
            .configure(handlers::auth_handler::configure_routes)
            .configure(handlers::category_handler::configure_routes)
            .configure(handlers::product_handler::configure_routes)
            .configure(handlers::blog_handler::configure_routes)
            .service(
                web::scope("")
                    .wrap(actix_web::middleware::from_fn(middleware::auth::auth_middleware))
                    .configure(handlers::user_handler::configure_routes)
                    .configure(handlers::category_handler::configure_admin_routes)
                    .configure(handlers::product_handler::configure_admin_routes)
                    .configure(handlers::product_handler::configure_wishlist_routes)
                    .configure(handlers::cart_handler::configure_routes)
                    .configure(handlers::order_handler::configure_routes)
                    .configure(handlers::order_handler::configure_admin_routes)
                    .configure(handlers::coupon_handler::configure_routes)
                    .configure(handlers::blog_handler::configure_auth_routes),
            ),
    );
}

pub async fn run(pool: PgPool, port: u16) -> std::io::Result<()> {
    log::info!("Starting server on port {}", port);

    // Initialize monitoring
    handlers::monitoring_handler::init_monitoring();

    // Get allowed origins from environment or use default
    let allowed_origin = std::env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    log::info!("CORS allowed origin: {}", allowed_origin);

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_origin("http://localhost:5173") // Vite dev server
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::rate_limit::api_rate_limiter())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
