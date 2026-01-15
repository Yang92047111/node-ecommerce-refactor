use crate::dto::auth_dto::{
    ForgotPasswordRequest, LoginRequest, MessageResponse, RefreshTokenRequest, RegisterRequest,
    ResetPasswordRequest,
};
use crate::errors::AppError;
use crate::services::auth_service::AuthService;
use crate::AppState;
use actix_web::{web, HttpResponse};
use validator::Validate;

/// POST /api/auth/register
pub async fn register(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Register user
    let response = AuthService::register(
        &state.db,
        &req.first_name,
        &req.last_name,
        &req.email,
        &req.mobile,
        &req.password,
    )
    .await?;

    Ok(HttpResponse::Created().json(response))
}

/// POST /api/auth/login
pub async fn login(
    state: web::Data<AppState>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Login user
    let response = AuthService::login(&state.db, &req.email, &req.password).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/auth/refresh
pub async fn refresh_token(
    state: web::Data<AppState>,
    req: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Refresh token
    let response = AuthService::refresh_token(&state.db, &req.refresh_token).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/auth/forgot-password
pub async fn forgot_password(
    state: web::Data<AppState>,
    req: web::Json<ForgotPasswordRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Generate reset token
    let response = AuthService::forgot_password(&state.db, &req.email).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// PUT /api/auth/reset-password/:token
pub async fn reset_password(
    state: web::Data<AppState>,
    token: web::Path<String>,
    req: web::Json<ResetPasswordRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Reset password
    AuthService::reset_password(&state.db, &token, &req.password).await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Password reset successfully".to_string(),
    }))
}

/// POST /api/auth/logout (placeholder - token management handled client-side)
pub async fn logout() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Logged out successfully".to_string(),
    }))
}

/// Configure auth routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh_token))
            .route("/forgot-password", web::post().to(forgot_password))
            .route("/reset-password/{token}", web::put().to(reset_password)),
    );
}
