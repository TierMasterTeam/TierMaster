use crate::services::{AuthService, TierlistService};
use domain::repositories::AbstractRepositoryFactory;

pub struct ServiceFactory {
    tierlist: TierlistService,
    auth: AuthService,
}

impl ServiceFactory {
    pub fn init(factory: Box<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            tierlist: TierlistService::new(factory.tierlist()),
            auth: AuthService::new(factory.auth(), factory.redis())
        }
    }

    pub fn tierlist(&self) -> &TierlistService {
        &self.tierlist
    }

    pub fn auth(&self) -> &AuthService {&self.auth}
}