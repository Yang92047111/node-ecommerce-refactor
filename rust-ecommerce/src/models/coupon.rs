use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Coupon {
    pub id: Uuid,
    pub code: String,
    pub discount_percentage: f64,
    pub expiry_date: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Coupon {
    pub fn new(code: String, discount_percentage: f64, expiry_date: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            code: code.to_uppercase(),
            discount_percentage,
            expiry_date,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Check if coupon is valid (active and not expired)
    pub fn is_valid(&self) -> bool {
        self.is_active && self.expiry_date > Utc::now()
    }

    /// Calculate discount amount based on the original price
    pub fn calculate_discount(&self, original_price: f64) -> f64 {
        if !self.is_valid() {
            return 0.0;
        }
        (original_price * self.discount_percentage / 100.0).round() / 100.0
    }
}
