use crate::errors::AppError;
use crate::models::category::Category;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn create(pool: &PgPool, title: &str) -> Result<Category, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            INSERT INTO categories (title)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(title)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create category: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Category title already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(category)
    }

    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Category>, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            SELECT * FROM categories WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find category by id: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(category)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Category>, AppError> {
        let categories = sqlx::query_as::<_, Category>(
            r#"
            SELECT * FROM categories ORDER BY title ASC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch all categories: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(categories)
    }

    pub async fn update(
        pool: &PgPool,
        id: &Uuid,
        title: &str,
    ) -> Result<Option<Category>, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            UPDATE categories
            SET title = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update category: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Category title already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(category)
    }

    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM categories WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete category: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_by_title(
        pool: &PgPool,
        title: &str,
    ) -> Result<Option<Category>, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            SELECT * FROM categories WHERE title = $1
            "#,
        )
        .bind(title)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find category by title: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(category)
    }
}
