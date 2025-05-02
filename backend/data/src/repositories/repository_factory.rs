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
    pub fn init(db: &Database, redis: RedisDb) -> Self {
        RepositoryFactory {
            tierlist: Arc::new(TierlistRepository::new(db)),
            user: Arc::new(UserRepository::new(db)),
            auth: Arc::new(AuthRepository::new(db)),
            redis: Arc::new(RedisRepository::new(redis)),
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
}