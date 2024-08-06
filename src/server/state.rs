use std::sync::Arc;

use axum::response::Redirect;
use axum::Json;
use minijinja::Environment;
use serde::Serialize;
use serde_json::json;

use crate::server::template::{init_templates, render_template};
use crate::server::{HtmlResponse, JsonResponse};
use crate::{App, AppError, Config};

#[derive(Clone)]
pub struct AppState {
    pub app: Arc<App>,
    pub templates: Environment<'static>,
}

impl AppState {
    pub fn new(config: &Config, app: Arc<App>) -> Self {
        Self { app, templates: init_templates(config) }
    }

    pub fn html(&self, template_name: &str, data: impl Serialize) -> HtmlResponse {
        render_template(template_name, data, &self.templates)
    }

    pub fn json(&self, data: impl Serialize) -> JsonResponse {
        Ok(Json(json!(data)))
    }
    pub fn redirect(&self, path: &str) -> Result<Redirect, AppError> {
        Ok(Redirect::to(path))
    }
}
