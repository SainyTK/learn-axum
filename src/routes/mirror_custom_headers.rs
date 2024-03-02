use axum::http::header::{HeaderMap};

pub async fn mirror_custom_headers(headers: HeaderMap) -> String {
    let header_value = headers.get("x-api-key").unwrap();
    let value = header_value.to_str().unwrap().to_owned();
    value
}
