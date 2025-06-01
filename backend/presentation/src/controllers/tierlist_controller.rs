use crate::error::ApiErrorResponse;
use crate::presenters::{CreateTierlistPresenter, SearchQueryPresenter, TierlistPresenter, UpdateTierlistPresenter};
use crate::states::AuthSession;
use application::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use axum_extra::json;
use domain::mappers::EntityMapper;
use std::sync::Arc;

pub struct TierlistController;

impl TierlistController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/", post(create_tierlist))
            .route("/", get(get_all_tierlists))
            .route("/{id}", get(get_tierlist_by_id))
            .route("/{id}", put(update_tierlist_by_id))
            .route("/user/{id}", get(get_tierlists_of_user))
            .route("/search", get(search_tierlist));

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
        .get_tierlist_by_id(id.as_str())
        .await?;

    let is_not_author_of_tierlist = auth.user_id.ne(&original_tierlist.author);
    let updating_is_public =  original_tierlist.is_public.ne(&tierlist.is_public);
    if is_not_author_of_tierlist && updating_is_public {
        return Err(ApiErrorResponse::new(
            StatusCode::UNAUTHORIZED,
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
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<TierlistPresenter, ApiErrorResponse> {
    let result = state.services()
        .tierlist()
        .get_tierlist_by_id(id.as_str())
        .await?;

    Ok(TierlistPresenter::from(result))
}

async fn get_tierlists_of_user(
    _auth: AuthSession,
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TierlistPresenter>>, ApiErrorResponse> {
    
    // TODO : add logic to check if user (auth.user_id) has the right to see tierlist 
    
    let result = state.services()
        .tierlist()
        .get_tierlists_of_user(id.as_str())
        .await?;

    Ok(Json(result.into_iter().map(TierlistPresenter::from).collect()))
}

async fn search_tierlist(
    Query(params): Query<SearchQueryPresenter>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TierlistPresenter>>, ApiErrorResponse> {

    let result = state.services()
        .tierlist()
        .search(params.query.as_str())
        .await?;

    Ok(Json(result.into_iter().map(TierlistPresenter::from).collect()))
}