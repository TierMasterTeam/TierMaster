use domain::entities::{CardRoomEntity, GradeRoomEntity, RoomEntity, RoomUserEntity, TierlistRoomEntity};
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct RoomPresenter {
    pub users: Vec<RoomUserPresenter>,
    pub tierlist: TierlistRoomPresenter,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RoomUserPresenter {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TierlistRoomPresenter {
    pub id: String,
    pub name: String,
    #[serde(rename(serialize = "isPublic", deserialize = "isPublic"))]
    pub is_public: bool,
    pub author: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardRoomPresenter>,
    pub grades: Vec<GradeRoomPresenter>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CardRoomPresenter {
    pub name: String,
    pub image: String,
    #[serde(rename(serialize = "isDragged", deserialize = "isDragged"))]
    pub is_dragged: bool,
    #[serde(rename(serialize = "draggedBy", deserialize = "draggedBy"))]
    pub dragged_by: Option<RoomUserPresenter>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GradeRoomPresenter {
    pub name: String,
    pub color: String,
    pub cards: Vec<CardRoomPresenter>,
}

impl From<RoomEntity> for RoomPresenter {
    fn from(value: RoomEntity) -> Self {
        Self {
            users: value.users.into_iter().map(Into::into).collect(),
            tierlist: value.tierlist.into(),
        }
    }
}

impl EntityMapper<RoomUserEntity> for RoomUserPresenter {
    fn to_entity(self) -> RoomUserEntity {
        RoomUserEntity {
            id: self.id,
            name: self.name,
            color: self.color,
        }
    }
}

impl From<RoomUserEntity> for RoomUserPresenter {
    fn from(value: RoomUserEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            color: value.color,
        }
    }
}

impl EntityMapper<RoomEntity> for RoomPresenter {
    fn to_entity(self) -> RoomEntity {
        RoomEntity {
            users: self.users.into_iter().map(EntityMapper::to_entity).collect(),
            tierlist: self.tierlist.to_entity(),
        }
    }
}

impl From<TierlistRoomEntity> for TierlistRoomPresenter {
    fn from(value: TierlistRoomEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            is_public: value.is_public,
            author: value.author,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        }
    }
}

impl EntityMapper<TierlistRoomEntity> for TierlistRoomPresenter {
    fn to_entity(self) -> TierlistRoomEntity {
        TierlistRoomEntity {
            id: self.id,
            name: self.name,
            is_public: self.is_public,
            author: self.author,
            tags: self.tags,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl From<CardRoomEntity> for CardRoomPresenter {
    fn from(value: CardRoomEntity) -> Self {
        Self {
            name: value.name,
            image: value.image,
            is_dragged: value.is_dragged,
            dragged_by: value.dragged_by.map(RoomUserPresenter::from),
        }
    }
}

impl EntityMapper<CardRoomEntity> for CardRoomPresenter {
    fn to_entity(self) -> CardRoomEntity {
        CardRoomEntity {
            name: self.name,
            image: self.image,
            is_dragged: self.is_dragged,
            dragged_by: self.dragged_by.map(|card| card.to_entity()),
        }
    }
}

impl From<GradeRoomEntity> for GradeRoomPresenter {
    fn from(value: GradeRoomEntity) -> Self {
        Self {
            name: value.name,
            color: value.color,
            cards: value.cards.into_iter().map(Into::into).collect(),
        }
    }
}

impl EntityMapper<GradeRoomEntity> for GradeRoomPresenter {
    fn to_entity(self) -> GradeRoomEntity {
        GradeRoomEntity {
            name: self.name,
            color: self.color,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}