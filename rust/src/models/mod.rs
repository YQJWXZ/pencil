use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub author_id: i64,
    pub category_id: Option<i64>,
    pub tags: Vec<String>,
    pub status: String,
    pub view_count: i64,
    pub cover_image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Asset {
    pub id: i64,
    pub filename: String,
    pub file_path: String,
    pub mime_type: String,
    pub size: i64,
    pub uploader_id: i64,
    pub created_at: DateTime<Utc>,
}
