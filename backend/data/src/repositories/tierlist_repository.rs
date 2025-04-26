use async_trait::async_trait;
use futures::StreamExt;
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
    async fn get_all_tierlists(&self) -> Result<Vec<TierlistEntity>, ApiError> {
        let cursor = self.collection.find(doc! {}) 
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        let result = cursor
            .filter_map(|item| async {
                item.ok().map(|tl| tl.to_entity())
            })
            .collect()
            .await;
        
        Ok(result)
    }

    async fn get_tierlist_by_id(&self, id: &str) -> Result<TierlistEntity, ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let tierlist = self.collection.find_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        match tierlist {
            None => Err(ApiError::NotFound(format!("Tierlist with id {id} not found"))),
            Some(tierlist) => Ok(tierlist.to_entity()),
        }
    }

    async fn get_tierlist_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError> {
        let user_object_id = ObjectId::parse_str(user_id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let query = doc! { "author": user_object_id };
        let cursor = self.collection.find(query)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        let result = cursor
            .filter_map(|item| async {
                item.ok().map(|tl| tl.to_entity())
            })
            .collect()
            .await;

        Ok(result)
    }

    async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<(), ApiError> {
        let tierlist = TierlistModel::try_from(tierlist)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        self.collection.insert_one(tierlist)
            .await
            .map_err(|err| ApiError::InternalError(format!("Failed to execute query: {err}")))?;

        Ok(())
    }
}