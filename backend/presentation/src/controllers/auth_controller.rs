use crate::error::ApiErrorResponse;
use crate::presenters::CredentialsPresenter;
use application::AppState;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use domain::error::ApiError;
use domain::mappers::EntityMapper;
use std::sync::Arc;

pub struct AuthController;

impl AuthController {
    pub fn get_router() -> Router<Arc<AppState>> {
        Router::new()
            .route("/login", post(login))
            .route("/signup", post(signup))
    }
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<CredentialsPresenter>,
) -> Result<impl IntoResponse, ApiErrorResponse> {
    
    let token = state.services
        .auth()
        .login(credentials.to_entity())
        .await?;
    
    let cookie_value = format!("token={token}; HttpOnly; Path=/; Max-Age=3600"); // 1 hour
    
    let mut response = StatusCode::OK.into_response();
    response.headers_mut().insert(SET_COOKIE, cookie_value.parse().unwrap());

    Ok(response)
}

async fn signup(    
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<CredentialsPresenter>,
) -> Result<impl IntoResponse, ApiErrorResponse> {

    let token = state.services
        .auth()
        .signup(credentials.to_entity())
        .await?;

    let cookie_value = format!("token={token}; HttpOnly; Secure; Path=/; Max-Age=3600"); // 1 hour
    let parsed_cookie = cookie_value.parse()
        .map_err(|e| ApiError::InternalError(format!("Failed to parse cookie string : {e}")))?;

    let mut response = StatusCode::OK.into_response();
    response.headers_mut().insert(SET_COOKIE, parsed_cookie);

    Ok(response)
}