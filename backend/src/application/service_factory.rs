use crate::application::services::tierlist_service::TierlistService;
use crate::data::repository_factory::RepositoryFactory;

pub struct ServiceFactory {
    tierlist_service: TierlistService,
}

impl ServiceFactory {
    pub fn init(repository_factory: &RepositoryFactory) -> Self {
        Self {
            tierlist_service: TierlistService::new(repository_factory.tierlist_repository()),
        }
    }

    pub fn tierlist_service(&self) -> &TierlistService {
        &self.tierlist_service
    }
}