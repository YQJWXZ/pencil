pub mod db;
pub mod handlers;
pub mod models;
pub mod proto; // Proto generated code
pub mod services;
pub mod tools;

// Re-export common types
pub use db::Pool;
pub use models::*;

// Re-export proto types for convenience
pub use proto::{
    ArticleService,
    ArticleServiceServer,
    AssetService,
    AssetServiceServer,
    // Services
    AuthService,
    AuthServiceServer,
    Empty,
    // Common types
    Pagination,
    PaginationMeta,
    Timestamp,
};
