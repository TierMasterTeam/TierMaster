use domain::repositories::AbstractRepositoryFactory;
use crate::services::ServiceFactory;

pub struct AppState {
    pub services: ServiceFactory,
}

impl AppState{
    pub fn new(repo: Box<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            services: ServiceFactory::init(repo),
        }
    }

    pub fn services(&self) -> &ServiceFactory {
        &self.services
    }
}