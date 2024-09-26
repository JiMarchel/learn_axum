mod body_json;
mod body_string;
mod hello;

use body_json::body_json;
use body_string::body_str;
use hello::hello_wib;

use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    axum::Router::new()
        .route("/", get(hello_wib))
        .route("/body_string", post(body_str))
        .route("/body_json", post(body_json))
}
