use tokio::fs::read_to_string;

use crate::config::Config;
use crate::error::AppError;

#[derive(Clone)]
pub struct SystemService {
    config: Config,
}

impl SystemService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn read_logfile(&self) -> Result<String, AppError> {
        let logfile = format!("{}/app.log", self.config.data_dir);
        Ok(read_to_string(logfile).await?)
    }

    pub async fn clean_logfile(&self) -> Result<(), AppError> {
        let logfile = format!("{}/app.log", self.config.data_dir);
        Ok(tokio::fs::write(logfile, "").await?)
    }
}
