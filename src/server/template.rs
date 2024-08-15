use minijinja::{value::ViaDeserialize, Environment};
use mm_base2::{init_jinja_env, Config};

use crate::db::Proxy;

pub fn init_templates(config: &Config) -> Environment<'static> {
    let mut env = init_jinja_env(config);
    minijinja_contrib::add_to_environment(&mut env);
    minijinja_embed::load_templates!(&mut env);
    env.add_filter("history_ok_count", history_ok_count);
    env.add_filter("history_down_count", history_down_count);

    env
}

fn history_ok_count(proxy: ViaDeserialize<Proxy>) -> usize {
    proxy.check_history.iter().filter(|x| **x).count()
}

fn history_down_count(proxy: ViaDeserialize<Proxy>) -> usize {
    proxy.check_history.iter().filter(|x| !(**x)).count()
}
