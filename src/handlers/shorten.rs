use axum::{
    extract::State,
    response::Json,
};
use once_cell::sync::Lazy;
use prometheus::Counter;
use sqlx::types::time::OffsetDateTime;
use time::Duration;
use url::Url;
use uuid::Uuid;

use crate::{
    AppState,
    config::CONFIG,
    error::AppError,
    models::link::{CreateLinkRequest, CreateLinkResponse, Link},
    services::shortener::{generate_short_code_base62, is_valid_custom_code},
};

static LINK_CREATION_COUNT: Lazy<Counter> = Lazy::new(|| {
    prometheus::register_counter!("link_creation_total", "Total number of links created")
        .expect("Failed to register prometheus counter")
});

pub async fn create_link(
    State(app_state): State<AppState>,
    Json(request): Json<CreateLinkRequest>,
) -> Result<Json<CreateLinkResponse>, AppError> {
    let parsed_url = Url::parse(&request.url)
        .map_err(|_| AppError::InvalidUrl("Invalid URL format".to_string()))?;
    
    if !matches!(parsed_url.scheme(), "http" | "https") {
        return Err(AppError::InvalidUrl("Only HTTP and HTTPS URLs are allowed".to_string()));
    }
    
    let short_code = if let Some(custom_code) = request.custom_code {
        if !is_valid_custom_code(&custom_code) {
            return Err(AppError::InvalidUrl("Invalid custom code format".to_string()));
        }
        
        if app_state.db.get_link_by_code(&custom_code).await?.is_some() {
            return Err(AppError::Conflict);
        }
        
        custom_code
    } else {
        generate_short_code_base62(&request.url)
    };
    
    let expires_at = match request.expires_in_hours {
        Some(hours) => Some(OffsetDateTime::now_utc() + time::Duration::hours(hours as i64)),
        None => None,
    };
    
    let link = Link {
        id: Uuid::new_v4(),
        short_code: short_code.clone(),
        original_url: request.url.clone(),
        clicks: 0,
        created_at: OffsetDateTime::now_utc(),
        expires_at,
    };
    
    app_state.db.create_link(&link).await?;
    
    {
        let mut cache = app_state.cache.lock().await;
        if let Err(e) = cache.set(
            &short_code, 
            &request.url, 
            std::time::Duration::from_secs(3600)
        ).await {
            tracing::warn!("Failed to cache link: {}", e);
        }
    }
    
    LINK_CREATION_COUNT.inc();
    
    let short_url = format!("{}/{}", CONFIG.base_url, short_code);
    
    Ok(Json(CreateLinkResponse {
        short_url,
        short_code: short_code.to_string(),
    }))
}