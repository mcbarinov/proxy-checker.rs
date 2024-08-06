use std::sync::Arc;

use proxy_checker::{run_scheduler, serve_server, App, Config};

#[tokio::main]
async fn main() {
    let config = Config::new();
    mm_base2::init_tracing("proxy_checker=debug", config.data_dir.as_str());

    let app = Arc::new(App::new(&config).await);
    run_scheduler(Arc::clone(&app));
    serve_server(&config, app).await;
}
