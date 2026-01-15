use regex::Regex;
use uuid::Uuid;

/// Generate a URL-friendly slug from a title
pub fn generate_slug(title: &str) -> String {
    // Convert to lowercase
    let mut slug = title.to_lowercase();

    // Replace spaces and special characters with hyphens
    let re = Regex::new(r"[^\w\s-]").unwrap();
    slug = re.replace_all(&slug, "").to_string();

    let re = Regex::new(r"[\s_]+").unwrap();
    slug = re.replace_all(&slug, "-").to_string();

    // Remove leading/trailing hyphens
    slug = slug.trim_matches('-').to_string();

    // Add UUID suffix to ensure uniqueness
    format!("{}-{}", slug, Uuid::new_v4().to_string().split('-').next().unwrap())
}

/// Generate a slug without UUID suffix (for search/display purposes)
pub fn generate_simple_slug(title: &str) -> String {
    // Convert to lowercase
    let mut slug = title.to_lowercase();

    // Replace spaces and special characters with hyphens
    let re = Regex::new(r"[^\w\s-]").unwrap();
    slug = re.replace_all(&slug, "").to_string();

    let re = Regex::new(r"[\s_]+").unwrap();
    slug = re.replace_all(&slug, "-").to_string();

    // Remove leading/trailing hyphens
    slug.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        let title = "My Awesome Product!";
        let slug = generate_slug(title);
        assert!(slug.starts_with("my-awesome-product-"));
        assert!(slug.len() > "my-awesome-product-".len());
    }

    #[test]
    fn test_generate_simple_slug() {
        let title = "My Awesome Product!";
        let slug = generate_simple_slug(title);
        assert_eq!(slug, "my-awesome-product");
    }

    #[test]
    fn test_slug_special_characters() {
        let title = "Product @ $100 & More!!!";
        let slug = generate_simple_slug(title);
        assert_eq!(slug, "product-100-more");
    }

    #[test]
    fn test_slug_multiple_spaces() {
        let title = "Product   With    Multiple   Spaces";
        let slug = generate_simple_slug(title);
        assert_eq!(slug, "product-with-multiple-spaces");
    }
}
