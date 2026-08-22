//! plan002 U2 新增端点测试。
//!
//! 路由可达性使用 mock_pool（无法建连）：GET 命中路由后 handler 返回 500；
//! POST 空 body 在 Json 提取层返回 400/415/422 —— 断言"非 404"即可证明
//! 路由已注册（404 = 未注册，405 不可能出现因为动词正确）。
//! Router 构建本身校验路径唯一性，重复注册会在构建时直接 panic。

#[cfg(test)]
mod u2_tests {
    use crate::router as express_router;
    use crate::endpoints::{capped, include_pii, string_list, ID_COUNT_LIMIT};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use shared::testing::mock_pool;
    use tower::util::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        let app = express_router(mock_pool());
        app.oneshot(
            Request::builder()
                .uri(uri)
                .method(method)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    // ── 路由可达性：person ────────────────────────────────────────────

    #[tokio::test]
    async fn u2_person_get_endpoints_reachable() {
        assert_eq!(
            status_of("GET", "/jaxrs/person/auth/info/p1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/person/nick/name/p1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/person/mobile/p1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/person/list/all").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/person/list/all/object").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_person_post_endpoints_registered() {
        for uri in [
            "/jaxrs/person/list",
            "/jaxrs/person/list/object",
            "/jaxrs/person/has/role",
            "/jaxrs/person/list/identity",
            "/jaxrs/person/list/group",
            "/jaxrs/person/list/role",
            "/jaxrs/person/list/filter/1/size/20",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn u2_identity_endpoints_registered() {
        for uri in [
            "/jaxrs/identity/list",
            "/jaxrs/identity/list/object",
            "/jaxrs/identity/list/person",
            "/jaxrs/identity/list/unit/sub/direct",
            "/jaxrs/identity/list/unit/sub/nested",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn u2_unit_get_and_post_endpoints_registered() {
        assert_eq!(
            status_of("GET", "/jaxrs/unit/list/all").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/unit/list/all/object").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        for uri in [
            "/jaxrs/unit/list",
            "/jaxrs/unit/list/object",
            "/jaxrs/unit/list/unit/sub/direct",
            "/jaxrs/unit/list/unit/sub/nested",
            "/jaxrs/unit/list/unit/sup/direct",
            "/jaxrs/unit/list/unit/sup/nested",
            "/jaxrs/unit/check/unit/has/person",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn u2_group_role_unitduty_endpoints_registered() {
        for uri in [
            "/jaxrs/group/list",
            "/jaxrs/group/list/object",
            "/jaxrs/group/list/person",
            "/jaxrs/role/list",
            "/jaxrs/role/list/person",
            "/jaxrs/unitduty/list/name",
            "/jaxrs/unitduty/list/name/unit",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn u2_unregistered_java_path_still_missing() {
        // 未实现的 Java 端点必须仍是 404，防止误报对齐
        assert_eq!(status_of("GET", "/jaxrs/person/detail/p1").await, StatusCode::NOT_FOUND);
        assert_eq!(status_of("GET", "/jaxrs/unit/list/type/t1/object").await, StatusCode::NOT_FOUND);
    }

    #[test]
    fn u2_router_builds_without_duplicate_paths() {
        let _ = express_router(mock_pool());
    }

    // ── 约定单元测试：批量上限 / PII 显式参数 / Wi 解析 ────────────────

    #[test]
    fn u2_batch_cap_rejects_over_100_ids() {
        let ids: Vec<String> = (0..=ID_COUNT_LIMIT).map(|i| i.to_string()).collect();
        assert_eq!(ids.len(), ID_COUNT_LIMIT + 1);
        assert!(capped(&ids).is_err(), ">100 IDs must be rejected");
    }

    #[test]
    fn u2_batch_cap_accepts_up_to_100_ids() {
        let ids: Vec<String> = (0..99).map(|i| i.to_string()).collect();
        assert!(capped(&ids).is_ok());
        assert!(capped(&[]).is_ok());
    }

    #[test]
    fn u2_pii_disabled_by_default_and_explicitly_enabled() {
        let body = serde_json::json!({"personList": ["a"]});
        assert!(!include_pii(&body), "PII must be excluded by default");
        let body = serde_json::json!({"includePii": true});
        assert!(include_pii(&body));
        let body = serde_json::json!({"includePii": false});
        assert!(!include_pii(&body));
    }

    #[test]
    fn u2_string_list_parses_wi_contract_and_ignores_non_strings() {
        let body = serde_json::json!({"personList": ["a", 1, null, true, "b"]});
        assert_eq!(string_list(&body, "personList"), vec!["a".to_string(), "b".to_string()]);
        assert!(string_list(&body, "missing").is_empty());
        assert!(string_list(&serde_json::json!({"x": "not-array"}), "x").is_empty());
    }

    #[test]
    fn u2_action_result_success_contract() {
        let result: ActionResult<Value> =
            ActionResult::success(serde_json::json!({"count": 0}));
        assert_eq!(result.r#type.as_deref(), Some("success"));
        assert!(result.data.is_some());
    }


    use serde_json::Value;
}
