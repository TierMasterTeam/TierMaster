use crate::presenters::{CardPresenter, TemplateGradePresenter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use domain::entities::TemplateEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePresenter {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub is_public: bool,
    pub author: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<TemplateGradePresenter>,
}

impl EntityMapper<TemplateEntity> for TemplatePresenter {
    fn to_entity(self) -> TemplateEntity {
        TemplateEntity {
            id: self.id,
            created_at: self.created_at,
            name: self.name,
            is_public: self.is_public,
            author: self.author,
            tags: self.tags,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl From<TemplateEntity> for TemplatePresenter {
    fn from(value: TemplateEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            is_public: value.is_public,
            author: value.author,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        }
    }
}

impl IntoResponse for TemplatePresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}