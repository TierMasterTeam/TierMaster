use crate::data::models::model::Model;
use crate::data::repositories::tierlist_repository;
use crate::domain::entities::tierlist_entity::TierlistEntity;
use crate::domain::errors::api_error::ApiError;
use mongodb::bson::oid::ObjectId;

pub async fn get_tierlists_of_user(user_id: &str) -> Result<Vec<TierlistEntity>, ApiError> {
    let object_id = match ObjectId::parse_str(user_id) {
        Ok(id) => id,
        Err(e) => {
            let error_message = format!("Failed to parse user_id to ObjectId : {e}");
            return Err(ApiError::BadRequest(error_message));
        }
    };

    let tierlists = match tierlist_repository::get_tierlists_of_user(object_id).await {
        Ok(tierlists) => tierlists,
        Err(error_message) => {
            return Err(ApiError::InternalError(error_message))
        }
    };

    let mut tierlist_models = Vec::new();
    for tierlist in tierlists {
        tierlist_models.push(tierlist.to_entity())
    }

    Ok(tierlist_models)
}