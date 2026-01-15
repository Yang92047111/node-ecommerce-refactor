use crate::dto::product_dto::{
    CreateProductRequest, CreateRatingRequest, ProductQueryParams, UpdateProductRequest,
};
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserRole;
use crate::services::product_service::{ProductService, RatingService, WishlistService};
use crate::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

/// GET /api/products
pub async fn get_products(
    state: web::Data<AppState>,
    query: web::Query<ProductQueryParams>,
) -> Result<HttpResponse, AppError> {
    let products = ProductService::get_products(&state.db, query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(products))
}

/// GET /api/products/:id
pub async fn get_product(
    state: web::Data<AppState>,
    product_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = product_id.into_inner();
    let product = ProductService::get_product(&state.db, &product_id).await?;
    Ok(HttpResponse::Ok().json(product))
}

/// POST /api/products (admin only)
pub async fn create_product(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    req: web::Json<CreateProductRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    // Validate request
    req.0.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let product = ProductService::create_product(&state.db, req.into_inner()).await?;
    Ok(HttpResponse::Created().json(product))
}

/// PUT /api/products/:id (admin only)
pub async fn update_product(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    product_id: web::Path<Uuid>,
    req: web::Json<UpdateProductRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let product_id = product_id.into_inner();
    let product = ProductService::update_product(&state.db, &product_id, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(product))
}

/// DELETE /api/products/:id (admin only)
pub async fn delete_product(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    product_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let product_id = product_id.into_inner();
    ProductService::delete_product(&state.db, &product_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Product deleted successfully"
    })))
}

/// POST /api/products/:id/ratings
pub async fn add_rating(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    product_id: web::Path<Uuid>,
    req: web::Json<CreateRatingRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.0.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let product_id = product_id.into_inner();
    let rating = RatingService::add_rating(&state.db, &product_id, &user.id, req.into_inner()).await?;
    Ok(HttpResponse::Created().json(rating))
}

/// GET /api/products/:id/ratings
pub async fn get_ratings(
    state: web::Data<AppState>,
    product_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = product_id.into_inner();
    let ratings = RatingService::get_product_ratings(&state.db, &product_id).await?;
    Ok(HttpResponse::Ok().json(ratings))
}

/// POST /api/users/wishlist/:productId
pub async fn add_to_wishlist(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    product_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = product_id.into_inner();
    WishlistService::add_to_wishlist(&state.db, &user.id, &product_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Product added to wishlist"
    })))
}

/// DELETE /api/users/wishlist/:productId
pub async fn remove_from_wishlist(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    product_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = product_id.into_inner();
    WishlistService::remove_from_wishlist(&state.db, &user.id, &product_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Product removed from wishlist"
    })))
}

/// GET /api/users/wishlist
pub async fn get_wishlist(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let wishlist = WishlistService::get_user_wishlist(&state.db, &user.id).await?;
    Ok(HttpResponse::Ok().json(wishlist))
}

/// Configure product routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_products))
            .route("/{id}", web::get().to(get_product))
            .route("/{id}/ratings", web::get().to(get_ratings)),
    );
}

/// Configure admin product routes (requires authentication)
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(create_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product))
            .route("/{id}/ratings", web::post().to(add_rating)),
    );
}

/// Configure wishlist routes (requires authentication)
pub fn configure_wishlist_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users/wishlist")
            .route("", web::get().to(get_wishlist))
            .route("/{product_id}", web::post().to(add_to_wishlist))
            .route("/{product_id}", web::delete().to(remove_from_wishlist)),
    );
}
