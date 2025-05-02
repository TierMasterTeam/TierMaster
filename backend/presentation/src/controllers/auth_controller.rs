use crate::error::ApiErrorResponse;
use crate::presenters::{CredentialsPresenter, UserPresenter};
use application::AppState;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
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
    
    let (token, user) = state.services
        .auth()
        .login(credentials.to_entity())
        .await?;

    make_auth_response(token, UserPresenter::from(user))
}

async fn signup(    
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<CredentialsPresenter>,
) -> Result<impl IntoResponse, ApiErrorResponse> {

    let (token, user) = state.services
        .auth()
        .signup(credentials.to_entity())
        .await?;

    make_auth_response(token, UserPresenter::from(user))
}

fn make_auth_response(token: String, user: UserPresenter) -> Result<impl IntoResponse, ApiErrorResponse> {
    // TODO adding our website domain when dotenv is setup ( Domain=.tiermaster.app )
    let cookie_value = format!("token={token}; HttpOnly; Secure; Path=/; SameSite=Strict; Max-Age=3600"); // 1 hour
    let parsed_cookie = cookie_value.parse()
        .map_err(|e| ApiError::InternalError(format!("Failed to parse cookie string : {e}")))?;

    let mut response = Json(user).into_response();
    response.headers_mut().insert(SET_COOKIE, parsed_cookie);

    Ok(response)
}