use crate::services::ServiceFactory;
use domain::repositories::AbstractRepositoryFactory;
use std::sync::Arc;

pub struct AppState {
    pub services: ServiceFactory,
}

impl AppState{
    pub fn new(repo: Arc<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            services: ServiceFactory::init(repo),
        }
    }

    pub fn services(&self) -> &ServiceFactory {
        &self.services
    }
}