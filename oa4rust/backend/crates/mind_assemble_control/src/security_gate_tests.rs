// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第六十批）：mind_assemble_control 写面 5 把。
//!
//! - `update_control_config`（UPDATE `x_mind_assemble_control_config`）→ admin 档；
//! - `save_folder` / `update_folder` / `folder_move_folderId` / `folder_id_force`
//!   （INSERT/UPDATE/软删 `x_mind`）→ 会话存在档（`x_mind.creator` 硬写 "system"，
//!   归属门无意义，至少要求有效会话）。
//!
//! 无会话一律 401（先于任何 SQL，离线可证）。
//!
//! DB 依赖臂：`cargo test --lib -p mind_assemble_control -- --ignored`
#[cfg(test)]
#[allow(clippy::module_inception)]
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
        shared::testing::test_session("b60-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，5 把必须全 401。
    #[tokio::test]
    async fn b60_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "update_control_config",
                crate::update_control_config(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "id": "global", "configData": "{}" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "save_folder",
                crate::save_folder(
                    Extension(pool.clone()),
                    None,
                    Json(serde_json::from_value(json!({ "name": "n" })).unwrap()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "update_folder",
                crate::update_folder(
                    Extension(pool.clone()),
                    None,
                    Path("f1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "folder_move_folderId",
                crate::folder_move_folderId(
                    Extension(pool.clone()),
                    None,
                    Path("f1".to_string()),
                    Json(json!({ "parentId": "p" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "folder_id_force",
                crate::folder_id_force(Extension(pool.clone()), None, Path("f1".to_string()))
                    .await
                    .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 5, "本批 5 把必须逐把点名");
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

    /// admin 臂：全局配置写非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b60_update_control_config_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let res = crate::update_control_config(
            Extension(pool),
            Some(Extension(non_admin())),
            Json(json!({ "id": "global", "configData": "{}" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 全局配置写必须 403，实得 {res:?}"
        );
    }

    /// R6：u2 `mind_save` 收会话存在档（无会话 401，门在 pool.get() 之前）。
    #[tokio::test]
    async fn r6_mind_save_rejects_without_session() {
        let pool = dead_pool();
        let res = crate::u2::mind_save(
            Extension(pool),
            None,
            Json(json!({ "name": "n", "content": "c" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 mind_save 必须 401（门在 pool.get() 之前）"
        );
    }
}
