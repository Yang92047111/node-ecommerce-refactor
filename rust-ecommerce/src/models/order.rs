use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "PascalCase")]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Processing => write!(f, "Processing"),
            OrderStatus::Shipped => write!(f, "Shipped"),
            OrderStatus::Delivered => write!(f, "Delivered"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String, // We'll convert to OrderStatus enum in handlers
    pub payment_method: String,
    pub shipping_price: Decimal,
    pub total_price: Decimal,
    pub shipping_address_street: String,
    pub shipping_address_city: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
}

impl Order {
    pub fn new(
        user_id: Uuid,
        payment_method: String,
        shipping_price: Decimal,
        total_price: Decimal,
        shipping_address_street: String,
        shipping_address_city: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            status: OrderStatus::Pending.to_string(),
            payment_method,
            shipping_price,
            total_price,
            shipping_address_street,
            shipping_address_city,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl OrderItem {
    pub fn new(order_id: Uuid, product_id: Uuid, quantity: i32, price: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            product_id,
            quantity,
            price,
            created_at: Utc::now(),
        }
    }
}
