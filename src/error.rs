use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Short code not found")]
    NotFound,
    
    #[error("Short code already exists")]
    Conflict,
    
    #[error("Internal server error")]
    Internal,
    
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error"),
            AppError::InvalidUrl(_) => (StatusCode::BAD_REQUEST, "Invalid URL"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Short code not found"),
            AppError::Conflict => (StatusCode::CONFLICT, "Short code already exists"),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
} 