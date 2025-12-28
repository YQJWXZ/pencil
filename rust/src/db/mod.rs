use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub type Pool = PgPool;

/// 创建 PostgreSQL 连接池
pub async fn create_pool(database_url: &str) -> Result<Pool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;

    Ok(pool)
}

/// 运行数据库迁移
#[allow(dead_code)]
pub async fn run_migrations(pool: &Pool) -> Result<()> {
    sqlx::migrate!("../migrations").run(pool).await?;
    Ok(())
}
