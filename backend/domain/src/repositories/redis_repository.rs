use crate::error::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait AbstractRedisRepository: Send + Sync {
    async fn store(&self, key: &str, value: String, expiration: Option<u64>) -> Result<(), ApiError>;

    async fn fetch(&self, key: &str) -> Result<String, ApiError>;
    
    async fn delete(&self, key: &str) -> Result<(), ApiError>;
}