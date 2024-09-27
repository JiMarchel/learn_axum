pub async fn always_errors() -> Result<(), axum::http::StatusCode> {
    Err(axum::http::StatusCode::IM_A_TEAPOT)
}
