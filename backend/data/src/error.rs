use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use domain::error::ApiError;

#[derive(Debug)]
pub struct DatabaseError(ApiError);

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.api_err())
    }
}

impl Error for DatabaseError {}

impl From<ApiError> for DatabaseError {
    fn from(value: ApiError) -> Self {
        Self(value)
    }
}

impl From<mongodb::error::Error> for DatabaseError {
    fn from(value: mongodb::error::Error) -> Self {
        Self(ApiError::InternalError(value.to_string()))
    }
}

impl DatabaseError {
    pub fn api_err(&self) -> &ApiError {
        &self.0
    }
}