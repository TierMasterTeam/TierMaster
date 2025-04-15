use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use crate::core::app_state::{AppState};
use crate::domain::mapper::EntityMapper;
use crate::presentation::error::ApiErrorResponse;
use crate::presentation::presenters::create_tierlist_presenter::CreateTierlistPresenter;
use crate::presentation::presenters::tierlist_presenter::TierlistPresenter;

pub struct TierlistController;

impl TierlistController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/", post(create_tierlist))
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

    state.service_factory()
        .tierlist_service()
        .create_tierlist(tierlist)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn get_tierlists_of_user(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TierlistPresenter>>, ApiErrorResponse> {

    let result = state.service_factory()
        .tierlist_service()
        .get_tierlists_of_user(id.as_str())
        .await?;

    Ok(Json(result.into_iter().map(TierlistPresenter::from).collect()))
}