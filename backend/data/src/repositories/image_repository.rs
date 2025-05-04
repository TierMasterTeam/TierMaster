use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use domain::error::ApiError;
use domain::repositories::AbstractImageRepository;
use std::path::Path;
use std::sync::Arc;

const BUCKET_NAME: &str = "tier-master";


#[derive(Clone)]
pub struct ImageRepository {
    s3_client: Arc<S3Client>,
}

impl ImageRepository {
    pub fn new(s3_client: &S3Client) -> Self {
        Self {
            s3_client: Arc::new(s3_client.clone()),
        }
    }
}

#[async_trait]
impl AbstractImageRepository for ImageRepository {
    async fn upload_image(&self, file_path: &str, key: &str) -> Result<String, ApiError> {
        let body = ByteStream::from_path(Path::new(file_path)).await
            .map_err(|e| ApiError::InternalError(format!("Failed to load image from path {file_path} : {e}")))?;

        self.s3_client
            .put_object()
            .bucket(BUCKET_NAME)
            .key(key)
            .body(body)
            .send()
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to upload image : {e}")))?;
        
        Ok(make_public_url(key))
    }
}

fn make_public_url(key: &str) -> String {
    format!("https://pub-b52a5664afb843f3914c1598888d508e.r2.dev/{key}")
}