use axum::response::Html;
use chrono::DateTime;
use minijinja::value::ViaDeserialize;
use minijinja::{Environment, Value};
use serde::de::Error;
use serde::Serialize;

use crate::db::Proxy;
use crate::server::HtmlResponse;
use crate::Config;

pub fn init_templates(config: &Config) -> Environment<'static> {
    let mut env = Environment::new();
    minijinja_contrib::add_to_environment(&mut env);
    minijinja_embed::load_templates!(&mut env);
    env.add_filter("none", none);
    env.add_filter("dt", dt);
    env.add_filter("history_ok_count", history_ok_count);
    env.add_filter("history_down_count", history_down_count);
    env.add_global("confirm", Value::from_safe_string(r#" onclick="return confirm('sure?')" "#.to_string()));
    env.add_global("config", Value::from_serialize(config));
    env
}

pub fn none(value: Value) -> Value {
    if value.is_undefined() || value.is_none() {
        Value::from("")
    } else {
        value
    }
}

pub fn dt(value: Value) -> Result<String, minijinja::Error> {
    if value.is_undefined() || value.is_none() {
        Ok("".to_string())
    } else {
        let res = DateTime::parse_from_rfc3339(&value.to_string())
            .map_err(|_| minijinja::Error::custom("dt filter failed to parse datetime"))?
            .to_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        Ok(res)
    }
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
