use std::borrow::Borrow;

use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct UserRequest {
    #[validate(email)]
    username: String,
    #[validate(length(min = 6, message = "must be at least 8 characters"))]
    password: String,
}

#[async_trait]
impl<B> FromRequest<B> for UserRequest
where
    B: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<Body>, _state: &B) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<UserRequest>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;
        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        dbg!(user.borrow());
        Ok(user)
    }
}

pub async fn custom_json_extractor(data: UserRequest) {
    dbg!(data);
}
