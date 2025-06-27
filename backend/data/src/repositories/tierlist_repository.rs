use crate::models::{CardModel, GradeModel, TierlistModel};
use async_trait::async_trait;
use domain::entities::{CreateTierlistEntity, TierlistEntity, UpdateTierlistEntity};
use domain::error::ApiError;
use domain::mappers::TryEntityMapper;
use domain::repositories::AbstractTierlistRepository;
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::{Collection, Cursor, Database};

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

        let result = collect_cursor_to_list_of_tierlist_entity(cursor).await;

        Ok(result)
    }

    async fn get_tierlist_by_id(&self, id: &str) -> Result<TierlistEntity, ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let tierlist = find_tierlist_by_id(&self.collection, id).await?;

        Ok(tierlist.to_entity())
    }

    async fn get_tierlist_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError> {
        let user_object_id = ObjectId::parse_str(user_id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let query = doc! { "author": user_object_id };
        let cursor = self.collection.find(query)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        let result = collect_cursor_to_list_of_tierlist_entity(cursor).await;

        Ok(result)
    }

    async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<String, ApiError> {
        let tierlist = TierlistModel::try_from(tierlist)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let result = self.collection.insert_one(tierlist)
            .await
            .map_err(|err| ApiError::InternalError(format!("Failed to execute query: {err}")))?;
        
        let id = result.inserted_id.as_object_id()
            .ok_or(ApiError::InternalError("Failed to retrieve tierlist id".to_owned()))?
            .to_string();

        Ok(id)
    }

    async fn update_tierlist_by_id(&self, id: &str, tierlist: UpdateTierlistEntity) -> Result<(), ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;
        
        let mut tierlist_entity = find_tierlist_by_id(&self.collection, id).await?;
        tierlist_entity.name = tierlist.name;
        tierlist_entity.is_public = tierlist.is_public;
        tierlist_entity.tags = tierlist.tags;
        tierlist_entity.cards = tierlist.cards.into_iter().map(CardModel::from).collect();
        tierlist_entity.grades = tierlist.grades.into_iter().map(GradeModel::from).collect();
        
        let query = doc! { "_id": id };
        self.collection.replace_one(query, tierlist_entity)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute update: {e}")))?;

        Ok(())
    }
}

async fn find_tierlist_by_id(collection: &Collection<TierlistModel>, id: ObjectId) -> Result<TierlistModel, ApiError> {
    let tierlist = collection.find_one(doc! { "_id": id })
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

    match tierlist {
        None => Err(ApiError::NotFound(format!("Tierlist with id {id} not found"))),
        Some(tierlist) => Ok(tierlist),
    }
}

async fn collect_cursor_to_list_of_tierlist_entity(cursor:  Cursor<TierlistModel>) -> Vec<TierlistEntity> {
    cursor.filter_map(|item| async {
            item.ok().map(|tl| tl.to_entity())
        })
        .collect()
        .await
}