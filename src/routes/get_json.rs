use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    id: i32,
    username: String
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "I'm in data".to_owned(),
        id: 1,
        username: "tanakorn".to_owned()
   };
   Json(data)
}