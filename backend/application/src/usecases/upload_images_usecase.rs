use std::io::Cursor;
use std::sync::Arc;
use derive_new::new;
use bytes::Bytes;
use domain::error::ApiError;
use domain::repositories::AbstractImageRepository;
use image::{ImageReader};
use domain::helpers::ImageHelper;

#[derive(new)]
pub struct UploadImagesUsecase {
    repository: Arc<dyn AbstractImageRepository>,
    images: Vec<Bytes>
}

impl UploadImagesUsecase {
    pub async fn execute(&self) -> Result<Vec<String>, ApiError> {
        let mut images = Vec::new();
        //1- Convert images to webp + Resizing
        for bytes in &self.images {
            let image = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()
                .map_err(|err| ApiError::BadRequest(err.to_string()))?
                .decode()
                .map_err(|err| ApiError::BadRequest(err.to_string()))?
                .resize_to_fit(480)
                .crop_to_square();
            
            images.push(image);
        }
        //2- Get a unique image id ?
        //3- Upload images with repo
        todo!()
    }
}