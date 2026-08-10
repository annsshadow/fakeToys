use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::env;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("connection pool error: {0}")]
    PoolError(String),
}

impl From<deadpool_postgres::PoolError> for DbError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        DbError::PoolError(e.to_string())
    }
}

pub async fn create_pool() -> Result<Pool, DbError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());

    let url = url::Url::parse(&database_url).expect("invalid DATABASE_URL");
    let host = url.host_str().expect("no host in DATABASE_URL");
    let port = url.port().unwrap_or(5432);
    let user = url.username();
    let password = url.password().unwrap_or("");
    let dbname = url.path().trim_start_matches('/');

    let mut cfg = Config::new();
    cfg.host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname(dbname);

    let mgr = Manager::new(cfg, NoTls);
    let wait_ms = env::var("POOL_WAIT_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(10_000);
    let pool = Pool::builder(mgr)
        .wait_timeout(Some(Duration::from_millis(wait_ms)))
        .build()
        .map_err(|e| DbError::PoolError(e.to_string()))?;

    Ok(pool)
}

/// 创建 SeaORM DatabaseConnection（与 create_pool 并行）
pub async fn create_sea_orm_pool() -> Result<DatabaseConnection, DbError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());

    let mut options = ConnectOptions::new(database_url);
    options.max_connections(20).sqlx_logging(false);

    Database::connect(options)
        .await
        .map_err(|e| DbError::PoolError(e.to_string()))
}
