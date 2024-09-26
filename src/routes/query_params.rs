use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    genres: String,
}

pub async fn query_params(Query(query): Query<Params>) -> Json<Params> {
    Json(query)
}
