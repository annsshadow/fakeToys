// ──────────────────────────────────────────────────────────────────────────────
// shared — OA4Rust 项目共享库
//
// 本 crate 为 OA4Rust 的各服务二进制 crate 提供：
//   - 统一的 HTTP 错误类型与响应格式（error.rs、response.rs）
//   - 跨服务的中间件：请求追踪日志、全局错误处理（middleware.rs）
//   - 数据库连接池初始化（db.rs）
//   - 路由组装入口（router.rs）
//
// 各二进制 crate（如 oa4rust）通过 `shared::router()` 获取带中间件的 Router，
// 并在启动时调用 `shared::db::create_pool()` 获取数据库连接池。
// ──────────────────────────────────────────────────────────────────────────────

pub mod db;
pub mod error;
pub mod input_validation;
pub mod middleware;
pub mod migrate;
pub mod mock_client;
pub mod rate_limit;
pub mod redis;
pub mod messaging;
pub mod scheduler;
pub mod response;
pub mod router;
pub mod session;
pub mod testing;

pub use deadpool_postgres::Pool;
pub use rate_limit::RateLimiter;
pub use session::SessionManager;

use crate::error::AppError;

pub use messaging::{
    Envelope, InMemoryBus, MessageBus, MessagingError, MessagingResult, RedisPubSubBus, TokenThresholdEvent,
};
use std::ops::Deref;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


// ---- RowGet ----
// Abstraction over a database row so tests can inject mock data.
// Implemented for tokio_postgres::Row (production) and MockRow (tests).

#[async_trait::async_trait]
pub trait RowGet: Send + Sync {
    fn get_i32(&self, col: &str) -> i32;
    fn get_i64(&self, col: &str) -> i64;
    fn get_str(&self, col: &str) -> &str;
    fn get_bool(&self, col: &str) -> bool;
}

#[async_trait::async_trait]
impl RowGet for deadpool_postgres::tokio_postgres::Row {
    fn get_i32(&self, col: &str) -> i32 {
        self.get(col)
    }
    fn get_i64(&self, col: &str) -> i64 {
        self.get(col)
    }
    fn get_str(&self, col: &str) -> &str {
        self.get(col)
    }
    fn get_bool(&self, col: &str) -> bool {
        self.get(col)
    }
}

// ---- ControlClient ----
// Abstraction over a database client so tests can inject a mock.
// Production impl wraps deadpool_postgres::Object (derefs to PgClient).

#[async_trait::async_trait]
pub trait ControlClient: Send + Sync {
    async fn ctrl_query(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>>;
    async fn ctrl_query_one(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>>;
    async fn ctrl_query_opt(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>>;
    async fn ctrl_execute(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait::async_trait]
impl ControlClient for deadpool_postgres::Object {
    async fn ctrl_query(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = self.deref().query(q, p).await?;
        Ok(rows.into_iter().map(|r| Box::new(r) as Box<dyn RowGet>).collect())
    }
    async fn ctrl_query_one(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>> {
        let row = self.deref().query_one(q, p).await?;
        Ok(Box::new(row) as Box<dyn RowGet>)
    }
    async fn ctrl_query_opt(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        let row_opt = self.deref().query_opt(q, p).await?;
        Ok(row_opt.map(|r| Box::new(r) as Box<dyn RowGet>))
    }
    async fn ctrl_execute(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.deref().execute(q, p).await.map_err(Into::into)
    }
}

// Arc<dyn ControlClient> delegates to the inner impl via this blanket.
#[async_trait::async_trait]
impl ControlClient for std::sync::Arc<dyn ControlClient> {
    async fn ctrl_query(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_query(q, p).await
    }
    async fn ctrl_query_one(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_query_one(q, p).await
    }
    async fn ctrl_query_opt(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_query_opt(q, p).await
    }
    async fn ctrl_execute(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_execute(q, p).await
    }
}

// ---- ControlPool ----

pub trait ControlPool: Send + Sync {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>> + Send + 'a>>;
}

#[async_trait::async_trait]
impl ControlPool for Pool {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>> + Send + 'a>>
    {
        Box::pin(async move {
            let object = self.get().await.map_err(|_| AppError::Internal)?;
            Ok(std::sync::Arc::new(object) as std::sync::Arc<dyn ControlClient>)
        })
    }
}

/// Wrapper allowing tests to inject a mock pool via `Arc<dyn ControlPool>`.
pub struct DynControlPool(std::sync::Arc<dyn ControlPool>);

impl DynControlPool {
    pub fn new(inner: std::sync::Arc<dyn ControlPool>) -> Self {
        Self(inner)
    }
}

#[async_trait::async_trait]
impl ControlPool for DynControlPool {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>> + Send + 'a>>
    {
        self.0.acquire()
    }
}
