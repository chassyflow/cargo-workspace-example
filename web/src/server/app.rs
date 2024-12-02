use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::{routes, server::state::State};

pub struct App {
    state: Arc<State>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: Arc::new(State::default()),
        }
    }
}

impl App {
    pub async fn router(self) -> anyhow::Result<Router> {
        Ok(Router::new()
            .route("/", get(routes::get))
            .route("/add/:a/:b", get(routes::add::get))
            .with_state(self.state))
    }
}
