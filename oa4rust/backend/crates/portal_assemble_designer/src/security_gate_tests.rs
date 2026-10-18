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

    /// 四轮审计第八批（R4-2 续）：portal 设计器主体（门户 / 页面 / 模板页 / 组件 / 输入 /
    /// 字典 / 文件 / 设计稿 / 权限位）的 26 把写同样「登录即可 + creator 硬写 system」。
    /// 门户内容与权限位决定渲染出来的 JS 与可见性，任一有效会话可覆写他人门户，
    /// 与上面 script 族同一档（系统级数据 = admin）收口。断言逐把点名，缺门即红。
    #[tokio::test]
    async fn designer_body_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin-designer");
        let body = serde_json::json!({ "name": "n", "content": "{}" });
        let portal = || {
            axum::Json(crate::CreatePortalRequest {
                name: Some("p".into()),
                description: None,
            })
        };
        let page = || {
            axum::Json(crate::CreatePageRequest {
                name: Some("p".into()),
                category: None,
                content: None,
            })
        };
        let savepage = || axum::Json(crate::SavePageRequest { content: None });
        let p = || axum::extract::Path("x1".to_string());

        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "create_portal",
                crate::create_portal(Extension(pool.clone()), Extension(sess.clone()), portal())
                    .await
                    .map(|_| ()),
            ),
            (
                "delete_portal",
                crate::delete_portal(Extension(pool.clone()), Extension(sess.clone()), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "update_portal",
                crate::update_portal(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "update_portal_icon",
                crate::update_portal_icon(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "portal_id_permission_post",
                crate::portal_id_permission_post(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "create_page",
                crate::create_page(Extension(pool.clone()), Extension(sess.clone()), page())
                    .await
                    .map(|_| ()),
            ),
            (
                "save_page",
                crate::save_page(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    savepage(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "delete_page",
                crate::delete_page(Extension(pool.clone()), Extension(sess.clone()), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "create_templatepage",
                crate::create_templatepage(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "delete_templatepage",
                crate::delete_templatepage(Extension(pool.clone()), Extension(sess.clone()), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "create_widget",
                crate::create_widget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "delete_widget",
                crate::delete_widget(Extension(pool.clone()), Extension(sess.clone()), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "update_widget",
                crate::update_widget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "widget_create",
                crate::widget_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "widget_save",
                crate::widget_save(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "widget_delete",
                crate::widget_delete(Extension(pool.clone()), Extension(sess.clone()), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "input_create",
                crate::input_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "input_cover",
                crate::input_cover(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "input_prepare_create",
                crate::input_prepare_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "file_id_upload",
                crate::file_id_upload(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_create",
                crate::dict_create(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_save",
                crate::dict_save(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "dict_delete",
                crate::dict_delete(Extension(pool.clone()), Extension(sess.clone()), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "create_design",
                crate::create_design(Extension(pool.clone()), Extension(sess.clone()), portal())
                    .await
                    .map(|_| ()),
            ),
            (
                "save_design",
                crate::save_design(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    p(),
                    axum::Json(body.clone()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "design_save",
                crate::design_save(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(body),
                )
                .await
                .map(|_| ()),
            ),
        ];

        assert_eq!(cases.len(), 26, "本批 26 把写必须逐条点名，少一把即漏格");
        let bad: Vec<&str> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(n, _)| *n)
            .collect();
        assert!(
            bad.is_empty(),
            "以下设计器写未返回 403（门缺失或门禁晚于 SQL）：{bad:?}"
        );
    }
}
