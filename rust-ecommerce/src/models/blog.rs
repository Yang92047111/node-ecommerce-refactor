use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Blog {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub author_id: Uuid,
    pub category_id: Option<Uuid>,
    pub images: JsonValue,
    pub views_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Blog {
    pub fn new(
        title: String,
        description: Option<String>,
        content: String,
        author_id: Uuid,
        category_id: Option<Uuid>,
        images: Option<Vec<String>>,
    ) -> Self {
        let images_json = match images {
            Some(imgs) => serde_json::to_value(imgs).unwrap_or(JsonValue::Array(vec![])),
            None => JsonValue::Array(vec![]),
        };

        Self {
            id: Uuid::new_v4(),
            title,
            description,
            content,
            author_id,
            category_id,
            images: images_json,
            views_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
