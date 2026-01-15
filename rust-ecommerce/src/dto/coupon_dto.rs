use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCouponRequest {
    #[validate(length(min = 1, max = 50, message = "Code must be between 1 and 50 characters"))]
    pub code: String,
    #[validate(range(min = 0.01, max = 100.0, message = "Discount percentage must be between 0.01 and 100"))]
    pub discount_percentage: f64,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCouponRequest {
    #[validate(length(min = 1, max = 50, message = "Code must be between 1 and 50 characters"))]
    pub code: Option<String>,
    #[validate(range(min = 0.01, max = 100.0, message = "Discount percentage must be between 0.01 and 100"))]
    pub discount_percentage: Option<f64>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ValidateCouponRequest {
    #[validate(length(min = 1, max = 50, message = "Code must be between 1 and 50 characters"))]
    pub code: String,
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponResponse {
    pub id: Uuid,
    pub code: String,
    pub discount_percentage: f64,
    pub expiry_date: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CouponsListResponse {
    pub coupons: Vec<CouponResponse>,
}

#[derive(Debug, Serialize)]
pub struct CouponValidationResponse {
    pub valid: bool,
    pub message: String,
    pub coupon: Option<CouponResponse>,
}
