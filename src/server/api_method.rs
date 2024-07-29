use axum::body::Body;
use axum::extract::{Host, Request, State};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::response::Response;
use reqwest::Client;

use crate::server::AppState;
use crate::AppError;

pub async fn api_method(state: State<AppState>, Host(hostname): Host, req: Request) -> Result<Response<Body>, AppError> {
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
