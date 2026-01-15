use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CategoriesListResponse {
    pub categories: Vec<CategoryResponse>,
}
