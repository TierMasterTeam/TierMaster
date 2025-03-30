use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::post;
use crate::presentation::controllers::AxumController;
use crate::presentation::presenters::tl_presenter::TlPresenter;

pub struct TlController;

impl AxumController for TlController {
    fn get_router() -> Router {
        Router::new()
            .route("/", post(TlController::create_tl_handler))
    }
}

impl TlController{
    async fn create_tl_handler(Json(_tl_presenter): Json<TlPresenter>) -> StatusCode {
        StatusCode::CREATED
    }
}