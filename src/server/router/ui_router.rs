use axum::Router;

use crate::server::AppState;

pub fn init() -> Router<AppState> {
    Router::new()
}
