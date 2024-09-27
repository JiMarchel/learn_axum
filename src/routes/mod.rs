mod always_errors;
mod body_json;
mod body_string;
mod hello;
mod middleware_custom_header;
mod middleware_message;
mod path_variables;
mod query_params;

use always_errors::always_errors;
use body_json::body_json;
use body_string::body_str;
use hello::hello_wib;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use hyper::Method;
use middleware_custom_header::middleware_custom_header;
use middleware_message::middleware_message;
use path_variables::path_variables;
use query_params::query_params;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from middleware".to_owned(),
    };

    axum::Router::new()
        .route("/", get(hello_wib))
        .route("/user/:user_id/team/:team_id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/middleware_message", get(middleware_message))
        .route("/body_string", post(body_str))
        .route("/body_json", post(body_json))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/middleware_custom_header", get(middleware_custom_header))
        .route("/error", get(always_errors))
}
