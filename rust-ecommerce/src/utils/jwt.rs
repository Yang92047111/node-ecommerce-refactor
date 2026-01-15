use crate::errors::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub role: String,
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// Get JWT secret from environment variable
fn get_jwt_secret() -> Result<String, AppError> {
    std::env::var("JWT_SECRET").map_err(|_| {
        AppError::InternalServerError("JWT_SECRET environment variable not set".to_string())
    })
}

/// Get JWT access token expiration time in seconds (default: 15 minutes)
fn get_access_token_expiration() -> i64 {
    std::env::var("JWT_ACCESS_EXPIRATION")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(900) // 15 minutes default
}

/// Get JWT refresh token expiration time in seconds (default: 7 days)
fn get_refresh_token_expiration() -> i64 {
    std::env::var("JWT_REFRESH_EXPIRATION")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(604800) // 7 days default
}

/// Generate an access token
pub fn generate_access_token(
    user_id: &Uuid,
    email: &str,
    role: &str,
) -> Result<String, AppError> {
    let secret = get_jwt_secret()?;
    let expiration = get_access_token_expiration();

    let now = Utc::now();
    let exp = (now + Duration::seconds(expiration)).timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp,
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        log::error!("Failed to generate access token: {}", e);
        AppError::InternalServerError("Failed to generate access token".to_string())
    })
}

/// Generate a refresh token
pub fn generate_refresh_token(
    user_id: &Uuid,
    email: &str,
    role: &str,
) -> Result<String, AppError> {
    let secret = get_jwt_secret()?;
    let expiration = get_refresh_token_expiration();

    let now = Utc::now();
    let exp = (now + Duration::seconds(expiration)).timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp,
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        log::error!("Failed to generate refresh token: {}", e);
        AppError::InternalServerError("Failed to generate refresh token".to_string())
    })
}

/// Generate both access and refresh tokens
pub fn generate_token_pair(
    user_id: &Uuid,
    email: &str,
    role: &str,
) -> Result<TokenPair, AppError> {
    let access_token = generate_access_token(user_id, email, role)?;
    let refresh_token = generate_refresh_token(user_id, email, role)?;

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

/// Verify and decode a JWT token
pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = get_jwt_secret()?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| {
        log::warn!("Failed to verify token: {}", e);
        AppError::Unauthorized("Invalid or expired token".to_string())
    })
}

/// Extract user ID from token
pub fn get_user_id_from_token(token: &str) -> Result<Uuid, AppError> {
    let claims = verify_token(token)?;
    Uuid::parse_str(&claims.sub).map_err(|e| {
        log::error!("Failed to parse user ID from token: {}", e);
        AppError::Unauthorized("Invalid token".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_env() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test_secret_key_for_testing_only");
            std::env::set_var("JWT_ACCESS_EXPIRATION", "900");
            std::env::set_var("JWT_REFRESH_EXPIRATION", "604800");
        }
    }

    #[test]
    fn test_generate_and_verify_token() {
        setup_test_env();

        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let role = "user";

        let token = generate_access_token(&user_id, email, role).unwrap();
        let claims = verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_token_pair_generation() {
        setup_test_env();

        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let role = "admin";

        let token_pair = generate_token_pair(&user_id, email, role).unwrap();

        // Verify both tokens
        let access_claims = verify_token(&token_pair.access_token).unwrap();
        let refresh_claims = verify_token(&token_pair.refresh_token).unwrap();

        assert_eq!(access_claims.sub, user_id.to_string());
        assert_eq!(refresh_claims.sub, user_id.to_string());
    }

    #[test]
    fn test_get_user_id_from_token() {
        setup_test_env();

        let user_id = Uuid::new_v4();
        let token = generate_access_token(&user_id, "test@example.com", "user").unwrap();

        let extracted_id = get_user_id_from_token(&token).unwrap();
        assert_eq!(extracted_id, user_id);
    }

    #[test]
    fn test_invalid_token() {
        setup_test_env();

        let result = verify_token("invalid_token");
        assert!(result.is_err());
    }
}
