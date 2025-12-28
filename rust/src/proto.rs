// ========== Generated Proto Modules ==========

#[allow(clippy::all)]
pub mod common {
    tonic::include_proto!("common");
}

#[allow(clippy::all)]
pub mod auth {
    tonic::include_proto!("auth");
}

#[allow(clippy::all)]
pub mod article {
    tonic::include_proto!("article");
}

#[allow(clippy::all)]
pub mod asset {
    tonic::include_proto!("asset");
}

// ========== Re-exports: Services & Messages ==========

// Auth Service
pub use auth::auth_service_client::AuthServiceClient;
pub use auth::auth_service_server::{AuthService, AuthServiceServer};
pub use auth::{
    LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, User, VerifyTokenRequest,
    VerifyTokenResponse,
};

// Article Service
pub use article::article_service_client::ArticleServiceClient;
pub use article::article_service_server::{ArticleService, ArticleServiceServer};
pub use article::{
    Article, Category, CreateArticleRequest, CreateCategoryRequest, DeleteArticleRequest,
    DeleteCategoryRequest, GetArticleRequest, GetCategoryRequest, ListArticlesRequest,
    ListArticlesResponse, ListCategoriesRequest, ListCategoriesResponse, UpdateArticleRequest,
    UpdateCategoryRequest,
};

// Asset Service
pub use asset::asset_service_client::AssetServiceClient;
pub use asset::asset_service_server::{AssetService, AssetServiceServer};
pub use asset::{
    Asset, DeleteAssetRequest, GetAssetRequest, ListAssetsRequest, ListAssetsResponse,
    UploadFileRequest, UploadFileResponse,
};

// Common Types
pub use common::{Empty, Error, Pagination, PaginationMeta, Timestamp};

// ========== Entity → Proto Conversion ==========

use crate::models;

// User Entity → Proto
impl From<models::User> for User {
    fn from(user: models::User) -> Self {
        User {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: Some(Timestamp {
                value: user.created_at.to_rfc3339(),
            }),
        }
    }
}

impl From<&models::User> for User {
    fn from(user: &models::User) -> Self {
        User {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            created_at: Some(Timestamp {
                value: user.created_at.to_rfc3339(),
            }),
        }
    }
}

// Article Entity → Proto
impl From<models::Article> for Article {
    fn from(article: models::Article) -> Self {
        Article {
            id: article.id,
            title: article.title,
            content: article.content,
            summary: article.summary.unwrap_or_default(),
            author_id: article.author_id,
            author_name: String::new(),
            category_id: article.category_id.unwrap_or(0),
            category_name: String::new(),
            tags: article.tags,
            status: article.status,
            view_count: article.view_count,
            cover_image: article.cover_image.unwrap_or_default(),
            created_at: Some(Timestamp {
                value: article.created_at.to_rfc3339(),
            }),
            updated_at: Some(Timestamp {
                value: article.updated_at.to_rfc3339(),
            }),
        }
    }
}

impl From<&models::Article> for Article {
    fn from(article: &models::Article) -> Self {
        Article {
            id: article.id,
            title: article.title.clone(),
            content: article.content.clone(),
            summary: article.summary.clone().unwrap_or_default(),
            author_id: article.author_id,
            author_name: String::new(),
            category_id: article.category_id.unwrap_or(0),
            category_name: String::new(),
            tags: article.tags.clone(),
            status: article.status.clone(),
            view_count: article.view_count,
            cover_image: article.cover_image.clone().unwrap_or_default(),
            created_at: Some(Timestamp {
                value: article.created_at.to_rfc3339(),
            }),
            updated_at: Some(Timestamp {
                value: article.updated_at.to_rfc3339(),
            }),
        }
    }
}

// Category Entity → Proto
impl From<models::Category> for Category {
    fn from(category: models::Category) -> Self {
        Category {
            id: category.id,
            name: category.name,
            slug: category.slug,
            description: category.description.unwrap_or_default(),
            article_count: 0,
        }
    }
}

impl From<&models::Category> for Category {
    fn from(category: &models::Category) -> Self {
        Category {
            id: category.id,
            name: category.name.clone(),
            slug: category.slug.clone(),
            description: category.description.clone().unwrap_or_default(),
            article_count: 0,
        }
    }
}

// Asset Entity → Proto
impl From<models::Asset> for Asset {
    fn from(asset: models::Asset) -> Self {
        Asset {
            id: asset.id,
            filename: asset.filename,
            file_path: asset.file_path.clone(),
            url: format!("/uploads/{}", asset.file_path),
            mime_type: asset.mime_type,
            size: asset.size,
            uploader_id: asset.uploader_id,
            created_at: Some(Timestamp {
                value: asset.created_at.to_rfc3339(),
            }),
        }
    }
}

impl From<&models::Asset> for Asset {
    fn from(asset: &models::Asset) -> Self {
        Asset {
            id: asset.id,
            filename: asset.filename.clone(),
            file_path: asset.file_path.clone(),
            url: format!("/uploads/{}", asset.file_path),
            mime_type: asset.mime_type.clone(),
            size: asset.size,
            uploader_id: asset.uploader_id,
            created_at: Some(Timestamp {
                value: asset.created_at.to_rfc3339(),
            }),
        }
    }
}
