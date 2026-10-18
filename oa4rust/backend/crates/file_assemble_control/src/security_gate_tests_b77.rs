//! 安全审计回归（round4 第七十七批）：file_assemble_control 控制配置写收 admin 门
//! + ControlPool 扩展接线修复。
//!
//! 背景（第六十二批记账的"接线缺陷"）：4 把 control-config handler 取
//! `Extension<Arc<dyn ControlPool>>`，但 router 从未提供该扩展 ⇒ extraction 先于
//! handler 失败，四条路全 500 死态（静态分析仍读出 `update_control_config` 是无门
//! 写面 ⇒ 长期挂在 R7）。本批两件事一起做：
//! 1. **接线修复**：router 补 `.layer(Extension(Arc::new(pool) as Arc<dyn ControlPool>))`；
//! 2. **收 admin 门**：`update_control_config`（全局配置写，管理面）无会话 401、
//!    非 admin 403（is_admin 查不到即 fail-closed），门在 `pool.acquire()` 之前。
//!
//! 证据臂：
//! 1. 死池 + 无会话 → `Unauthorized`（门在 DB 之前）。
//! 2. 死池 + 非 admin 会话 → `Forbidden`（is_admin fail-closed，DB 不可用也拒）。
//! 3. **路由活性回归**：整 router 打匿名 PUT `/api/file/assemble/control/update/control/config`
//!    → **401**（既不是 404 也不是 500）——证明接线修复后路由复活且带门。

#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests_b77 {
    use crate::update_control_config;
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;
    use tower::ServiceExt;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn mock_control_pool() -> std::sync::Arc<dyn crate::ControlPool> {
        shared::mock_client::mock_control_pool(shared::mock_client::MockControlClient::empty())
    }

    #[tokio::test]
    async fn b77_writes_reject_without_session() {
        let res = update_control_config(
            Extension(mock_control_pool()),
            Extension(dead_pool()),
            None,
            axum::extract::Json(json!({})),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 update_control_config 必须 401（门在 DB 访问前）"
        );
    }

    #[tokio::test]
    async fn b77_writes_forbidden_for_non_admin() {
        let session = Some(Extension(shared::testing::test_session("b77-nonadmin")));
        let res = update_control_config(
            Extension(mock_control_pool()),
            Extension(dead_pool()),
            session,
            axum::extract::Json(json!({})),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 会话调用 update_control_config 必须 403（is_admin fail-closed）"
        );
    }

    /// 路由活性回归：接线修复后匿名 PUT 必须拿 401（复活且带门），
    /// 而不是接线缺陷期的 500（extraction 失败）或 404（未注册）。
    #[tokio::test]
    async fn b77_route_alive_and_gated_after_wiring_fix() {
        let app = crate::file_assemble_control_router(dead_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/update/control/config")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "接线修复后该路由必须复活且带会话门（匿名 → 401，而非 500/404）"
        );
    }

    /// 真库臂（迁移自 tests.rs 的写通道臂，`--ignored` 跑）：
    /// admin 会话过门 + 写通道在真库上成功（round4-b77 起 UPDATE 按文本绑定，
    /// 适配 TEXT 列 schema）。先幂等造数 global 行，写回同值，不脏数据。
    /// 依赖 b56 造数种子 admin `b56-cfg-admin@u2@P`。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn b77_admin_write_succeeds_on_real_db() {
        if !shared::testing::is_db_available().await {
            return;
        }
        let pool = shared::testing::test_pool();
        // 幂等造数 global 行（TEXT 列，值同 handler 默认）。
        {
            let client = pool.get().await.expect("db");
            client
                .execute(
                    "INSERT INTO x_file_assemble_control_config (id, enabled, default_storage, max_upload_size) \
                     VALUES ('global', 'true', 'local', '104857600') \
                     ON CONFLICT (id) DO NOTHING",
                    &[],
                )
                .await
                .expect("seed global config row");
        }
        let ctrl: std::sync::Arc<dyn crate::ControlPool> =
            std::sync::Arc::new(pool.clone()) as std::sync::Arc<dyn crate::ControlPool>;
        let session = Some(Extension(shared::testing::test_session(
            "b56-cfg-admin@u2@P",
        )));
        let res = update_control_config(
            Extension(ctrl),
            Extension(pool),
            session,
            axum::extract::Json(json!({
                "enabled": true,
                "defaultStorage": "local",
                "maxUploadSize": 104857600i64,
            })),
        )
        .await;
        assert!(
            matches!(&res, Ok(r) if r.0.r#type.as_deref() == Some("success")),
            "admin 会话 + 真 ControlPool → 写通道必须成功（文本绑定适配 TEXT 列）"
        );
    }
}
