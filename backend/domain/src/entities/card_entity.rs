use crate::entities::CardRoomEntity;

#[derive(Debug, Clone)]
pub struct CardEntity {
    pub name: String,
    pub image: String,
}

impl From<CardRoomEntity> for CardEntity {
    fn from(value: CardRoomEntity) -> Self {
        Self { 
            name: value.name,
            image: value.image
        }
    }
}