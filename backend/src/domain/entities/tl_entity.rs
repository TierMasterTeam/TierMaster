use derive_new::new;
use crate::domain::entities::Entity;

#[derive(new, Debug, Clone)]
pub struct TlEntity{
    pub name: String,
}

impl Entity for TlEntity{}