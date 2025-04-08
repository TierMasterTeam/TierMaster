use crate::domain::errors::api_error::ApiError;
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub fn api_error_to_response(api_error: ApiError) -> Response {
    match api_error {
        ApiError::NotFound(msg) =>
            (StatusCode::OK, Json(json!({ "NotFound": msg }))).into_response(),
        ApiError::InternalError(msg) =>
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "InternalError": msg }))).into_response(),
        ApiError::BadRequest(msg) =>
            (StatusCode::BAD_REQUEST, Json(json!({ "BadRequest": msg }))).into_response(),
    }
}