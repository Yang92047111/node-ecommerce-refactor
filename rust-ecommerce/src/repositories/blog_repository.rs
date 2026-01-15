use crate::errors::AppError;
use crate::models::blog::Blog;
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use uuid::Uuid;

pub struct BlogRepository;

impl BlogRepository {
    /// Create a new blog post
    pub async fn create(
        pool: &PgPool,
        title: &str,
        description: Option<&str>,
        content: &str,
        author_id: &Uuid,
        category_id: Option<&Uuid>,
        images: Option<Vec<String>>,
    ) -> Result<Blog, AppError> {
        let images_json = match images {
            Some(imgs) => serde_json::to_value(imgs).unwrap_or(JsonValue::Array(vec![])),
            None => JsonValue::Array(vec![]),
        };

        let blog = sqlx::query_as::<_, Blog>(
            r#"
            INSERT INTO blogs (title, description, content, author_id, category_id, images)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(content)
        .bind(author_id)
        .bind(category_id)
        .bind(images_json)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create blog: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(blog)
    }

    /// Find blog by ID
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Blog>, AppError> {
        let blog = sqlx::query_as::<_, Blog>(
            r#"
            SELECT * FROM blogs WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find blog by id: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(blog)
    }

    /// Find all blogs with pagination
    pub async fn find_all(
        pool: &PgPool,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Blog>, AppError> {
        let offset = (page - 1) * page_size;

        let blogs = sqlx::query_as::<_, Blog>(
            r#"
            SELECT * FROM blogs 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch blogs: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(blogs)
    }

    /// Count total blogs
    pub async fn count(pool: &PgPool) -> Result<i64, AppError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM blogs
            "#,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to count blogs: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(count.0)
    }

    /// Find blogs by author
    pub async fn find_by_author(
        pool: &PgPool,
        author_id: &Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Blog>, AppError> {
        let offset = (page - 1) * page_size;

        let blogs = sqlx::query_as::<_, Blog>(
            r#"
            SELECT * FROM blogs 
            WHERE author_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(author_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch blogs by author: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(blogs)
    }

    /// Find blogs by category
    pub async fn find_by_category(
        pool: &PgPool,
        category_id: &Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Blog>, AppError> {
        let offset = (page - 1) * page_size;

        let blogs = sqlx::query_as::<_, Blog>(
            r#"
            SELECT * FROM blogs 
            WHERE category_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(category_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch blogs by category: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(blogs)
    }

    /// Update blog
    pub async fn update(
        pool: &PgPool,
        id: &Uuid,
        title: Option<&str>,
        description: Option<&str>,
        content: Option<&str>,
        category_id: Option<&Uuid>,
        images: Option<Vec<String>>,
    ) -> Result<Option<Blog>, AppError> {
        // First fetch the existing blog
        let existing = Self::find_by_id(pool, id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let existing = existing.unwrap();
        let title = title.unwrap_or(&existing.title);
        let description_val = description.or(existing.description.as_deref());
        let content = content.unwrap_or(&existing.content);
        let category_id_val = category_id.or(existing.category_id.as_ref());

        let images_json = match images {
            Some(imgs) => serde_json::to_value(imgs).unwrap_or(existing.images.clone()),
            None => existing.images.clone(),
        };

        let blog = sqlx::query_as::<_, Blog>(
            r#"
            UPDATE blogs
            SET title = $1, description = $2, content = $3, 
                category_id = $4, images = $5, updated_at = NOW()
            WHERE id = $6
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(description_val)
        .bind(content)
        .bind(category_id_val)
        .bind(images_json)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update blog: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(blog)
    }

    /// Delete blog
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM blogs WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete blog: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(result.rows_affected() > 0)
    }

    /// Increment views count
    pub async fn increment_views(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE blogs
            SET views_count = views_count + 1
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to increment blog views: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }
}
