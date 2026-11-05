// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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

pub mod config;
pub mod crud;
pub mod db;
pub mod error;
pub mod graceful;
pub mod input_validation;
pub mod middleware;
pub mod migrate;
pub mod mock_client;
pub mod netguard;
pub mod panic_guard;
pub mod rate_limit;
pub mod response;
pub mod router;
pub mod session;
pub mod storage;
pub mod testing;

pub use crate::error::AppError;
pub use crud::{crud_create, crud_delete, crud_delete_owned, crud_save, crud_save_owned, CrudSpec};
pub use deadpool_postgres::Pool;
pub use rate_limit::RateLimiter;
pub use session::SessionManager;

use std::ops::Deref;

#[cfg(test)]
#[allow(clippy::module_inception)]
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
        Ok(rows
            .into_iter()
            .map(|r| Box::new(r) as Box<dyn RowGet>)
            .collect())
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

/// acquire 的 boxed future 返回类型
pub type ControlClientFuture<'a> = std::pin::Pin<
    Box<
        dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>>
            + Send
            + 'a,
    >,
>;

pub trait ControlPool: Send + Sync {
    fn acquire<'a>(&'a self) -> ControlClientFuture<'a>;
}

#[async_trait::async_trait]
impl ControlPool for Pool {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            let object = self.get().await.map_err(classify_acquire_error)?;
            Ok(std::sync::Arc::new(object) as std::sync::Arc<dyn ControlClient>)
        })
    }
}

/// 把 `Pool::get()` 的失败分类成**可重试**与**不可重试**两类。
///
/// `get()` 阶段失败全部发生在「拿到可用连接之前」：排队超时、建连失败、
/// 池已关闭。这类失败与 SQL 执行错误（在 `query` 阶段才出现）不同——它们
/// 不说明服务端有缺陷，只说明数据库此刻不可用，客户端/网关应重试或降级。
///
/// 修前只有 `Timeout` 被映射 503，`Backend`（连接被拒、握手失败、连接被
/// 回收时发现断开）与 `Closed`（池已关闭，典型为重启/迁移窗口）一律落到
/// 500：调用方看到 500 会当成「服务端 bug」，既不敢重试也触发错误率告警。
/// 配置类错误（未指定 runtime、post-create hook 失败）仍保持 500，因为
/// 重试不会自愈。
pub(crate) fn classify_acquire_error(e: deadpool_postgres::PoolError) -> AppError {
    use deadpool_postgres::PoolError;
    match e {
        PoolError::Timeout(kind) => {
            AppError::Unavailable(format!("database pool timeout ({kind:?})"))
        }
        PoolError::Backend(err) => {
            AppError::Unavailable(format!("database connection failed: {err}"))
        }
        PoolError::Closed => AppError::Unavailable("database pool closed".to_string()),
        other => {
            tracing::error!(error = %other, "pool acquire failed with non-transient error");
            AppError::Internal
        }
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
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>>
                + Send
                + 'a,
        >,
    > {
        self.0.acquire()
    }
}

#[cfg(test)]
mod dyn_control_pool_tests {
    use super::*;
    use crate::mock_client::{mock_control_pool, MockQueryResult};
    use std::sync::Arc;

    /// DynControlPool 是给下游 crate 测试注入 mock 池的包装器；本测试在 shared
    /// 自身走一遍 acquire → Arc<dyn ControlClient> 委托 → 查询的完整链路，
    /// 覆盖 `ControlPool for DynControlPool` 与 `ControlClient for Arc<...>` 的
    /// 委托方法（这些行只被下游测试触发，若本仓库无人用会悄悄退化）。
    #[tokio::test]
    async fn dyn_control_pool_acquire_and_delegate_full_chain() {
        // mock 是 LIFO 弹栈（pop 从末尾取）：队列按调用顺序**逆序**排列。
        let results = Arc::new(tokio::sync::Mutex::new(vec![
            MockQueryResult::Row(vec![("count", serde_json::json!(1))]),
            MockQueryResult::Row(vec![("id", serde_json::json!(7))]),
            MockQueryResult::Row(vec![("id", serde_json::json!(7))]),
            MockQueryResult::Rows(vec![vec![("id", serde_json::json!(7))]]),
        ]));
        let pool = mock_control_pool(results);
        let client = pool.acquire().await.expect("acquire 应成功");

        let rows = client.ctrl_query("SELECT 1", &[]).await.expect("query");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get_i32("id"), 7);

        let one = client
            .ctrl_query_one("SELECT 1", &[])
            .await
            .expect("query_one");
        assert_eq!(one.get_i32("id"), 7);

        let opt = client
            .ctrl_query_opt("SELECT 1", &[])
            .await
            .expect("query_opt");
        assert!(opt.is_some());

        let n = client.ctrl_execute("UPDATE t", &[]).await.expect("execute");
        assert_eq!(n, 1);
    }
}

#[cfg(test)]
mod pool_acquire_error_tests {
    use super::*;
    use axum::response::IntoResponse;

    /// 池已关闭（重启/迁移窗口）时，客户端必须拿到可重试的 503 而不是 500。
    #[tokio::test]
    async fn closed_pool_acquire_maps_to_503() {
        let pool = crate::testing::test_pool();
        pool.close();
        let err = match pool.acquire().await {
            Ok(_) => panic!("关闭后的池不应再发出连接"),
            Err(e) => e,
        };
        assert!(
            matches!(err, AppError::Unavailable(_)),
            "Closed 必须归类为暂时不可用，实际: {err:?}"
        );
        assert_eq!(
            err.into_response().status(),
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        );
    }

    /// 数据库连不上（连接被拒）时同理：这是「库挂了」，不是「服务端有 bug」。
    /// 用指向 127.0.0.1:1 的失败池（连接被拒即时返回，不依赖网络环境）。
    #[tokio::test]
    async fn unreachable_backend_acquire_maps_to_503() {
        let pool = crate::testing::failing_pool();
        let err = match pool.acquire().await {
            Ok(_) => panic!("不可达后端必须报错"),
            Err(e) => e,
        };
        assert!(
            matches!(err, AppError::Unavailable(_)),
            "Backend 必须归类为暂时不可用，实际: {err:?}"
        );
        assert_eq!(
            err.into_response().status(),
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        );
    }

    /// 回归：健康池必须继续正常发出连接（分类改动不能把成功路径弄坏）。
    #[tokio::test]
    async fn healthy_pool_acquire_still_succeeds() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available");
            return;
        }
        let pool = crate::testing::test_pool();
        pool.acquire().await.expect("健康池应能拿到连接");
    }
}
