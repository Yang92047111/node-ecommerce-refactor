use crate::errors::AppError;
use crate::models::cart::{Cart, CartItem};
use sqlx::PgPool;
use uuid::Uuid;

pub struct CartRepository;

impl CartRepository {
    // Cart operations
    pub async fn create(pool: &PgPool, user_id: Uuid) -> Result<Cart, AppError> {
        let cart = sqlx::query_as::<_, Cart>(
            r#"
            INSERT INTO carts (user_id)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create cart: {}", e);
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("User already has a cart".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(cart)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: &Uuid) -> Result<Option<Cart>, AppError> {
        let cart = sqlx::query_as::<_, Cart>(
            r#"
            SELECT * FROM carts WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find cart by user_id: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(cart)
    }

    pub async fn find_or_create(pool: &PgPool, user_id: Uuid) -> Result<Cart, AppError> {
        // Try to find existing cart
        if let Some(cart) = Self::find_by_user_id(pool, &user_id).await? {
            return Ok(cart);
        }

        // Create new cart if not found
        Self::create(pool, user_id).await
    }

    pub async fn delete(pool: &PgPool, user_id: &Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM carts WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete cart: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    // Cart item operations
    pub async fn add_item(
        pool: &PgPool,
        cart_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<CartItem, AppError> {
        // Check if item already exists in cart
        if let Some(existing_item) = Self::find_item_by_product(pool, &cart_id, &product_id).await? {
            // Update quantity
            return Self::update_item_quantity(pool, &existing_item.id, existing_item.quantity + quantity).await;
        }

        // Add new item
        let cart_item = sqlx::query_as::<_, CartItem>(
            r#"
            INSERT INTO cart_items (cart_id, product_id, quantity)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(cart_id)
        .bind(product_id)
        .bind(quantity)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to add cart item: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(cart_item)
    }

    pub async fn find_item_by_id(pool: &PgPool, item_id: &Uuid) -> Result<Option<CartItem>, AppError> {
        let item = sqlx::query_as::<_, CartItem>(
            r#"
            SELECT * FROM cart_items WHERE id = $1
            "#,
        )
        .bind(item_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find cart item: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(item)
    }

    pub async fn find_item_by_product(
        pool: &PgPool,
        cart_id: &Uuid,
        product_id: &Uuid,
    ) -> Result<Option<CartItem>, AppError> {
        let item = sqlx::query_as::<_, CartItem>(
            r#"
            SELECT * FROM cart_items WHERE cart_id = $1 AND product_id = $2
            "#,
        )
        .bind(cart_id)
        .bind(product_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find cart item by product: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(item)
    }

    pub async fn find_items_by_cart(pool: &PgPool, cart_id: &Uuid) -> Result<Vec<CartItem>, AppError> {
        let items = sqlx::query_as::<_, CartItem>(
            r#"
            SELECT * FROM cart_items WHERE cart_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(cart_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch cart items: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(items)
    }

    pub async fn update_item_quantity(
        pool: &PgPool,
        item_id: &Uuid,
        quantity: i32,
    ) -> Result<CartItem, AppError> {
        let item = sqlx::query_as::<_, CartItem>(
            r#"
            UPDATE cart_items 
            SET quantity = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(quantity)
        .bind(item_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update cart item quantity: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(item)
    }

    pub async fn delete_item(pool: &PgPool, item_id: &Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM cart_items WHERE id = $1
            "#,
        )
        .bind(item_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete cart item: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    pub async fn clear_cart(pool: &PgPool, cart_id: &Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM cart_items WHERE cart_id = $1
            "#,
        )
        .bind(cart_id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to clear cart items: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }
}
