// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第五十六批）：file_assemble_control 剩余写面 3 把。
//!
//! - `delete_file`（POST/DELETE `/api/file/assemble/control/file/delete/{id}`，软删 x_file）
//!   收归属档：按 `x_file.creator` 校归属，无会话 401，行不存在保留 not-found；
//! - `create_file_entity`（POST `/api/file/core/entity/file/create`，INSERT x_file）
//!   收会话档（无会话 401）；creator 仍硬写 "system"（残留记 §五）；
//! - `u2_config_save_system_config`（POST `/api/config`，UPSERT 系统参数）收 admin 档
//!   （`u2_require_admin`：无会话 401 / 非 admin 403）。
//!
//! 余下载/列表/校验族（`*_download`、`*_download_stream`、`u2_attachment2_list_*`、
//! `u2_share_password_verify`）为只读 false-positive，按口径不动。
//! `update_control_config`（PUT `/update/control/config`）因 `Arc<dyn ControlPool>`
//! 扩展未被 router 提供（其生成路由测试标 "not accessible"），extraction 先于 handler
//! 失败，暂记 §五 待修，不在本批加门。
//!
//! 归属臂需真实 PostgreSQL：`cargo test --lib -p file_assemble_control -- --ignored`
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
        shared::testing::test_session("b56-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，3 把必须全 401（门在 pool.get() 之前）。
    #[tokio::test]
    async fn b56_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "delete_file",
                crate::delete_file(Extension(pool.clone()), None, Path("f1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "create_file_entity",
                crate::create_file_entity(Extension(pool.clone()), None, Json(json!({})))
                    .await
                    .map(|_| ()),
            ),
            (
                "u2_config_save_system_config",
                crate::u2_config_save_system_config(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "configs": [] })),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 3, "本批 3 把必须逐把点名");
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

    /// admin 臂：系统参数写非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b56_config_save_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let res = crate::u2_config_save_system_config(
            Extension(pool),
            Some(Extension(non_admin())),
            Json(json!({ "configs": [] })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 系统参数写必须 403，实得 {res:?}"
        );
    }

    /// 归属臂（需真实 PostgreSQL）：x_file 归属他人且本会话非 admin → 软删 403。
    #[tokio::test]
    #[ignore = "需真实 PostgreSQL（DATABASE_URL）"]
    async fn b56_delete_file_non_owner_forbidden() {
        let pool = shared::testing::test_pool();
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_file WHERE id = 'b56-file-owned-by-other'", &[])
                .await
                .unwrap();
            c.execute(
                "INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) \
                 VALUES ('b56-file-owned-by-other','n','p',1,'b56-owner-elsewhere',NOW()::text,'f')",
                &[],
            )
            .await
            .unwrap();
        }
        let res = crate::delete_file(
            Extension(pool.clone()),
            Some(Extension(non_admin())),
            Path("b56-file-owned-by-other".to_string()),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非属主且非 admin 软删文件必须 403，实得 {res:?}"
        );
        pool.get()
            .await
            .unwrap()
            .execute("DELETE FROM x_file WHERE id='b56-file-owned-by-other'", &[])
            .await
            .unwrap();
    }
}
