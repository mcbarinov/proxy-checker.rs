use axum::response::Html;
use minijinja::{value::ViaDeserialize, Environment, Value};
use mm_base2::init_jinja_env;
use serde::Serialize;

use crate::{db::Proxy, server::HtmlResponse, Config};

pub fn init_templates(config: &Config) -> Environment<'static> {
    let mut env = init_jinja_env();
    minijinja_contrib::add_to_environment(&mut env);
    minijinja_embed::load_templates!(&mut env);
    env.add_filter("history_ok_count", history_ok_count);
    env.add_filter("history_down_count", history_down_count);
    env.add_global("config", Value::from_serialize(config));
    env
}

fn history_ok_count(proxy: ViaDeserialize<Proxy>) -> usize {
    proxy.check_history.iter().filter(|x| **x).count()
}

fn history_down_count(proxy: ViaDeserialize<Proxy>) -> usize {
    proxy.check_history.iter().filter(|x| !(**x)).count()
}

pub fn render_template<S: Serialize>(template_name: &str, context: S, env: &Environment) -> HtmlResponse {
    let tmpl = env.get_template(template_name)?;
    let content = tmpl.render(context)?;
    Ok(Html(content))
}
