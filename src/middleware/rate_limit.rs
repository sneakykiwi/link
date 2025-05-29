use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    store: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub fn is_allowed(&self, key: &str) -> bool {
        let mut store = self.store.lock().unwrap();
        let now = Instant::now();

        match store.get_mut(key) {
            Some((count, last_reset)) => {
                if now.duration_since(*last_reset) >= self.window {
                    *count = 1;
                    *last_reset = now;
                    true
                } else if *count < self.max_requests {
                    *count += 1;
                    true
                } else {
                    false
                }
            }
            None => {
                store.insert(key.to_string(), (1, now));
                true
            }
        }
    }
}

pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let client_ip = "127.0.0.1"; // TODO: Placeholder - extract from headers
    
    // Create rate limiter: 100 requests per minute
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));
    
    if !rate_limiter.is_allowed(client_ip) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
} 