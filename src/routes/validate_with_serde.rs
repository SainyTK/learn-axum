use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserData {
    username: Option<String>,
    password: String
}

pub async fn validate_with_serde(Json(data): Json<UserData>) {
    dbg!(data);
}