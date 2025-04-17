use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use application::AppState;
use domain::mappers::EntityMapper;
use crate::error::ApiErrorResponse;
use crate::presenters::{CreateTierlistPresenter, TierlistPresenter};

pub struct TierlistController;

impl TierlistController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/", post(create_tierlist))
            .route("/{id}", get(get_tierlist_by_id))
            .route("/user/{id}", get(get_tierlists_of_user));

        Router::new()
            .nest("/tierlist", router)
    }
}

async fn create_tierlist (
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

async fn get_tierlists_of_user(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TierlistPresenter>>, ApiErrorResponse> {

    let result = state.services()
        .tierlist()
        .get_tierlists_of_user(id.as_str())
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