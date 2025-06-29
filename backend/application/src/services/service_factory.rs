use crate::services::image_service::ImageService;
use crate::services::template_service::TemplateService;
use crate::services::websocket_service::WebsocketService;
use crate::services::{AuthService, TierlistService, UserService};
use domain::repositories::AbstractRepositoryFactory;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceFactory {
    tierlist: TierlistService,
    template: TemplateService,
    auth: AuthService,
    user: UserService,
    image: ImageService,
    websocket: WebsocketService,
}

impl ServiceFactory {
    pub fn init(factory: Arc<dyn AbstractRepositoryFactory>) -> Self {
        Self {
            tierlist: TierlistService::new(factory.tierlist()),
            template: TemplateService::new(factory.template()),
            user: UserService::new(factory.user()),
            auth: AuthService::new(factory.auth(), factory.redis()),
            image: ImageService::new(factory.image()),
            websocket: WebsocketService::new()
        }
    }

    pub fn tierlist(&self) -> &TierlistService {
        &self.tierlist
    }

    pub fn template(&self) -> &TemplateService {
        &self.template
    }

    pub fn user(&self) -> &UserService {
        &self.user
    }

    pub fn auth(&self) -> &AuthService {&self.auth}

    pub fn image(&self) -> &ImageService {&self.image}

    pub fn websocket(&self) -> &WebsocketService {&self.websocket}
}