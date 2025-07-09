use crate::{entities::{CardEntity, GradeEntity, TierlistEntity}, utils::CancellableTask};

#[derive(Clone)]
pub struct RoomEntity {
    pub users: Vec<RoomUserEntity>,
    pub tierlist: TierlistRoomEntity,
    pub save_task: Option<CancellableTask>
}

#[derive(Clone)]
pub struct RoomUserEntity {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Clone)]
pub struct TierlistRoomEntity {
    pub id: String,
    pub name: String,
    pub is_public: bool,
    pub cover_image: String,
    pub author: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardRoomEntity>,
    pub grades: Vec<GradeRoomEntity>,
}

#[derive(Clone)]
pub struct CardRoomEntity {
    pub name: String,
    pub image: String,
    pub is_dragged: bool,
    pub dragged_by: Option<RoomUserEntity>,
}

#[derive(Clone)]
pub struct GradeRoomEntity {
    pub name: String,
    pub color: String,
    pub cards: Vec<CardRoomEntity>,
}

impl TierlistRoomEntity {
    pub fn from_tierlist_entity(tierlist_entity: TierlistEntity) -> Self {
        Self {
            id: tierlist_entity.id,
            name: tierlist_entity.name,
            is_public: tierlist_entity.is_public,
            author: tierlist_entity.author,
            tags: tierlist_entity.tags,
            cards: tierlist_entity.cards.into_iter().map(CardRoomEntity::from_card_entity).collect(),
            grades: tierlist_entity.grades.into_iter().map(GradeRoomEntity::from_grade_entity).collect(),
            cover_image: tierlist_entity.cover_image,
        }
    }
}

impl CardRoomEntity {
    fn from_card_entity(card_entity: CardEntity) -> Self {
        Self {
            name: card_entity.name,
            image: card_entity.image,
            is_dragged: false,
            dragged_by: None,
        }
    }
}

impl GradeRoomEntity {
    fn from_grade_entity(grade_entity: GradeEntity) -> Self {
        Self {
            name: grade_entity.name,
            color: grade_entity.color,
            cards: grade_entity.cards.into_iter().map(CardRoomEntity::from_card_entity).collect(),
        }
    }
}
