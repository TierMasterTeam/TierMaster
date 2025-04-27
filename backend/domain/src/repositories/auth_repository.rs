use crate::entities::{CreateUserEntity, UserEntity};
use crate::error::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait AbstractAuthRepository: Send + Sync {
    async fn get_user_by_email(&self, email: &str) -> Result<UserEntity, ApiError>;
    
    async fn create_user(&self, user: CreateUserEntity) -> Result<(), ApiError>;
}