use crate::error::ApiErrorResponse;
use crate::presenters::{CreateTemplatePresenter, SearchQueryPresenter, TemplatePresenter, UpdateTemplatePresenter};
use crate::states::{AuthSession, OptionalAuthSession};
use crate::types::PaginatedResponse;
use application::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use axum_extra::json;
use domain::mappers::EntityMapper;
use domain::types::Pagination;
use std::sync::Arc;

pub struct TemplateController;

impl TemplateController {
    pub fn get_router() -> Router<Arc<AppState>> {
        let router = Router::new()
            .route("/", post(create_template))
            .route("/{id}", get(get_template_by_id))
            .route("/{id}", put(update_template_by_id))
            .route("/user/{id}", get(get_templates_of_user))
            .route("/search", get(search_template));

        Router::new()
            .nest("/template", router)
    }
}

async fn create_template (
    _auth: AuthSession,
    State(state): State<Arc<AppState>>,
    Json(template): Json<CreateTemplatePresenter>,
) -> Result<impl IntoResponse, ApiErrorResponse> {
    let template = template.to_entity();

    let result = state.services()
        .template()
        .create_template(template)
        .await?;

    Ok((StatusCode::CREATED, json!({"id": result})))
}

async fn update_template_by_id(
    auth: AuthSession,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(template): Json<UpdateTemplatePresenter>,
) -> Result<StatusCode, ApiErrorResponse> {
    let template = template.to_entity();

    let original_template = state.services()
        .template()
        .get_template_by_id(id.as_str(), Some(auth.user_id.clone()))
        .await?;

    let is_not_author_of_template = auth.user_id.ne(&original_template.author);
    let updating_is_public =  original_template.is_public.ne(&template.is_public);
    if is_not_author_of_template && updating_is_public {
        return Err(ApiErrorResponse::new(
            StatusCode::FORBIDDEN,
            "Only the author has the right to change the visibility of a template.".to_string()
        ));
    }

    state.services()
        .template()
        .update_template_by_id(id.as_str(), template)
        .await?;

    Ok(StatusCode::OK)
}

async fn get_template_by_id(
    optional_auth_session: OptionalAuthSession,
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<TemplatePresenter, ApiErrorResponse> {
    let user_id = optional_auth_session.auth_state.map(|auth| auth.user_id);
    
    let result = state.services()
        .template()
        .get_template_by_id(id.as_str(), user_id)
        .await?;

    Ok(TemplatePresenter::from(result))
}

async fn get_templates_of_user(
    auth: AuthSession,
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TemplatePresenter>>, ApiErrorResponse> {

    let result = state.services()
        .template()
        .get_templates_of_user(id.as_str(), auth.user_id == id)
        .await?;

    Ok(Json(result.into_iter().map(TemplatePresenter::from).collect()))
}

async fn search_template(
    Query(params): Query<SearchQueryPresenter>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<PaginatedResponse<TemplatePresenter>>, ApiErrorResponse> {

    let pagination = Pagination {
        page: params.page,
        per_page: params.per_page
    };

    let result = state.services()
        .template()
        .search(params.query.as_str(), pagination.clone())
        .await?;

    let response = PaginatedResponse {
        page: pagination.page,
        per_page: pagination.per_page,
        data: result.into_iter().map(TemplatePresenter::from).collect(),
    };

    Ok(Json(response))
}