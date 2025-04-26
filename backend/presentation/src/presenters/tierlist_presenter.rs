use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use domain::entities::TierlistEntity;
use domain::mappers::EntityMapper;
use crate::presenters::{CardPresenter, GradePresenter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierlistPresenter {
    pub id: String,
    pub name: String,
    pub author: String,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<GradePresenter>,
}

impl EntityMapper<TierlistEntity> for TierlistPresenter {
    fn to_entity(self) -> TierlistEntity {
        TierlistEntity {
            id: self.id,
            name: self.name,
            author: self.author,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl From<TierlistEntity> for TierlistPresenter {
    fn from(value: TierlistEntity) -> Self {
        Self{
            id: value.id,
            name: value.name,
            author: value.author,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        }
    }
}

impl IntoResponse for TierlistPresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}