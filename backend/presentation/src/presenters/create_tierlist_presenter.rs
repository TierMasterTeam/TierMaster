use crate::presenters::{CardPresenter, GradePresenter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::entities::CreateTierlistEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTierlistPresenter {
    pub name: String,
    pub is_public: bool,
    pub author: String,
    #[serde(default, rename = "coverImage")]
    pub cover_image: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<GradePresenter>,
}

impl EntityMapper<CreateTierlistEntity> for CreateTierlistPresenter {
    fn to_entity(self) -> CreateTierlistEntity {
        CreateTierlistEntity {
            name: self.name,
            is_public: self.is_public,
            author: self.author,
            tags: self.tags,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
            cover_image: self.cover_image,
        }
    }
}

impl From<CreateTierlistEntity> for CreateTierlistPresenter {
    fn from(value: CreateTierlistEntity) -> Self {
        Self{
            name: value.name,
            is_public: value.is_public,
            author: value.author,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
            cover_image: value.cover_image,
        }
    }
}

impl IntoResponse for CreateTierlistPresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}