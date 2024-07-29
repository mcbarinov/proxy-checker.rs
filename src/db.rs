use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::NoneAsEmptyString;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::{AppError, Result};

pub struct Db {
    pool: PgPool,
}

// models

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub link: Option<String>,
    pub default_protocol: Option<String>,
    pub default_username: Option<String>,
    pub default_password: Option<String>,
    pub default_port: Option<i32>,
    pub items: Vec<String>,
    pub proxy_count: i32,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    pub id: i64,
    pub status: String,
    pub source_id: String,
    pub url: String,
    pub ip: String,
    pub protocol: String,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
    pub last_ok_at: Option<DateTime<Utc>>,
    pub check_history: Vec<bool>,
}

// args

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSource {
    pub id: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub link: Option<String>,
}

pub struct CreateProxy {
    pub source_id: String,
    pub url: String,
    pub ip: String,
    pub protocol: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSourceDefaults {
    #[serde_as(as = "NoneAsEmptyString")]
    pub default_protocol: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub default_username: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub default_password: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub default_port: Option<i32>,
}

impl CreateProxy {
    pub fn new(source_id: &str, url: String) -> Option<Self> {
        let url = url::Url::parse(&url).ok()?;
        let ip = url.host_str()?.to_string();
        let protocol = url.scheme().to_string();
        Some(CreateProxy { source_id: source_id.to_string(), url: url.to_string(), ip, protocol })
    }
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Db { pool })
    }

    pub async fn create_source(&self, args: CreateSource) -> Result<()> {
        sqlx::query!(r"insert into source (id, link) values ($1, $2)", args.id, args.link).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_sources(&self) -> Result<Vec<Source>> {
        let sources: Vec<Source> = sqlx::query_as!(Source, "select * from source order by id").fetch_all(&self.pool).await?;
        Ok(sources)
    }

    pub async fn get_source(&self, id: &str) -> Result<Source> {
        sqlx::query_as!(Source, "select * from source where id = $1", id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn update_source_checked_at(&self, id: &str) -> Result<()> {
        sqlx::query!("update source set checked_at = now() where id = $1", id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_source_items(&self, id: &str, items: Vec<String>) -> Result<()> {
        sqlx::query!("update source set items = $1 where id = $2", &items, id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_source_defaults(&self, id: &str, args: UpdateSourceDefaults) -> Result<()> {
        sqlx::query!(
            r"
            update source
            set default_protocol = $1, default_username = $2, default_password = $3, default_port = $4
            where id = $5",
            args.default_protocol,
            args.default_username,
            args.default_password,
            args.default_port,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_source_proxy_count(&self, id: &str) -> Result<()> {
        sqlx::query!(r"update source set proxy_count = (select count(*) from proxy where source_id = $1 ) where id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_source(&self, id: &str) -> Result<()> {
        sqlx::query!("delete from proxy where source_id = $1", id).execute(&self.pool).await?;
        sqlx::query!("delete from source where id = $1", id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn create_proxy(&self, args: CreateProxy) -> Result<()> {
        sqlx::query!(
            r"
            insert into proxy (source_id, url, ip, protocol)
            values ($1, $2, $3, $4) on conflict do nothing",
            args.source_id,
            args.url,
            args.ip,
            args.protocol
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_proxies(&self) -> Result<Vec<Proxy>> {
        let proxies: Vec<Proxy> = sqlx::query_as!(Proxy, "select * from proxy order by id").fetch_all(&self.pool).await?;
        Ok(proxies)
    }

    pub async fn get_proxy(&self, id: i64) -> Result<Proxy> {
        sqlx::query_as!(Proxy, "select * from proxy where id = $1", id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn delete_proxies_by_source(&self, source_id: &str) -> Result<()> {
        sqlx::query!("delete from proxy where source_id = $1", source_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_proxy_status(&self, id: i64, ok: bool) -> Result<()> {
        let proxy = self.get_proxy(id).await?;
        // keep last 100 elements only
        let mut check_history = proxy.check_history;
        check_history.insert(0, ok);
        check_history.truncate(100);

        if ok {
            sqlx::query!("update proxy set last_ok_at = now() where id = $1", id).execute(&self.pool).await?;
        }

        let status = if ok { "ok" } else { "down" };
        sqlx::query!(
            "update proxy set status = $1, checked_at = now(), check_history = $2 where id = $3",
            status,
            &check_history,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
