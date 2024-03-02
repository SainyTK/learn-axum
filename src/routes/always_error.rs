use axum::{http::StatusCode, response::Response};

pub async fn always_error() -> Result<Response, StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}