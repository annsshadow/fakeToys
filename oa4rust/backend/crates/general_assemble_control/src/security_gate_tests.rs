// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! general 管理写族越权修复——系统配置/参考数据（维护模式、注册开关、考勤范围、
//! 地区、安全等级）仅 admin 可写；此前中间件仅兜底 Authenticated，任一登录用户
//! 即可全局启停维护模式或改安全等级。无 DB 时 is_admin fail-closed 返回 false，
//! 非 admin 会话一律 403，故本组断言不依赖数据库。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::Extension;
    use axum::Json;
    use serde_json::json;
    use shared::error::AppError;

    use crate::{
        area_create, area_delete, area_update, attendscope_create, general_control_create,
        general_control_delete, general_control_save, securityclearance_create,
        securityclearance_delete, securityclearance_enable, securityclearance_update,
        update_general_control_status,
    };

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn general_admin_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");

        assert!(
            matches!(
                update_general_control_status(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Json(json!({"maintenanceMode": true}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "status_update 非 admin 必须 403"
        );
        assert!(
            matches!(
                general_control_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Json(json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "general_control_create 非 admin 必须 403"
        );
        assert!(
            matches!(
                general_control_save(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("id1".to_string()),
                    Json(json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "general_control_save 非 admin 必须 403"
        );
        assert!(
            matches!(
                general_control_delete(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("id1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "general_control_delete 非 admin 必须 403"
        );
        assert!(
            matches!(
                area_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Json(json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "area_create 非 admin 必须 403"
        );
        assert!(
            matches!(
                area_update(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("id1".to_string()),
                    Json(json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "area_update 非 admin 必须 403"
        );
        assert!(
            matches!(
                area_delete(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("id1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "area_delete 非 admin 必须 403"
        );
        assert!(
            matches!(
                securityclearance_enable(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Json(json!({"enabled": true}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "securityclearance_enable 非 admin 必须 403"
        );
        assert!(
            matches!(
                securityclearance_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Json(json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "securityclearance_create 非 admin 必须 403"
        );
        assert!(
            matches!(
                securityclearance_update(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("id1".to_string()),
                    Json(json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "securityclearance_update 非 admin 必须 403"
        );
        assert!(
            matches!(
                securityclearance_delete(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("id1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "securityclearance_delete 非 admin 必须 403"
        );
    }

    #[tokio::test]
    async fn attendscope_create_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");
        let res = attendscope_create(
            Extension(pool),
            Extension(sess),
            Json(crate::AttendScopeCreateRequest {
                name: Some("n".to_string()),
                unit_id: None,
            }),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "attendscope_create 非 admin 必须 403"
        );
    }
}
