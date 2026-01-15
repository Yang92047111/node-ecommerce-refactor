use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 1, message = "Payment method is required"))]
    pub payment_method: String,

    pub shipping_price: Decimal,

    #[validate(length(min = 1, message = "Shipping address street is required"))]
    pub shipping_address_street: String,

    #[validate(length(min = 1, message = "Shipping address city is required"))]
    pub shipping_address_city: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderStatusRequest {
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderQueryParams {
    #[serde(default = "default_page")]
    pub page: i64,

    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    10
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: Decimal,
    pub product_title: Option<String>,
    pub subtotal: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub payment_method: String,
    pub shipping_price: Decimal,
    pub total_price: Decimal,
    pub shipping_address_street: String,
    pub shipping_address_city: String,
    pub items: Vec<OrderItemResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OrdersListResponse {
    pub orders: Vec<OrderResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}
