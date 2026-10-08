// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! plan002 U2 新增端点测试。
//!
//! 路由可达性使用 mock_pool（无法建连）：GET 命中路由后 handler 返回 500；
//! POST 空 body 在 Json 提取层返回 400/415/422 —— 断言"非 404"即可证明
//! 路由已注册（404 = 未注册，405 不可能出现因为动词正确）。
//! Router 构建本身校验路径唯一性，重复注册会在构建时直接 panic。

#[cfg(test)]
mod u2_tests {
    use crate::endpoints::{capped, include_pii, string_list, ID_COUNT_LIMIT};
    use crate::router as express_router;
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
            status_of("GET", "/api/person/auth/info/p1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/api/person/nick/name/p1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/api/person/mobile/p1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/api/person/list/all").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/api/person/list/all/object").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_person_post_endpoints_registered() {
        for uri in [
            "/api/person/list",
            "/api/person/list/object",
            "/api/person/has/role",
            "/api/person/list/identity",
            "/api/person/list/group",
            "/api/person/list/role",
            "/api/person/list/filter/1/size/20",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn u2_identity_endpoints_registered() {
        for uri in [
            "/api/identity/list",
            "/api/identity/list/object",
            "/api/identity/list/person",
            "/api/identity/list/unit/sub/direct",
            "/api/identity/list/unit/sub/nested",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn u2_unit_get_and_post_endpoints_registered() {
        assert_eq!(
            status_of("GET", "/api/unit/list/all").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/api/unit/list/all/object").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        for uri in [
            "/api/unit/list",
            "/api/unit/list/object",
            "/api/unit/list/unit/sub/direct",
            "/api/unit/list/unit/sub/nested",
            "/api/unit/list/unit/sup/direct",
            "/api/unit/list/unit/sup/nested",
            "/api/unit/check/unit/has/person",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn u2_group_role_unitduty_endpoints_registered() {
        for uri in [
            "/api/group/list",
            "/api/group/list/object",
            "/api/group/list/person",
            "/api/role/list",
            "/api/role/list/person",
            "/api/unitduty/list/name",
            "/api/unitduty/list/name/unit",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn u2_unregistered_legacy_path_still_missing() {
        // 未实现的路径必须仍是 404；person/detail 已注册为 POST，
        // 其 GET 变体返回 405（方法不匹配）而非 404
        assert_eq!(
            status_of("GET", "/api/person/detail/p1").await,
            StatusCode::METHOD_NOT_ALLOWED
        );
        assert_eq!(
            status_of("POST", "/api/person/no/such/legacy/action").await,
            StatusCode::NOT_FOUND
        );
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
        assert_eq!(
            string_list(&body, "personList"),
            vec!["a".to_string(), "b".to_string()]
        );
        assert!(string_list(&body, "missing").is_empty());
        assert!(string_list(&serde_json::json!({"x": "not-array"}), "x").is_empty());
    }

    #[test]
    fn u2_action_result_success_contract() {
        let result: ActionResult<Value> = ActionResult::success(serde_json::json!({"count": 0}));
        assert_eq!(result.r#type.as_deref(), Some("success"));
        assert!(result.data.is_some());
    }

    use serde_json::Value;
}
