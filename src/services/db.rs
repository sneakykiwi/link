use sqlx::PgPool;
use crate::models::link::Link;
use crate::error::AppError;

pub struct DbService {
    pool: PgPool,
}

impl DbService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_link(&self, link: &Link) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO links (id, short_code, original_url, clicks, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            link.id,
            link.short_code,
            link.original_url,
            link.clicks,
            link.created_at,
            link.expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_link_by_code(&self, short_code: &str) -> Result<Option<Link>, AppError> {
        let link = sqlx::query_as!(
            Link,
            r#"
            SELECT id, short_code, original_url, clicks as "clicks!: i64", created_at as "created_at!: _", expires_at
            FROM links 
            WHERE short_code = $1 
            AND (expires_at IS NULL OR expires_at > NOW())
            "#,
            short_code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(link)
    }

    pub async fn increment_clicks(&self, short_code: &str) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE links SET clicks = clicks + 1 WHERE short_code = $1",
            short_code
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn short_code_exists(&self, short_code: &str) -> Result<bool, AppError> {
        let exists = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM links WHERE short_code = $1)",
            short_code
        )
        .fetch_one(&self.pool)
        .await?
        .exists
        .unwrap_or(false);

        Ok(exists)
    }
} 