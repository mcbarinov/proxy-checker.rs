use std::sync::Arc;

use axum::{response::Redirect, Json};
use minijinja::Environment;
use mm_base2::Config;
use serde::Serialize;
use serde_json::json;

use crate::{
    server::{
        template::{init_templates, render_template},
        HtmlResponse, JsonResponse,
    },
    App, AppError,
};

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
