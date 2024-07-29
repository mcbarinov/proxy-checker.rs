use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Host, Request, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use minijinja::Environment;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Method};
use serde_json::Value;

use crate::app::App;
use crate::config::Config;
use crate::server::asset::asset_router;
use crate::server::router::{api_router, ui_router};
use crate::server::template::init_templates;
use crate::AppError;

mod asset;
mod router;
mod template;

pub type JsonResponse = Result<Json<Value>, AppError>;
pub type HtmlResponse = Result<Html<String>, AppError>;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub app: Arc<App>,
    pub templates: Environment<'static>,
}

pub async fn serve_server(config: &Config, app: Arc<App>) {
    let state = AppState { config: config.clone(), app, templates: init_templates(config) };

    let axum_app = Router::new()
        .merge(ui_router::init())
        .nest("/api", api_router::init())
        .route("/api-post/*path", get(api_method))
        .route("/api-delete/*path", get(api_method))
        .merge(asset_router())
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

async fn api_method(state: State<AppState>, Host(hostname): Host, req: Request) -> Result<Response<Body>, AppError> {
    let mut uri = req.uri().to_string();
    let method = if uri.starts_with("/api-post/") {
        uri = uri.replacen("/api-post/", "/api/", 1);
        Method::POST
    } else if uri.starts_with("/api-delete/") {
        uri = uri.replacen("/api-delete/", "/api/", 1);
        Method::DELETE
    } else {
        panic!("unsupported api method: {}", uri);
    };
    let schema = if state.config.https_schema { "https" } else { "http" };
    let url = format!("{schema}://{hostname}{uri}");
    let res = Client::new().request(method, url).header("access-token", &state.config.access_token).send().await?;
    let headers = res.headers().clone();

    let mut response: Response<Body> = Response::builder().body(Body::from_stream(res.bytes_stream())).unwrap();

    response.headers_mut().extend(headers.into_iter().map(|(name, value)| {
        let name = HeaderName::from_bytes(name.unwrap().as_ref()).unwrap();
        let value = HeaderValue::from_bytes(value.as_ref()).unwrap();
        (name, value)
    }));
    Ok(response)
}
