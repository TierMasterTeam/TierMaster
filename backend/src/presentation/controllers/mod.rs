use axum::Router;

pub mod tl_controller;

pub trait AxumController{
    fn get_router() -> Router;
}