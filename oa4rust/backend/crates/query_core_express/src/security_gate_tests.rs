// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 A2-F12）：`/api/query/core/express/execute` 此前只判
//! `Statement::Query`，行级过滤靠列名拼接——任意认证用户可读全库任意表（自带
//! `x_identity`/`x_unit_id` 别名即贯穿；过滤器取不到身份/单位时更是原样执行未过滤 SQL）。
//! 修复后：裸 SQL 执行面收 admin（无 DB 时 `is_admin` fail-closed → 403），
//! 且权限谓词拼不出时直接拒绝而非放行。
//!
//! fail-closed 那一支（admin 但无身份/单位行）需要真实 PostgreSQL 才能走到，
//! 本机离线只能证到 admin 门；跑真门的命令见 §四。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use shared::error::AppError;

    use crate::{cache_query_result, execute_query, QueryRequest};

    #[tokio::test]
    async fn query_express_writes_and_raw_sql_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin");

        assert!(
            matches!(
                execute_query(
                    axum::extract::Extension(pool.clone()),
                    axum::extract::Extension(sess.clone()),
                    axum::extract::Json(QueryRequest {
                        query: Some("SELECT * FROM auth_person".to_string()),
                        params: None,
                        timeout: None,
                    }),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "execute_query 非 admin 必须 403"
        );
        assert!(
            matches!(
                cache_query_result(
                    axum::extract::Extension(pool.clone()),
                    axum::extract::Extension(sess.clone()),
                    axum::extract::Path("q1".to_string()),
                    axum::Json(serde_json::json!({ "ttl": 60 })),
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "cache_query_result 非 admin 必须 403"
        );
    }

    /// 未过滤 SQL 的兜底：where_clause 为空即拒绝，不得回落成原样执行。
    #[test]
    fn empty_permission_predicate_is_not_a_free_pass() {
        assert!(
            crate::build_where_clause(&None, &None).is_empty(),
            "空输入必须拼出空谓词——正是 execute_query 里 fail-closed 的触发条件"
        );
    }
}
