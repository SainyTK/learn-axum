use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let message = headers.get("message").ok_or_else(|| {
        StatusCode::BAD_REQUEST
    })?;
    let message = message.to_str().map_err(|_error| {
        StatusCode::BAD_REQUEST
    })?.to_owned();
    let extensions = req.extensions_mut();
    extensions.insert(HeaderMessage(message.to_owned()));
    Ok(next.run(req).await)
}
