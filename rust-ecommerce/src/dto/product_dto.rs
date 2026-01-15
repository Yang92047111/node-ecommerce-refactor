use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,

    #[validate(length(min = 1))]
    pub description: String,

    pub price: Decimal,
    pub quantity: i32,
    pub brand: Option<String>,
    pub category_id: Option<Uuid>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub quantity: Option<i32>,
    pub brand: Option<String>,
    pub category_id: Option<Uuid>,
    pub discount: Option<Decimal>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRatingRequest {
    #[validate(range(min = 1, max = 5, message = "Rating must be between 1 and 5"))]
    pub rating: i32,

    #[validate(length(max = 1000, message = "Comment must be less than 1000 characters"))]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProductQueryParams {
    #[serde(default = "default_page")]
    pub page: i64,

    #[serde(default = "default_limit")]
    pub limit: i64,

    pub category_id: Option<Uuid>,
    pub search: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    10
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub price: Decimal,
    pub quantity: i32,
    pub brand: Option<String>,
    pub category_id: Option<Uuid>,
    pub sold: i32,
    pub discount: Decimal,
    pub images: JsonValue,
    pub total_ratings: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProductsListResponse {
    pub products: Vec<ProductResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RatingsListResponse {
    pub ratings: Vec<RatingResponse>,
    pub average: Decimal,
}
