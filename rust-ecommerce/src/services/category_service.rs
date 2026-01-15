use crate::dto::category_dto::{CategoriesListResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest};
use crate::errors::AppError;
use crate::models::category::Category;
use crate::repositories::category_repository::CategoryRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CategoryService;

impl CategoryService {
    pub async fn create_category(
        pool: &PgPool,
        request: CreateCategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        // Check if category with same title already exists
        if let Some(_) = CategoryRepository::find_by_title(pool, &request.title).await? {
            return Err(AppError::Conflict(
                "Category with this title already exists".to_string(),
            ));
        }

        let category = CategoryRepository::create(pool, &request.title).await?;

        Ok(Self::to_response(&category))
    }

    pub async fn get_category(pool: &PgPool, id: &Uuid) -> Result<CategoryResponse, AppError> {
        let category = CategoryRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

        Ok(Self::to_response(&category))
    }

    pub async fn get_all_categories(pool: &PgPool) -> Result<CategoriesListResponse, AppError> {
        let categories = CategoryRepository::find_all(pool).await?;

        let categories_response: Vec<CategoryResponse> = categories
            .iter()
            .map(|c| Self::to_response(c))
            .collect();

        Ok(CategoriesListResponse {
            categories: categories_response,
        })
    }

    pub async fn update_category(
        pool: &PgPool,
        id: &Uuid,
        request: UpdateCategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        // Check if category exists
        CategoryRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

        // Check if another category with same title exists
        if let Some(existing) = CategoryRepository::find_by_title(pool, &request.title).await? {
            if &existing.id != id {
                return Err(AppError::Conflict(
                    "Category with this title already exists".to_string(),
                ));
            }
        }

        let category = CategoryRepository::update(pool, id, &request.title)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

        Ok(Self::to_response(&category))
    }

    pub async fn delete_category(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
        let deleted = CategoryRepository::delete(pool, id).await?;

        if !deleted {
            return Err(AppError::NotFound("Category not found".to_string()));
        }

        Ok(())
    }

    fn to_response(category: &Category) -> CategoryResponse {
        CategoryResponse {
            id: category.id,
            title: category.title.clone(),
            created_at: category.created_at,
            updated_at: category.updated_at,
        }
    }
}
