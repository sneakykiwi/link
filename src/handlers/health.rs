use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use time::OffsetDateTime;

use crate::AppState;

pub async fn health_check(
    State(app_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match app_state.db.short_code_exists("health_check_dummy").await {
        Ok(_) => Ok(Json(json!({
            "status": "healthy",
            "timestamp": OffsetDateTime::now_utc().to_string()
        }))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
} 