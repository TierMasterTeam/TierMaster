use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_new::new;
use domain::error::ApiError;

#[derive(new)]
pub struct ApiErrorResponse {
    pub code: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

impl From<ApiError> for ApiErrorResponse {
    fn from(value: ApiError) -> Self {
        let (code, message) = match value {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg)
        };

        Self { code, message }
    }
}

impl From<MultipartError> for ApiErrorResponse {
    fn from(value: MultipartError) -> Self {
        Self {
            code: value.status(),
            message: value.body_text(),
        }
    }
}