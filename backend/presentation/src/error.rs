use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use domain::error::ApiError;

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
        };

        Self { code, message }
    }
}