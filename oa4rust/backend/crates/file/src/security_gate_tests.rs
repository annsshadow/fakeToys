// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第六十一批）：file crate 写面 5 把。
//!
//! - `file_upload`（INSERT `FILE_FILE`）、`folder_create`（INSERT `FILE_FOLDER`）、
//!   `folder_update`（UPDATE）、`folder_remove`（软删）→ 会话存在档（无会话 401）；
//! - `permission_set`（UPSERT `FILE_PERMISSION`）→ admin 档（无会话 401 / 非 admin 403）。
//! `file_upload`/`folder_create` 的 person 仍取表单/body（冒名残留记 §五）。
//!
//! DB 依赖臂：`cargo test --lib -p file -- --ignored`
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, FromRequest, Multipart};
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
        shared::testing::test_session("b61-nonadmin")
    }

    async fn empty_multipart() -> Multipart {
        let req = axum::http::Request::builder()
            .header("content-type", "multipart/form-data; boundary=XB")
            .body(axum::body::Body::from("--XB--\r\n"))
            .unwrap();
        Multipart::from_request(req, &()).await.unwrap()
    }

    /// 无会话臂：dead pool + None 会话，5 把必须全 401。
    #[tokio::test]
    async fn b61_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "file_upload",
                crate::file_upload(Extension(pool.clone()), None, empty_multipart().await)
                    .await
                    .map(|_| ()),
            ),
            (
                "folder_create",
                crate::folder_create(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "name": "n" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "folder_update",
                crate::folder_update(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "id": "f1", "name": "n" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "folder_remove",
                crate::folder_remove(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "id": "f1" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "permission_set",
                crate::permission_set(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "targetType": "folder", "targetId": "f1" })),
                )
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

    /// admin 臂：权限配置写非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b61_permission_set_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let res = crate::permission_set(
            Extension(pool),
            Some(Extension(non_admin())),
            Json(json!({ "targetType": "folder", "targetId": "f1" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 权限配置写必须 403，实得 {res:?}"
        );
    }
}
