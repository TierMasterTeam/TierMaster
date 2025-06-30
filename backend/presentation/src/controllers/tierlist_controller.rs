use crate::error::ApiErrorResponse;
use crate::presenters::{CreateTierlistPresenter, TierlistPresenter, UpdateTierlistPresenter};
use crate::states::{AuthSession, OptionalAuthSession};
use application::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use axum_extra::json;
use domain::error::ApiError;
use domain::mappers::EntityMapper;
use std::sync::Arc;

pub struct TierlistController;

impl TierlistController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/", post(create_tierlist))
            .route("/", get(get_all_tierlists))
            .route("/{id}", get(get_tierlist_by_id))
            .route("/{id}", delete(delete_by_id))
            .route("/{id}", put(update_tierlist_by_id))
            .route("/user/{id}", get(get_tierlists_of_user));

        Router::new()
            .nest("/tierlist", router)
    }
}

async fn create_tierlist (
    _auth: AuthSession,
    State(state): State<Arc<AppState>>,
    Json(tierlist): Json<CreateTierlistPresenter>,
) -> Result<impl IntoResponse, ApiErrorResponse> {
    let tierlist = tierlist.to_entity();

    let result = state.services()
        .tierlist()
        .create_tierlist(tierlist)
        .await?;

    Ok((StatusCode::CREATED, json!({"id": result})))
}

async fn update_tierlist_by_id(
    auth: AuthSession,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(tierlist): Json<UpdateTierlistPresenter>,
) -> Result<StatusCode, ApiErrorResponse> {
    let tierlist = tierlist.to_entity();

    let original_tierlist = state.services()
        .tierlist()
        .get_tierlist_by_id(id.as_str(), Some(auth.user_id.clone()))
        .await?;

    let is_not_author_of_tierlist = auth.user_id.ne(&original_tierlist.author);
    let updating_is_public = original_tierlist.is_public.ne(&tierlist.is_public);
    if is_not_author_of_tierlist && updating_is_public {
        return Err(ApiErrorResponse::new(
            StatusCode::FORBIDDEN,
            "Only the author has the right to change the visibility of a tierlist.".to_string()
        ));
    }

    state.services()
        .tierlist()
        .update_tierlist_by_id(id.as_str(), tierlist)
        .await?;

    Ok(StatusCode::OK)
}

async fn get_all_tierlists(State(state): State<Arc<AppState>>) -> Result<Json<Vec<TierlistPresenter>>, ApiErrorResponse> {
    let result = state.services()
        .tierlist()
        .get_all_tierlists()
        .await?;

    Ok(Json(result.into_iter().map(TierlistPresenter::from).collect()))
}

async fn get_tierlist_by_id(
    auth: OptionalAuthSession,
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<TierlistPresenter, ApiErrorResponse> {
    
    let user_id = auth.auth_state.map(|session| session.user_id);
    
    let result = state.services()
        .tierlist()
        .get_tierlist_by_id(id.as_str(), user_id)
        .await?;

    Ok(TierlistPresenter::from(result))
}

async fn get_tierlists_of_user(
    auth: AuthSession,
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TierlistPresenter>>, ApiErrorResponse> {
    
    let result = state.services()
        .tierlist()
        .get_tierlists_of_user(id.as_str(), auth.user_id == id)
        .await?;

    Ok(Json(result.into_iter().map(TierlistPresenter::from).collect()))
}

async fn delete_by_id(
    auth: AuthSession,
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>
) -> Result<(), ApiErrorResponse> {

    let tierlist = state.services
        .tierlist()
        .get_tierlist_by_id(id.as_str(), Some(auth.user_id.clone()))
        .await?;

    if tierlist.author.ne(&auth.user_id.clone()) {
        let error = ApiError::Forbidden("You do not have permissions to delete this tierlist".to_string());
        return Err(ApiErrorResponse::from(error));
    }

    state.services
        .tierlist()
        .delete_by_id(id.as_str())
        .await?;

    Ok(())
}