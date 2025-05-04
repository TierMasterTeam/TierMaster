use crate::repositories::{AbstractAuthRepository, AbstractImageRepository, AbstractRedisRepository, AbstractTierlistRepository, AbstractUserRepository};
use std::sync::Arc;

pub trait AbstractRepositoryFactory: Send + Sync {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository>;
    fn image(&self) -> Arc<dyn AbstractImageRepository>;
    fn user(&self) -> Arc<dyn AbstractUserRepository>;
    fn auth(&self) -> Arc<dyn AbstractAuthRepository>;
    fn redis(&self) -> Arc<dyn AbstractRedisRepository>;
}