// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第五十七批）：processplatform_assemble_designer 写面 6 把。
//!
//! 设计器（设计面）的 6 把真写：`save_flow`（UPDATE x_process_definition）、
//! `delete_flow`（DELETE x_process_definition）、`file_id_upload`（UPDATE PP_E_FILE）、
//! `item_access_delete_exact` / `item_access_delete_process_processId_path_path`
//! （DELETE pp_e_item_access）、`u2_script::delete`（软删 PP_E_SCRIPT）。
//! 全部是设计面管理动作，统一收 admin 档（无会话 401 / 非 admin 403）。
//!
//! 余 14 条（paging/list/estimate/projection/onlyRemoveNotCompleted/merge_data 等）经体读
//! 全为 SELECT 只读 false-positive，按口径不动。
//!
//! DB 依赖臂：`cargo test --lib -p processplatform_assemble_designer -- --ignored`
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, Path};
    use axum::Json;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn non_admin() -> shared::session::Session {
        shared::testing::test_session("b57-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，6 把必须全 401（门在 pool.get() 之前）。
    #[tokio::test]
    async fn b57_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "save_flow",
                crate::save_flow(
                    Extension(pool.clone()),
                    None,
                    Path("p1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "delete_flow",
                crate::delete_flow(Extension(pool.clone()), None, Path("p1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "file_id_upload",
                crate::file_id_upload(
                    Extension(pool.clone()),
                    None,
                    Path("f1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "item_access_delete_exact",
                crate::item_access_delete_exact(
                    Extension(pool.clone()),
                    None,
                    Path(("p1".to_string(), "x".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "item_access_delete_process_processId_path_path",
                crate::item_access_delete_process_processId_path_path(
                    Extension(pool.clone()),
                    None,
                    Path("p1".to_string()),
                    Path("x".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "script_delete",
                crate::u2_script::delete(Extension(pool.clone()), None, Path("s1".to_string()))
                    .await
                    .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 6, "本批 6 把必须逐把点名");
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }

    /// admin 臂：6 把非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b57_writes_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let s = || Some(Extension(non_admin()));
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "save_flow",
                crate::save_flow(
                    Extension(pool.clone()),
                    s(),
                    Path("p1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "delete_flow",
                crate::delete_flow(Extension(pool.clone()), s(), Path("p1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "file_id_upload",
                crate::file_id_upload(
                    Extension(pool.clone()),
                    s(),
                    Path("f1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "item_access_delete_exact",
                crate::item_access_delete_exact(
                    Extension(pool.clone()),
                    s(),
                    Path(("p1".to_string(), "x".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "item_access_delete_process_processId_path_path",
                crate::item_access_delete_process_processId_path_path(
                    Extension(pool.clone()),
                    s(),
                    Path("p1".to_string()),
                    Path("x".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "script_delete",
                crate::u2_script::delete(Extension(pool.clone()), s(), Path("s1".to_string()))
                    .await
                    .map(|_| ()),
            ),
        ];
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "非管理员未回 403（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }
}
