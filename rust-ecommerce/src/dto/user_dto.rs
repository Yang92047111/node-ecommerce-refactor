use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,

    #[validate(length(min = 10, max = 20, message = "Mobile must be between 10-20 characters"))]
    pub mobile: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile: String,
    pub role: String,
    pub is_blocked: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct UsersListResponse {
    pub users: Vec<UserResponse>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
