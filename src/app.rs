use std::sync::Arc;

use crate::config::Config;
use crate::db::Db;
use crate::service::SourceService;

pub struct App {
    pub db: Arc<Db>,
    pub source_service: SourceService,
}

impl App {
    pub async fn new(config: &Config) -> Self {
        let db = Db::new(&config.database_url).await.expect("can't init database");
        let db = Arc::new(db);
        let source_service = SourceService::new(db.clone());

        Self { db, source_service }
    }
}
