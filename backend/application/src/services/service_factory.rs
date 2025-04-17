use domain::repositories::AbstractRepositoryFactory;
use crate::services::TierlistService;

pub struct ServiceFactory {
    tierlist: TierlistService,
}

impl ServiceFactory {
    pub fn init(factory: Box<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            tierlist: TierlistService::new(factory.tierlist()),
        }
    }

    pub fn tierlist(&self) -> &TierlistService {
        &self.tierlist
    }
}