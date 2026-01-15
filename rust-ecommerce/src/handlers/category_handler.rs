use crate::dto::category_dto::{CreateCategoryRequest, UpdateCategoryRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserRole;
use crate::services::category_service::CategoryService;
use crate::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

/// GET /api/categories
pub async fn get_all_categories(
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let categories = CategoryService::get_all_categories(&state.db).await?;
    Ok(HttpResponse::Ok().json(categories))
}

/// GET /api/categories/:id
pub async fn get_category(
    state: web::Data<AppState>,
    category_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let category_id = category_id.into_inner();
    let category = CategoryService::get_category(&state.db, &category_id).await?;
    Ok(HttpResponse::Ok().json(category))
}

/// POST /api/categories (admin only)
pub async fn create_category(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    req: web::Json<CreateCategoryRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let category = CategoryService::create_category(&state.db, req.into_inner()).await?;
    Ok(HttpResponse::Created().json(category))
}

/// PUT /api/categories/:id (admin only)
pub async fn update_category(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    category_id: web::Path<Uuid>,
    req: web::Json<UpdateCategoryRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let category_id = category_id.into_inner();
    let category = CategoryService::update_category(&state.db, &category_id, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(category))
}

/// DELETE /api/categories/:id (admin only)
pub async fn delete_category(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    category_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let category_id = category_id.into_inner();
    CategoryService::delete_category(&state.db, &category_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Category deleted successfully"
    })))
}

/// Configure category routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::get().to(get_all_categories))
            .route("/{id}", web::get().to(get_category)),
    );
}

/// Configure admin category routes (requires authentication)
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::post().to(create_category))
            .route("/{id}", web::put().to(update_category))
            .route("/{id}", web::delete().to(delete_category)),
    );
}
