use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::domain::entities::create_tierlist_entity::CreateTierlistEntity;
use crate::domain::mapper::EntityMapper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTierlistPresenter {
    pub name: String,
    pub author: String,
}

impl EntityMapper<CreateTierlistEntity> for CreateTierlistPresenter {
    fn to_entity(self) -> CreateTierlistEntity {
        CreateTierlistEntity {
            name: self.name,
            author: self.author,
        }
    }
}

impl From<CreateTierlistEntity> for CreateTierlistPresenter {
    fn from(value: CreateTierlistEntity) -> Self {
        Self{
            name: value.name,
            author: value.author,
        }
    }
}

impl IntoResponse for CreateTierlistPresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}