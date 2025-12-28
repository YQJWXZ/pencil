use crate::db::Pool;
use crate::models::Asset;
use anyhow::Result;

pub struct AssetRepository {
    pool: Pool,
}

impl AssetRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// 根据 ID 查找资产
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Asset>> {
        let asset = sqlx::query_as::<_, Asset>(
            "SELECT id, filename, file_path, mime_type, size, uploader_id, created_at
             FROM assets
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(asset)
    }

    /// 列出资产（支持分页和 MIME 类型过滤）
    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
        mime_type_prefix: Option<&str>,
    ) -> Result<(Vec<Asset>, i64)> {
        let (_where_clause, count_query, select_query) = if let Some(_prefix) = mime_type_prefix {
            (
                "WHERE mime_type LIKE $1",
                "SELECT COUNT(*) FROM assets WHERE mime_type LIKE $1",
                "SELECT id, filename, file_path, mime_type, size, uploader_id, created_at
                 FROM assets
                 WHERE mime_type LIKE $1
                 ORDER BY created_at DESC
                 LIMIT $2 OFFSET $3",
            )
        } else {
            (
                "",
                "SELECT COUNT(*) FROM assets",
                "SELECT id, filename, file_path, mime_type, size, uploader_id, created_at
                 FROM assets
                 ORDER BY created_at DESC
                 LIMIT $1 OFFSET $2",
            )
        };

        // 获取总数
        let total: (i64,) = if let Some(prefix) = mime_type_prefix {
            let pattern = format!("{}%", prefix);
            sqlx::query_as(count_query)
                .bind(&pattern)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query_as(count_query).fetch_one(&self.pool).await?
        };

        // 获取分页数据
        let offset = (page - 1) * page_size;
        let assets = if let Some(prefix) = mime_type_prefix {
            let pattern = format!("{}%", prefix);
            sqlx::query_as::<_, Asset>(select_query)
                .bind(&pattern)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, Asset>(select_query)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        };

        Ok((assets, total.0))
    }

    /// 创建资产记录
    pub async fn create(
        &self,
        filename: &str,
        file_path: &str,
        mime_type: &str,
        size: i64,
        uploader_id: i64,
    ) -> Result<Asset> {
        let asset = sqlx::query_as::<_, Asset>(
            "INSERT INTO assets (filename, file_path, mime_type, size, uploader_id)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, filename, file_path, mime_type, size, uploader_id, created_at",
        )
        .bind(filename)
        .bind(file_path)
        .bind(mime_type)
        .bind(size)
        .bind(uploader_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(asset)
    }

    /// 删除资产记录
    pub async fn delete(&self, id: i64) -> Result<u64> {
        let result = sqlx::query("DELETE FROM assets WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
