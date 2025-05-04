use crate::usecases::ResizeImageUsecase;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use derive_new::new;
use domain::entities::ImageEntity;
use domain::error::ApiError;
use domain::repositories::AbstractImageRepository;
use std::sync::Arc;
use std::{env, fs};

const TEMP_IMAGE_FOLDER: &str = "resized_image";

const TIERLIST_IMAGE_S3_FOLDER: &str = "tierlist_images"; 

#[derive(new)]
pub struct ImageService {
    repo: Arc<dyn AbstractImageRepository>
}

impl ImageService {

    pub async fn upload_image(&self, image: ImageEntity) -> Result<String, ApiError> {
        let temp_dest_path = generate_temp_path(TEMP_IMAGE_FOLDER)?;
        let random_file_name = generate_random_key();
        let filename = format!("{random_file_name}.wepb");
        let temp_file_path = format!("{temp_dest_path}/{filename}");

        let resize_image_use_case = ResizeImageUsecase::new(&image.data, temp_file_path.as_str());
        resize_image_use_case.execute().await?;
        
        let key = format!("{TIERLIST_IMAGE_S3_FOLDER}/{filename}");
        let url_result = self.repo.upload_image(temp_file_path.as_str(), key.as_str()).await;

        if let Err(e) = fs::remove_file(&temp_file_path) {
            tracing::warn!("Failed to delete temp file {}: {:?}", temp_file_path, e);
        }

        url_result
    }
    
    pub async fn updload_many_images(&self, images: Vec<ImageEntity>) -> Result<Vec<String>, ApiError> {
        let mut urls = Vec::new();
        let mut errors = Vec::new();
        
        for image in images.clone() {
            let result = self.upload_image(image).await;
            
            if result.is_ok() {
               urls.push(result.unwrap()) 
            } else {
                errors.push(result.err().unwrap());
            }
        }

        if errors.len() == images.len() {
            let mut error_message = String::new();

            for error in errors {
                error_message.push_str("\n\t");
                error_message.push_str(error.to_string().as_str());
            }

            return Err(ApiError::InternalError(format!("Errors uploading files : {error_message}")));
        }
        
        Ok(urls)
    }
}

fn generate_random_key() -> String {
    let mut rng = OsRng;
    let mut token = [0u8; 24]; // 24 bytes → 32 base64 chars
    rng.fill_bytes(&mut token);

    // encode to base64 safe to use in URls
    URL_SAFE_NO_PAD.encode(&token)
}

fn generate_temp_path(subfolder: &str) -> Result<String, ApiError> {
    let mut path = env::temp_dir(); // e.g. /tmp on Linux
    path.push(subfolder);

    let path_string = path.to_string_lossy().into_owned();

    std::fs::create_dir_all(&path)
        .map_err(|e| ApiError::InternalError(format!("Failed to create temp dir {path_string} : {e}")))?;

    Ok(path_string)
}