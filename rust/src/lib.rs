pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod tools;

// Re-export common types
pub use db::Pool;
pub use models::*;

// Re-export proto types for convenience
pub use tools::proto::{
    ArticleService,
    ArticleServiceServer,
    AssetService,
    AssetServiceServer,
    Empty,
    // Common types
    Pagination,
    PaginationMeta,
    Timestamp,
    // Services
    UserService,
    UserServiceServer,
};
