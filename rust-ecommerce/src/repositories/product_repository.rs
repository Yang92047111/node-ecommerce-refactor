use crate::errors::AppError;
use crate::models::product::{Product, Rating, Wishlist};
use rust_decimal::Decimal;
use sqlx::{PgPool, Error as SqlxError};
use uuid::Uuid;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn create(
        pool: &PgPool,
        title: &str,
        slug: &str,
        description: &str,
        price: Decimal,
        quantity: i32,
        brand: Option<&str>,
        category_id: Option<Uuid>,
        images: sqlx::types::JsonValue,
    ) -> Result<Product, AppError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO products (title, slug, description, price, quantity, brand, category_id, images)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(slug)
        .bind(description)
        .bind(price)
        .bind(quantity)
        .bind(brand)
        .bind(category_id)
        .bind(images)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create product: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Product slug already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(product)
    }

    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Product>, AppError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM products WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find product by id: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(product)
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Product>, AppError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM products WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find product by slug: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(product)
    }

    pub async fn find_all(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM products 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch all products: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(products)
    }

    pub async fn count_all(pool: &PgPool) -> Result<i64, AppError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM products
            "#,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to count products: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(count.0)
    }

    pub async fn search(
        pool: &PgPool,
        query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM products 
            WHERE to_tsvector('english', title || ' ' || description) @@ plainto_tsquery('english', $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to search products: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(products)
    }

    pub async fn update(
        pool: &PgPool,
        id: &Uuid,
        title: &str,
        slug: &str,
        description: &str,
        price: Decimal,
        quantity: i32,
        brand: Option<&str>,
        category_id: Option<Uuid>,
        discount: Decimal,
        images: sqlx::types::JsonValue,
    ) -> Result<Option<Product>, AppError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
            UPDATE products
            SET title = $1, slug = $2, description = $3, price = $4, 
                quantity = $5, brand = $6, category_id = $7, discount = $8,
                images = $9, updated_at = NOW()
            WHERE id = $10
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(slug)
        .bind(description)
        .bind(price)
        .bind(quantity)
        .bind(brand)
        .bind(category_id)
        .bind(discount)
        .bind(images)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update product: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Product slug already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(product)
    }

    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM products WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete product: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_by_category(
        pool: &PgPool,
        category_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM products 
            WHERE category_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(category_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find products by category: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(products)
    }

    pub async fn increment_sold(pool: &PgPool, id: &Uuid, amount: i32) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE products
            SET sold = sold + $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(amount)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to increment sold count: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    /// Update product quantity and sold count after order
    pub async fn update_quantity(
        pool: &PgPool,
        product_id: &Uuid,
        quantity_sold: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE products 
            SET quantity = quantity - $1,
                sold = sold + $1,
                updated_at = NOW()
            WHERE id = $2 AND quantity >= $1
            "#,
        )
        .bind(quantity_sold)
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update product quantity: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }
}

// Rating Repository
pub struct RatingRepository;

impl RatingRepository {
    pub async fn create(
        pool: &PgPool,
        product_id: &Uuid,
        user_id: &Uuid,
        rating: i32,
        comment: Option<&str>,
    ) -> Result<Rating, AppError> {
        let rating_record = sqlx::query_as::<_, Rating>(
            r#"
            INSERT INTO ratings (product_id, user_id, rating, comment)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(product_id)
        .bind(user_id)
        .bind(rating)
        .bind(comment)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create rating: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("You have already rated this product".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(rating_record)
    }

    pub async fn find_by_product(
        pool: &PgPool,
        product_id: &Uuid,
    ) -> Result<Vec<Rating>, AppError> {
        let ratings = sqlx::query_as::<_, Rating>(
            r#"
            SELECT * FROM ratings 
            WHERE product_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(product_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find ratings by product: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(ratings)
    }

    pub async fn calculate_average(pool: &PgPool, product_id: &Uuid) -> Result<Decimal, AppError> {
        let result: Option<(Option<Decimal>,)> = sqlx::query_as(
            r#"
            SELECT AVG(rating) FROM ratings WHERE product_id = $1
            "#,
        )
        .bind(product_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to calculate average rating: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(result
            .and_then(|r| r.0)
            .unwrap_or(Decimal::from(0)))
    }

    pub async fn update_product_rating(
        pool: &PgPool,
        product_id: &Uuid,
    ) -> Result<(), AppError> {
        let avg_rating = Self::calculate_average(pool, product_id).await?;

        sqlx::query(
            r#"
            UPDATE products
            SET total_ratings = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(avg_rating)
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update product rating: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }
}

// Wishlist Repository
pub struct WishlistRepository;

impl WishlistRepository {
    pub async fn add(pool: &PgPool, user_id: &Uuid, product_id: &Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO wishlist (user_id, product_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, product_id) DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to add to wishlist: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    pub async fn remove(
        pool: &PgPool,
        user_id: &Uuid,
        product_id: &Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM wishlist 
            WHERE user_id = $1 AND product_id = $2
            "#,
        )
        .bind(user_id)
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to remove from wishlist: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_user_wishlist(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT p.* FROM products p
            INNER JOIN wishlist w ON p.id = w.product_id
            WHERE w.user_id = $1
            ORDER BY w.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e: SqlxError| {
            log::error!("Failed to get user wishlist: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(products)
    }

    pub async fn is_in_wishlist(
        pool: &PgPool,
        user_id: &Uuid,
        product_id: &Uuid,
    ) -> Result<bool, AppError> {
        let result: Option<(bool,)> = sqlx::query_as(
            r#"
            SELECT EXISTS(SELECT 1 FROM wishlist WHERE user_id = $1 AND product_id = $2)
            "#,
        )
        .bind(user_id)
        .bind(product_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to check wishlist: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(result.map(|r| r.0).unwrap_or(false))
    }
}
