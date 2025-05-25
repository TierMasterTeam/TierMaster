use crate::error::ApiErrorResponse;
use application::AppState;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;

const BODY_SIZE_LIMIT: usize = 20 * 1024 * 1024;

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
    
    let mut images = Vec::new();
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().map(str::to_string).unwrap_or_default();
        
        if !name.starts_with("image") {
            continue;
        }
        
        let data = field.bytes().await?.clone();

        images.push(data);
    }

    let urls = state.services.image().upload_images(images).await?;

    Ok(Json(urls))
}