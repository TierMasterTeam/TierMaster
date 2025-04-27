use crate::databases::RedisDb;
use async_trait::async_trait;
use domain::error::ApiError;
use domain::repositories::AbstractRedisRepository;
use redis::Commands;

#[derive(Clone)]
pub struct RedisRepository {
    redis_db: RedisDb,
}

impl RedisRepository {
    pub fn new(redis_db: RedisDb) -> Self {
        Self {
            redis_db,
        }
    }

    fn map_redis_error(e: redis::RedisError) -> ApiError {
        ApiError::InternalError(e.to_string())
    }
}

#[async_trait]
impl AbstractRedisRepository for RedisRepository {
    async fn store(&self, key: &str, value: String, expiration: Option<u64>) -> Result<(), ApiError> {
        let mut connection = self.redis_db.client().get_connection().map_err(Self::map_redis_error)?;
        
        if let Some(exp) = expiration {
            let _: () = connection.set_ex(key, value, exp).map_err(Self::map_redis_error)?;
        } else {
            let _: () = connection.set(key, value).map_err(Self::map_redis_error)?;
        }
        
        Ok(())
    }

    async fn fetch(&self, key: &str) -> Result<String, ApiError> {
        let mut connection = self.redis_db.client().get_connection().map_err(Self::map_redis_error)?;
        let value: String = connection.get(key).map_err(Self::map_redis_error)?;
        Ok(value)
    }
    
    async fn delete(&self, key: &str) -> Result<(), ApiError> {
        let mut connection = self.redis_db.client().get_connection().map_err(Self::map_redis_error)?;
        connection.del(key).map_err(Self::map_redis_error)
    }
}
