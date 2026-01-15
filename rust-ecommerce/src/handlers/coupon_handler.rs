use crate::dto::coupon_dto::{CreateCouponRequest, UpdateCouponRequest, ValidateCouponRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserRole;
use crate::services::coupon_service::CouponService;
use crate::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

/// GET /api/coupons (admin only)
pub async fn get_all_coupons(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let coupons = CouponService::get_all_coupons(&state.db).await?;
    Ok(HttpResponse::Ok().json(coupons))
}

/// GET /api/coupons/active (admin only)
pub async fn get_active_coupons(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let coupons = CouponService::get_active_coupons(&state.db).await?;
    Ok(HttpResponse::Ok().json(coupons))
}

/// GET /api/coupons/:id (admin only)
pub async fn get_coupon(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    coupon_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let coupon_id = coupon_id.into_inner();
    let coupon = CouponService::get_coupon(&state.db, &coupon_id).await?;
    Ok(HttpResponse::Ok().json(coupon))
}

/// POST /api/coupons/validate
pub async fn validate_coupon(
    state: web::Data<AppState>,
    _user: AuthenticatedUser,
    req: web::Json<ValidateCouponRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let validation = CouponService::validate_coupon(&state.db, &req.code).await?;
    Ok(HttpResponse::Ok().json(validation))
}

/// POST /api/coupons (admin only)
pub async fn create_coupon(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    req: web::Json<CreateCouponRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let coupon = CouponService::create_coupon(&state.db, req.into_inner()).await?;
    Ok(HttpResponse::Created().json(coupon))
}

/// PUT /api/coupons/:id (admin only)
pub async fn update_coupon(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    coupon_id: web::Path<Uuid>,
    req: web::Json<UpdateCouponRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let coupon_id = coupon_id.into_inner();
    let coupon = CouponService::update_coupon(&state.db, &coupon_id, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(coupon))
}

/// DELETE /api/coupons/:id (admin only)
pub async fn delete_coupon(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    coupon_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let coupon_id = coupon_id.into_inner();
    CouponService::delete_coupon(&state.db, &coupon_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Coupon deleted successfully"
    })))
}

/// Configure coupon routes (requires authentication)
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/coupons")
            .route("", web::get().to(get_all_coupons))
            .route("/active", web::get().to(get_active_coupons))
            .route("/validate", web::post().to(validate_coupon))
            .route("/{id}", web::get().to(get_coupon))
            .route("", web::post().to(create_coupon))
            .route("/{id}", web::put().to(update_coupon))
            .route("/{id}", web::delete().to(delete_coupon)),
    );
}
