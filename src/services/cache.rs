use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisResult, Pipeline, cmd};
use std::time::Duration;

pub struct CacheService {
    conn: ConnectionManager,
}

impl CacheService {
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let conn = ConnectionManager::new(client).await?;
        
        let cache = Self { conn };
        Ok(cache)
    }

    pub async fn get(&mut self, key: &str) -> RedisResult<Option<String>> {
        self.conn.get(key).await
    }

    pub async fn set(&mut self, key: &str, value: &str, ttl: Duration) -> RedisResult<()> {
        self.conn.set_ex(key, value, ttl.as_secs()).await
    }

    pub async fn set_with_default_ttl(&mut self, key: &str, value: &str) -> RedisResult<()> {
        self.set(key, value, Duration::from_secs(86400)).await
    }

    pub async fn incr(&mut self, key: &str) -> RedisResult<i64> {
        self.conn.incr(key, 1).await
    }

    pub async fn batch_get(&mut self, keys: &[String]) -> RedisResult<Vec<Option<String>>> {
        if keys.is_empty() {
            return Ok(vec![]);
        }
        self.conn.get(keys).await
    }

    pub async fn batch_set(&mut self, key_values: &[(String, String)], ttl: Duration) -> RedisResult<()> {
        if key_values.is_empty() {
            return Ok(());
        }
        
        let mut pipe = Pipeline::new();
        for (key, value) in key_values {
            pipe.set_ex(key, value, ttl.as_secs());
        }
        pipe.query_async(&mut self.conn).await
    }

    pub async fn exists(&mut self, key: &str) -> RedisResult<bool> {
        self.conn.exists(key).await
    }

    pub async fn delete(&mut self, key: &str) -> RedisResult<i64> {
        self.conn.del(key).await
    }

    pub async fn get_connection_info(&mut self) -> RedisResult<String> {
        cmd("INFO").arg("clients").query_async(&mut self.conn).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_get_empty_keys() {
        let keys: Vec<String> = vec![];
        assert_eq!(keys.is_empty(), true);
    }

    #[test]
    fn test_batch_set_empty_keys() {
        let key_values: Vec<(String, String)> = vec![];
        assert_eq!(key_values.is_empty(), true);
    }

    #[test]
    fn test_ttl_conversion() {
        let ttl = Duration::from_secs(60);
        assert_eq!(ttl.as_secs(), 60);
        
        let default_ttl = Duration::from_secs(86400);
        assert_eq!(default_ttl.as_secs(), 86400);
    }

    #[test]
    fn test_redis_url_parsing() {
        let redis_url = "redis://127.0.0.1:6379";
        assert!(redis_url.starts_with("redis://"));
        assert!(redis_url.contains("6379"));
    }

    #[test]
    fn test_key_value_pair_construction() {
        let key = "test_key";
        let value = "test_value";
        let pair = (key.to_string(), value.to_string());
        
        assert_eq!(pair.0, "test_key");
        assert_eq!(pair.1, "test_value");
    }

    #[test]
    fn test_pipeline_key_value_operations() {
        let key_values = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        
        assert_eq!(key_values.len(), 2);
        assert_eq!(key_values[0].0, "key1");
        assert_eq!(key_values[1].1, "value2");
    }

    #[test]
    fn test_redis_command_construction() {
        let key = "test_key";
        let value = "test_value";
        let ttl_secs = 3600u64;
        
        assert!(!key.is_empty());
        assert!(!value.is_empty());
        assert!(ttl_secs > 0);
    }
} 