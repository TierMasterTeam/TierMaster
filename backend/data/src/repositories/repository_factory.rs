use crate::databases::RedisDb;
use crate::repositories::auth_repository::AuthRepository;
use crate::repositories::redis_repository::RedisRepository;
use crate::repositories::tierlist_repository::TierlistRepository;
use domain::repositories::{AbstractAuthRepository, AbstractRedisRepository, AbstractRepositoryFactory, AbstractTierlistRepository};
use mongodb::Database;
use std::sync::Arc;

pub struct RepositoryFactory {
    tierlist: Arc<TierlistRepository>,
    auth: Arc<AuthRepository>,
    redis: Arc<RedisRepository>
}

impl RepositoryFactory {
    pub fn init(db: &Database, redis: RedisDb) -> Self {
        RepositoryFactory {
            tierlist: Arc::new(TierlistRepository::new(db)),
            auth: Arc::new(AuthRepository::new(db)),
            redis: Arc::new(RedisRepository::new(redis)),
        }
    }
}

impl AbstractRepositoryFactory for RepositoryFactory {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository> {
        self.tierlist.clone()
    }

    fn auth(&self) -> Arc<dyn AbstractAuthRepository> {
        self.auth.clone()
    }

    fn redis(&self) -> Arc<dyn AbstractRedisRepository> {
        self.redis.clone()
    }
}