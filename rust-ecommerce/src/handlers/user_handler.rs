use crate::dto::user_dto::{MessageResponse, UpdateUserRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserRole;
use crate::services::user_service::UserService;
use crate::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

/// GET /api/users (admin only)
pub async fn get_all_users(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let users = UserService::get_all_users(&state.db).await?;
    Ok(HttpResponse::Ok().json(users))
}

/// GET /api/users/:id
pub async fn get_user_by_id(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = user_id.into_inner();

    // Users can view their own profile, admins can view any profile
    if user.id != user_id && user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "You can only view your own profile".to_string(),
        ));
    }

    let user_response = UserService::get_user_by_id(&state.db, &user_id).await?;
    Ok(HttpResponse::Ok().json(user_response))
}

/// PUT /api/users/:id
pub async fn update_user(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
    req: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = user_id.into_inner();

    // Users can only update their own profile, admins can update any profile
    if user.id != user_id && user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "You can only update your own profile".to_string(),
        ));
    }

    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user_response = UserService::update_user(
        &state.db,
        &user_id,
        &req.first_name,
        &req.last_name,
        &req.mobile,
    )
    .await?;

    Ok(HttpResponse::Ok().json(user_response))
}

/// DELETE /api/users/:id (admin only)
pub async fn delete_user(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let user_id = user_id.into_inner();

    // Prevent admin from deleting themselves
    if user.id == user_id {
        return Err(AppError::BadRequest(
            "Cannot delete your own account".to_string(),
        ));
    }

    UserService::delete_user(&state.db, &user_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "User deleted successfully".to_string(),
    }))
}

/// PUT /api/users/:id/block (admin only)
pub async fn block_user(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let user_id = user_id.into_inner();

    // Prevent admin from blocking themselves
    if user.id == user_id {
        return Err(AppError::BadRequest(
            "Cannot block your own account".to_string(),
        ));
    }

    let user_response = UserService::block_user(&state.db, &user_id).await?;

    Ok(HttpResponse::Ok().json(user_response))
}

/// PUT /api/users/:id/unblock (admin only)
pub async fn unblock_user(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let user_id = user_id.into_inner();
    let user_response = UserService::unblock_user(&state.db, &user_id).await?;

    Ok(HttpResponse::Ok().json(user_response))
}

/// Configure user routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_all_users))
            .route("/{id}", web::get().to(get_user_by_id))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
            .route("/{id}/block", web::put().to(block_user))
            .route("/{id}/unblock", web::put().to(unblock_user)),
    );
}
