use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde::Deserialize;

use crate::db::Db;
use crate::Result;

pub struct ProxyService {
    db: Arc<Db>,
}

#[derive(Deserialize, Debug)]
struct IpResponse {
    origin: String,
}

impl ProxyService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: i64) -> Result<bool> {
        let proxy = self.db.get_proxy(id).await?;
        let client = Client::builder().timeout(Duration::from_secs(5)).proxy(reqwest::Proxy::all(proxy.url).unwrap()).build()?;
        let mut ok = false;
        if let Ok(res) = client.get("https://httpbin.org/ip").send().await {
            if let Ok(res) = res.json::<IpResponse>().await {
                if res.origin == proxy.ip {
                    ok = true;
                }
            }
        }
        self.db.update_proxy_status(id, ok).await?;
        Ok(ok)
    }
}
