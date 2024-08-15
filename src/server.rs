use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use mm_base2::Config;
use serde_json::Value;
pub use state::AppState;

use crate::{
    app::App,
    server::routers::{api_router, ui_router},
    AppError,
};

mod routers;
mod state;
mod template;

pub type JsonResponse = Result<Json<Value>, AppError>;
pub type HtmlResponse = Result<Html<String>, AppError>;

pub async fn serve_server(config: &Config, app: Arc<App>) {
    let state = AppState::new(config, app);

    let router = mm_base2::router_without_state(
        &config.access_token,
        config.https_schema,
        &config.data_dir,
        state.templates.clone(),
        ui_router::init(),
        api_router::init(),
        api_router::swagger(),
    );
    let router = router.with_state(state);

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
