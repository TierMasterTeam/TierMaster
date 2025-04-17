use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use domain::entities::{CreateTierlistEntity, TierlistEntity};
use domain::error::ApiError;
use domain::mappers::TryEntityMapper;
use domain::repositories::AbstractTierlistRepository;
use crate::models::TierlistModel;

#[derive(Clone)]
pub struct TierlistRepository{
    collection: Collection<TierlistModel>
}

impl TierlistRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("tierlist");
        Self { collection }
    }
}

#[async_trait]
impl AbstractTierlistRepository for TierlistRepository {
    async fn get_tierlist_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError> {
        let user_id = ObjectId::parse_str(user_id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let query = doc! { "author": user_id };
        let mut cursor = self.collection.find(query)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        let mut tierlists = Vec::new();
        while let Ok(Some(result)) = cursor.try_next().await {
            tierlists.push(result.to_entity());
        }

        Ok(tierlists)
    }

    async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<(), ApiError> {
        let tierlist = TierlistModel::try_from(tierlist)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        self.collection.insert_one(tierlist)
            .await
            .map_err(|err| ApiError::InternalError(format!("Failed to execute query: {err}")))?;

        Ok(())
    }

    async fn get_tierlist_by_id(&self, id: &str) -> Result<TierlistEntity, ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let tierlist = self.collection.find_one(doc! { "author": id })
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        match tierlist {
            None => Err(ApiError::NotFound(format!("Tierlist with id {id} not found"))),
            Some(tierlist) => Ok(tierlist.to_entity()),
        }
    }
}