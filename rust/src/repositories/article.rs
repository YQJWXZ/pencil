use crate::db::Pool;
use crate::models::Article;
use anyhow::Result;

pub struct ArticleRepository {
    pool: Pool,
}

impl ArticleRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// 根据 ID 查找文章
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Article>> {
        let article = sqlx::query_as::<_, Article>(
            "SELECT id, title, content, summary, author_id, category_id, tags, status, view_count, cover_image, created_at, updated_at
             FROM articles
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(article)
    }

    /// 列出文章（支持分页、过滤、搜索）
    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
        category_id: Option<i64>,
        status: Option<&str>,
        keyword: Option<&str>,
        order_by: &str,
    ) -> Result<(Vec<Article>, i64)> {
        // 简化查询：使用固定的查询并用 CASE 处理可选参数
        let search_pattern = keyword.map(|kw| format!("%{}%", kw));

        let count_sql = r#"
            SELECT COUNT(*)
            FROM articles
            WHERE ($1::BIGINT IS NULL OR category_id = $1)
              AND ($2::TEXT IS NULL OR status = $2)
              AND ($3::TEXT IS NULL OR (title ILIKE $3 OR content ILIKE $3))
        "#;

        let total: (i64,) = sqlx::query_as(count_sql)
            .bind(category_id)
            .bind(status)
            .bind(search_pattern.as_deref())
            .fetch_one(&self.pool)
            .await?;

        let order_clause = if order_by == "view_count" {
            "ORDER BY view_count DESC"
        } else {
            "ORDER BY created_at DESC"
        };

        let query_sql = format!(
            r#"
            SELECT id, title, content, summary, author_id, category_id, tags, status, view_count, cover_image, created_at, updated_at
            FROM articles
            WHERE ($1::BIGINT IS NULL OR category_id = $1)
              AND ($2::TEXT IS NULL OR status = $2)
              AND ($3::TEXT IS NULL OR (title ILIKE $3 OR content ILIKE $3))
            {}
            LIMIT $4 OFFSET $5
            "#,
            order_clause
        );

        let offset = (page - 1) * page_size;
        let articles = sqlx::query_as::<_, Article>(&query_sql)
            .bind(category_id)
            .bind(status)
            .bind(search_pattern.as_deref())
            .bind(page_size)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok((articles, total.0))
    }

    /// 创建文章
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        title: &str,
        content: &str,
        summary: Option<&str>,
        author_id: i64,
        category_id: Option<i64>,
        tags: Vec<String>,
        status: &str,
        cover_image: Option<&str>,
    ) -> Result<Article> {
        let article = sqlx::query_as::<_, Article>(
            "INSERT INTO articles (title, content, summary, author_id, category_id, tags, status, cover_image)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, title, content, summary, author_id, category_id, tags, status, view_count, cover_image, created_at, updated_at"
        )
        .bind(title)
        .bind(content)
        .bind(summary)
        .bind(author_id)
        .bind(category_id)
        .bind(&tags)
        .bind(status)
        .bind(cover_image)
        .fetch_one(&self.pool)
        .await?;

        Ok(article)
    }

    /// 更新文章
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        id: i64,
        title: Option<&str>,
        content: Option<&str>,
        summary: Option<&str>,
        category_id: Option<i64>,
        tags: Option<Vec<String>>,
        status: Option<&str>,
        cover_image: Option<&str>,
    ) -> Result<Article> {
        let article = sqlx::query_as::<_, Article>(
            "UPDATE articles
             SET title = COALESCE($2, title),
                 content = COALESCE($3, content),
                 summary = COALESCE($4, summary),
                 category_id = COALESCE($5, category_id),
                 tags = COALESCE($6, tags),
                 status = COALESCE($7, status),
                 cover_image = COALESCE($8, cover_image),
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = $1
             RETURNING id, title, content, summary, author_id, category_id, tags, status, view_count, cover_image, created_at, updated_at"
        )
        .bind(id)
        .bind(title)
        .bind(content)
        .bind(summary)
        .bind(category_id)
        .bind(&tags)
        .bind(status)
        .bind(cover_image)
        .fetch_one(&self.pool)
        .await?;

        Ok(article)
    }

    /// 删除文章
    pub async fn delete(&self, id: i64) -> Result<u64> {
        let result = sqlx::query("DELETE FROM articles WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// 增加浏览量
    pub async fn increment_view_count(&self, id: i64) -> Result<()> {
        sqlx::query("UPDATE articles SET view_count = view_count + 1 WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
