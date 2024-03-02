mod always_error;
mod get_json;
mod hello_world;
mod middleware_data;
mod mirror_custom_headers;
mod mirror_headers;
mod mirror_hello;
mod mirror_json;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod return_201;
mod set_middleware_custom_header;
mod validate_with_serde;

use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

use always_error::always_error;
use hello_world::get_hello;
use middleware_data::middleware_data;
use mirror_custom_headers::mirror_custom_headers;
use mirror_headers::mirror_headers;
use mirror_hello::mirror_hello;
use mirror_json::mirror_json;
use path_variables::{hardcoded_path, path_variables};
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use return_201::return_201;
use set_middleware_custom_header::set_middleware_custom_header;

use self::{get_json::get_json, validate_with_serde::validate_with_serde};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(get_hello))
        .route("/mirror_hello", post(mirror_hello))
        .route("/mirror_json", post(mirror_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hardcoded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_headers", get(mirror_headers))
        .route("/mirror_custom_headers", get(mirror_custom_headers))
        .route("/shared_data", get(middleware_data))
        .route("/always_error", get(always_error))
        .route("/return_201", post(return_201))
        .route("/get_json", get(get_json))
        .route("/validate_with_serde", post(validate_with_serde))
        .layer(cors)
        .layer(Extension(shared_data))
}
