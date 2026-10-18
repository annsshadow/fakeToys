// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! 门户内容管理写（surface/layout/page/dict）此前中间件兜底仅 Authenticated，
//! 任一登录用户可增删改门户内容或翻转发布开关。修复后仅 admin；无 DB 时
//! is_admin fail-closed → 非 admin 403。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{
        create_layout, create_surface, delete_layout, page_id_delete, publish_surface,
        surface_publish, CreateLayoutRequest, CreateSurfaceRequest,
    };

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn portal_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");

        assert!(
            matches!(
                create_surface(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(CreateSurfaceRequest {
                        name: Some("n".to_string()),
                        template: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "create_surface 非 admin 必须 403"
        );
        assert!(
            matches!(
                publish_surface(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("s1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "publish_surface 非 admin 必须 403"
        );
        assert!(
            matches!(
                surface_publish(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(serde_json::json!({ "id": "s1" }))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "surface_publish 非 admin 必须 403"
        );
        assert!(
            matches!(
                page_id_delete(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("p1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "page_id_delete 非 admin 必须 403"
        );
        assert!(
            matches!(
                create_layout(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(CreateLayoutRequest {
                        id: None,
                        name: Some("n".to_string()),
                        category: None,
                        content: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "create_layout 非 admin 必须 403"
        );
        assert!(
            matches!(
                delete_layout(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(serde_json::json!({ "id": "l1" }))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_layout 非 admin 必须 403"
        );
    }

    /// 四轮 A3-N2：同 crate 的 dict 数据写族 + script 写族（S5 时漏收的四把，
    /// 当时连 session 都不取，任一登录用户可删/覆写任意 portal_flag 的字典数据、
    /// 往任意门户插入脚本）。
    #[tokio::test]
    async fn portal_dict_and_script_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");
        let path3 = || axum::extract::Path(("d1".to_string(), "p1".to_string(), "x".to_string()));

        assert!(
            matches!(
                crate::dict_dictFlag_portal_portalFlag_path_data_delete(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    path3(),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "dict data delete 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::dict_dictFlag_portal_portalFlag_path_data_post(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    path3(),
                    axum::Json(serde_json::json!({ "v": 1 })),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "dict data post 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::dict_dictFlag_portal_portalFlag_path_data_put(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    path3(),
                    axum::Json(serde_json::json!({ "v": 1 })),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "dict data put 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::script_portal_portal_name_post(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path(("p1".to_string(), "s1".to_string())),
                    axum::Json(serde_json::json!({ "content": "eval('x')" })),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "script portal name post 非 admin 必须 403"
        );
    }
}
