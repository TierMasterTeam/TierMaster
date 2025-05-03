use std::sync::Arc;
use domain::repositories::{AbstractRepositoryFactory, AbstractTierlistRepository, AbstractImageRepository};
use crate::databases::Databases;
use crate::repositories::image_repository::ImageRepository;
use crate::databases::RedisDb;
use crate::repositories::auth_repository::AuthRepository;
use crate::repositories::redis_repository::RedisRepository;
use crate::repositories::tierlist_repository::TierlistRepository;
use crate::repositories::user_repository::UserRepository;
use domain::repositories::{AbstractAuthRepository, AbstractRedisRepository, AbstractRepositoryFactory, AbstractTierlistRepository, AbstractUserRepository};
use mongodb::Database;
use std::sync::Arc;

pub struct RepositoryFactory {
    tierlist: Arc<TierlistRepository>,
    user: Arc<UserRepository>,
    auth: Arc<AuthRepository>,
    redis: Arc<RedisRepository>
}

impl RepositoryFactory {
    pub fn init(db: &Databases, redis: RedisDb) -> Self {
        RepositoryFactory {
            tierlist: Arc::new(TierlistRepository::new(db.mongo())),
            tierlist: Arc::new(TierlistRepository::new(db)),
            user: Arc::new(UserRepository::new(db)),
            auth: Arc::new(AuthRepository::new(db)),
            redis: Arc::new(RedisRepository::new(redis)),
            image: Arc::new(ImageRepository::new()),
        }
    }
}

impl AbstractRepositoryFactory for RepositoryFactory {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository> {
        self.tierlist.clone()
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

    fn image(&self) -> Arc<dyn AbstractImageRepository> {
        self.image.clone()
    }
}