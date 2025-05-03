use std::sync::Arc;
use domain::repositories::AbstractRepositoryFactory;
use crate::services::ServiceFactory;

pub struct AppState {
    pub services: ServiceFactory,
    pub repositories: Arc<dyn AbstractRepositoryFactory>,
}

impl AppState{
    pub fn new(repo: Arc<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            services: ServiceFactory::init(repo.clone()),
            repositories: repo,
        }
    }

    pub fn services(&self) -> &ServiceFactory {
        &self.services
    }
    pub fn repositories(&self) -> Arc<dyn AbstractRepositoryFactory> {self.repositories.clone()}
}