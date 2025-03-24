use crate::domain::entities::Entity;

pub trait Presenter<E: Entity> {
    fn to_entity(self) -> E;
    fn from_entity(entity: E) -> Self;
}