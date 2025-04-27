use crate::error::ApiErrorResponse;
use crate::presenters::{CreateTierlistPresenter, TierlistPresenter, UpdateTierlistPresenter};
use crate::states::AuthSession;
use application::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use axum::{Json, Router};
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
            .route("/user/{id}", get(get_tierlists_of_user));

        Router::new()
            .nest("/tierlist", router)
    }
}

async fn create_tierlist (
    auth: AuthSession,
    State(state): State<Arc<AppState>>,
    Json(tierlist): Json<CreateTierlistPresenter>,
) -> Result<StatusCode, ApiErrorResponse> {
    let tierlist = tierlist.to_entity();

    state.services()
        .tierlist()
        .create_tierlist(tierlist)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn update_tierlist_by_id(
    auth: AuthSession,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(tierlist): Json<UpdateTierlistPresenter>,
) -> Result<StatusCode, ApiErrorResponse> {
    let tierlist = tierlist.to_entity();

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
    auth: AuthSession,
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