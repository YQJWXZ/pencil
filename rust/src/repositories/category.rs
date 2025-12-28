use crate::db::Pool;
use crate::models::Category;
use anyhow::Result;

pub struct CategoryRepository {
    pool: Pool,
}

impl CategoryRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// 根据 ID 查找分类
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Category>> {
        let category = sqlx::query_as::<_, Category>(
            "SELECT id, name, slug, description, created_at, updated_at
             FROM categories
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    /// 根据 slug 查找分类
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>> {
        let category = sqlx::query_as::<_, Category>(
            "SELECT id, name, slug, description, created_at, updated_at
             FROM categories
             WHERE slug = $1",
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    /// 列出所有分类（分页）
    pub async fn list(&self, page: i64, page_size: i64) -> Result<(Vec<Category>, i64)> {
        // 获取总数
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
            .fetch_one(&self.pool)
            .await?;

        // 获取分页数据
        let offset = (page - 1) * page_size;
        let categories = sqlx::query_as::<_, Category>(
            "SELECT id, name, slug, description, created_at, updated_at
             FROM categories
             ORDER BY created_at DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((categories, total.0))
    }

    /// 创建分类
    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
    ) -> Result<Category> {
        let category = sqlx::query_as::<_, Category>(
            "INSERT INTO categories (name, slug, description)
             VALUES ($1, $2, $3)
             RETURNING id, name, slug, description, created_at, updated_at",
        )
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    /// 更新分类
    pub async fn update(
        &self,
        id: i64,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
    ) -> Result<Category> {
        let category = sqlx::query_as::<_, Category>(
            "UPDATE categories
             SET name = COALESCE($2, name),
                 slug = COALESCE($3, slug),
                 description = COALESCE($4, description),
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = $1
             RETURNING id, name, slug, description, created_at, updated_at",
        )
        .bind(id)
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    /// 删除分类
    pub async fn delete(&self, id: i64) -> Result<u64> {
        let result = sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
