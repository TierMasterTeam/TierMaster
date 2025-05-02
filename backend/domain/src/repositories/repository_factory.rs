use crate::repositories::user_repository::AbstractUserRepository;
use crate::repositories::{AbstractAuthRepository, AbstractRedisRepository, AbstractTierlistRepository};
use std::sync::Arc;

pub trait AbstractRepositoryFactory {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository>;
    fn user(&self) -> Arc<dyn AbstractUserRepository>;
    fn auth(&self) -> Arc<dyn AbstractAuthRepository>;
    fn redis(&self) -> Arc<dyn AbstractRedisRepository>;
}