use axum::http::header::{HeaderMap, USER_AGENT};

pub async fn mirror_headers(headers: HeaderMap) -> String {
    headers
        .get(USER_AGENT)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
