use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Address {
    city: String,
    postal_code: i32,
}

pub async fn json_response() -> Json<Address> {
    let addr = Address {
        city: "Sidoarjo".to_owned(),
        postal_code: 1273,
    };

    Json(addr)
}
