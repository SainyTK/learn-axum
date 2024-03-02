mod hello_world;
mod mirror_hello;
mod mirror_json;

use axum::{routing::{get, post}, Router};

use hello_world::get_hello;
use mirror_hello::mirror_hello;
use mirror_json::mirror_json;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_hello))
        .route("/mirror_hello", post(mirror_hello))
        .route("/mirror_json", post(mirror_json))
}