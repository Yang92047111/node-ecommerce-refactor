use actix_governor::{governor::middleware::NoOpMiddleware, Governor, GovernorConfigBuilder, PeerIpKeyExtractor};

/// Create rate limiter for authentication endpoints
/// Limit: 5 requests per minute
pub fn auth_rate_limiter() -> Governor<PeerIpKeyExtractor, NoOpMiddleware> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(12000) // 60000ms / 5 requests = 12000ms per request
        .burst_size(5)
        .finish()
        .unwrap();

    Governor::new(&governor_conf)
}

/// Create rate limiter for general API endpoints
/// Limit: 100 requests per minute
pub fn api_rate_limiter() -> Governor<PeerIpKeyExtractor, NoOpMiddleware> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(600) // 60000ms / 100 requests = 600ms per request
        .burst_size(100)
        .finish()
        .unwrap();

    Governor::new(&governor_conf)
}

/// Create rate limiter for admin endpoints
/// Limit: 50 requests per minute
pub fn admin_rate_limiter() -> Governor<PeerIpKeyExtractor, NoOpMiddleware> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(1200) // 60000ms / 50 requests = 1200ms per request
        .burst_size(50)
        .finish()
        .unwrap();

    Governor::new(&governor_conf)
}
