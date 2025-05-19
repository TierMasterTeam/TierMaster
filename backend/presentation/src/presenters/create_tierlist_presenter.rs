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
    pub author: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<GradePresenter>,
}

impl EntityMapper<CreateTierlistEntity> for CreateTierlistPresenter {
    fn to_entity(self) -> CreateTierlistEntity {
        CreateTierlistEntity {
            name: self.name,
            author: self.author,
            tags: self.tags,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl From<CreateTierlistEntity> for CreateTierlistPresenter {
    fn from(value: CreateTierlistEntity) -> Self {
        Self{
            name: value.name,
            author: value.author,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        }
    }
}

impl IntoResponse for CreateTierlistPresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}