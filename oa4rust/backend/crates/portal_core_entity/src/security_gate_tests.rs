// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 F2）：门户页面写（create/update/remove）此前中间件
//! 兜底仅 Authenticated，任一登录用户可建/改/软删任意门户页面。修复后仅 admin。
//!
//! 与 `portal` crate 的同类门禁测试不同，本 crate 的 handler 取
//! `Extension<DatabaseConnection>`（sea-orm），而 `shared::testing::test_sea_orm_pool()`
//! 需要真实 PostgreSQL 才能构造连接——deadpool 那条离线路径（`test_pool()` +
//! `is_admin` fail-closed）在这里不可用。故本回归标 `#[ignore]`，
//! 离线证据来自 `tests.rs::test_page_*_returns_error_without_db`（缺会话/缺连接时
//! 请求不得进入 SQL 分支）与代码审查；接 DB 后用下面这条命令跑真门：
//! `cargo test --lib -p portal_core_entity -- --ignored`
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{page_create, page_remove, page_update};

    async fn db_or_panic() -> sea_orm::DatabaseConnection {
        match shared::testing::test_sea_orm_pool().await {
            Ok(db) => db,
            Err(e) => {
                panic!("需要 PostgreSQL 才能跑本门禁回归（请在有库的环境带 --ignored 跑）：{e}");
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn portal_page_entity_writes_forbidden_for_non_admin() {
        let db = db_or_panic().await;
        let sess = shared::testing::test_session("u-nonadmin");
        let payload = || axum::Json(serde_json::json!({ "id": "p1", "name": "n" }));

        assert!(
            matches!(
                page_create(
                    Extension(db.clone()),
                    Extension(sess.clone()),
                    axum::Json(serde_json::json!({ "portalId": "x", "name": "n" })),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "page_create 非 admin 必须 403"
        );
        assert!(
            matches!(
                page_update(Extension(db.clone()), Extension(sess.clone()), payload()).await,
                Err(AppError::Forbidden)
            ),
            "page_update 非 admin 必须 403"
        );
        assert!(
            matches!(
                page_remove(Extension(db), Extension(sess), payload()).await,
                Err(AppError::Forbidden)
            ),
            "page_remove 非 admin 必须 403"
        );
    }
}
