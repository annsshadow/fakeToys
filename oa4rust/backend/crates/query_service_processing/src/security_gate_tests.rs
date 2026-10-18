// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第五十二批）：query_service_processing 的写面 4 把。
//!
//! 这 4 把原本不读会话、无门禁——尺子归成 R7「已认证无会话无门禁」：任何登录用户都能
//! 重置查询服务（清缓存 + 计数归 1，事务二写）、UPSERT 附加文档、按 bundle 写动态表行。
//! 全部是无归属全局运维数据，按本 crate 既定「无归属系统数据写 = 管理员档」口径收门
//! （无会话 401 / 非管理员 403，`is_admin` 对不可用库 fail-closed）。门在 `pool.get()` 之前，离线即证。
//!
//! 本 crate 另有 touch / neural 两族 GET 运维面 18 把已在第十八批收管理员档（tests.rs 内离线臂证）；
//! 其余 4 条 R7（process_query / batch_process / design_search / index_directory_document_count）
//! 为 SELECT 只读体 false-positive，按口径不动。

#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, Json, Path};
    use crate::u2::UpdateExtraDocumentWi;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;
    use shared::session::Session;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn non_admin() -> Session {
        shared::testing::test_session("b52-nonadmin")
    }

    // index_update_extra_document 的 body 类型，门在体校验之前触发，body 内容无关紧要。
    fn empty_extra_doc() -> UpdateExtraDocumentWi {
        UpdateExtraDocumentWi {
            r#type: None,
            key: None,
            id: None,
            createTime: None,
            updateTime: None,
            extra: Default::default(),
        }
    }

    // 无会话臂：连不上库的 pool + None 会话，4 把必须全 401——出现 Internal 即门插在 pool.get() 之后。
    #[tokio::test]
    async fn b52_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "reset_service",
                crate::reset_service(Extension(pool.clone()), None).await.map(|_| ()),
            ),
            (
                "index_update_extra_document",
                crate::u2::index_update_extra_document(Extension(pool.clone()), None, Json(empty_extra_doc()))
                    .await
                    .map(|_| ()),
            ),
            (
                "table_insert",
                crate::u2::table_insert(
                    Extension(pool.clone()),
                    None,
                    Path("x".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "table_update_with_bundle",
                crate::u2::table_update_with_bundle(
                    Extension(pool.clone()),
                    None,
                    Path(("x".to_string(), "y".to_string())),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 4, "本批 4 把必须逐把点名");
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

    // 非管理员臂：真库连通时非管理员会话必须 403（is_admin 对不可用库 fail-closed，亦 403）。
    #[tokio::test]
    async fn b52_r7_writes_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "reset_service",
                crate::reset_service(Extension(pool.clone()), Some(Extension(non_admin())))
                    .await
                    .map(|_| ()),
            ),
            (
                "index_update_extra_document",
                crate::u2::index_update_extra_document(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Json(empty_extra_doc()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "table_insert",
                crate::u2::table_insert(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path("x".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "table_update_with_bundle",
                crate::u2::table_update_with_bundle(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path(("x".to_string(), "y".to_string())),
                    Json(json!({})),
                )
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
