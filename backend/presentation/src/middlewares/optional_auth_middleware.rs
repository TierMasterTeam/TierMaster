use crate::error::ApiErrorResponse;
use crate::states::{AuthSession, OptionalAuthSession};
use application::AppState;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::headers::Cookie;
use axum_extra::typed_header::TypedHeader;
use std::sync::Arc;

#[async_trait]
impl FromRequestParts<Arc<AppState>> for OptionalAuthSession {
    type Rejection = ApiErrorResponse;

    fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> impl Future<Output = Result<OptionalAuthSession, ApiErrorResponse>> {
        Box::pin(async move {
            let result_cookies = TypedHeader::<Cookie>::from_request_parts(parts, state)
                .await;

            if result_cookies.is_err() {
                return Ok(OptionalAuthSession::none());
            }

            let cookies = result_cookies.unwrap();
            let signed_token = cookies.get("token");

            if signed_token.is_none() {
                return Ok(OptionalAuthSession::none());
            }

            if state.services.auth().verify_token(signed_token.unwrap()).is_err() {
                return Ok(OptionalAuthSession::none());
            }

            let result = state
                .services
                .auth()
                .validate_session(signed_token.unwrap())
                .await;

            if result.is_err() {
                return Ok(OptionalAuthSession::none());
            }

            let (token, user_id) = result.unwrap();
            Ok(OptionalAuthSession::some(AuthSession {
                token,
                user_id
            }))
        })
    }
}