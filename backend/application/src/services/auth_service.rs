use crate::services::auth_redis_service::AuthRedisService;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use argon2::{password_hash::PasswordHasher, password_hash::PasswordVerifier, password_hash::SaltString, Argon2, PasswordHash};
use base64::engine::general_purpose;
use base64::Engine;
use domain::entities::{CreateUserEntity, CredentialsEntity, UserEntity};
use domain::error::ApiError;
use domain::repositories::{AbstractAuthRepository, AbstractRedisRepository};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthService {
    repo: Arc<dyn AbstractAuthRepository>,
    redis_service: AuthRedisService
}

impl AuthService {
    pub(crate) fn new(repo: Arc<dyn AbstractAuthRepository>, redis: Arc<dyn AbstractRedisRepository>) -> Self {
        Self {
            repo,
            redis_service: AuthRedisService::new(redis),
        }
    }

    pub async fn logout(&self, token: String) -> Result<(), ApiError> {
        self.redis_service.remove_token_session(token.as_str()).await?;

        Ok(())
    }

    pub async fn login(&self, credentials: CredentialsEntity) -> Result<(String, UserEntity), ApiError> {
        let user = self.verify_credentials(credentials).await?;
        let user_id = user.id.clone();

        let existing_refresh_token = self.redis_service
            .fetch_refresh_token_with_user_id(user_id.as_str()).await;
        
        if existing_refresh_token.is_ok() {
            // don't generate a new refresh token, only an access token
            let token = self.generate_token().await?;
            let refresh_token = existing_refresh_token?;
            self.redis_service.store_token_session(token.as_str(), refresh_token.as_str(), user_id.as_str()).await?;

            return Ok((token, user));
        }

        let token = self.generate_token().await?;
        let refresh_token = self.generate_token().await?;

        self.redis_service.store_full_session(token.as_str(), refresh_token.as_str(), user_id.as_str()).await?;

        Ok((token, user))
    }

    pub async fn signup(&self, credentials: CredentialsEntity) -> Result<(String, UserEntity), ApiError> {
        if credentials.username.is_none() {
            return Err(ApiError::BadRequest("The username is required to create an account".to_string()));
        }

        let credential_clone = credentials.clone();
        let password_hash = self.hash_password(credential_clone.password).await?;
        let user = CreateUserEntity {
            email: credential_clone.email,
            password_hash,
            username: credential_clone.username.unwrap(),
        };

        self.repo.create_user(user).await?;

        self.login(credentials).await
    }

    pub async fn verify_credentials(&self, credentials: CredentialsEntity) -> Result<UserEntity, ApiError> {
        let user_entity = self.repo.get_user_by_email(credentials.email.as_str()).await?;
        let password_hash = user_entity.password_hash.clone()
            .ok_or(ApiError::InternalError(format!("UserEntity from DB has no password (id: {})", user_entity.id)))?;

        let password_hash_in_db = PasswordHash::new(&password_hash.as_ref())
            .map_err(|_| ApiError::InternalError("Failed to parse hashed password".to_string()))?;

        if Argon2::default().verify_password(credentials.password.as_bytes(), &password_hash_in_db).is_ok() {
            Ok(user_entity)
        } else {
            Err(ApiError::Unauthorized("Invalid credentials".into()))
        }
    }

    pub fn verify_token(&self, signed_token: &str) -> Result<(), ApiError> {
        let parts: Vec<&str> = signed_token.split('.').collect();
        if parts.len() != 2 {
            return Err(ApiError::Unauthorized("Invalid token format".into()));
        }

        let (token, signature) = (parts[0], parts[1]);

        let mut mac = self.create_hmac()?;
        mac.update(token.as_bytes());

        let expected_signature = general_purpose::URL_SAFE_NO_PAD.decode(signature)
            .map_err(|_| ApiError::Unauthorized("Invalid base64 signature".into()))?;

        mac.verify_slice(&expected_signature)
            .map_err(|_| ApiError::Unauthorized("Invalid token signature".into()))?;

        Ok(())
    }

    pub async fn validate_session(&self, token: &str) -> Result<(String, String), ApiError> {
        match self.redis_service.fetch_user_id(token).await {
            Err(_) => self.try_to_refresh_session(token).await,
            Ok(user_id) => Ok((token.to_string(), user_id))
        }
    }

    async fn try_to_refresh_session(&self, token: &str) -> Result<(String, String), ApiError> {
        let refresh_token = self.redis_service.fetch_refresh_token_with_token(token).await?;

        let user_id = self.redis_service.fetch_user_id(refresh_token.as_str()).await?;

        self.check_use_session_duration_still_valid(refresh_token.as_str()).await?;

        let new_token = self.generate_token().await?;
        let new_refresh_token = self.generate_token().await?;
        self.redis_service.store_full_session(new_token.as_str(), new_refresh_token.as_str(), 
                                              user_id.as_str()).await?;

        Ok((new_token, user_id))
    }

    async fn check_use_session_duration_still_valid(&self, refresh_token: &str) -> Result<(), ApiError> {
        self.redis_service.fetch_start_session_timestamp(refresh_token).await?;
        Ok(())
    }

    async fn generate_token(&self) -> Result<String, ApiError> {
        let mut rng = OsRng;
        let mut token = [0u8; 32]; // 256-bit token
        rng.fill_bytes(&mut token);

        // base 64 encoding
        let encoded_token = general_purpose::URL_SAFE_NO_PAD.encode(token);

        let mut mac = self.create_hmac()?;
        mac.update(encoded_token.as_bytes());
        let signature = mac.finalize().into_bytes();
        let encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(signature);

        Ok(format!("{encoded_token}.{encoded_signature}"))
    }

    fn create_hmac(&self) -> Result<Hmac<Sha256>, ApiError> {
        let secret_key = std::env::var("TOKEN_SECRET_KEY")
            .map_err(|_| ApiError::InternalError("Missing TOKEN_SECRET_KEY".to_string()))?;

        Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
            .map_err(|_| ApiError::InternalError("Failed to create HMAC".to_string()))
    }

    async fn hash_password(&self, password: String) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        match argon2.hash_password(password.as_ref(), &salt) {
            Err(e) => Err(ApiError::InternalError(format!("Failed to hash credentials' password : {e}").to_string())),
            Ok(password_hash) => Ok(password_hash.to_string())
        }
    }
}