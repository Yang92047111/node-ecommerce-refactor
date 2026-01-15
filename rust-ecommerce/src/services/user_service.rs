use crate::dto::user_dto::UserResponse;
use crate::errors::AppError;
use crate::repositories::user_repository::UserRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    /// Get all users (admin only)
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<UserResponse>, AppError> {
        let users = UserRepository::find_all(pool).await?;

        let user_responses: Vec<UserResponse> = users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id.to_string(),
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                mobile: user.mobile,
                role: user.role,
                is_blocked: user.is_blocked,
                created_at: user.created_at.to_rfc3339(),
                updated_at: user.updated_at.to_rfc3339(),
            })
            .collect();

        Ok(user_responses)
    }

    /// Get user by ID
    pub async fn get_user_by_id(pool: &PgPool, user_id: &Uuid) -> Result<UserResponse, AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserResponse {
            id: user.id.to_string(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            mobile: user.mobile,
            role: user.role,
            is_blocked: user.is_blocked,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        })
    }

    /// Update user
    pub async fn update_user(
        pool: &PgPool,
        user_id: &Uuid,
        first_name: &str,
        last_name: &str,
        mobile: &str,
    ) -> Result<UserResponse, AppError> {
        let user = UserRepository::update(pool, user_id, first_name, last_name, mobile).await?;

        Ok(UserResponse {
            id: user.id.to_string(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            mobile: user.mobile,
            role: user.role,
            is_blocked: user.is_blocked,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        })
    }

    /// Delete user (admin only)
    pub async fn delete_user(pool: &PgPool, user_id: &Uuid) -> Result<(), AppError> {
        UserRepository::delete(pool, user_id).await
    }

    /// Block user (admin only)
    pub async fn block_user(pool: &PgPool, user_id: &Uuid) -> Result<UserResponse, AppError> {
        let user = UserRepository::block_user(pool, user_id).await?;

        Ok(UserResponse {
            id: user.id.to_string(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            mobile: user.mobile,
            role: user.role,
            is_blocked: user.is_blocked,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        })
    }

    /// Unblock user (admin only)
    pub async fn unblock_user(pool: &PgPool, user_id: &Uuid) -> Result<UserResponse, AppError> {
        let user = UserRepository::unblock_user(pool, user_id).await?;

        Ok(UserResponse {
            id: user.id.to_string(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            mobile: user.mobile,
            role: user.role,
            is_blocked: user.is_blocked,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        })
    }
}
