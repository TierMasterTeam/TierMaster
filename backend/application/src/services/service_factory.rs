use crate::services::image_service::ImageService;
use crate::services::{AuthService, TierlistService, UserService};
use domain::repositories::AbstractRepositoryFactory;
use std::sync::Arc;

pub struct ServiceFactory {
    tierlist: TierlistService,
    auth: AuthService,
    user: UserService,
    image: ImageService,
}

impl ServiceFactory {
    pub fn init(factory: Arc<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            tierlist: TierlistService::new(factory.tierlist()),
            user: UserService::new(factory.user()),
            auth: AuthService::new(factory.auth(), factory.redis()),
            image: ImageService::new(factory.image())
        }
    }

    pub fn tierlist(&self) -> &TierlistService {
        &self.tierlist
    }

    pub fn user(&self) -> &UserService {
        &self.user
    }

    pub fn auth(&self) -> &AuthService {&self.auth}

    pub fn image(&self) -> &ImageService {&self.image}
}