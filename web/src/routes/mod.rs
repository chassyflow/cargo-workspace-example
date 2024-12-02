use axum::response::{IntoResponse, Response};
use tracing::debug;

pub mod add;

pub async fn get() -> Response {
    debug!("Hit the index route");
    "Service healthy".into_response()
}
