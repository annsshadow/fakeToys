// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! 组件管理写（x_component CRUD/全表软删 + 控制配置）此前无角色校验，任一登录
//! 用户可增删改组件或全表软删。修复后仅 admin；无 DB 时 is_admin fail-closed
//! → 非 admin 403。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{
        component_delete_all, create_component, delete_component, save_component,
        update_control_config, ComponentRequest,
    };

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn component_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");

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
        assert!(
            matches!(
                create_component(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(ComponentRequest {
                        name: Some("n".to_string()),
                        component_type: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "create_component 非 admin 必须 403"
        );
        assert!(
            matches!(
                save_component(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("c1".to_string()),
                    axum::Json(ComponentRequest {
                        name: Some("n".to_string()),
                        component_type: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "save_component 非 admin 必须 403"
        );
        assert!(
            matches!(
                delete_component(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("c1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_component 非 admin 必须 403"
        );
        assert!(
            matches!(
                component_delete_all(Extension(pool.clone()), Extension(sess.clone())).await,
                Err(AppError::Forbidden)
            ),
            "component_delete_all 非 admin 必须 403"
        );
    }
}
