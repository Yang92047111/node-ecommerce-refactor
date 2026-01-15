pub mod config;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

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
            .configure(handlers::auth_handler::configure_routes)
            .configure(handlers::category_handler::configure_routes)
            .configure(handlers::product_handler::configure_routes)
            .service(
                web::scope("")
                    .wrap(actix_web::middleware::from_fn(middleware::auth::auth_middleware))
                    .configure(handlers::user_handler::configure_routes)
                    .configure(handlers::category_handler::configure_admin_routes)
                    .configure(handlers::product_handler::configure_admin_routes)
                    .configure(handlers::product_handler::configure_wishlist_routes),
            ),
    );
}

pub async fn run(pool: PgPool, port: u16) -> std::io::Result<()> {
    log::info!("Starting server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
