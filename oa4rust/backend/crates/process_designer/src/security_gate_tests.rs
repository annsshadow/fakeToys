// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! 流程设计器 PROCESS_APPLICATION（无属主列的系统级流程定义）写操作此前任一登录
//! 用户可建改删。修复后仅 admin；无 DB 时 is_admin fail-closed → 非 admin 403。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{
        application_create, application_remove, application_update, ApplicationCreateRequest,
        ApplicationRemoveRequest, ApplicationUpdateRequest,
    };

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn process_application_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");

        assert!(
            matches!(
                application_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(ApplicationCreateRequest {
                        name: "n".to_string(),
                        description: None,
                        form_definition: None,
                        status: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "application_create 非 admin 必须 403"
        );
        assert!(
            matches!(
                application_update(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(ApplicationUpdateRequest {
                        id: "a1".to_string(),
                        name: None,
                        description: None,
                        form_definition: None,
                        status: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "application_update 非 admin 必须 403"
        );
        assert!(
            matches!(
                application_remove(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(ApplicationRemoveRequest {
                        id: "a1".to_string()
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "application_remove 非 admin 必须 403"
        );
    }
}
