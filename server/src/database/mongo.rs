use mongodb::{Client, error::Error, options::ClientOptions};

pub async fn make_client() -> String {
    "connected to the database".to_owned()
}

pub async fn create_mongodb_client() -> Result<Client, Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("my-app".to_string());
    Client::with_options(client_options)
}