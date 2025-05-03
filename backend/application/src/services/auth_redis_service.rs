use chrono::Utc;
use derive_new::new;
use domain::error::ApiError;
use domain::repositories::AbstractRedisRepository;
use std::sync::Arc;

const TOKEN_EXPIRATION: u64 = 3600; // 1 hour
const REFRESH_TOKEN_EXPIRATION: u64 = 604800; // 1 week
const MAX_USER_SESSION_DURATION: u64 = 86400 * 30 * 3; // ~ 3 months

#[derive(new)]
pub struct AuthRedisService { 
    redis: Arc<dyn AbstractRedisRepository>
}


impl AuthRedisService {
    /// store all info for both access token and refresh token
    pub async fn store_full_session(&self, token: &str, refresh_token: &str, user_id: &str) -> Result<(), ApiError> {
        self.store_user_id_with_token(token, user_id).await?;
        self.store_user_id_with_refresh_token(refresh_token, user_id).await?;

        self.store_refresh_token_with_token(token, refresh_token).await?;
        self.store_refresh_token_with_user_id(user_id, refresh_token).await?;

        self.store_start_session_timestamp(refresh_token).await
    }

    /// store info only for the access token
    pub async fn store_token_session(&self, token: &str, refresh_token: &str, user_id: &str) -> Result<(), ApiError> {
        self.store_refresh_token_with_token(token, refresh_token).await?;
        self.store_user_id_with_token(token, user_id).await
    }
    
    pub async fn fetch_user_id(&self, token: &str) -> Result<String, ApiError> {
        self.redis.fetch(token).await
    }

    pub async fn fetch_refresh_token_with_token(&self, token: &str) -> Result<String, ApiError> {
        let key = self.make_key_to_store_refresh_token_with_token(token);
        self.redis.fetch(key.as_str()).await
    }
    
    pub async fn fetch_refresh_token_with_user_id(&self, user_id: &str) -> Result<String, ApiError> {
        self.redis.fetch(user_id).await
    }

    pub async fn fetch_start_session_timestamp(&self, refresh_token: &str) -> Result<String, ApiError> {
        let key = self.make_key_to_store_start_session_timestamp(refresh_token);
        self.redis.fetch(key.as_str()).await
    }
    
    pub async fn remove_token_session(&self, token: &str) -> Result<(), ApiError> {
        let _ = self.redis.delete(token).await; // may fail if token has expired, which is fine...
        
        let refresh_token = self.fetch_refresh_token_with_token(token).await?;
        self.redis.delete(refresh_token.as_str()).await?; 
        
        let refresh_token_key = self.make_key_to_store_refresh_token_with_token(token);
        self.redis.delete(refresh_token_key.as_str()).await?;
        
        self.redis.delete(refresh_token.as_str()).await?;
        
        let start_session_key = self.make_key_to_store_start_session_timestamp(refresh_token.as_str());
        self.redis.delete(start_session_key.as_str()).await?;
        
        Ok(())
    } 
    
    
    async fn store_user_id_with_refresh_token(&self, refresh_token: &str, user_id: &str) -> Result<(), ApiError> {
        self.redis.store(refresh_token, user_id.to_string(), Some(REFRESH_TOKEN_EXPIRATION)).await
    }

    async fn store_user_id_with_token(&self, token: &str, user_id: &str) -> Result<(), ApiError> {
        self.redis.store(token, user_id.to_string(), Some(TOKEN_EXPIRATION)).await
    }

    async fn store_refresh_token_with_user_id(&self, user_id: &str, refresh_token: &str) -> Result<(), ApiError> {
        self.redis.store(user_id, refresh_token.to_string(), Some(REFRESH_TOKEN_EXPIRATION)).await
    }

    async fn store_refresh_token_with_token(&self, token: &str, refresh_token: &str) -> Result<(), ApiError> {
        let refresh_token_key = self.make_key_to_store_refresh_token_with_token(token);
        self.redis.store(refresh_token_key.as_str(), refresh_token.to_string(), Some(REFRESH_TOKEN_EXPIRATION)).await
    }

    async fn store_start_session_timestamp(&self, refresh_token: &str) -> Result<(), ApiError> {
        let user_session_key = self.make_key_to_store_refresh_token_with_token(refresh_token);
        self.redis.store(user_session_key.as_str(), Utc::now().timestamp().to_string(), Some(MAX_USER_SESSION_DURATION)).await
    }
    
    fn make_key_to_store_refresh_token_with_token(&self, token: &str) -> String {
        format!("{token}_refresh_token")
    }

    fn make_key_to_store_start_session_timestamp(&self, refresh_token: &str) -> String {
        format!("{refresh_token}_start_session")
    }
}