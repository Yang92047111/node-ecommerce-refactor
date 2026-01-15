use crate::dto::auth_dto::{
    AuthResponse, ForgotPasswordResponse, RefreshTokenResponse, UserResponse,
};
use crate::errors::AppError;
use crate::models::user::{User, UserRole};
use crate::repositories::user_repository::UserRepository;
use crate::utils::{jwt, password};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    /// Register a new user
    pub async fn register(
        pool: &PgPool,
        first_name: &str,
        last_name: &str,
        email: &str,
        mobile: &str,
        password_str: &str,
    ) -> Result<AuthResponse, AppError> {
        // Check if user already exists
        if let Some(_) = UserRepository::find_by_email(pool, email).await? {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Hash the password
        let password_hash = password::hash_password(password_str)?;

        // Create the user with default role "user"
        let user = UserRepository::create(
            pool,
            first_name,
            last_name,
            email,
            mobile,
            &password_hash,
            UserRole::User.as_str(),
        )
        .await?;

        // Generate tokens
        let token_pair = jwt::generate_token_pair(&user.id, &user.email, &user.role)?;

        Ok(AuthResponse {
            user: UserResponse {
                id: user.id.to_string(),
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                mobile: user.mobile,
                role: user.role,
                is_blocked: user.is_blocked,
            },
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
        })
    }

    /// Login user
    pub async fn login(
        pool: &PgPool,
        email: &str,
        password_str: &str,
    ) -> Result<AuthResponse, AppError> {
        // Find user by email
        let user = UserRepository::find_by_email(pool, email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        // Check if user is blocked
        if user.is_blocked {
            return Err(AppError::Forbidden("User account is blocked".to_string()));
        }

        // Verify password
        if !password::verify_password(password_str, &user.password_hash)? {
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }

        // Generate tokens
        let token_pair = jwt::generate_token_pair(&user.id, &user.email, &user.role)?;

        Ok(AuthResponse {
            user: UserResponse {
                id: user.id.to_string(),
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                mobile: user.mobile,
                role: user.role,
                is_blocked: user.is_blocked,
            },
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
        })
    }

    /// Refresh access token
    pub async fn refresh_token(
        pool: &PgPool,
        refresh_token: &str,
    ) -> Result<RefreshTokenResponse, AppError> {
        // Verify refresh token
        let claims = jwt::verify_token(refresh_token)?;

        // Parse user ID from claims
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            AppError::Unauthorized("Invalid token".to_string())
        })?;

        // Fetch user to ensure they still exist and are not blocked
        let user = UserRepository::find_by_id(pool, &user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        if user.is_blocked {
            return Err(AppError::Forbidden("User account is blocked".to_string()));
        }

        // Generate new token pair
        let token_pair = jwt::generate_token_pair(&user.id, &user.email, &user.role)?;

        Ok(RefreshTokenResponse {
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
        })
    }

    /// Initiate password reset
    pub async fn forgot_password(
        pool: &PgPool,
        email: &str,
    ) -> Result<ForgotPasswordResponse, AppError> {
        // Find user by email
        let user = UserRepository::find_by_email(pool, email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Generate reset token (in production, use a random token)
        let reset_token = Uuid::new_v4().to_string();

        // Set token expiration (1 hour from now)
        let expires = Utc::now() + Duration::hours(1);

        // Save reset token to database
        UserRepository::set_password_reset_token(pool, &user.id, &reset_token, expires).await?;

        // In production, send this via email instead of returning it
        Ok(ForgotPasswordResponse {
            message: "Password reset token generated".to_string(),
            reset_token,
        })
    }

    /// Reset password with token
    pub async fn reset_password(
        pool: &PgPool,
        token: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        // Find user by reset token
        let user = UserRepository::find_by_reset_token(pool, token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired reset token".to_string()))?;

        // Hash new password
        let password_hash = password::hash_password(new_password)?;

        // Update password
        UserRepository::update_password(pool, &user.id, &password_hash).await?;

        // Clear reset token
        UserRepository::clear_password_reset_token(pool, &user.id).await?;

        Ok(())
    }

    /// Verify if a user is authenticated (helper method)
    pub async fn verify_user(pool: &PgPool, user_id: &Uuid) -> Result<User, AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        if user.is_blocked {
            return Err(AppError::Forbidden("User account is blocked".to_string()));
        }

        Ok(user)
    }
}
