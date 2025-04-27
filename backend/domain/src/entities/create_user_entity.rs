
#[derive(Debug, Clone)]
pub struct CreateUserEntity {
    pub email: String,
    pub password_hash: String,
    pub username: String,
}