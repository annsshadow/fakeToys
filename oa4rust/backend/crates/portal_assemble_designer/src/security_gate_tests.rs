// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 A3-N1）：portal 设计器脚本写此前整族无门禁——`update` 按 IDOR
//! 覆写任意 `x_portal_script`、`delete` 连 session 都不取。脚本正文由 portal surface 读回渲染，
//! CSP 允许 `script-src 'self' 'unsafe-eval'`，等价于在受害者门户里执行存储型 JS。
//! 修复后 create/update/delete 三把写收 admin（无 DB 时 `is_admin` fail-closed → 403）。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::u2_script::{create, delete, update};

    #[tokio::test]
    async fn designer_script_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin");
        let body = serde_json::json!({ "name": "n", "content": "eval('x')" });

        assert!(
            matches!(
                create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "u2_script::create 非 admin 必须 403"
        );
        assert!(
            matches!(
                update(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("s1".to_string()),
                    axum::Json(body.clone()),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "u2_script::update 非 admin 必须 403"
        );
        assert!(
            matches!(
                delete(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("s1".to_string()),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "u2_script::delete 非 admin 必须 403"
        );
    }
}
