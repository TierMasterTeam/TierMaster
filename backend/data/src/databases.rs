use crate::error::DatabaseError;
use crate::repositories::RepositoryFactory;
use application::AppState;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::Client as S3Client;
use domain::error::ApiError;
use mongodb::options::{AuthMechanism, ClientOptions, Credential, ServerApi, ServerApiVersion};
use mongodb::{Client, Database};
use std::env;
use std::sync::Arc;
use url::Url;

pub struct Databases {
    mongo: MongoDB,
    aws_bucket: AwsBucket,
    redis: RedisDb,
}

impl Databases {
    pub async fn connect() -> Result<AppState, DatabaseError> {
        let mongo = MongoDB::connect().await?;
        let aws_bucket = AwsBucket::connect().await?;
        let redis = RedisDb::connect().await?;

        let databases = Databases {
            mongo,
            aws_bucket,
            redis
        };

        let factory = RepositoryFactory::init(&databases);

        Ok(AppState::new(Arc::new(factory)))
    }

    pub fn mongo(&self) -> &Database {
        self.mongo.db()
    }
    pub fn aws(&self) -> &AwsBucket {
        &self.aws_bucket
    }

    pub fn redis(&self) -> &redis::Client {
        &self.redis.client()
    }
}


pub struct MongoDB(Database);

impl MongoDB {
    pub async fn connect() -> Result<Self, DatabaseError> {
        let uri = env::var("DATABASE_URL")
            .unwrap_or("mongodb://localhost:27017".to_string());
        let db_name = env::var("MONGO_DATABASE")
            .unwrap_or("MongoDB".to_string());
        let user = env::var("MONGO_USER")
            .unwrap_or_default();
        let password = env::var("MONGO_PASSWORD")
            .unwrap_or_default();

        let mut options = ClientOptions::parse(uri).await?;

        let credentials = Credential::builder()
            .username(user)
            .password(password)
            .mechanism(AuthMechanism::ScramSha256)
            .build();

        let server_api = ServerApi::builder()
            .version(ServerApiVersion::V1)
            .build();

        options.server_api = Some(server_api);
        options.credential = Some(credentials);

        let client = Client::with_options(options)?;
        
        let database = client.database(&db_name);
        
        println!("Successfully connected to MongoDB");
        Ok(Self(database))
    }

    pub fn db(&self) -> &Database {
        &self.0
    }
}

#[derive(Clone)]
pub struct RedisDb(redis::Client);

impl RedisDb {
    pub async fn connect() -> Result<Self, DatabaseError> {
        let uri = env::var("REDIS_URL")
            .unwrap_or("redis://localhost:6379".to_string());

        let client = redis::Client::open(uri)
            .map_err(|e| DatabaseError::from(
                ApiError::InternalError(format!("Failed to connect to redis : {e}"))))?;

        println!("Successfully connected to Redis");
        Ok(Self(client))
    }

    pub fn client(&self) -> &redis::Client {&self.0}
}


pub struct AwsBucket {
    client: S3Client,
    public_url: Url,
}
impl AwsBucket {
    pub async fn connect() -> Result<Self, DatabaseError> {
        let access_key_id = get_env_var("S3_ACCESS_KEY_ID")?;
        let secret_access_key = get_env_var("S3_SECRET_ACCESS_KEY")?;
        let endpoint = get_env_var("S3_ENDPOINT")?;
        let url = get_env_var("S3_PUBLIC_URL")?;
        let public_url = Url::parse(&url)
            .map_err(|_| DatabaseError::from(ApiError::InternalError(format!("Invalid URL: {url}"))))?;

        let credentials = Credentials::builder()
            .provider_name("r2")
            .access_key_id(access_key_id)
            .secret_access_key(secret_access_key)
            .build();

        let config = aws_config::defaults(BehaviorVersion::v2025_01_17())
            .region("us-east-1")
            .credentials_provider(credentials)
            .endpoint_url(endpoint)
            .load()
            .await;

        let client = S3Client::new(&config);

        println!("Successfully connected to AWS S3");
        Ok(Self{client, public_url})
    }
    pub fn client(&self) -> &S3Client {
        &self.client
    }
    pub fn pub_url(&self) -> &Url {
        &self.public_url
    }
}

fn get_env_var(var_key: &str) -> Result<String, DatabaseError> {
    let var = env::var(var_key)
        .map_err(|_| DatabaseError::from(ApiError::InternalError(format!("Missing '{var_key}' environment variable"))))?;

    Ok(var)
}