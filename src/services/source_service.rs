use std::{net::Ipv4Addr, str::FromStr, sync::Arc, time::Duration};

use itertools::Itertools;
use reqwest::Client;
use url::Url;

use crate::{
    async_synchronized,
    db::{CreateProxy, Db, Source},
};

use mm_base2::Result;

pub struct SourceService {
    db: Arc<Db>,
}

impl SourceService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn next_check(&self) -> Result<()> {
        async_synchronized!();
        if let Some(source_id) = self.db.get_source_for_next_check().await? {
            self.check(&source_id).await?;
        }
        Ok(())
    }

    pub async fn check(&self, id: &str) -> Result<Vec<String>> {
        tracing::debug!("SourceService.check: id={}", id);
        let source = self.db.get_source(id).await?;
        self.db.update_source_checked_at(id).await?;

        let mut urls: Vec<String> = vec![];
        urls.append(&mut self.collect_urls_from_link(&source).await?);
        urls.append(&mut self.collect_urls_from_items(&source).await?);

        let urls: Vec<String> = urls.into_iter().unique().collect();
        for url in &urls {
            if let Some(create_proxy) = CreateProxy::new(id, url.clone()) {
                self.db.create_proxy(create_proxy).await?;
            }
        }

        self.db.update_source_proxy_count(id).await?;
        Ok(urls)
    }

    async fn collect_urls_from_link(&self, source: &Source) -> Result<Vec<String>> {
        let mut urls: Vec<String> = vec![];
        if source.link.is_none() {
            return Ok(urls);
        }
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let res = client.get(source.link.as_ref().unwrap()).send().await?.text().await?;
        let ip4_addresses: Vec<String> = parse_ip4_addresses(&res).into_iter().unique().collect();
        for ip in ip4_addresses {
            if let Some(url) = source.proxy_url(&ip) {
                urls.push(url);
            }
        }
        Ok(urls)
    }

    async fn collect_urls_from_items(&self, source: &Source) -> Result<Vec<String>> {
        let mut urls: Vec<String> = vec![];
        for item in &source.items {
            if let Some(url) = source.proxy_url(item) {
                urls.push(url);
            }
        }
        Ok(urls)
    }
}

impl Source {
    fn proxy_url(&self, item: &str) -> Option<String> {
        // item is a link
        if item.starts_with("http://") || item.starts_with("socks5://") {
            let mut url = Url::parse(item).ok()?;
            if url.username().is_empty() {
                if let Some(default_username) = self.default_username.as_deref() {
                    let _ = url.set_username(default_username);
                }
            }
            if url.password().is_none() {
                if let Some(default_password) = self.default_password.as_deref() {
                    let _ = url.set_password(Some(default_password));
                }
            }
            if url.port().is_none() {
                if let Some(default_port) = self.default_port {
                    let _ = url.set_port(Some(default_port as u16));
                }
            }
            return Some(url.to_string());
        }

        if Ipv4Addr::from_str(item).is_ok()
            && self.default_protocol.is_some()
            && self.default_username.is_some()
            && self.default_port.is_some()
        {
            let protocol = self.default_protocol.as_ref().unwrap();
            let username = self.default_username.as_ref().unwrap();
            let password = self.default_password.as_ref().unwrap();
            let port = self.default_port.as_ref().unwrap();
            return Some(format!("{protocol}://{username}:{password}@{item}:{port}"));
        }
        None
    }
}

fn parse_ip4_addresses(data: &str) -> Vec<String> {
    data.split_whitespace().filter(|x| Ipv4Addr::from_str(x).is_ok()).map(|x| x.to_string()).collect()
}
