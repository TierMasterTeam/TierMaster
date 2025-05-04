use crate::error::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait AbstractImageRepository: Send + Sync {

    async fn upload_image(&self,  file_path: &str, key: &str) -> Result<String, ApiError>;
}