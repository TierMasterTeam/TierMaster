use crate::models::UserModel;
use async_trait::async_trait;
use domain::entities::{CreateUserEntity, UserEntity};
use domain::error::ApiError;
use domain::mappers::TryEntityMapper;
use domain::repositories::AbstractAuthRepository;
use mongodb::bson::doc;
use mongodb::{Collection, Database};

#[derive(Clone)]
pub struct AuthRepository {
    collection: Collection<UserModel>,
}

impl AuthRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("user");
        Self { collection }
    }
}

#[async_trait]
impl AbstractAuthRepository for AuthRepository {
    async fn get_user_by_email(&self, email: &str) -> Result<UserEntity, ApiError> {
        let query = doc! { "email": email };
        let user = self.collection.find_one(query).await
            .map_err(|e| ApiError::NotFound(format!("{e}")))?;

        match user {
            None => Err(ApiError::NotFound(format!("User with email {email} not found"))),
            Some(user) => Ok(user.to_entity())
        }
    }

    async fn create_user(&self, user: CreateUserEntity) -> Result<(), ApiError> {
        let user = UserModel::try_from(user)
            .map_err(|e| ApiError::InternalError(format!("Failed to parse user entity to user model : {e}")))?;
        
        self.collection.insert_one(user).await
            .map_err(|e| ApiError::InternalError(format!("Failed to create user {e}")))?;
        
        Ok(())
    }
}