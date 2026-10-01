// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"modelFlag": "test", "generating": true}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_router_builds() {
        let pool = build_test_pool();
        let _ = crate::router(pool);
    }

    #[tokio::test]
    async fn test_neural_generate_model_route_exists() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/service/neural/generate/test-model")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Route exists - should not return 404
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_neural_list_model_route_exists() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/service/neural/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_validate_query_rejects_dangerous_sql_keywords_case_insensitively() {
        // 意图：neural query 服务会把用户原文直接用于取数，validate_query 是
        // 防注入的第一道闸——8 个危险关键词任一出现（大小写不敏感、子串匹配）
        // 都必须拒绝，否则恶意用户可借自然语言夹带 DDL/DML。
        assert!(!crate::validate_query("SELECT * FROM t"));
        assert!(!crate::validate_query("drop database oa4rust"));
        assert!(!crate::validate_query("please update the report")); // 子串命中 UPDATE
        assert!(!crate::validate_query("DELETE me"));
        assert!(!crate::validate_query("truncate x"));
        assert!(!crate::validate_query("alter system"));
        assert!(!crate::validate_query("create table x"));
        assert!(!crate::validate_query("insert into x"));
    }

    #[test]
    fn test_validate_query_allows_plain_natural_language() {
        // 合法的自然语言取数（含中文）必须放行，否则功能不可用。
        assert!(crate::validate_query("show me monthly sales"));
        assert!(crate::validate_query("统计上个月订单数量"));
        assert!(crate::validate_query("  ")); // 空白串本身合法，trim 校验由调用方负责
    }
}
