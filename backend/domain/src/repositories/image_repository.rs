use crate::error::ApiError;
use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait AbstractImageRepository: Send + Sync {

    async fn upload_image(&self,  image: Bytes, key: &str) -> Result<String, ApiError>;
}