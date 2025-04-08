use serde::de::DeserializeOwned;

pub mod tierlist_entity;

pub trait Entity: Send + Sync + 'static  {}