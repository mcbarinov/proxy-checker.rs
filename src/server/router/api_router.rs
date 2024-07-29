use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Router;

use crate::server::{AppState, JsonResponse};

pub fn init() -> Router<AppState> {
    Router::new().route("/sources/:id", get(get_source)).route("/sources/:id/check", post(check_source))
}

async fn get_source(state: State<AppState>, Path(id): Path<String>) -> JsonResponse {
    state.json(state.app.db.get_source(&id).await?)
}

async fn check_source(state: State<AppState>, Path(id): Path<String>) -> JsonResponse {
    state.json(state.app.source_service.check(&id).await?)
}
