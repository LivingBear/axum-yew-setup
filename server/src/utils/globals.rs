use std::env;
use std::sync::Arc;

use mongodb::Client;
use mongodb::options::ClientOptions;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref MONGO_SESSION: Arc<RwLock<Option<Client>>> = Arc::new(RwLock::new(None));
    pub static ref JWT_SECRET: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(Vec::new()));
}

pub async fn init_mongo_session() {
    let mongo_uri = env::var("MONGO_DB_DEV")
        .unwrap_or_else(|e| panic!("no MONGO_URI in .env: {}", e.to_string()));
    let mut client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    client_options.app_name = Some("my-app".to_string());
    let client = Client::with_options(client_options).unwrap();
    let mut mongo_session = MONGO_SESSION.write().await;
    *mongo_session = Some(client); // <-- Set the value to `Some(client)` instead of `None`
}

pub async fn init_jwt_secret() {
    let jwt_secret = env::var("JWT_SECRET_STRING")
        .unwrap_or_else(|e| panic!("no JWT_SECRET_STRING in .env: {}", e.to_string()));
    let mut secret = JWT_SECRET.write().await;
    *secret = jwt_secret.into_bytes();
}
