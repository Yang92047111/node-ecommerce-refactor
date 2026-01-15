use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: u64,
    pub version: String,
    pub database: DatabaseHealth,
    pub uptime: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseHealth {
    pub status: String,
    pub connections: ConnectionStats,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionStats {
    pub active: u32,
    pub idle: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub checks: Vec<Check>,
}

#[derive(Serialize, Deserialize)]
pub struct Check {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MetricsResponse {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_error: u64,
    pub average_response_time_ms: f64,
}

static START_TIME: std::sync::OnceLock<SystemTime> = std::sync::OnceLock::new();

pub fn init_monitoring() {
    START_TIME.get_or_init(SystemTime::now);
}

/// Health check endpoint - returns overall system health
pub async fn health_check(pool: web::Data<crate::AppState>) -> impl Responder {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let start_time = START_TIME.get().unwrap_or(&SystemTime::now());
    let uptime = SystemTime::now()
        .duration_since(*start_time)
        .unwrap()
        .as_secs();

    // Check database connection
    let db_health = check_database_health(&pool.db).await;

    let health = HealthResponse {
        status: if db_health.status == "healthy" {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        timestamp,
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_health,
        uptime,
    };

    if health.status == "healthy" {
        HttpResponse::Ok().json(health)
    } else {
        HttpResponse::ServiceUnavailable().json(health)
    }
}

/// Liveness probe - simple check that the application is running
pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "alive",
        "timestamp": SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

/// Readiness probe - checks if the application is ready to serve traffic
pub async fn readiness(pool: web::Data<crate::AppState>) -> impl Responder {
    let mut checks = vec![];

    // Check database connection
    let db_check = sqlx::query("SELECT 1")
        .fetch_one(&pool.db)
        .await;

    checks.push(Check {
        name: "database".to_string(),
        status: if db_check.is_ok() {
            "ready".to_string()
        } else {
            "not_ready".to_string()
        },
        message: db_check.err().map(|e| e.to_string()),
    });

    let all_ready = checks.iter().all(|c| c.status == "ready");

    let response = ReadinessResponse {
        ready: all_ready,
        checks,
    };

    if all_ready {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

/// Check database health and connection pool statistics
async fn check_database_health(pool: &PgPool) -> DatabaseHealth {
    // Try a simple query to verify database connectivity
    let query_result = sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await;

    let status = if query_result.is_ok() {
        "healthy".to_string()
    } else {
        "unhealthy".to_string()
    };

    // Get pool statistics
    let active = pool.size();
    let idle = pool.num_idle();
    let max = pool.options().get_max_connections();

    DatabaseHealth {
        status,
        connections: ConnectionStats {
            active,
            idle,
            max,
        },
    }
}

/// Configure monitoring routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/monitoring")
            .route("/health", web::get().to(health_check))
            .route("/liveness", web::get().to(liveness))
            .route("/readiness", web::get().to(readiness)),
    );
}
