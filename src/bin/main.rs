use indexmap::indexmap;
use mm_base2::Config;
use proxy_checker::{run_scheduler, serve_server, App};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let main_menu = indexmap! {
        "/sources".to_string() => "sources".to_string(),
        "/proxies".to_string() => "proxies".to_string(),
    };
    let config = Config::new(env!("CARGO_PKG_VERSION"), main_menu);

    mm_base2::init_tracing("proxy_checker=debug", config.data_dir.as_str());

    let app = Arc::new(App::new(&config).await);
    run_scheduler(Arc::clone(&app));
    serve_server(&config, app).await;
}
