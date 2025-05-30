use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use domain::error::ApiError;
use domain::repositories::AbstractImageRepository;
use std::sync::Arc;
use bytes::Bytes;
use url::Url;
use crate::databases::AwsBucket;

const BUCKET_NAME: &str = "tier-master";


#[derive(Clone)]
pub struct ImageRepository {
    s3_client: Arc<S3Client>,
    pub_url: Url,
}

impl ImageRepository {
    pub fn new(bucket: &AwsBucket) -> Self {
        Self {
            s3_client: Arc::new(bucket.client().clone()),
            pub_url: bucket.pub_url().clone(),
        }
    }
}

#[async_trait]
impl AbstractImageRepository for ImageRepository {
    async fn upload_image(&self, image: Bytes, key: &str) -> Result<String, ApiError> {
        let body = ByteStream::from(image);
        
        self.s3_client
            .put_object()
            .bucket(BUCKET_NAME)
            .key(key)
            .body(body)
            .content_type("image/webp")
            .send()
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to upload image : {e}")))?;
        
        let url = self.pub_url.join(key)
            .map_err(|e| ApiError::InternalError(format!("Invalid image url : {e}")))?;
        
        Ok(url.to_string())
    }
}