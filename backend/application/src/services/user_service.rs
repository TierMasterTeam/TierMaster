use derive_new::new;
use domain::entities::UserEntity;
use domain::error::ApiError;
use domain::repositories::AbstractUserRepository;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct UserService {
    repo: Arc<dyn AbstractUserRepository>,
}

impl UserService {
    pub async fn get_by_id(&self, id: &str) -> Result<UserEntity, ApiError> {
        self.repo.get_by_id(id).await
    }
}
