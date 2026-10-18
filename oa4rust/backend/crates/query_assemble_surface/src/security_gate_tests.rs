// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第五十九批）：query_assemble_surface 动态查询数据写 10 把。
//!
//! - 动态表行：`table_tableFlag_row_id_mockputtopost`（UPDATE x_query_table_data）、
//!   `table_tableFlag_row_id_part_update`（jsonb_set 局部 UPDATE）、
//!   `table_row_insert` / `table_row_insert_one`（INSERT x_query_table_data）；
//! - 视图内容：`view_id_{bundle,excel,execute}_mockputtopost` 与
//!   `view_flag_flag_query_queryFlag_{bundle,excel,execute}_mockputtopost`（UPDATE x_query_view）。
//!
//! 全部是查询设计/运行数据写，收 admin 档（`require_admin_opt`：无会话 401 / 非 admin 403），
//! 与同 crate `importmodel_record_delete` 同口径。
//!
//! 共 10 把 handler → 17 条 R7 路由。
//! DB 依赖臂：`cargo test --lib -p query_assemble_surface -- --ignored`
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
        shared::testing::test_session("b59-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，10 把必须全 401（门在 pool.get() 之前）。
    #[tokio::test]
    async fn b59_writes_reject_without_session() {
        let pool = dead_pool();
        let two = || Path(("t1".to_string(), "r1".to_string()));
        let one = || Path("t1".to_string());
        let body = || Json(json!({}));
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "table_tableFlag_row_id_mockputtopost",
                crate::table_tableFlag_row_id_mockputtopost(Extension(pool.clone()), None, two(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "table_tableFlag_row_id_part_update",
                crate::table_tableFlag_row_id_part_update(Extension(pool.clone()), None, two(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "table_row_insert",
                crate::u2_closures::table_row_insert(Extension(pool.clone()), None, one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "table_row_insert_one",
                crate::u2_closures::table_row_insert_one(Extension(pool.clone()), None, one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_id_bundle_mockputtopost",
                crate::view_id_bundle_mockputtopost(Extension(pool.clone()), None, one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_id_excel_mockputtopost",
                crate::view_id_excel_mockputtopost(Extension(pool.clone()), None, one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_id_execute_mockputtopost",
                crate::view_id_execute_mockputtopost(Extension(pool.clone()), None, one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_flag_flag_query_queryFlag_bundle_mockputtopost",
                crate::view_flag_flag_query_queryFlag_bundle_mockputtopost(
                    Extension(pool.clone()),
                    None,
                    Path(("f1".to_string(), "q1".to_string())),
                    body(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "view_flag_flag_query_queryFlag_excel_mockputtopost",
                crate::view_flag_flag_query_queryFlag_excel_mockputtopost(
                    Extension(pool.clone()),
                    None,
                    Path(("f1".to_string(), "q1".to_string())),
                    body(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "view_flag_flag_query_queryFlag_execute_mockputtopost",
                crate::view_flag_flag_query_queryFlag_execute_mockputtopost(
                    Extension(pool.clone()),
                    None,
                    Path(("f1".to_string(), "q1".to_string())),
                    body(),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 10, "本批 10 把必须逐把点名");
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

    /// admin 臂：10 把非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b59_writes_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let s = || Some(Extension(non_admin()));
        let two = || Path(("t1".to_string(), "r1".to_string()));
        let one = || Path("t1".to_string());
        let body = || Json(json!({}));
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "table_tableFlag_row_id_mockputtopost",
                crate::table_tableFlag_row_id_mockputtopost(Extension(pool.clone()), s(), two(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "table_tableFlag_row_id_part_update",
                crate::table_tableFlag_row_id_part_update(Extension(pool.clone()), s(), two(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "table_row_insert",
                crate::u2_closures::table_row_insert(Extension(pool.clone()), s(), one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "table_row_insert_one",
                crate::u2_closures::table_row_insert_one(Extension(pool.clone()), s(), one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_id_bundle_mockputtopost",
                crate::view_id_bundle_mockputtopost(Extension(pool.clone()), s(), one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_id_excel_mockputtopost",
                crate::view_id_excel_mockputtopost(Extension(pool.clone()), s(), one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_id_execute_mockputtopost",
                crate::view_id_execute_mockputtopost(Extension(pool.clone()), s(), one(), body())
                    .await
                    .map(|_| ()),
            ),
            (
                "view_flag_flag_query_queryFlag_bundle_mockputtopost",
                crate::view_flag_flag_query_queryFlag_bundle_mockputtopost(
                    Extension(pool.clone()),
                    s(),
                    Path(("f1".to_string(), "q1".to_string())),
                    body(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "view_flag_flag_query_queryFlag_excel_mockputtopost",
                crate::view_flag_flag_query_queryFlag_excel_mockputtopost(
                    Extension(pool.clone()),
                    s(),
                    Path(("f1".to_string(), "q1".to_string())),
                    body(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "view_flag_flag_query_queryFlag_execute_mockputtopost",
                crate::view_flag_flag_query_queryFlag_execute_mockputtopost(
                    Extension(pool.clone()),
                    s(),
                    Path(("f1".to_string(), "q1".to_string())),
                    body(),
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
