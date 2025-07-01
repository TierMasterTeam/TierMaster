use crate::error::ApiErrorResponse;
use crate::states::AuthSession;
use application::AppState;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::headers::Cookie;
use axum_extra::typed_header::TypedHeader;
use domain::error::ApiError;
use std::sync::Arc;

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthSession {
    type Rejection = ApiErrorResponse;

    fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> impl Future<Output = Result<AuthSession, ApiErrorResponse>> {
        Box::pin(async move {
            let cookies = TypedHeader::<Cookie>::from_request_parts(parts, state)
                .await
                .map_err(|_| ApiError::Unauthorized("Missing cookies".into()))?;

            let signed_token = cookies
                .get("token")
                .ok_or_else(|| ApiError::Unauthorized("Missing token cookie".into()))?;

            state.services.auth().verify_token(signed_token)
                .map_err(|_| ApiError::Unauthorized("Invalid or expired token".into()))?;

            let (token, user_id) = state
                .services
                .auth()
                .validate_session(signed_token)
                .await
                .map_err(|_| ApiError::Unauthorized("Invalid session".into()))?;
            
            
            Ok(AuthSession {
                token,
                user_id
            })
        })
    }
}