use crate::domain::errors::Error;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    InternalError(String),
    BadRequest(String),
}
impl Error for ApiError {}