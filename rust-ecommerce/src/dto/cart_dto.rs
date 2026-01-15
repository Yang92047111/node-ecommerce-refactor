use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct AddCartItemRequest {
    pub product_id: Uuid,

    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCartItemRequest {
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub product_title: Option<String>,
    pub product_price: Option<Decimal>,
    pub product_image: Option<String>,
    pub subtotal: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<CartItemResponse>,
    pub total_items: i32,
    pub subtotal: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CartSummary {
    pub total_items: i32,
    pub subtotal: Decimal,
    pub shipping: Decimal,
    pub total: Decimal,
}
