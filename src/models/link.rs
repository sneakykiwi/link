use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Link {
    pub id: Uuid,
    pub short_code: String,
    pub original_url: String,
    pub clicks: i64,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601::option")]
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLinkRequest {
    pub url: String,
    pub custom_code: Option<String>,
    pub expires_in_hours: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkResponse {
    pub short_url: String,
    pub short_code: String,
} 