use crate::domain::services::tierlist_service;
use crate::presentation::controllers::controller::{Controller, Route};
use crate::presentation::utils::api_error_utils::api_error_to_response;
use axum::response::Response;
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    routing::MethodRouter,
    Json
};

pub struct TierListController;

impl<S> Controller<S> for TierListController
where S: Clone + Send + Sync + 'static {
    fn get_routes(&self) -> Vec<Box<dyn Route<S>>> {
        let get_tierlists_of_user_route = Box::new(GerTierlistsOfUserRoute);

        vec![
            get_tierlists_of_user_route
        ]
    }
}

struct GerTierlistsOfUserRoute;

impl<S> Route<S> for GerTierlistsOfUserRoute
where S: Clone + Send + Sync + 'static {
    fn path(&self) -> &str {
        "/tierlist/user/{id}"
    }

    fn method(&self) -> MethodRouter<S> {
        get(get_tierlists_of_user)
    }
}

async fn get_tierlists_of_user(Path(id): Path<String>) -> Response {
    match tierlist_service::get_tierlists_of_user(&id).await {
        Ok(tierlists) => (StatusCode::OK, Json(tierlists)).into_response(),
        Err(err) => api_error_to_response(err),
    }
}