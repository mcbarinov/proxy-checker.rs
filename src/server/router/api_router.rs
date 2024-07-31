use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post};
use axum::Router;
use serde_json::json;

use crate::db::LiveProxiesParams;
use crate::server::{AppState, JsonResponse};
use crate::AppError;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/sources/:id", get(get_source))
        .route("/sources/:id/check", post(check_source))
        .route("/sources/:id/delete-proxies", delete(delete_proxies_by_source))
        .route("/sources/:id", delete(delete_source))
        .route("/proxies/live", get(get_live_proxies))
        .route("/proxies/:id", get(get_proxy))
        .route("/proxies/:id/url", get(get_proxy_url))
        .route("/proxies/:id/check", post(check_proxy))
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

async fn get_proxy(state: State<AppState>, Path(id): Path<i64>) -> JsonResponse {
    state.json(state.app.db.get_proxy(id).await?)
}

async fn get_proxy_url(state: State<AppState>, Path(id): Path<i64>) -> Result<String, AppError> {
    Ok(state.app.db.get_proxy(id).await?.url)
}

async fn check_proxy(state: State<AppState>, Path(id): Path<i64>) -> JsonResponse {
    state.json(state.app.proxy_service.check(id).await?)
}

async fn get_live_proxies(state: State<AppState>, Query(params): Query<LiveProxiesParams>) -> JsonResponse {
    let proxies = state.app.db.get_live_proxies(params).await?;
    state.json(json!({ "proxies": proxies }))
}
