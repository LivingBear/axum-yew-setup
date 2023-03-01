use chrono::{DateTime, Utc};
use mongodb::{bson::doc, Client};

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}


pub async fn create_user() -> String  {
    "create user".to_owned()

    // let collection = client.database("my_db").collection("users");
    // let document = doc! {
    //     "username": username,
    //     "email": email,
    //     "password": password
    // };
    // collection.insert_one(document, None).await?;
    // Ok(())
}

pub async fn get_user() -> String {
    "get user".to_owned()
}

pub async fn update_user() -> String {
    "get user".to_owned()
}
pub async fn delete_user() -> String {
    "get user".to_owned()
}
