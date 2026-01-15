use crate::errors::AppError;
use crate::utils::jwt;
use actix_web::{dev::ServiceRequest, web, Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{ready, Ready};
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

/// Authenticated user information extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Try to get user from request extensions (set by middleware)
        if let Some(user) = req.extensions().get::<AuthenticatedUser>() {
            return ready(Ok(user.clone()));
        }

        // If not found in extensions, try to extract from Authorization header
        match extract_user_from_request(req) {
            Ok(user) => ready(Ok(user)),
            Err(e) => ready(Err(actix_web::error::ErrorUnauthorized(e))),
        }
    }
}

/// Extract user from Authorization header
fn extract_user_from_request(req: &HttpRequest) -> Result<AuthenticatedUser, AppError> {
    // Get Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

    // Extract token from "Bearer <token>"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid Authorization format".to_string()))?;

    // Verify token and extract claims
    let claims = jwt::verify_token(token)?;

    // Parse user ID
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        AppError::Unauthorized("Invalid user ID in token".to_string())
    })?;

    Ok(AuthenticatedUser {
        id: user_id,
        email: claims.email,
        role: claims.role,
    })
}

/// Middleware function to authenticate requests
pub async fn auth_middleware(
    req: ServiceRequest,
    next: actix_web::middleware::Next<actix_web::body::BoxBody>,
) -> Result<actix_web::dev::ServiceResponse<actix_web::body::BoxBody>, Error> {
    // Extract user from request
    match extract_user_from_request(req.request()) {
        Ok(user) => {
            // Insert user into request extensions
            req.extensions_mut().insert(user);

            // Continue to next middleware/handler
            next.call(req).await
        }
        Err(e) => {
            // Return unauthorized error
            Err(actix_web::error::ErrorUnauthorized(e))
        }
    }
}

/// Helper function to check if user is admin
pub fn require_admin(user: &AuthenticatedUser) -> Result<(), AppError> {
    if user.role != "admin" {
        return Err(AppError::Forbidden(
            "Admin access required".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[test]
    fn test_authenticated_user_creation() {
        let user = AuthenticatedUser {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: "user".to_string(),
        };

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.role, "user");
    }

    #[test]
    fn test_require_admin() {
        let admin_user = AuthenticatedUser {
            id: Uuid::new_v4(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
        };

        let normal_user = AuthenticatedUser {
            id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            role: "user".to_string(),
        };

        assert!(require_admin(&admin_user).is_ok());
        assert!(require_admin(&normal_user).is_err());
    }
}
