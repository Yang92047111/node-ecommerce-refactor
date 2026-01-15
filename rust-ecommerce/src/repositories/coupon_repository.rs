use crate::errors::AppError;
use crate::models::coupon::Coupon;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct CouponRepository;

impl CouponRepository {
    /// Create a new coupon
    pub async fn create(
        pool: &PgPool,
        code: &str,
        discount_percentage: f64,
        expiry_date: DateTime<Utc>,
    ) -> Result<Coupon, AppError> {
        let coupon = sqlx::query_as::<_, Coupon>(
            r#"
            INSERT INTO coupons (code, discount_percentage, expiry_date)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(code.to_uppercase())
        .bind(discount_percentage)
        .bind(expiry_date)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create coupon: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Coupon code already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(coupon)
    }

    /// Find coupon by ID
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Coupon>, AppError> {
        let coupon = sqlx::query_as::<_, Coupon>(
            r#"
            SELECT * FROM coupons WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find coupon by id: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(coupon)
    }

    /// Find coupon by code
    pub async fn find_by_code(pool: &PgPool, code: &str) -> Result<Option<Coupon>, AppError> {
        let coupon = sqlx::query_as::<_, Coupon>(
            r#"
            SELECT * FROM coupons WHERE code = $1
            "#,
        )
        .bind(code.to_uppercase())
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find coupon by code: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(coupon)
    }

    /// Find all coupons
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Coupon>, AppError> {
        let coupons = sqlx::query_as::<_, Coupon>(
            r#"
            SELECT * FROM coupons ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch all coupons: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(coupons)
    }

    /// Update coupon
    pub async fn update(
        pool: &PgPool,
        id: &Uuid,
        code: Option<&str>,
        discount_percentage: Option<f64>,
        expiry_date: Option<DateTime<Utc>>,
        is_active: Option<bool>,
    ) -> Result<Option<Coupon>, AppError> {
        // First fetch the existing coupon
        let existing = Self::find_by_id(pool, id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let existing = existing.unwrap();
        let code = code.unwrap_or(&existing.code);
        let discount_percentage = discount_percentage.unwrap_or(existing.discount_percentage);
        let expiry_date = expiry_date.unwrap_or(existing.expiry_date);
        let is_active = is_active.unwrap_or(existing.is_active);

        let coupon = sqlx::query_as::<_, Coupon>(
            r#"
            UPDATE coupons
            SET code = $1, discount_percentage = $2, expiry_date = $3, 
                is_active = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(code.to_uppercase())
        .bind(discount_percentage)
        .bind(expiry_date)
        .bind(is_active)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update coupon: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Coupon code already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(coupon)
    }

    /// Delete coupon
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM coupons WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete coupon: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(result.rows_affected() > 0)
    }

    /// Deactivate coupon
    pub async fn deactivate(pool: &PgPool, id: &Uuid) -> Result<Option<Coupon>, AppError> {
        let coupon = sqlx::query_as::<_, Coupon>(
            r#"
            UPDATE coupons
            SET is_active = false, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to deactivate coupon: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(coupon)
    }

    /// Get active coupons
    pub async fn find_active(pool: &PgPool) -> Result<Vec<Coupon>, AppError> {
        let coupons = sqlx::query_as::<_, Coupon>(
            r#"
            SELECT * FROM coupons 
            WHERE is_active = true AND expiry_date > NOW()
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch active coupons: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(coupons)
    }
}
