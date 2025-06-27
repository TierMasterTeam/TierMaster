use crate::databases::Databases;
use crate::repositories::auth_repository::AuthRepository;
use crate::repositories::image_repository::ImageRepository;
use crate::repositories::redis_repository::RedisRepository;
use crate::repositories::template_repository::TemplateRepository;
use crate::repositories::tierlist_repository::TierlistRepository;
use crate::repositories::user_repository::UserRepository;
use domain::repositories::{AbstractAuthRepository, AbstractImageRepository, AbstractRedisRepository, AbstractRepositoryFactory, AbstractTemplateRepository, AbstractTierlistRepository, AbstractUserRepository};
use std::sync::Arc;

pub struct RepositoryFactory {
    tierlist: Arc<TierlistRepository>,
    template: Arc<TemplateRepository>,
    user: Arc<UserRepository>,
    auth: Arc<AuthRepository>,
    redis: Arc<RedisRepository>,
    image: Arc<ImageRepository>
}

impl RepositoryFactory {
    pub fn init(db: &Databases) -> Self {
        RepositoryFactory {
            tierlist: Arc::new(TierlistRepository::new(db.mongo())),
            template: Arc::new(TemplateRepository::new(db.mongo())),
            auth: Arc::new(AuthRepository::new(db.mongo())),
            redis: Arc::new(RedisRepository::new(db.redis())),
            image: Arc::new(ImageRepository::new(db.aws())),
            user: Arc::new(UserRepository::new(db.mongo())),
        }
    }
}

impl AbstractRepositoryFactory for RepositoryFactory {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository> {
        self.tierlist.clone()
    }

    fn template(&self) -> Arc<dyn AbstractTemplateRepository> {
        self.template.clone()
    }

    fn image(&self) -> Arc<dyn AbstractImageRepository> {
        self.image.clone()
    }

    fn user(&self) -> Arc<dyn AbstractUserRepository> {
        self.user.clone()
    }

    fn auth(&self) -> Arc<dyn AbstractAuthRepository> {
        self.auth.clone()
    }

    fn redis(&self) -> Arc<dyn AbstractRedisRepository> {
        self.redis.clone()
    }
}