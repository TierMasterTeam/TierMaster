use crate::presenters::{CardPresenter, GradePresenter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::entities::UpdateTierlistEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTierlistPresenter {
    pub name: String,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<GradePresenter>
}

impl EntityMapper<UpdateTierlistEntity> for UpdateTierlistPresenter {
    fn to_entity(self) -> UpdateTierlistEntity {
        UpdateTierlistEntity {
            name: self.name,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl From<UpdateTierlistEntity> for UpdateTierlistPresenter {
    fn from(value: UpdateTierlistEntity) -> Self {
        Self{
            name: value.name,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        }
    }
}

impl IntoResponse for UpdateTierlistPresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}