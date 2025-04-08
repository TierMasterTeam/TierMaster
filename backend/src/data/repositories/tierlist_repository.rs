use crate::core::app_state::APP_STATE;
use crate::data::models::tierlist_model::TierlistModel;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

fn get_tierlist_collection() ->  Collection<TierlistModel> {
    APP_STATE.get().unwrap().mongodb().collection("Tierlist")
}

pub async fn get_tierlists_of_user(user_id: ObjectId) -> Result<Vec<TierlistModel>, String> {
    let tierlist_collection = get_tierlist_collection();

    let query = doc! { "author": user_id };
    let mut cursor = match tierlist_collection.find(query).await {
        Ok(cursor) => cursor,
        Err(e) => {
            return Err(format!("Failed to execute query: {e}"));
        }
    };

    let mut tierlists = Vec::new();
    while let Ok(Some(result)) = cursor.try_next().await {
        tierlists.push(result);
    }

    Ok(tierlists)
}