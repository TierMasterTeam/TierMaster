use std::sync::Arc;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{post};
use application::AppState;
use application::usecases::UploadImagesUsecase;
use crate::error::ApiErrorResponse;

const BODY_SIZE_LIMIT: usize = 97 * 1024;
const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png"];

pub struct ImageController;

impl ImageController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/", post(upload_images))
            .layer(DefaultBodyLimit::max(BODY_SIZE_LIMIT));

        Router::new()
            .nest("/image", router)
    }
}

async fn upload_images(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart
) -> Result<Json<Vec<String>>, ApiErrorResponse> {
    //1- Fetching images in the body.
    let mut images = Vec::new();
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default();
        if name != "image" {
            continue;
        }

        let content_type = field.content_type().unwrap_or_default();
        if !ALLOWED_MIME_TYPES.contains(&content_type) {
            return Err(ApiErrorResponse::new(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                format!("Unsupported content type: {content_type}")
            ));
        }

        let data = field.bytes().await?;
        
        images.push(data);
    }
    
    //2- Uploading found images
    let urls = UploadImagesUsecase::new(
        state.repositories.image(),
        images,
    ).execute().await?;
    
    Ok(Json(urls))
}