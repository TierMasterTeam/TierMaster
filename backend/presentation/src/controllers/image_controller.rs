use crate::error::ApiErrorResponse;
use application::AppState;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use domain::entities::ImageEntity;
use std::sync::Arc;

const BODY_SIZE_LIMIT: usize = 5 * 1024 * 1024;
const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png"];

pub struct ImageController;

impl ImageController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/", post(upload_images));
        
        Router::new()
            .nest("/image", router)
            .layer(DefaultBodyLimit::max(BODY_SIZE_LIMIT))
    }
}

async fn upload_images(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart
) -> Result<Json<Vec<String>>, ApiErrorResponse> {
    println!("UPLOADING IMAGES");

    //1- Fetching images in the body.
    let mut images = Vec::new();
    while let Some(mut field) = multipart.next_field().await? {
        let name = field.name().map(str::to_string).unwrap_or_default();
        if !name.starts_with("image") {
            continue;
        }

        let content_type = field.content_type().map(str::to_string).unwrap_or_default();
        if !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
            return Err(ApiErrorResponse::new(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                format!("Unsupported content type: {content_type}")
            ));
        }

        let image_name = field.file_name().map(str::to_string).unwrap_or_else(|| "image".to_string());

        // Only do this after extracting metadata
        let data = field.bytes().await?.clone();

        images.push(ImageEntity {
            content_type,
            image_name,
            data,
        });
    }

    let urls = state.services.image().updload_many_images(images).await?;
    
    Ok(Json(urls))
}