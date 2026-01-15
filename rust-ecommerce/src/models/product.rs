use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
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
    pub images: sqlx::types::JsonValue,
    pub total_ratings: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Rating {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wishlist {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Rating {
    pub fn new(product_id: Uuid, user_id: Uuid, rating: i32, comment: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            product_id,
            user_id,
            rating,
            comment,
            created_at: Utc::now(),
        }
    }
}
