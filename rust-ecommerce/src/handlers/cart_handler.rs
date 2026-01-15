use crate::dto::cart_dto::{AddCartItemRequest, UpdateCartItemRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::cart_service::CartService;
use crate::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

/// GET /api/cart
pub async fn get_cart(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let cart = CartService::get_cart(&state.db, &user.id).await?;
    Ok(HttpResponse::Ok().json(cart))
}

/// POST /api/cart/items
pub async fn add_cart_item(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    req: web::Json<AddCartItemRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.0.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let cart = CartService::add_item(&state.db, &user.id, req.into_inner()).await?;
    Ok(HttpResponse::Created().json(cart))
}

/// PUT /api/cart/items/:id
pub async fn update_cart_item(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    item_id: web::Path<Uuid>,
    req: web::Json<UpdateCartItemRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.0.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let item_id = item_id.into_inner();
    let cart = CartService::update_item(&state.db, &user.id, &item_id, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(cart))
}

/// DELETE /api/cart/items/:id
pub async fn delete_cart_item(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    item_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let item_id = item_id.into_inner();
    let cart = CartService::remove_item(&state.db, &user.id, &item_id).await?;
    Ok(HttpResponse::Ok().json(cart))
}

/// DELETE /api/cart
pub async fn clear_cart(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    CartService::clear_cart(&state.db, &user.id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Cart cleared successfully"
    })))
}

/// Configure cart routes (requires authentication)
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("", web::get().to(get_cart))
            .route("", web::delete().to(clear_cart))
            .route("/items", web::post().to(add_cart_item))
            .route("/items/{id}", web::put().to(update_cart_item))
            .route("/items/{id}", web::delete().to(delete_cart_item)),
    );
}
