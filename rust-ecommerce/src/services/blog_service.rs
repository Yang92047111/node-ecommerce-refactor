use crate::dto::blog_dto::{BlogResponse, BlogsListResponse, CreateBlogRequest, UpdateBlogRequest};
use crate::errors::AppError;
use crate::models::blog::Blog;
use crate::repositories::blog_repository::BlogRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct BlogService;

impl BlogService {
    pub async fn create_blog(
        pool: &PgPool,
        request: CreateBlogRequest,
        author_id: &Uuid,
    ) -> Result<BlogResponse, AppError> {
        let blog = BlogRepository::create(
            pool,
            &request.title,
            request.description.as_deref(),
            &request.content,
            author_id,
            request.category_id.as_ref(),
            request.images,
        )
        .await?;

        Ok(Self::to_response(&blog))
    }

    pub async fn get_blog(pool: &PgPool, id: &Uuid, increment_view: bool) -> Result<BlogResponse, AppError> {
        let blog = BlogRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Blog not found".to_string()))?;

        // Increment view count if requested
        if increment_view {
            let _ = BlogRepository::increment_views(pool, id).await;
        }

        Ok(Self::to_response(&blog))
    }

    pub async fn get_all_blogs(
        pool: &PgPool,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<BlogsListResponse, AppError> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);

        if page < 1 {
            return Err(AppError::ValidationError(
                "Page must be greater than 0".to_string(),
            ));
        }

        if page_size < 1 || page_size > 100 {
            return Err(AppError::ValidationError(
                "Page size must be between 1 and 100".to_string(),
            ));
        }

        let blogs = BlogRepository::find_all(pool, page, page_size).await?;
        let total = BlogRepository::count(pool).await?;

        let blogs_response: Vec<BlogResponse> =
            blogs.iter().map(|b| Self::to_response(b)).collect();

        Ok(BlogsListResponse {
            blogs: blogs_response,
            total,
            page,
            page_size,
        })
    }

    pub async fn get_blogs_by_author(
        pool: &PgPool,
        author_id: &Uuid,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<BlogsListResponse, AppError> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);

        if page < 1 {
            return Err(AppError::ValidationError(
                "Page must be greater than 0".to_string(),
            ));
        }

        if page_size < 1 || page_size > 100 {
            return Err(AppError::ValidationError(
                "Page size must be between 1 and 100".to_string(),
            ));
        }

        let blogs = BlogRepository::find_by_author(pool, author_id, page, page_size).await?;
        let total = BlogRepository::count(pool).await?;

        let blogs_response: Vec<BlogResponse> =
            blogs.iter().map(|b| Self::to_response(b)).collect();

        Ok(BlogsListResponse {
            blogs: blogs_response,
            total,
            page,
            page_size,
        })
    }

    pub async fn update_blog(
        pool: &PgPool,
        id: &Uuid,
        request: UpdateBlogRequest,
        user_id: &Uuid,
        is_admin: bool,
    ) -> Result<BlogResponse, AppError> {
        // Check if blog exists and user has permission
        let existing = BlogRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Blog not found".to_string()))?;

        // Only the author or admin can update
        if &existing.author_id != user_id && !is_admin {
            return Err(AppError::Forbidden(
                "You don't have permission to update this blog".to_string(),
            ));
        }

        let blog = BlogRepository::update(
            pool,
            id,
            request.title.as_deref(),
            request.description.as_deref(),
            request.content.as_deref(),
            request.category_id.as_ref(),
            request.images,
        )
        .await?
        .ok_or_else(|| AppError::NotFound("Blog not found".to_string()))?;

        Ok(Self::to_response(&blog))
    }

    pub async fn delete_blog(
        pool: &PgPool,
        id: &Uuid,
        user_id: &Uuid,
        is_admin: bool,
    ) -> Result<(), AppError> {
        // Check if blog exists and user has permission
        let existing = BlogRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Blog not found".to_string()))?;

        // Only the author or admin can delete
        if &existing.author_id != user_id && !is_admin {
            return Err(AppError::Forbidden(
                "You don't have permission to delete this blog".to_string(),
            ));
        }

        let deleted = BlogRepository::delete(pool, id).await?;

        if !deleted {
            return Err(AppError::NotFound("Blog not found".to_string()));
        }

        Ok(())
    }

    fn to_response(blog: &Blog) -> BlogResponse {
        BlogResponse {
            id: blog.id,
            title: blog.title.clone(),
            description: blog.description.clone(),
            content: blog.content.clone(),
            author_id: blog.author_id,
            category_id: blog.category_id,
            images: blog.images.clone(),
            views_count: blog.views_count,
            created_at: blog.created_at,
            updated_at: blog.updated_at,
        }
    }
}
