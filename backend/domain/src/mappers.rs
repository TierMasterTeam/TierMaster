pub trait EntityMapper<T>: From<T> {
    fn to_entity(self) -> T;
}

pub trait TryEntityMapper<T>: TryFrom<T> {
    fn to_entity(self) -> T;
}

pub trait FallibleEntityMapper<T>: From<T> {
    type Error;
    fn try_to_entity(self) -> Result<T, Self::Error>;
}

pub trait FallibleTryEntityMapper<T>: TryFrom<T> {
    fn try_to_entity(self) -> Result<T, <Self as TryFrom<T>>::Error>;
}