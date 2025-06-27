use crate::presenters::{CardPresenter, TemplateGradePresenter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::entities::UpdateTemplateEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplatePresenter {
    pub name: String,
    pub is_public: bool,
    pub tags: Vec<String>,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<TemplateGradePresenter>
}

impl EntityMapper<UpdateTemplateEntity> for UpdateTemplatePresenter {
    fn to_entity(self) -> UpdateTemplateEntity {
        UpdateTemplateEntity {
            name: self.name,
            is_public: self.is_public,
            tags: self.tags,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl From<UpdateTemplateEntity> for UpdateTemplatePresenter {
    fn from(value: UpdateTemplateEntity) -> Self {
        Self {
            name: value.name,
            is_public: value.is_public,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        }
    }
}

impl IntoResponse for UpdateTemplatePresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}