use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::Router;

use crate::server::{AppState, JsonResponse};

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/sources/:id", get(get_source))
        .route("/sources/:id/check", post(check_source))
        .route("/sources/:id/delete-proxies", delete(delete_proxies_by_source))
        .route("/sources/:id", delete(delete_source))
}

async fn get_source(state: State<AppState>, Path(id): Path<String>) -> JsonResponse {
    state.json(state.app.db.get_source(&id).await?)
}

async fn check_source(state: State<AppState>, Path(id): Path<String>) -> JsonResponse {
    state.json(state.app.source_service.check(&id).await?)
}

async fn delete_source(state: State<AppState>, Path(id): Path<String>) -> JsonResponse {
    state.app.db.delete_source(&id).await?;
    state.json("ok")
}

async fn delete_proxies_by_source(state: State<AppState>, Path(id): Path<String>) -> JsonResponse {
    state.app.db.delete_proxies_by_source(&id).await?;
    state.json("ok")
}
