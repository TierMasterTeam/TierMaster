use domain::entities::UserEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPresenter {
    pub id: String,
    pub email: String,
    pub username: String,
}

impl From<UserEntity> for UserPresenter {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id,
            email: value.email,
            username: value.username,
        }
    }
}

impl EntityMapper<UserEntity> for UserPresenter {
    fn to_entity(self) -> UserEntity {
        UserEntity {
            id: self.id,
            email: self.email,
            username: self.username,
            password_hash: None,
        }
    }
}