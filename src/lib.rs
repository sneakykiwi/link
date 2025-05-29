pub mod config;
pub mod models;
pub mod handlers;
pub mod services;
pub mod middleware;
pub mod error;

use std::sync::Arc;
use tokio::sync::Mutex;
use services::{cache::CacheService, db::DbService};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbService>,
    pub cache: Arc<Mutex<CacheService>>,
} 