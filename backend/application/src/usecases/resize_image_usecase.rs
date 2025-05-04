use bytes::Bytes;
use derive_new::new;
use domain::error::ApiError;
use domain::helpers::ImageHelper;
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

#[derive(new)]
pub struct ResizeImageUsecase<'a> {
    image_data: &'a Bytes,
    dest_path: &'a str,
}

impl ResizeImageUsecase<'_> {
    pub async fn execute(&self) -> Result<(), ApiError> {

        ImageReader::new(Cursor::new(self.image_data))
            .with_guessed_format()
            .map_err(|e| ApiError::BadRequest(format!("Failed to guess format of image : {e}")))?
            .decode()
            .map_err(|e| ApiError::BadRequest(format!("Failed to decode image : {e}")))?
            .resize_to_fit(480)
            .crop_to_square()
            .save_with_format(self.dest_path, ImageFormat::WebP)
            .map_err(|e| ApiError::InternalError(format!("Failed to save image to WebP format : {e}")))?;

        Ok(())
    }
}