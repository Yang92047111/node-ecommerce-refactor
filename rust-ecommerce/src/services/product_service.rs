use crate::dto::product_dto::{
    CreateProductRequest, CreateRatingRequest, ProductQueryParams, ProductResponse,
    ProductsListResponse, RatingResponse, RatingsListResponse, UpdateProductRequest,
};
use crate::errors::AppError;
use crate::models::product::{Product, Rating};
use crate::repositories::product_repository::{ProductRepository, RatingRepository, WishlistRepository};
use crate::utils::slug::generate_slug;
use rust_decimal::Decimal;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ProductService;

impl ProductService {
    pub async fn create_product(
        pool: &PgPool,
        request: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let slug = generate_slug(&request.title);

        let images = if let Some(imgs) = request.images {
            json!(imgs)
        } else {
            json!([])
        };

        let product = ProductRepository::create(
            pool,
            &request.title,
            &slug,
            &request.description,
            request.price,
            request.quantity,
            request.brand.as_deref(),
            request.category_id,
            images,
        )
        .await?;

        Ok(Self::to_response(&product))
    }

    pub async fn get_product(pool: &PgPool, id: &Uuid) -> Result<ProductResponse, AppError> {
        let product = ProductRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        Ok(Self::to_response(&product))
    }

    pub async fn get_products(
        pool: &PgPool,
        params: ProductQueryParams,
    ) -> Result<ProductsListResponse, AppError> {
        let limit = params.limit.min(100).max(1);
        let page = params.page.max(1);
        let offset = (page - 1) * limit;

        let (products, total) = if let Some(query) = params.search {
            let products = ProductRepository::search(pool, &query, limit, offset).await?;
            let total = products.len() as i64; // For simplicity, not doing count query for search
            (products, total)
        } else if let Some(category_id) = params.category_id {
            let products =
                ProductRepository::find_by_category(pool, &category_id, limit, offset).await?;
            let total = products.len() as i64; // For simplicity
            (products, total)
        } else {
            let products = ProductRepository::find_all(pool, limit, offset).await?;
            let total = ProductRepository::count_all(pool).await?;
            (products, total)
        };

        let products_response: Vec<ProductResponse> =
            products.iter().map(|p| Self::to_response(p)).collect();

        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Ok(ProductsListResponse {
            products: products_response,
            total,
            page,
            limit,
            total_pages,
        })
    }

    pub async fn update_product(
        pool: &PgPool,
        id: &Uuid,
        request: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let product = ProductRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        let title = request.title.unwrap_or(product.title);
        let slug = generate_slug(&title);
        let description = request.description.unwrap_or(product.description);
        let price = request.price.unwrap_or(product.price);
        let quantity = request.quantity.unwrap_or(product.quantity);
        let brand = request.brand.or(product.brand);
        let category_id = request.category_id.or(product.category_id);
        let discount = request.discount.unwrap_or(product.discount);
        let images = if let Some(imgs) = request.images {
            json!(imgs)
        } else {
            product.images
        };

        let updated_product = ProductRepository::update(
            pool,
            id,
            &title,
            &slug,
            &description,
            price,
            quantity,
            brand.as_deref(),
            category_id,
            discount,
            images,
        )
        .await?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        Ok(Self::to_response(&updated_product))
    }

    pub async fn delete_product(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
        let deleted = ProductRepository::delete(pool, id).await?;

        if !deleted {
            return Err(AppError::NotFound("Product not found".to_string()));
        }

        Ok(())
    }

    fn to_response(product: &Product) -> ProductResponse {
        ProductResponse {
            id: product.id,
            title: product.title.clone(),
            slug: product.slug.clone(),
            description: product.description.clone(),
            price: product.price,
            quantity: product.quantity,
            brand: product.brand.clone(),
            category_id: product.category_id,
            sold: product.sold,
            discount: product.discount,
            images: product.images.clone(),
            total_ratings: product.total_ratings,
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

pub struct RatingService;

impl RatingService {
    pub async fn add_rating(
        pool: &PgPool,
        product_id: &Uuid,
        user_id: &Uuid,
        request: CreateRatingRequest,
    ) -> Result<RatingResponse, AppError> {
        // Check if product exists
        ProductRepository::find_by_id(pool, product_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        let rating = RatingRepository::create(
            pool,
            product_id,
            user_id,
            request.rating,
            request.comment.as_deref(),
        )
        .await?;

        // Update product's average rating
        RatingRepository::update_product_rating(pool, product_id).await?;

        Ok(Self::to_response(&rating))
    }

    pub async fn get_product_ratings(
        pool: &PgPool,
        product_id: &Uuid,
    ) -> Result<RatingsListResponse, AppError> {
        // Check if product exists
        ProductRepository::find_by_id(pool, product_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        let ratings = RatingRepository::find_by_product(pool, product_id).await?;
        let average = RatingRepository::calculate_average(pool, product_id).await?;

        let ratings_response: Vec<RatingResponse> =
            ratings.iter().map(|r| Self::to_response(r)).collect();

        Ok(RatingsListResponse {
            ratings: ratings_response,
            average,
        })
    }

    fn to_response(rating: &Rating) -> RatingResponse {
        RatingResponse {
            id: rating.id,
            product_id: rating.product_id,
            user_id: rating.user_id,
            rating: rating.rating,
            comment: rating.comment.clone(),
            created_at: rating.created_at,
        }
    }
}

pub struct WishlistService;

impl WishlistService {
    pub async fn add_to_wishlist(
        pool: &PgPool,
        user_id: &Uuid,
        product_id: &Uuid,
    ) -> Result<(), AppError> {
        // Check if product exists
        ProductRepository::find_by_id(pool, product_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        WishlistRepository::add(pool, user_id, product_id).await?;

        Ok(())
    }

    pub async fn remove_from_wishlist(
        pool: &PgPool,
        user_id: &Uuid,
        product_id: &Uuid,
    ) -> Result<(), AppError> {
        let removed = WishlistRepository::remove(pool, user_id, product_id).await?;

        if !removed {
            return Err(AppError::NotFound(
                "Product not found in wishlist".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn get_user_wishlist(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<ProductsListResponse, AppError> {
        let products = WishlistRepository::get_user_wishlist(pool, user_id).await?;

        let products_response: Vec<ProductResponse> = products
            .iter()
            .map(|p| ProductService::to_response(p))
            .collect();

        let total = products_response.len() as i64;

        Ok(ProductsListResponse {
            products: products_response,
            total,
            page: 1,
            limit: total,
            total_pages: 1,
        })
    }
}
