use domain::entities::TierlistEntity;
use domain::error::ApiError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type RoomStore = HashMap<String, TierlistEntity>;

#[derive(Clone, Default)]
pub struct WebsocketService {
    tierlists: Arc<RwLock<RoomStore>>,
}

impl WebsocketService {
    pub fn new() -> Self {
        Self {
            tierlists: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, room_id: &str) -> Result<TierlistEntity, ApiError> {
        match self.tierlists.read().await.get(room_id) {
            None => Err(ApiError::NotFound(format!("No room found in the store with id {} so get tierlist !", room_id))),
            Some(tierlist) => Ok(tierlist.clone())
        }
    }

    pub async fn update(&self, room_id: &str, tierlist: TierlistEntity) -> Result<TierlistEntity, ApiError> {
        match self.tierlists.write().await.insert(room_id.to_string(), tierlist) {
            None => Err(ApiError::NotFound(format!("No room found in the store with id {} to update tierlist !", room_id))),
            Some(tierlist) => Ok(tierlist.clone())
        }
    }
}