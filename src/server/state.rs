use std::sync::Arc;

use minijinja::Environment;
use mm_base2::{Base2State, Config};

use crate::{server::template::init_templates, App};

#[derive(Clone)]
pub struct AppState {
    pub app: Arc<App>,
    pub templates: Environment<'static>,
}

impl AppState {
    pub fn new(config: &Config, app: Arc<App>) -> Self {
        Self { app, templates: init_templates(config) }
    }
}

impl Base2State for AppState {
    fn templates(&self) -> &Environment<'static> {
        &self.templates
    }
}
