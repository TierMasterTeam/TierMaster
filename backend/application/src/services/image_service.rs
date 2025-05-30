use std::io::Cursor;
use webp::Encoder;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use derive_new::new;
use domain::error::ApiError;
use domain::repositories::AbstractImageRepository;
use std::sync::Arc;
use bytes::Bytes;
use domain::helpers::ImageHelper;
use futures::stream::{FuturesUnordered, StreamExt};
use image::ImageReader;


const TIERLIST_IMAGE_FOLDER: &str = "tierlist_images";
const IMAGE_QUALITY: f32 = 75f32;

#[derive(new)]
pub struct ImageService {
    repo: Arc<dyn AbstractImageRepository>
}

impl ImageService {
    async fn upload_image(&self, image: Bytes) -> Result<String, ApiError> {
        let image_id = generate_random_key();
        let cleaned_img = clean_image(image)?;
        let key = format!("{TIERLIST_IMAGE_FOLDER}/{image_id}.webp");
        self.repo.upload_image(cleaned_img, key.as_str()).await
    }
    
    pub async fn upload_images(&self, images: Vec<Bytes>) -> Result<Vec<String>, ApiError> {
        let mut urls = Vec::new();
        let mut errors = Vec::new();
        let mut futures = FuturesUnordered::new();

        for image in images {
            let future = self.upload_image(image);
            futures.push(future);
        }

        while let Some(result) = futures.next().await {
            match result {
                Ok(url) => { urls.push(url); },
                Err(e) => { errors.push(e); },
            }
        }
        
        if urls.is_empty() && !errors.is_empty() {
            return Err(errors.first().unwrap().clone());
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

fn clean_image(bytes: Bytes) -> Result<Bytes, ApiError> {
    
    let image = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| ApiError::BadRequest(format!("Invalid Image : {e}")))?
        .decode()
        .map_err(|e| ApiError::BadRequest(format!("Invalid Image : {e}")))?
        .resize_to_fit(480)
        .crop_to_square();

    let webp = Encoder::from_image(&image)
        .map_err(|e| ApiError::BadRequest(format!("Image encoding failed : {e}")))?
        .encode(IMAGE_QUALITY);

    Ok(Bytes::copy_from_slice(&webp))
}