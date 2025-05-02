use crate::services::{AuthService, TierlistService, UserService};
use domain::repositories::AbstractRepositoryFactory;

pub struct ServiceFactory {
    tierlist: TierlistService,
    auth: AuthService,
    user: UserService,
}

impl ServiceFactory {
    pub fn init(factory: Box<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            tierlist: TierlistService::new(factory.tierlist()),
            user: UserService::new(factory.user()),
            auth: AuthService::new(factory.auth(), factory.redis())
        }
    }

    pub fn tierlist(&self) -> &TierlistService {
        &self.tierlist
    }

    pub fn user(&self) -> &UserService {
        &self.user
    }

    pub fn auth(&self) -> &AuthService {&self.auth}
}