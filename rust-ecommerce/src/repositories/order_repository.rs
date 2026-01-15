use crate::errors::AppError;
use crate::models::order::{Order, OrderItem};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub struct OrderRepository;

impl OrderRepository {
    // Order operations
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        payment_method: &str,
        shipping_price: Decimal,
        total_price: Decimal,
        shipping_address_street: &str,
        shipping_address_city: &str,
    ) -> Result<Order, AppError> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO orders (user_id, payment_method, shipping_price, total_price, shipping_address_street, shipping_address_city)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(payment_method)
        .bind(shipping_price)
        .bind(total_price)
        .bind(shipping_address_street)
        .bind(shipping_address_city)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create order: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(order)
    }

    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Order>, AppError> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            SELECT * FROM orders WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find order by id: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(order)
    }

    pub async fn find_by_user(
        pool: &PgPool,
        user_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, AppError> {
        let orders = sqlx::query_as::<_, Order>(
            r#"
            SELECT * FROM orders 
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find orders by user: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(orders)
    }

    pub async fn find_all(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, AppError> {
        let orders = sqlx::query_as::<_, Order>(
            r#"
            SELECT * FROM orders 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to find all orders: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(orders)
    }

    pub async fn count_by_user(pool: &PgPool, user_id: &Uuid) -> Result<i64, AppError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM orders WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to count user orders: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(count.0)
    }

    pub async fn count_all(pool: &PgPool) -> Result<i64, AppError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM orders
            "#,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to count all orders: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(count.0)
    }

    pub async fn update_status(
        pool: &PgPool,
        order_id: &Uuid,
        status: &str,
    ) -> Result<Order, AppError> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            UPDATE orders 
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(status)
        .bind(order_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update order status: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(order)
    }

    // Order item operations
    pub async fn add_item(
        pool: &PgPool,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
        price: Decimal,
    ) -> Result<OrderItem, AppError> {
        let order_item = sqlx::query_as::<_, OrderItem>(
            r#"
            INSERT INTO order_items (order_id, product_id, quantity, price)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(order_id)
        .bind(product_id)
        .bind(quantity)
        .bind(price)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to add order item: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(order_item)
    }

    pub async fn find_items_by_order(pool: &PgPool, order_id: &Uuid) -> Result<Vec<OrderItem>, AppError> {
        let items = sqlx::query_as::<_, OrderItem>(
            r#"
            SELECT * FROM order_items WHERE order_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(order_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch order items: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(items)
    }
}
