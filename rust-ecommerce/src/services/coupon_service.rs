use crate::dto::coupon_dto::{
    CouponResponse, CouponValidationResponse, CouponsListResponse, CreateCouponRequest,
    UpdateCouponRequest,
};
use crate::errors::AppError;
use crate::models::coupon::Coupon;
use crate::repositories::coupon_repository::CouponRepository;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CouponService;

impl CouponService {
    pub async fn create_coupon(
        pool: &PgPool,
        request: CreateCouponRequest,
    ) -> Result<CouponResponse, AppError> {
        // Validate expiry date is in the future
        if request.expiry_date <= Utc::now() {
            return Err(AppError::ValidationError(
                "Expiry date must be in the future".to_string(),
            ));
        }

        // Check if coupon with same code already exists
        if let Some(_) = CouponRepository::find_by_code(pool, &request.code).await? {
            return Err(AppError::Conflict(
                "Coupon with this code already exists".to_string(),
            ));
        }

        let coupon = CouponRepository::create(
            pool,
            &request.code,
            request.discount_percentage,
            request.expiry_date,
        )
        .await?;

        Ok(Self::to_response(&coupon))
    }

    pub async fn get_coupon(pool: &PgPool, id: &Uuid) -> Result<CouponResponse, AppError> {
        let coupon = CouponRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Coupon not found".to_string()))?;

        Ok(Self::to_response(&coupon))
    }

    pub async fn get_all_coupons(pool: &PgPool) -> Result<CouponsListResponse, AppError> {
        let coupons = CouponRepository::find_all(pool).await?;

        let coupons_response: Vec<CouponResponse> =
            coupons.iter().map(|c| Self::to_response(c)).collect();

        Ok(CouponsListResponse {
            coupons: coupons_response,
        })
    }

    pub async fn get_active_coupons(pool: &PgPool) -> Result<CouponsListResponse, AppError> {
        let coupons = CouponRepository::find_active(pool).await?;

        let coupons_response: Vec<CouponResponse> =
            coupons.iter().map(|c| Self::to_response(c)).collect();

        Ok(CouponsListResponse {
            coupons: coupons_response,
        })
    }

    pub async fn validate_coupon(
        pool: &PgPool,
        code: &str,
    ) -> Result<CouponValidationResponse, AppError> {
        let coupon = CouponRepository::find_by_code(pool, code).await?;

        match coupon {
            None => Ok(CouponValidationResponse {
                valid: false,
                message: "Coupon not found".to_string(),
                coupon: None,
            }),
            Some(coupon) => {
                if !coupon.is_active {
                    Ok(CouponValidationResponse {
                        valid: false,
                        message: "Coupon is not active".to_string(),
                        coupon: Some(Self::to_response(&coupon)),
                    })
                } else if coupon.expiry_date <= Utc::now() {
                    Ok(CouponValidationResponse {
                        valid: false,
                        message: "Coupon has expired".to_string(),
                        coupon: Some(Self::to_response(&coupon)),
                    })
                } else {
                    Ok(CouponValidationResponse {
                        valid: true,
                        message: "Coupon is valid".to_string(),
                        coupon: Some(Self::to_response(&coupon)),
                    })
                }
            }
        }
    }

    pub async fn update_coupon(
        pool: &PgPool,
        id: &Uuid,
        request: UpdateCouponRequest,
    ) -> Result<CouponResponse, AppError> {
        // Check if coupon exists
        CouponRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Coupon not found".to_string()))?;

        // Validate expiry date is in the future if provided
        if let Some(expiry_date) = request.expiry_date {
            if expiry_date <= Utc::now() {
                return Err(AppError::ValidationError(
                    "Expiry date must be in the future".to_string(),
                ));
            }
        }

        // Check if another coupon with same code exists
        if let Some(code) = &request.code {
            if let Some(existing) = CouponRepository::find_by_code(pool, code).await? {
                if &existing.id != id {
                    return Err(AppError::Conflict(
                        "Coupon with this code already exists".to_string(),
                    ));
                }
            }
        }

        let coupon = CouponRepository::update(
            pool,
            id,
            request.code.as_deref(),
            request.discount_percentage,
            request.expiry_date,
            request.is_active,
        )
        .await?
        .ok_or_else(|| AppError::NotFound("Coupon not found".to_string()))?;

        Ok(Self::to_response(&coupon))
    }

    pub async fn delete_coupon(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
        let deleted = CouponRepository::delete(pool, id).await?;

        if !deleted {
            return Err(AppError::NotFound("Coupon not found".to_string()));
        }

        Ok(())
    }

    fn to_response(coupon: &Coupon) -> CouponResponse {
        CouponResponse {
            id: coupon.id,
            code: coupon.code.clone(),
            discount_percentage: coupon.discount_percentage,
            expiry_date: coupon.expiry_date,
            is_active: coupon.is_active,
            created_at: coupon.created_at,
            updated_at: coupon.updated_at,
        }
    }
}
