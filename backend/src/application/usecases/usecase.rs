use async_trait::async_trait;
use crate::domain::error::ApiError;

#[async_trait]
pub trait UseCase<E>: Send + Sync {
    async fn execute(&self) -> Result<E, ApiError>;
}