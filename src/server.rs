use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::{middleware, Json, Router};
use serde_json::Value;

pub use state::AppState;

use crate::app::App;
use crate::config::Config;
use crate::server::routers::{api_router, ui_router};
use crate::AppError;

mod routers;
mod state;
mod template;

pub type JsonResponse = Result<Json<Value>, AppError>;
pub type HtmlResponse = Result<Html<String>, AppError>;

pub async fn serve_server(config: &Config, app: Arc<App>) {
    let state = AppState::new(config, app);
    let auth_state = mm_base2::auth_state(&config.access_token, state.templates.clone());

    let router = Router::new()
        .merge(api_router::swagger())
        .merge(ui_router::init())
        .nest("/api", api_router::init())
        .merge(mm_base2::auth_router(&config.access_token, state.templates.clone()))
        .merge(mm_base2::api_method_router(&config.access_token, config.https_schema))
        .merge(mm_base2::assets_router())
        .merge(mm_base2::system_router(&config.data_dir))
        .layer(middleware::from_fn_with_state(auth_state, mm_base2::access_token_middleware)) // must be the last middleware
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_address).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("AppError: {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
