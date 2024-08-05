use std::sync::Arc;

use crate::config::Config;
use crate::db::Db;
use crate::services::{ProxyService, SourceService, SystemService};

pub struct App {
    pub db: Arc<Db>,
    pub source_service: SourceService,
    pub proxy_service: ProxyService,
    pub system_service: SystemService,
}

impl App {
    pub async fn new(config: &Config) -> Self {
        let db = Db::new(&config.database_url).await.expect("can't init database");
        let db = Arc::new(db);
        let source_service = SourceService::new(db.clone());
        let proxy_service = ProxyService::new(db.clone());
        let system_service = SystemService::new(config.clone());
        Self { db, source_service, proxy_service, system_service }
    }
}
