
#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub username: String,
}
