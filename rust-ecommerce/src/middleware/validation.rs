use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage,
};
use std::future::{ready, Ready};

/// Validation middleware for request sanitization and validation
/// This middleware performs basic input validation and sanitization
/// to prevent common security issues like XSS, SQL injection, etc.

/// Sanitize string input by removing potentially dangerous characters
pub fn sanitize_input(input: &str) -> String {
    // Remove common XSS patterns
    input
        .replace("<script>", "")
        .replace("</script>", "")
        .replace("<iframe>", "")
        .replace("</iframe>", "")
        .replace("javascript:", "")
        .replace("onerror=", "")
        .replace("onclick=", "")
        .trim()
        .to_string()
}

/// Validate that a string doesn't contain SQL injection patterns
pub fn is_sql_safe(input: &str) -> bool {
    let dangerous_patterns = [
        "'; DROP TABLE",
        "'; DELETE FROM",
        "'; UPDATE",
        "'; INSERT INTO",
        "' OR '1'='1",
        "' OR 1=1",
        "UNION SELECT",
        "1' OR '1' = '1",
    ];

    let input_upper = input.to_uppercase();
    !dangerous_patterns
        .iter()
        .any(|pattern| input_upper.contains(&pattern.to_uppercase()))
}

/// Validate email format
pub fn is_valid_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();
    
    email_regex.is_match(email)
}

/// Validate UUID format
pub fn is_valid_uuid(uuid_str: &str) -> bool {
    uuid::Uuid::parse_str(uuid_str).is_ok()
}

/// Validate that input doesn't exceed maximum length
pub fn is_valid_length(input: &str, max_length: usize) -> bool {
    input.len() <= max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_input() {
        let input = "<script>alert('xss')</script>Hello";
        let sanitized = sanitize_input(input);
        assert!(!sanitized.contains("<script>"));
        assert!(sanitized.contains("Hello"));
    }

    #[test]
    fn test_is_sql_safe() {
        assert!(is_sql_safe("normal input"));
        assert!(!is_sql_safe("'; DROP TABLE users--"));
        assert!(!is_sql_safe("1' OR '1' = '1"));
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user.name+tag@example.co.uk"));
        assert!(!is_valid_email("invalid.email"));
        assert!(!is_valid_email("@example.com"));
    }

    #[test]
    fn test_is_valid_uuid() {
        assert!(is_valid_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!is_valid_uuid("not-a-uuid"));
        assert!(!is_valid_uuid(""));
    }

    #[test]
    fn test_is_valid_length() {
        assert!(is_valid_length("short", 10));
        assert!(!is_valid_length("this is too long", 10));
    }
}
