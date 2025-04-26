use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use domain::entities::CreateTierlistEntity;
use domain::mappers::EntityMapper;
use crate::presenters::{CardPresenter, GradePresenter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTierlistPresenter {
    pub name: String,
    pub author: String,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<GradePresenter>,
}

impl EntityMapper<CreateTierlistEntity> for CreateTierlistPresenter {
    fn to_entity(self) -> CreateTierlistEntity {
        CreateTierlistEntity {
            name: self.name,
            author: self.author,
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