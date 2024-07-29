use axum::extract::{Path, State};
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{Form, Router};
use minijinja::context;

use crate::db::{CreateSource, UpdateSourceDefaults};
use crate::server::{AppState, HtmlResponse};
use crate::AppError;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { Redirect::to("/sources") }))
        .route("/sources", get(sources_page))
        .route("/sources", post(create_source))
        .route("/sources/:id/defaults", get(source_defaults_page))
        .route("/sources/:id/defaults", post(set_source_defaults))
}

async fn sources_page(state: State<AppState>) -> HtmlResponse {
    let sources = state.app.db.get_sources().await?;
    state.html("sources.html", context! {sources=>sources})
}

async fn source_defaults_page(state: State<AppState>, Path(id): Path<String>) -> HtmlResponse {
    let source = state.app.db.get_source(&id).await?;
    state.html("source_defaults.html", context! {source => source})
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
