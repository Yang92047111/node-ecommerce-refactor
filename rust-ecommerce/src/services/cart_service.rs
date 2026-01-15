use crate::dto::cart_dto::{AddCartItemRequest, CartItemResponse, CartResponse, UpdateCartItemRequest};
use crate::errors::AppError;
use crate::repositories::cart_repository::CartRepository;
use crate::repositories::product_repository::ProductRepository;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CartService;

impl CartService {
    /// Get user's cart with all items and product details
    pub async fn get_cart(pool: &PgPool, user_id: &Uuid) -> Result<CartResponse, AppError> {
        // Find or create cart for user
        let cart = CartRepository::find_or_create(pool, *user_id).await?;

        // Get all cart items
        let items = CartRepository::find_items_by_cart(pool, &cart.id).await?;

        // Enrich items with product details
        let mut enriched_items = Vec::new();
        let mut subtotal = Decimal::ZERO;

        for item in items {
            // Get product details
            let product = ProductRepository::find_by_id(pool, &item.product_id).await?;

            if let Some(product) = product {
                let item_subtotal = product.price * Decimal::from(item.quantity);
                subtotal += item_subtotal;

                // Get first image if available
                let product_image = if let Some(images) = product.images.as_array() {
                    images.first().and_then(|v| v.as_str()).map(|s| s.to_string())
                } else {
                    None
                };

                enriched_items.push(CartItemResponse {
                    id: item.id,
                    cart_id: item.cart_id,
                    product_id: item.product_id,
                    quantity: item.quantity,
                    product_title: Some(product.title),
                    product_price: Some(product.price),
                    product_image,
                    subtotal: Some(item_subtotal),
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                });
            }
        }

        let total_items = enriched_items.iter().map(|item| item.quantity).sum();

        Ok(CartResponse {
            id: cart.id,
            user_id: cart.user_id,
            items: enriched_items,
            total_items,
            subtotal,
            created_at: cart.created_at,
            updated_at: cart.updated_at,
        })
    }

    /// Add item to cart
    pub async fn add_item(
        pool: &PgPool,
        user_id: &Uuid,
        req: AddCartItemRequest,
    ) -> Result<CartResponse, AppError> {
        // Verify product exists
        let product = ProductRepository::find_by_id(pool, &req.product_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        // Check if product has enough quantity
        if product.quantity < req.quantity {
            return Err(AppError::BadRequest(
                format!("Insufficient stock. Only {} available", product.quantity),
            ));
        }

        // Find or create cart
        let cart = CartRepository::find_or_create(pool, *user_id).await?;

        // Add item to cart
        CartRepository::add_item(pool, cart.id, req.product_id, req.quantity).await?;

        // Return updated cart
        Self::get_cart(pool, user_id).await
    }

    /// Update cart item quantity
    pub async fn update_item(
        pool: &PgPool,
        user_id: &Uuid,
        item_id: &Uuid,
        req: UpdateCartItemRequest,
    ) -> Result<CartResponse, AppError> {
        // Find cart item
        let cart_item = CartRepository::find_item_by_id(pool, item_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cart item not found".to_string()))?;

        // Verify the cart belongs to the user
        let cart = CartRepository::find_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cart not found".to_string()))?;

        if cart_item.cart_id != cart.id {
            return Err(AppError::Forbidden(
                "You don't have permission to modify this cart item".to_string(),
            ));
        }

        // Verify product has enough quantity
        let product = ProductRepository::find_by_id(pool, &cart_item.product_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        if product.quantity < req.quantity {
            return Err(AppError::BadRequest(
                format!("Insufficient stock. Only {} available", product.quantity),
            ));
        }

        // Update quantity
        CartRepository::update_item_quantity(pool, item_id, req.quantity).await?;

        // Return updated cart
        Self::get_cart(pool, user_id).await
    }

    /// Remove item from cart
    pub async fn remove_item(
        pool: &PgPool,
        user_id: &Uuid,
        item_id: &Uuid,
    ) -> Result<CartResponse, AppError> {
        // Find cart item
        let cart_item = CartRepository::find_item_by_id(pool, item_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cart item not found".to_string()))?;

        // Verify the cart belongs to the user
        let cart = CartRepository::find_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cart not found".to_string()))?;

        if cart_item.cart_id != cart.id {
            return Err(AppError::Forbidden(
                "You don't have permission to modify this cart item".to_string(),
            ));
        }

        // Delete item
        CartRepository::delete_item(pool, item_id).await?;

        // Return updated cart
        Self::get_cart(pool, user_id).await
    }

    /// Clear all items from cart
    pub async fn clear_cart(pool: &PgPool, user_id: &Uuid) -> Result<(), AppError> {
        // Find cart
        let cart = CartRepository::find_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cart not found".to_string()))?;

        // Clear all items
        CartRepository::clear_cart(pool, &cart.id).await?;

        Ok(())
    }
}
