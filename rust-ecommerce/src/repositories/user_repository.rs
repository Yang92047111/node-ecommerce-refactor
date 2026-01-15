use crate::errors::AppError;
use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        first_name: &str,
        last_name: &str,
        email: &str,
        mobile: &str,
        password_hash: &str,
        role: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (first_name, last_name, email, mobile, password_hash, role)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(mobile)
        .bind(password_hash)
        .bind(role)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create user: {}", e);
            if e.to_string().contains("duplicate key") {
                if e.to_string().contains("email") {
                    AppError::Conflict("Email already exists".to_string())
                } else if e.to_string().contains("mobile") {
                    AppError::Conflict("Mobile number already exists".to_string())
                } else {
                    AppError::Conflict("User already exists".to_string())
                }
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &PgPool, user_id: &Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find user by id: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find user by email: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch all users: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(users)
    }

    pub async fn update(
        pool: &PgPool,
        user_id: &Uuid,
        first_name: &str,
        last_name: &str,
        mobile: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET first_name = $2, last_name = $3, mobile = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(first_name)
        .bind(last_name)
        .bind(mobile)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update user: {}", e);
            if e.to_string().contains("not found") {
                AppError::NotFound("User not found".to_string())
            } else if e.to_string().contains("duplicate key") {
                AppError::Conflict("Mobile number already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, user_id: &Uuid) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete user: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    pub async fn update_password(
        pool: &PgPool,
        user_id: &Uuid,
        new_password_hash: &str,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET password_hash = $2, password_changed_at = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .bind(new_password_hash)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update password: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    pub async fn set_password_reset_token(
        pool: &PgPool,
        user_id: &Uuid,
        token: &str,
        expires: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET password_reset_token = $2, password_reset_expires = $3, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .bind(token)
        .bind(expires)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to set password reset token: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    pub async fn find_by_reset_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users 
            WHERE password_reset_token = $1 
            AND password_reset_expires > NOW()
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find user by reset token: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    pub async fn clear_password_reset_token(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET password_reset_token = NULL, password_reset_expires = NULL, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to clear password reset token: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    pub async fn block_user(pool: &PgPool, user_id: &Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET is_blocked = TRUE, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to block user: {}", e);
            if e.to_string().contains("not found") {
                AppError::NotFound("User not found".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(user)
    }

    pub async fn unblock_user(pool: &PgPool, user_id: &Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET is_blocked = FALSE, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to unblock user: {}", e);
            if e.to_string().contains("not found") {
                AppError::NotFound("User not found".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(user)
    }
}
