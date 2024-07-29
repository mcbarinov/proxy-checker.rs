use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Json, Router};
use serde_json::Value;

pub use state::AppState;

use crate::app::App;
use crate::config::Config;
use crate::server::api_method::api_method;
use crate::server::asset::asset_router;
use crate::server::auth::auth_router;
use crate::server::router::{api_router, ui_router};
use crate::AppError;

mod api_method;
mod asset;
mod auth;
mod router;
mod state;
mod template;

pub type JsonResponse = Result<Json<Value>, AppError>;
pub type HtmlResponse = Result<Html<String>, AppError>;

pub async fn serve_server(config: &Config, app: Arc<App>) {
    let state = AppState::new(config, app);

    let axum_app = Router::new()
        .merge(ui_router::init())
        .nest("/api", api_router::init())
        .route("/api-post/*path", get(api_method))
        .route("/api-delete/*path", get(api_method))
        .merge(auth_router())
        .merge(asset_router())
        .layer(middleware::from_fn_with_state(state.clone(), auth::access_token_middleware))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_address).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, axum_app).await.unwrap();
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("AppError: {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
