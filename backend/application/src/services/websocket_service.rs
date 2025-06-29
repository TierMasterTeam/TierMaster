use domain::entities::{RoomEntity, RoomUserEntity, TierlistRoomEntity};
use domain::error::ApiError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type RoomStore = HashMap<String, RoomEntity>;

pub type UserPerRoomStore = HashMap<String, String>;

#[derive(Clone, Default)]
pub struct WebsocketService {
    tierlist_rooms: Arc<RwLock<RoomStore>>,
    user_per_room: Arc<RwLock<UserPerRoomStore>>
}


impl WebsocketService {
    pub fn new() -> Self {
        Self {
            tierlist_rooms: Arc::new(RwLock::new(HashMap::new())),
            user_per_room: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn join(&self, room_id: &str, room_user: RoomUserEntity) -> Result<RoomEntity, ApiError> {
        let mut tierlist_room = self.get(room_id).await?;
        if ! tierlist_room.users.iter().any(|user| user.id.eq(&room_user.clone().id)) {
            tierlist_room.users.push(room_user.clone());
            self.user_per_room.write().await.insert(room_user.id, room_id.to_string());
        }

        self.update(room_id, tierlist_room).await
    }

    pub async fn leave(&self, room_id: &str, room_user: RoomUserEntity) -> Result<RoomEntity, ApiError> {
        let mut tierlist_room = self.get(room_id).await?;
        tierlist_room.users.retain(|user| user.id != room_user.id);
        self.user_per_room.write().await.remove(&room_user.id);

        if tierlist_room.users.is_empty() {
            self.tierlist_rooms.write().await.remove(room_id);
            return Ok(tierlist_room);
        }

        self.update(room_id, tierlist_room).await
    }

    pub async fn disconnected(&self, room_user: RoomUserEntity) -> Result<(RoomEntity, String), ApiError> {
        let room_id = match self.user_per_room.read().await.get(room_user.clone().id.as_str()) {
            None => Err(ApiError::NotFound(format!("The user {} was not connected to a room", room_user.id))),
            Some(id) => Ok(id.to_string())
        }?;

        let result = self.leave(room_id.clone().as_str(), room_user).await?;

        Ok((result, room_id.clone().to_string()))
    }

    pub async fn get(&self, room_id: &str) -> Result<RoomEntity, ApiError> {
        match self.tierlist_rooms.read().await.get(room_id) {
            None => Err(ApiError::NotFound(format!("No room found in the store with id {} so get tierlist !", room_id))),
            Some(tierlist_entity) => Ok(tierlist_entity.clone())
        }
    }

    pub async fn update(&self, room_id: &str, tierlist_room: RoomEntity) -> Result<RoomEntity, ApiError> {
        match self.tierlist_rooms.write().await.insert(room_id.to_string(), tierlist_room.clone()) {
            None => Err(ApiError::NotFound(format!("No room found in the store with id {} to update tierlist !", room_id))),
            Some(_) => Ok(tierlist_room)
        }
    }

    pub async fn update_tierlist(&self, room_id: &str, tierlist: TierlistRoomEntity) -> Result<RoomEntity, ApiError> {
        let mut tierlist_room = self.get(room_id).await?;
        tierlist_room.tierlist = tierlist;

        self.update(room_id, tierlist_room).await
    }
    
    pub async fn create(&self, room_id: &str, tierlist_room: RoomEntity) -> Result<(), ApiError> {
        match self.tierlist_rooms.write().await.insert(room_id.to_string(), tierlist_room) {
            None => Ok(()),
            Some(_) => Err(ApiError::InternalError(format!("A room with this id already exists in the store ({}) !", room_id)))
        }
    }
}