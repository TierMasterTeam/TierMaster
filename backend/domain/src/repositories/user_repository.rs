use crate::entities::UserEntity;
use crate::error::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait AbstractUserRepository: Send + Sync {
    async fn get_by_id(&self, id: &str) -> Result<UserEntity, ApiError>;
}