use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub async fn return_201() -> Response {
    (StatusCode::CREATED, "Inserted 1 row".to_owned()).into_response()
}