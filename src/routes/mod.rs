use axum::{
    routing::{get, post},
    Router,
};
mod body_string;
mod hello;
use body_string::body_str;
use hello::hello_wib;

pub fn create_routes() -> Router {
    axum::Router::new()
        .route("/", get(hello_wib))
        .route("/body", post(body_str))
}
