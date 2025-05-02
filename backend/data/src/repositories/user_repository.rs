use crate::models::UserModel;
use async_trait::async_trait;
use domain::entities::UserEntity;
use domain::error::ApiError;
use domain::mappers::TryEntityMapper;
use domain::repositories::AbstractUserRepository;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<UserModel>
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("user");
        Self { collection }
    }
}

#[async_trait]
impl AbstractUserRepository for UserRepository {

    async fn get_by_id(&self, id: &str) -> Result<UserEntity, ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let user = self.collection.find_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        match user {
            None => Err(ApiError::NotFound(format!("User with id {id} not found"))),
            Some(user) => Ok(user.to_entity())
        }
    }
}
