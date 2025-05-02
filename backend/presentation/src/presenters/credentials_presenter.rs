use domain::entities::CredentialsEntity;
use domain::mappers::EntityMapper;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CredentialsPresenter {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
}

impl EntityMapper<CredentialsEntity> for CredentialsPresenter {
    fn to_entity(self) -> CredentialsEntity {
        CredentialsEntity {
            email: self.email,
            password: self.password,
            username: self.username
        }
    }
}

impl From<CredentialsEntity> for CredentialsPresenter {
    fn from(value: CredentialsEntity) -> Self {
        Self {
            email: value.email,
            password: value.password,
            username: value.username
        }
    }
}