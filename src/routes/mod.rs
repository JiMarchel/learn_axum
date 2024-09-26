mod body_json;
mod body_string;
mod hello;
mod path_variables;
mod query_params;

use body_json::body_json;
use body_string::body_str;
use hello::hello_wib;

use axum::{
    routing::{get, post},
    Router,
};
use path_variables::path_variables;
use query_params::query_params;

pub fn create_routes() -> Router {
    axum::Router::new()
        .route("/", get(hello_wib))
        .route("/user/:user_id/team/:team_id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/body_string", post(body_str))
        .route("/body_json", post(body_json))
}
