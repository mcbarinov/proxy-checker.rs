use crate::{
    db::Db,
    services::{ProxyService, SourceService},
};
use mm_base2::Config;
use std::sync::Arc;

pub struct App {
    pub db: Arc<Db>,
    pub source_service: SourceService,
    pub proxy_service: ProxyService,
}

impl App {
    pub async fn new(config: &Config) -> Self {
        let db = Db::new(&config.database_url).await.expect("can't init database");
        let db = Arc::new(db);
        let source_service = SourceService::new(db.clone());
        let proxy_service = ProxyService::new(db.clone());
        Self { db, source_service, proxy_service }
    }
}
