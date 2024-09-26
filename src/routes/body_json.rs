use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserOutput {
    username: String,
    password: String,
    message: String,
}

pub async fn body_json(Json(body): Json<User>) -> Json<UserOutput> {
    Json(UserOutput {
        username: body.username,
        password: body.password,
        message: "User Created".to_owned(),
    })
}
