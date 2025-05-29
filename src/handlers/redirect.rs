use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use crate::{
    error::AppError,
    AppState,
};

pub async fn redirect(
    Path(short_code): Path<String>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let mut cache = app_state.cache.lock().await;
    
    if let Ok(Some(url)) = cache.get(&short_code).await {
        let click_key = format!("clicks:{}", short_code);
        let _ = cache.incr(&click_key).await;
        
        let etag = format!("\"{}\"", &short_code);
        
        let response = Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("location", &url)
            .header("etag", &etag)
            .header("cache-control", "public, max-age=31536000, immutable")
            .header("x-cache", "HIT")
            .body(axum::body::Body::empty())
            .map_err(|_| AppError::InternalServerError)?;
            
        return Ok(response);
    }
    
    drop(cache);

    if let Some(link) = app_state.db.get_link_by_code(&short_code).await? {
        let mut cache = app_state.cache.lock().await;
        let _ = cache.set_with_default_ttl(&short_code, &link.original_url).await;
        
        let db_service = app_state.db.clone();
        let code_for_bg = short_code.clone();
        tokio::spawn(async move {
            let _ = db_service.increment_clicks(&code_for_bg).await;
        });
        
        let etag = format!("\"{}\"", &short_code);
        
        let response = Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("location", &link.original_url)
            .header("etag", &etag)
            .header("cache-control", "public, max-age=31536000, immutable")
            .header("x-cache", "MISS")
            .body(axum::body::Body::empty())
            .map_err(|_| AppError::InternalServerError)?;
            
        Ok(response)
    } else {
        Err(AppError::NotFound)
    }
} 