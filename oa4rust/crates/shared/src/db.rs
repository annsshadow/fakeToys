use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use dotenvy::dotenv;
use std::env;
use thiserror::Error;

// ──────────────────────────────────────────────────────────────────────────────
// DbError
//
// 数据库连接池相关的自定义错误类型，用于在 `create_pool` 失败时返回更具体的错误信息。
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Error, Debug)]
pub enum DbError {
    /// 连接池创建失败（池配置错误或连接地址无效）
    #[error("connection pool error: {0}")]
    PoolError(String),
}

// 将 deadpool_postgres::PoolError 转换为 DbError
impl From<deadpool_postgres::PoolError> for DbError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        DbError::PoolError(e.to_string())
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// create_pool
//
// 读取环境变量 DATABASE_URL，解析后构建 PostgreSQL 连接池并返回。
//
// 连接池由 deadpool-postgres 管理，支持异步获取/归还连接，避免每次查询新建连接。
//
// 环境变量优先级：
//   1. DATABASE_URL（来自 .env 文件或系统环境变量）
//   2. 默认值 postgres://o2server:password@localhost:5432/o2server_rust（仅用于本地开发）
//
// 返回值：
//   Ok(Pool) — 连接池对象，可通过 pool.get().await 获取单条连接
//   Err(DbError) — 解析失败或池创建失败
// ──────────────────────────────────────────────────────────────────────────────
pub async fn create_pool() -> Result<Pool, DbError> {
    // 加载 .env 文件（失败时静默忽略，允许纯环境变量覆盖）
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/o2server_rust".to_string());

    // 解析 URL，提取 host / port / user / password / dbname
    let url = url::Url::parse(&database_url).expect("invalid DATABASE_URL");
    let host = url.host_str().expect("no host in DATABASE_URL");
    let port = url.port().unwrap_or(5432);
    let user = url.username();
    let password = url.password().unwrap_or("");
    let dbname = url.path().trim_start_matches('/');

    // 构建 tokio-postgres 连接配置
    let mut cfg = Config::new();
    cfg.host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname(dbname);

    // 使用 NoTls（本地开发场景）， Manager 负责用配置创建新连接
    let mgr = Manager::new(cfg, NoTls);
    let pool = Pool::builder(mgr)
        .build()
        .map_err(|e| DbError::PoolError(e.to_string()))?;

    Ok(pool)
}
