use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBlogRequest {
    #[validate(length(min = 1, max = 500, message = "Title must be between 1 and 500 characters"))]
    pub title: String,
    pub description: Option<String>,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
    pub category_id: Option<Uuid>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBlogRequest {
    #[validate(length(min = 1, max = 500, message = "Title must be between 1 and 500 characters"))]
    pub title: Option<String>,
    pub description: Option<String>,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: Option<String>,
    pub category_id: Option<Uuid>,
    pub images: Option<Vec<String>>,
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub author_id: Uuid,
    pub category_id: Option<Uuid>,
    pub images: serde_json::Value,
    pub views_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BlogsListResponse {
    pub blogs: Vec<BlogResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
