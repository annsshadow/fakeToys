// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 F3）：门户页面写（save/delete）此前中间件兜底仅
//! Authenticated，任一登录用户可改任意页面正文或**物理删**任意页面。修复后仅 admin；
//! 无 DB 时 `is_admin` fail-closed → 非 admin 403。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use shared::error::AppError;

    use crate::{delete_page, save_page};

    #[tokio::test]
    async fn portal_page_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin");

        assert!(
            matches!(
                save_page(
                    axum::extract::Extension(pool.clone()),
                    axum::extract::Extension(sess.clone()),
                    axum::extract::Path("p1".to_string()),
                    axum::Json(serde_json::json!({ "content": "x" })),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "save_page 非 admin 必须 403"
        );
        assert!(
            matches!(
                delete_page(
                    axum::extract::Extension(pool.clone()),
                    axum::extract::Extension(sess.clone()),
                    axum::extract::Path("p1".to_string()),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_page 非 admin 必须 403"
        );
    }
}
