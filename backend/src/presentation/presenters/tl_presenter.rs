use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::domain::entities::tl_entity::TlEntity;
use crate::presentation::presenters::presenter::Presenter;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct TlPresenter {
    pub name: String,
}

impl Presenter<TlEntity> for TlPresenter {
    fn to_entity(self) -> TlEntity {
        TlEntity {
            name: self.name,
        }
    }

    fn from_entity(entity: TlEntity) -> Self {
        Self {
            name: entity.name,
        }
    }
}