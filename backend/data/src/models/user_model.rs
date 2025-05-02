use domain::entities::{CreateUserEntity, UserEntity};
use domain::mappers::TryEntityMapper;
use mongodb::bson::oid::{Error, ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>, // can't update password without specific route
    pub username: String,
}

impl TryFrom<UserEntity> for UserModel {
    type Error = Error;

    fn try_from(value: UserEntity) -> Result<Self, Self::Error> {
        let id = ObjectId::parse_str(value.id)?;

        Ok(Self {
            id: Some(id),
            email: value.email,
            password_hash: value.password_hash,
            username: value.username
        })
    }
}

impl TryEntityMapper<UserEntity> for UserModel {
    fn to_entity(self) -> UserEntity {
        UserEntity {
            id: self.id.unwrap_or_default().to_string(),
            email: self.email,
            password_hash: self.password_hash,
            username: self.username
        }
    }
}

impl From<CreateUserEntity> for UserModel {
    fn from(value: CreateUserEntity) -> Self{
        Self {
            id: None,
            email: value.email,
            password_hash: Some(value.password_hash),
            username: value.username
        }
    }
}