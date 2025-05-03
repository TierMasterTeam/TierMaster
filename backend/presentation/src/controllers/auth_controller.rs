use crate::error::ApiErrorResponse;
use crate::presenters::{CredentialsPresenter, UserPresenter};
use crate::states::AuthSession;
use application::AppState;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use domain::error::ApiError;
use domain::mappers::EntityMapper;
use std::sync::Arc;

pub struct AuthController;

impl AuthController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/login", post(login))
            .route("/signup", post(signup))
            .route("/logout", post(logout))
            .route("/me", get(me));

        Router::new()
            .nest("/auth", router)
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

async fn logout(
    auth_session: AuthSession,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiErrorResponse> {

    state.services
        .auth()
        .logout(auth_session.token)
        .await?;

    Ok(())
}

async fn me(
    auth_session: AuthSession,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiErrorResponse> {

    let user = state.services
        .user()
        .get_by_id(auth_session.user_id.as_str())
        .await?;

    Ok(Json(UserPresenter::from(user)))
}

fn make_auth_response(token: String, user: UserPresenter) -> Result<impl IntoResponse, ApiErrorResponse> {
    // TODO adding our website domain when dotenv is setup ( Domain=.tiermaster.app )
    // TODO adding Secure; when environment is prod to only support https
    let cookie_value = format!("token={token}; HttpOnly; Path=/; Max-Age=3600"); // 1 hour
    let parsed_cookie = cookie_value.parse()
        .map_err(|e| ApiError::InternalError(format!("Failed to parse cookie string : {e}")))?;

    let mut response = Json(user).into_response();
    response.headers_mut().insert(SET_COOKIE, parsed_cookie);

    Ok(response)
}