use mongodb::Database;
use crate::application::service_factory::ServiceFactory;
use crate::data::repository_factory::RepositoryFactory;

/// An application state containing information such as databases connections.
pub struct AppState {
    service_factory: ServiceFactory,
}
impl AppState {
    pub fn init(database: Database) -> Self {
        let repository_factory = RepositoryFactory::init(&database);
        Self {
            service_factory: ServiceFactory::init(&repository_factory)
        }
    }

    pub fn service_factory(&self) -> &ServiceFactory {
        &self.service_factory
    }
}