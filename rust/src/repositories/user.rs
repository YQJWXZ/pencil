use crate::db::Pool;
use crate::models::User;
use anyhow::Result;
use sqlx;

pub struct UserRepository {
    pool: Pool,
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, role, created_at, updated_at
             FROM users
             WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 根据邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, role, created_at, updated_at
             FROM users
             WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 根据 ID 查找用户
    pub async fn find_by_id(&self, user_id: i64) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, role, created_at, updated_at
             FROM users
             WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 创建新用户
    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
        role: &str,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash, role)
             VALUES ($1, $2, $3, $4)
             RETURNING id, username, email, password_hash, role, created_at, updated_at",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// 更新用户信息
    pub async fn update(
        &self,
        user_id: i64,
        username: Option<&str>,
        email: Option<&str>,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users
             SET username = COALESCE($2, username),
                 email = COALESCE($3, email),
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = $1
             RETURNING id, username, email, password_hash, role, created_at, updated_at",
        )
        .bind(user_id)
        .bind(username)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
