// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! hotpic 轮播图/控制配置 = 管理台内容，此前任一登录用户可增删改（creator 恒写
//! "system"）。修复后仅 admin；无 DB 时 is_admin fail-closed → 非 admin 403。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{create_hotpic, delete_hotpic, save_hotpic, update_control_config};

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn hotpic_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");

        assert!(
            matches!(
                create_hotpic(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(crate::HotpicRequest {
                        title: Some("t".to_string()),
                        imageUrl: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "create_hotpic 非 admin 必须 403"
        );
        assert!(
            matches!(
                save_hotpic(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("h1".to_string()),
                    axum::Json(crate::HotpicRequest {
                        title: Some("t".to_string()),
                        imageUrl: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "save_hotpic 非 admin 必须 403"
        );
        assert!(
            matches!(
                delete_hotpic(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("h1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_hotpic 非 admin 必须 403"
        );
        assert!(
            matches!(
                update_control_config(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(serde_json::json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "update_control_config 非 admin 必须 403"
        );
    }
}
