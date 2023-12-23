use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::config;
use crate::errors::{Error, Result};

use super::user::UserTable;

pub type Db = Pool<Postgres>;

pub async fn start_pg_pool() -> Result<Db> {
    let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&config().database_url)
        .await
        .map_err(Error::DbConnection)
}

pub struct Model {
    pub db: Db,
}

impl Model {
    pub async fn new() -> Result<Self> {
        let db = start_pg_pool().await?;

        Ok(Self { db })
    }

    pub async fn test_db(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.db)
            .await
            .map(|_| ())
            .map_err(Error::DbConnection)
    }

    pub async fn init_db(&self) -> Result<()> {
        sqlx::query(&UserTable::create_table())
            .execute(&self.db)
            .await
            .map(|_| ())
            .map_err(Error::DbConnection)
    }
}
