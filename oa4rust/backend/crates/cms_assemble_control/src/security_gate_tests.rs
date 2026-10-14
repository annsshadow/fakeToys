// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 四轮审计第八批（R4-2 续 / N-07）：CMS 装配控制台的**元数据写族**此前整族「登录即可」——
//! 既不取会话也不校归属，`creator` 恒写 `"system"`。这类表（`x_cms_dict` / `x_cms_form` /
//! `x_cms_xform` / `x_cms_view` / `x_cms_templateform`）是**结构定义**而不是业务数据：
//! 任一有效会话可改表单/视图定义，把后续所有读取与提交导向攻击者的字段（一次改动影响全站），
//! `update_control_config` 还能直接翻安全相关开关，`permission_management_refresh_all`
//! 是一条无门的全表权限缓存重刷。按「无属主的系统级数据 = admin」档收口，与 §三.4 的
//! `dict_edit` / `job_delete` 同族。`is_admin` 无库时 fail-closed，故拒绝侧离线即可证。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, Path};
    use axum::Json;
    use shared::error::AppError;

    /// 17 把元数据/配置写：非 admin（含无库 fail-closed）全部必须 403，逐把点名。
    #[tokio::test]
    async fn cms_metadata_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin-cms");
        let body = || Json(serde_json::json!({ "name": "n", "content": "{}" }));
        let p = || Path("x1".to_string());
        let ext = || Extension(sess.clone());

        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "update_control_config",
                crate::update_control_config(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "permission_management_refresh_all",
                crate::permission_management_refresh_all(Extension(pool.clone()), ext())
                    .await
                    .map(|_| ()),
            ),
            (
                "dict_create",
                crate::dict_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "dict_save",
                crate::dict_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "dict_delete",
                crate::dict_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "form_create",
                crate::form_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "form_save",
                crate::form_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "form_delete",
                crate::form_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "xform_create",
                crate::xform_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "xform_save",
                crate::xform_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "xform_delete",
                crate::xform_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_create",
                crate::view_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_save",
                crate::view_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_delete",
                crate::view_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "templateform_create",
                crate::templateform_create(Extension(pool.clone()), ext(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "templateform_save",
                crate::templateform_save(Extension(pool.clone()), ext(), p(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "templateform_delete",
                crate::templateform_delete(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
        ];

        assert_eq!(cases.len(), 17, "本批 17 把写必须逐条点名，少一把即漏格");
        let bad: Vec<&str> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(n, _)| *n)
            .collect();
        assert!(
            bad.is_empty(),
            "以下写未返回 403（门缺失或门禁晚于 SQL）：{bad:?}"
        );
    }

    /// 尺子自证：同一池、同一会话，去掉门禁直接落进业务体时错误形态**不是** 403
    /// （离线拿不到库 → `Internal`）。否则上面的「全部 403」会因为库缺失而恒真。
    #[tokio::test]
    async fn gate_is_what_rejects_not_the_missing_database() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin-cms");

        let gated = crate::dict_delete(
            Extension(pool.clone()),
            Extension(sess.clone()),
            Path("x1".to_string()),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(gated, Err(AppError::Forbidden)),
            "带门必须 Forbidden，实际 {gated:?}"
        );

        let ungated = shared::crud_delete(&pool, &crate::dict_spec(), "x1")
            .await
            .map(|_| ());
        assert!(
            !matches!(ungated, Err(AppError::Forbidden)),
            "无门路径不应伪装成 403，否则上面的断言恒真：{ungated:?}"
        );
    }
}
