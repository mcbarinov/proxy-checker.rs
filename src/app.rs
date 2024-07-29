use std::sync::Arc;

use crate::config::Config;
use crate::db::Db;

pub struct App {
    pub db: Arc<Db>,
}

impl App {
    pub async fn new(config: &Config) -> Self {
        let db = Db::new(&config.database_url).await.expect("can't init database");
        let db = Arc::new(db);

        Self { db }
    }
}
