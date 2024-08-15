use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Router,
};
use mm_base2::{Base2State, JsonResult};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{db::LiveProxiesParams, server::AppState, AppError};

#[derive(OpenApi)]
#[openapi(paths(get_live_proxies, mm_base2::system::clean_logfile, mm_base2::system::get_logfile))]
pub struct ApiDoc;

pub fn swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi())
}

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

async fn get_source(state: State<AppState>, Path(id): Path<String>) -> JsonResult {
    state.json(state.app.db.get_source(&id).await?)
}

async fn check_source(state: State<AppState>, Path(id): Path<String>) -> JsonResult {
    state.json(state.app.source_service.check(&id).await?)
}

async fn delete_source(state: State<AppState>, Path(id): Path<String>) -> JsonResult {
    state.app.db.delete_source(&id).await?;
    state.json("ok")
}

async fn delete_proxies_by_source(state: State<AppState>, Path(id): Path<String>) -> JsonResult {
    state.app.db.delete_proxies_by_source(&id).await?;
    state.json("ok")
}

async fn get_proxy(state: State<AppState>, Path(id): Path<i64>) -> JsonResult {
    state.json(state.app.db.get_proxy(id).await?)
}

async fn get_proxy_url(state: State<AppState>, Path(id): Path<i64>) -> Result<String, AppError> {
    Ok(state.app.db.get_proxy(id).await?.url)
}

async fn check_proxy(state: State<AppState>, Path(id): Path<i64>) -> JsonResult {
    state.json(state.app.proxy_service.check(id).await?)
}

#[utoipa::path(get, path = "/api/proxies/live")]
async fn get_live_proxies(state: State<AppState>, Query(params): Query<LiveProxiesParams>) -> JsonResult {
    let proxies = state.app.db.get_live_proxies(params).await?;
    state.json(json!({ "proxies": proxies }))
}
