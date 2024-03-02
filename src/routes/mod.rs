mod hello_world;
mod mirror_hello;
mod mirror_json;
mod path_variables;
mod query_params;
mod mirror_headers;

use axum::{routing::{get, post}, Router};

use hello_world::get_hello;
use mirror_hello::mirror_hello;
use mirror_json::mirror_json;
use path_variables::{path_variables, hardcoded_path};
use query_params::query_params;
use mirror_headers::mirror_headers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_hello))
        .route("/mirror_hello", post(mirror_hello))
        .route("/mirror_json", post(mirror_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hardcoded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_headers", get(mirror_headers))
}