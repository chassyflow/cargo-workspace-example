use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use tracing::debug;

pub async fn get(Path((a, b)): Path<(i32, i32)>) -> Response {
    debug!("hit the add route");
    format!("{a} + {b} = {}", jmath::add(a, b)).into_response()
}
