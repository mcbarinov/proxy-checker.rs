use std::sync::Arc;

use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

use proxy_checker::{serve_server, App, Config};

#[tokio::main]
async fn main() {
    let config = Config::new();

    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::never(&config.data_dir, "app.log"));
    let file_layer = fmt::Layer::new().with_writer(non_blocking.with_max_level(Level::WARN)).json();
    let console_layer = fmt::layer()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_filter(EnvFilter::from("proxy_checker=debug"));
    tracing_subscriber::registry().with(file_layer).with(console_layer).init();

    let app = Arc::new(App::new(&config).await);
    serve_server(&config, app).await;
}
