use crate::dto::order_dto::{
    CreateOrderRequest, OrderItemResponse, OrderQueryParams, OrderResponse, OrdersListResponse,
    UpdateOrderStatusRequest,
};
use crate::errors::AppError;
use crate::models::order::OrderStatus;
use crate::repositories::cart_repository::CartRepository;
use crate::repositories::order_repository::OrderRepository;
use crate::repositories::product_repository::ProductRepository;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub struct OrderService;

impl OrderService {
    /// Create order from cart
    pub async fn create_order(
        pool: &PgPool,
        user_id: &Uuid,
        req: CreateOrderRequest,
    ) -> Result<OrderResponse, AppError> {
        // Get user's cart
        let cart = CartRepository::find_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cart not found".to_string()))?;

        // Get cart items
        let cart_items = CartRepository::find_items_by_cart(pool, &cart.id).await?;

        if cart_items.is_empty() {
            return Err(AppError::BadRequest("Cart is empty".to_string()));
        }

        // Calculate total price and verify stock
        let mut total_price = Decimal::ZERO;
        let mut order_items_data = Vec::new();

        for item in &cart_items {
            let product = ProductRepository::find_by_id(pool, &item.product_id)
                .await?
                .ok_or_else(|| AppError::NotFound(format!("Product {} not found", item.product_id)))?;

            // Check stock
            if product.quantity < item.quantity {
                return Err(AppError::BadRequest(
                    format!(
                        "Insufficient stock for product '{}'. Only {} available",
                        product.title, product.quantity
                    ),
                ));
            }

            let item_total = product.price * Decimal::from(item.quantity);
            total_price += item_total;

            order_items_data.push((item.product_id, item.quantity, product.price, product.title));
        }

        total_price += req.shipping_price;

        // Create order
        let order = OrderRepository::create(
            pool,
            *user_id,
            &req.payment_method,
            req.shipping_price,
            total_price,
            &req.shipping_address_street,
            &req.shipping_address_city,
        )
        .await?;

        // Create order items and update product quantities
        let mut order_items = Vec::new();

        for (product_id, quantity, price, title) in order_items_data {
            // Add order item
            let order_item = OrderRepository::add_item(pool, order.id, product_id, quantity, price).await?;

            // Update product quantity and sold count
            ProductRepository::update_quantity(pool, &product_id, quantity).await?;

            let subtotal = price * Decimal::from(quantity);
            order_items.push(OrderItemResponse {
                id: order_item.id,
                order_id: order_item.order_id,
                product_id: order_item.product_id,
                quantity: order_item.quantity,
                price: order_item.price,
                product_title: Some(title),
                subtotal,
                created_at: order_item.created_at,
            });
        }

        // Clear cart after successful order
        CartRepository::clear_cart(pool, &cart.id).await?;

        Ok(OrderResponse {
            id: order.id,
            user_id: order.user_id,
            status: order.status,
            payment_method: order.payment_method,
            shipping_price: order.shipping_price,
            total_price: order.total_price,
            shipping_address_street: order.shipping_address_street,
            shipping_address_city: order.shipping_address_city,
            items: order_items,
            created_at: order.created_at,
            updated_at: order.updated_at,
        })
    }

    /// Get order by ID with items
    pub async fn get_order(pool: &PgPool, order_id: &Uuid) -> Result<OrderResponse, AppError> {
        let order = OrderRepository::find_by_id(pool, order_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

        let order_items = OrderRepository::find_items_by_order(pool, order_id).await?;

        // Enrich items with product details
        let mut enriched_items = Vec::new();
        for item in order_items {
            let product = ProductRepository::find_by_id(pool, &item.product_id).await?;
            let product_title = product.map(|p| p.title);

            let subtotal = item.price * Decimal::from(item.quantity);
            enriched_items.push(OrderItemResponse {
                id: item.id,
                order_id: item.order_id,
                product_id: item.product_id,
                quantity: item.quantity,
                price: item.price,
                product_title,
                subtotal,
                created_at: item.created_at,
            });
        }

        Ok(OrderResponse {
            id: order.id,
            user_id: order.user_id,
            status: order.status,
            payment_method: order.payment_method,
            shipping_price: order.shipping_price,
            total_price: order.total_price,
            shipping_address_street: order.shipping_address_street,
            shipping_address_city: order.shipping_address_city,
            items: enriched_items,
            created_at: order.created_at,
            updated_at: order.updated_at,
        })
    }

    /// Get user's orders
    pub async fn get_user_orders(
        pool: &PgPool,
        user_id: &Uuid,
        params: OrderQueryParams,
    ) -> Result<OrdersListResponse, AppError> {
        let limit = params.limit.min(100).max(1);
        let page = params.page.max(1);
        let offset = (page - 1) * limit;

        let orders = OrderRepository::find_by_user(pool, user_id, limit, offset).await?;
        let total = OrderRepository::count_by_user(pool, user_id).await?;

        // Enrich each order with items
        let mut enriched_orders = Vec::new();
        for order in orders {
            let order_response = Self::get_order(pool, &order.id).await?;
            enriched_orders.push(order_response);
        }

        let total_pages = (total + limit - 1) / limit;

        Ok(OrdersListResponse {
            orders: enriched_orders,
            total,
            page,
            limit,
            total_pages,
        })
    }

    /// Get all orders (admin)
    pub async fn get_all_orders(
        pool: &PgPool,
        params: OrderQueryParams,
    ) -> Result<OrdersListResponse, AppError> {
        let limit = params.limit.min(100).max(1);
        let page = params.page.max(1);
        let offset = (page - 1) * limit;

        let orders = OrderRepository::find_all(pool, limit, offset).await?;
        let total = OrderRepository::count_all(pool).await?;

        // Enrich each order with items
        let mut enriched_orders = Vec::new();
        for order in orders {
            let order_response = Self::get_order(pool, &order.id).await?;
            enriched_orders.push(order_response);
        }

        let total_pages = (total + limit - 1) / limit;

        Ok(OrdersListResponse {
            orders: enriched_orders,
            total,
            page,
            limit,
            total_pages,
        })
    }

    /// Update order status (admin)
    pub async fn update_order_status(
        pool: &PgPool,
        order_id: &Uuid,
        req: UpdateOrderStatusRequest,
    ) -> Result<OrderResponse, AppError> {
        // Validate status
        let valid_statuses = ["Pending", "Processing", "Shipped", "Delivered", "Cancelled"];
        if !valid_statuses.contains(&req.status.as_str()) {
            return Err(AppError::BadRequest(format!(
                "Invalid status. Must be one of: {}",
                valid_statuses.join(", ")
            )));
        }

        // Update status
        OrderRepository::update_status(pool, order_id, &req.status).await?;

        // Return updated order
        Self::get_order(pool, order_id).await
    }
}
