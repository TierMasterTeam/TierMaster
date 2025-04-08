use axum::routing::MethodRouter;

pub trait Route<S> where S: Clone + Send + Sync + 'static {
    fn path(&self) -> &str;
    fn method(&self) -> MethodRouter<S>;
}

pub trait Controller<S> where S: Clone + Send + Sync + 'static {
    fn get_routes(&self) -> Vec<Box<dyn Route<S>>>;
}