use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use itertools::Itertools;
use minijinja::context;
use serde::{Deserialize, Serialize};

use crate::{
    db::{CreateSource, UpdateSourceDefaults},
    server::{AppState, HtmlResponse},
    AppError,
};

#[derive(Serialize, Deserialize, Debug)]
struct SetSourceItems {
    items: String,
}

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { Redirect::to("/sources") }))
        .route("/sources", get(sources_page))
        .route("/sources", post(create_source))
        .route("/sources/:id/defaults", get(source_defaults_page))
        .route("/sources/:id/defaults", post(set_source_defaults))
        .route("/sources/:id/items", get(source_items_page))
        .route("/sources/:id/items", post(set_source_items))
        .route("/proxies", get(proxies_page))
}

async fn sources_page(state: State<AppState>) -> HtmlResponse {
    let sources = state.app.db.get_sources().await?;
    state.html("sources.html", context! {sources=>sources})
}

async fn source_defaults_page(state: State<AppState>, Path(id): Path<String>) -> HtmlResponse {
    let source = state.app.db.get_source(&id).await?;
    state.html("source_defaults.html", context! {source => source})
}

async fn source_items_page(state: State<AppState>, Path(id): Path<String>) -> HtmlResponse {
    let source = state.app.db.get_source(&id).await?;
    state.html("source_items.html", context! {source => source})
}

async fn proxies_page(state: State<AppState>) -> HtmlResponse {
    let proxies = state.app.db.get_proxies().await?;
    state.html("proxies.html", context! {proxies=>proxies})
}

async fn create_source(state: State<AppState>, Form(form): Form<CreateSource>) -> Result<Redirect, AppError> {
    state.app.db.create_source(form).await?;
    state.redirect("/sources")
}

async fn set_source_defaults(
    state: State<AppState>,
    Path(id): Path<String>,
    Form(form): Form<UpdateSourceDefaults>,
) -> Result<Redirect, AppError> {
    state.app.db.update_source_defaults(&id, form).await?;
    state.redirect("/sources") // TODO: add flash message
}

async fn set_source_items(
    state: State<AppState>,
    Path(id): Path<String>,
    Form(form): Form<SetSourceItems>,
) -> Result<Redirect, AppError> {
    let items: Vec<String> =
        form.items.trim().split('\n').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).unique().collect();
    state.app.db.update_source_items(&id, items).await?;
    state.redirect("/sources") // TODO: add flash message
}
