use std::{sync::Arc, time::Duration};

use futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;

use crate::{async_synchronized, db::Db, Result};

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

    pub async fn next_check(&self) -> Result<()> {
        async_synchronized!();
        let proxies_to_check = self.db.get_proxies_for_next_check().await?;
        join_all(proxies_to_check.into_iter().map(|id| self.check(id)).collect::<Vec<_>>()).await;
        Ok(())
    }
}
