use axum::extract::{Path, State};
use axum::routing::get;
use axum::Router;

use crate::server::{AppState, JsonResponse};

pub fn init() -> Router<AppState> {
    Router::new().route("/sources/:id", get(get_source))
}

async fn get_source(state: State<AppState>, Path(id): Path<String>) -> JsonResponse {
    let source = state.app.db.get_source(&id).await?;
    state.json(source)
}
