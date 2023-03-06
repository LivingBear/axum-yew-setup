use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegistrationResponse {
    email: String,
    username: String,
    password: String,
    message: String,
}

pub async fn registration_mirror(Json(body): Json<RegistrationRequest>) -> Json<RegistrationResponse> {
    dbg!(&body);
    Json(RegistrationResponse { email: body.email, username: body.username, password: body.password, message: "registration request received.".to_owned()})
}
