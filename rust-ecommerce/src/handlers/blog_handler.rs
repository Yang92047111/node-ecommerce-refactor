use crate::dto::blog_dto::{CreateBlogRequest, UpdateBlogRequest};
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserRole;
use crate::services::blog_service::BlogService;
use crate::AppState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

/// GET /api/blogs
pub async fn get_all_blogs(
    state: web::Data<AppState>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let blogs = BlogService::get_all_blogs(&state.db, query.page, query.page_size).await?;
    Ok(HttpResponse::Ok().json(blogs))
}

/// GET /api/blogs/:id
pub async fn get_blog(
    state: web::Data<AppState>,
    blog_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let blog_id = blog_id.into_inner();
    // Increment view count when getting a blog
    let blog = BlogService::get_blog(&state.db, &blog_id, true).await?;
    Ok(HttpResponse::Ok().json(blog))
}

/// GET /api/blogs/author/:author_id
pub async fn get_blogs_by_author(
    state: web::Data<AppState>,
    author_id: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let author_id = author_id.into_inner();
    let blogs =
        BlogService::get_blogs_by_author(&state.db, &author_id, query.page, query.page_size)
            .await?;
    Ok(HttpResponse::Ok().json(blogs))
}

/// POST /api/blogs (admin only)
pub async fn create_blog(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    req: web::Json<CreateBlogRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    if user.role != UserRole::Admin.as_str() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user_id = user.id;

    let blog = BlogService::create_blog(&state.db, req.into_inner(), &user_id).await?;
    Ok(HttpResponse::Created().json(blog))
}

/// PUT /api/blogs/:id (author or admin)
pub async fn update_blog(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    blog_id: web::Path<Uuid>,
    req: web::Json<UpdateBlogRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let blog_id = blog_id.into_inner();
    let user_id = user.id;
    let is_admin = user.role == UserRole::Admin.as_str();

    let blog = BlogService::update_blog(&state.db, &blog_id, req.into_inner(), &user_id, is_admin)
        .await?;
    Ok(HttpResponse::Ok().json(blog))
}

/// DELETE /api/blogs/:id (author or admin)
pub async fn delete_blog(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    blog_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let blog_id = blog_id.into_inner();
    let user_id = user.id;
    let is_admin = user.role == UserRole::Admin.as_str();

    BlogService::delete_blog(&state.db, &blog_id, &user_id, is_admin).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Blog deleted successfully"
    })))
}

/// Configure blog routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blogs")
            .route("", web::get().to(get_all_blogs))
            .route("/{id}", web::get().to(get_blog))
            .route("/author/{author_id}", web::get().to(get_blogs_by_author)),
    );
}

/// Configure authenticated blog routes
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blogs")
            .route("", web::post().to(create_blog))
            .route("/{id}", web::put().to(update_blog))
            .route("/{id}", web::delete().to(delete_blog)),
    );
}
