use crate::error::DatabaseError;
use std::env;
use std::sync::Arc;
use aws_config::{BehaviorVersion};
use aws_sdk_s3::Client as S3Client;
use mongodb::{Client, Database};
use mongodb::options::{AuthMechanism, ClientOptions, Credential, ServerApi, ServerApiVersion};
use application::AppState;
use domain::error::ApiError;
use crate::repositories::RepositoryFactory;

pub struct Databases {
    mongo: MongoDB,
    aws_bucket: AwsBucket,
}

impl Databases {
    pub async fn connect() -> Result<AppState, DatabaseError> {
        let mongo = MongoDB::connect()
            .await?;

        let aws_bucket = AwsBucket::connect()
            .await;

        let databases = Databases {
            mongo,
            aws_bucket,
        };

        let factory = RepositoryFactory::init(&databases);
        let redis_db = RedisDb::connect().await?;

        let factory = RepositoryFactory::init(&db.db(), redis_db);

        Ok(AppState::new(Arc::new(factory)))
    }

    pub fn mongo(&self) -> &Database {
        self.mongo.db()
    }
    pub fn aws_bucket(&self) -> &S3Client {
        &self.aws_bucket.client()
    }
}


pub struct MongoDB(mongodb::Database);

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

    pub fn db(&self) -> &mongodb::Database {
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

        Ok(Self(client))
    }

    pub fn client(&self) -> &redis::Client {&self.0}
}


pub struct AwsBucket(S3Client);
impl AwsBucket {
    pub async fn connect() ->Self {
        let config = aws_config::defaults(BehaviorVersion::v2025_01_17())
            .endpoint_url(env::var("R2_ENDPOINT").unwrap_or_default())
            .load().await;

        let client = S3Client::new(&config);

        Self(client)
    }
    pub fn client(&self) -> &S3Client {
        &self.0
    }
}