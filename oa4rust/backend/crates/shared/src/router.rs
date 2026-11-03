// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::Duration;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

use crate::Pool;

// ──────────────────────────────────────────────────────────────────────────────
// router
//
// 提供当前服务的基础路由（无中间件）。
//
// 路由结构：
//   GET /health  → 存活探针，**不碰数据库**，返回 "ok"
//   GET /ready   → 就绪探针，真查一次数据库，可用 200 / 不可用 503
//
// ⚠️ 两者必须分开（1000 轮计划 R16）。
// `/health` 只回答「进程还活着」：数据库挂了、迁移没跑完、连接池全空，
// 它仍然 200 —— 那是**有意**的，因为把它接进 DB 会让「数据库短暂抖动」
// 直接触发编排器重启进程，而重启救不了数据库，只会把在途请求一起砍掉。
// `/ready` 回答「现在能不能接业务流量」：取一条池连接并 `SELECT 1`，
// 失败/超时一律 503，让负载均衡把流量挪走、把实例从 Service 摘掉，
// 而不是让用户拿到一堆 500。
//
// 认证/授权/限流/安全头中间件由 main.rs 在顶层统一挂载一次
// （SecurityState 单一实例），确保所有 /api/* 路由都经过认证，
// 避免各 crate router 各自挂载造成的重复或遗漏。
// ──────────────────────────────────────────────────────────────────────────────

/// 就绪探针的检查上界。
///
/// 为什么不直接用池的 acquire 超时（默认 10s）：探针是**被高频调用**的
/// （k8s 默认每 10s 一次），10s 的探测等于「探针自己先超时」。2s 足够覆盖
/// 一次握手 + `SELECT 1`，又能让负载均衡在几秒内看到失败。
pub const READY_CHECK_TIMEOUT: Duration = Duration::from_secs(2);

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ready", get(ready))
        .with_state(pool)
}

/// 就绪检查：取池连接 → `SELECT 1` → 成功 200，其它一律 503。
///
/// 响应体刻意只有 `ready` / `not ready`：探针的判据是状态码，
/// 而失败原因（连接被拒、口令错、库不存在）**不能**回给未认证调用方 ——
/// 那是把内部拓扑与配置细节暴露在公开端点上。原因只进服务端日志。
async fn ready(State(pool): State<Pool>) -> (StatusCode, &'static str) {
    let check = async {
        let client = pool.get().await.map_err(|e| format!("acquire: {e}"))?;
        client
            .query_one("SELECT 1", &[])
            .await
            .map_err(|e| format!("query: {e}"))?;
        Ok::<(), String>(())
    };
    match tokio::time::timeout(READY_CHECK_TIMEOUT, check).await {
        Ok(Ok(())) => (StatusCode::OK, "ready"),
        Ok(Err(reason)) => {
            tracing::warn!(reason = %reason, "readiness: 数据库不可用");
            (StatusCode::SERVICE_UNAVAILABLE, "not ready")
        }
        Err(_) => {
            tracing::warn!(
                timeout_ms = READY_CHECK_TIMEOUT.as_millis() as u64,
                "readiness: 数据库检查超时"
            );
            (StatusCode::SERVICE_UNAVAILABLE, "not ready")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{router, READY_CHECK_TIMEOUT};
    use crate::testing::{is_db_available, mock_pool, test_pool};
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    /// 指向本机 1 号端口（必然无人监听）的惰性池：`get()` 立即 connection refused。
    /// 用 127.0.0.1 而不是 localhost —— Windows 上 localhost 可能走 ::1 并超时。
    fn dead_pool() -> crate::Pool {
        let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
        cfg.host("127.0.0.1")
            .port(1)
            .user("o2server")
            .password("password")
            .dbname("oa4rust");
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            cfg,
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .max_size(1)
        .build()
        .unwrap()
    }

    async fn get_text(app: axum::Router, uri: &str) -> (StatusCode, String) {
        let res = app
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = res.status();
        let body = to_bytes(res.into_body(), 4096).await.unwrap();
        (status, String::from_utf8_lossy(&body).to_string())
    }

    #[tokio::test]
    async fn health_route_returns_ok() {
        // /health 不碰库：池完全不可用也必须 200（存活与就绪是两件事）。
        let (status, body) = get_text(router(mock_pool()), "/health").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "ok");
    }

    #[tokio::test]
    async fn unknown_route_returns_404() {
        let (status, _) = get_text(router(mock_pool()), "/api/nope").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn ready_returns_503_when_database_unreachable() {
        // 数据库不可达必须映射成 503（可重试、让编排器摘流量），而不是 500。
        //
        // 两种失败形态都落在这里：Linux 上拒绝连接是即时的；Windows 上
        // 打到无人监听的端口可能被静默丢弃（连接一直挂着），此时由探针
        // 自己的 READY_CHECK_TIMEOUT 兜住。**两种都不能超过探针上界** ——
        // 这正是加 timeout 的理由（池的 acquire 超时是 10s，探针等不起）。
        let started = std::time::Instant::now();
        let (status, body) = get_text(router(dead_pool()), "/ready").await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(body, "not ready");
        assert!(
            started.elapsed() < READY_CHECK_TIMEOUT + Duration::from_secs(1),
            "就绪检查耗时 {:?}，应被 {:?} 上界兜住",
            started.elapsed(),
            READY_CHECK_TIMEOUT
        );
    }

    #[tokio::test]
    async fn ready_returns_503_with_empty_pool_config() {
        // 空配置的池：`get()` 直接报配置错误，同样不得 200、不得 500。
        let (status, _) = get_text(router(mock_pool()), "/ready").await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn ready_returns_ok_with_live_database() {
        if !is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        let (status, body) = get_text(router(test_pool()), "/ready").await;
        assert_eq!(status, StatusCode::OK, "真实库就绪应为 200");
        assert_eq!(body, "ready");
    }
}
