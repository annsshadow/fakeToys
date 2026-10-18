// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! 系统消息发布（x_msg_message INSERT）此前任一登录用户可伪造全员可见的系统
//! 通知。修复后仅 admin 可发布；无 DB 时 is_admin fail-closed → 非 admin 403。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::custom_create;

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn custom_create_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let res = custom_create(
            Extension(pool),
            Extension(build_session("u-nonadmin")),
            axum::Json(crate::CreateMessageRequest {
                title: Some("t".to_string()),
                body: None,
                r#type: None,
                consumer: None,
            }),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "custom_create 非 admin 必须 403（无 DB 时 is_admin fail-closed）"
        );
    }
}
