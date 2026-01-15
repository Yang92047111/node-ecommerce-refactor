use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartItem {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Cart {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl CartItem {
    pub fn new(cart_id: Uuid, product_id: Uuid, quantity: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            cart_id,
            product_id,
            quantity,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
