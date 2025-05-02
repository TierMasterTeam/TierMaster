use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CredentialsEntity {
    pub email: String,
    pub password: String,
    pub username: Option<String>
}