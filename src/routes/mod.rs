mod hello_world;
mod mirror_custom_headers;
mod mirror_headers;
mod mirror_hello;
mod mirror_json;
mod path_variables;
mod query_params;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use hello_world::get_hello;
use mirror_custom_headers::mirror_custom_headers;
use mirror_headers::mirror_headers;
use mirror_hello::mirror_hello;
use mirror_json::mirror_json;
use path_variables::{hardcoded_path, path_variables};
use query_params::query_params;

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(get_hello))
        .route("/mirror_hello", post(mirror_hello))
        .route("/mirror_json", post(mirror_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hardcoded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_headers", get(mirror_headers))
        .route("/mirror_custom_headers", get(mirror_custom_headers))
        .layer(cors)
}
