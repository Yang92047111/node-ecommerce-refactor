use crate::dto::order_dto::{CreateOrderRequest, OrderQueryParams, UpdateOrderStatusRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserRole;
use crate::services::order_service::OrderService;
use crate::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

/// POST /api/orders
pub async fn create_order(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    req: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.0.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let order = OrderService::create_order(&state.db, &user.id, req.into_inner()).await?;
    Ok(HttpResponse::Created().json(order))
}

/// GET /api/orders
pub async fn get_orders(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<OrderQueryParams>,
) -> Result<HttpResponse, AppError> {
    let orders = OrderService::get_user_orders(&state.db, &user.id, query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(orders))
}

/// GET /api/orders/:id
pub async fn get_order(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    order_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let order_id = order_id.into_inner();
    let order = OrderService::get_order(&state.db, &order_id).await?;

    // Verify order belongs to user (unless admin)
    if order.user_id != user.id && user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "You don't have permission to view this order".to_string(),
        ));
    }

    Ok(HttpResponse::Ok().json(order))
}

/// PUT /api/orders/:id/status (admin only)
pub async fn update_order_status(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    order_id: web::Path<Uuid>,
    req: web::Json<UpdateOrderStatusRequest>,
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

    let order_id = order_id.into_inner();
    let order = OrderService::update_order_status(&state.db, &order_id, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(order))
}

/// GET /api/orders/user/:userId (admin only)
pub async fn get_user_orders_admin(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
    query: web::Query<OrderQueryParams>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let user_id = user_id.into_inner();
    let orders = OrderService::get_user_orders(&state.db, &user_id, query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(orders))
}

/// GET /api/admin/orders (admin only - all orders)
pub async fn get_all_orders(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<OrderQueryParams>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }

    let orders = OrderService::get_all_orders(&state.db, query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(orders))
}

/// Configure order routes (requires authentication)
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("", web::post().to(create_order))
            .route("", web::get().to(get_orders))
            .route("/{id}", web::get().to(get_order)),
    );
}

/// Configure admin order routes (requires authentication and admin role)
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("/{id}/status", web::put().to(update_order_status))
            .route("/user/{user_id}", web::get().to(get_user_orders_admin)),
    )
    .service(
        web::scope("/admin/orders")
            .route("", web::get().to(get_all_orders)),
    );
}
