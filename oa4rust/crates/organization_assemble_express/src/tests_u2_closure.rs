//! plan002 U2 收尾测试：98 条新增路由可达性 + 约定 helper 单测。
//!
//! mock_pool 无法建连：GET 命中 handler 后 500；POST 空 body 在 Json 提取层
//! 返回 4xx —— 断言"非 404"即证明注册（404 = 未注册）。Router 构建本身校验
//! 路径唯一性，重复注册会在构建时 panic。

#[cfg(test)]
mod closure_tests {
    use crate::endpoints::{
        bool_field, int_list, normalize_flags, string_field, wrap_bool, ID_COUNT_LIMIT,
    };
    use crate::router as express_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
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

    #[tokio::test]
    async fn closure_person_endpoints_registered() {
        for uri in [
            "/jaxrs/person/list/login/after",
            "/jaxrs/person/list/login/after/object",
            "/jaxrs/person/list/login/recent",
            "/jaxrs/person/list/login/recent/object",
            "/jaxrs/person/list/pair/identity",
            "/jaxrs/person/detail/p1",
            "/jaxrs/person/list/group/object",
            "/jaxrs/person/list/identity/object",
            "/jaxrs/person/list/personattribute",
            "/jaxrs/person/list/personattribute/object",
            "/jaxrs/person/list/person/sub/direct",
            "/jaxrs/person/list/person/sub/direct/object",
            "/jaxrs/person/list/person/sub/nested",
            "/jaxrs/person/list/person/sub/nested/object",
            "/jaxrs/person/list/person/sup/direct",
            "/jaxrs/person/list/person/sup/direct/object",
            "/jaxrs/person/list/person/sup/nested",
            "/jaxrs/person/list/person/sup/nested/object",
            "/jaxrs/person/list/role/object",
            "/jaxrs/person/list/unit/sub/direct",
            "/jaxrs/person/list/unit/sub/direct/object",
            "/jaxrs/person/list/unit/sub/nested",
            "/jaxrs/person/list/unit/sub/nested/object",
            "/jaxrs/person/list/unit/sub/direct/like",
            "/jaxrs/person/list/unit/sub/direct/like/object",
            "/jaxrs/person/list/unit/sub/nested/like",
            "/jaxrs/person/list/unit/sub/nested/like/object",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn closure_unit_endpoints_registered() {
        for uri in [
            "/jaxrs/unit/identity/level",
            "/jaxrs/unit/identity/level/object",
            "/jaxrs/unit/identity/type",
            "/jaxrs/unit/identity/type/object",
            "/jaxrs/unit/list/identity",
            "/jaxrs/unit/list/identity/object",
            "/jaxrs/unit/list/identity/sup/nested",
            "/jaxrs/unit/list/identity/sup/nested/object",
            "/jaxrs/unit/list/level",
            "/jaxrs/unit/list/level/object",
            "/jaxrs/unit/list/level/name/object",
            "/jaxrs/unit/list/person",
            "/jaxrs/unit/list/person/object",
            "/jaxrs/unit/list/person/sup/nested",
            "/jaxrs/unit/list/person/sup/nested/object",
            "/jaxrs/unit/list/unitattribute",
            "/jaxrs/unit/list/unitattribute/object",
            "/jaxrs/unit/list/unitduty",
            "/jaxrs/unit/list/unitduty/object",
            "/jaxrs/unit/list/unit/sub/direct/object",
            "/jaxrs/unit/list/unit/sub/nested/object",
            "/jaxrs/unit/list/unit/sup/direct/object",
            "/jaxrs/unit/list/unit/sup/nested/object",
            "/jaxrs/unit/list/unit/tree",
            "/jaxrs/unit/check/unit/has/identity",
            "/jaxrs/unit/check/unit/has/unit",
            "/jaxrs/unit/list/types",
            "/jaxrs/unit/list/types/object",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn closure_identity_endpoints_registered() {
        for uri in [
            "/jaxrs/identity/list/person/object",
            "/jaxrs/identity/list/unit/sub/direct/object",
            "/jaxrs/identity/list/unit/sub/nested/object",
            "/jaxrs/identity/list/unit/person",
            "/jaxrs/identity/list/unit/person/object",
            "/jaxrs/identity/list/group",
            "/jaxrs/identity/list/group/object",
            "/jaxrs/identity/list/major/person",
            "/jaxrs/identity/list/major/person/object",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn closure_group_role_duty_endpoints_registered() {
        for uri in [
            "/jaxrs/group/has/role",
            "/jaxrs/group/list/group/sub/direct",
            "/jaxrs/group/list/group/sub/direct/object",
            "/jaxrs/group/list/group/sub/nested",
            "/jaxrs/group/list/group/sub/nested/object",
            "/jaxrs/group/list/group/sup/direct",
            "/jaxrs/group/list/group/sup/direct/object",
            "/jaxrs/group/list/group/sup/nested",
            "/jaxrs/group/list/group/sup/nested/object",
            "/jaxrs/group/list/person/object",
            "/jaxrs/group/list/identity",
            "/jaxrs/group/list/identity/object",
            "/jaxrs/group/list/group/tree",
            "/jaxrs/role/list/object",
            "/jaxrs/role/list/person/object",
            "/jaxrs/unitduty/list/identity/unit/name",
            "/jaxrs/unitduty/list/identity/unit/name/object",
            "/jaxrs/unitduty/list/name/identity",
            "/jaxrs/unitduty/list/unit/object",
            "/jaxrs/unitduty/find/by/unit/name",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn closure_attr_misc_endpoints_registered() {
        for uri in [
            "/jaxrs/personattribute/list/name/person",
            "/jaxrs/personattribute/list/attribute/person/name",
            "/jaxrs/personattribute/list/person/object",
            "/jaxrs/personattribute/set/person/name",
            "/jaxrs/personattribute/append/person/name",
            "/jaxrs/unitattribute/list/name/unit",
            "/jaxrs/unitattribute/list/attribute/unit/name",
            "/jaxrs/unitattribute/list/unit/object",
            "/jaxrs/unitattribute/set/unit/name",
            "/jaxrs/unitattribute/append/unit/name",
            "/jaxrs/empower/list/identity/object",
            "/jaxrs/empowerlog",
            "/jaxrs/distinguishedname/list",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn closure_get_unit_type_endpoint_reachable() {
        // 唯一新增 GET 端点：命中路由后因无法建连返回 500（非 404）
        assert_eq!(
            status_of("GET", "/jaxrs/unit/list/type/dept/object").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn closure_router_builds_without_duplicate_paths() {
        let _ = express_router(mock_pool());
    }

    // ── 约定 helper 单测 ─────────────────────────────────────────────

    #[test]
    fn closure_normalize_flags_trims_and_dedupes() {
        let flags = vec![
            "  a ".to_string(),
            "a".to_string(),
            "".to_string(),
            "   ".to_string(),
            "b".to_string(),
        ];
        assert_eq!(normalize_flags(flags), vec!["a".to_string(), "b".to_string()]);
        assert!(normalize_flags(vec![]).is_empty());
    }

    #[test]
    fn closure_bool_field_defaults() {
        let body = serde_json::json!({"recursive": false});
        assert!(!bool_field(&body, "recursive", true));
        let body = serde_json::json!({});
        assert!(bool_field(&body, "recursive", true));
        assert!(!bool_field(&body, "recursive", false));
        // 非布尔值 → 默认值
        let body = serde_json::json!({"x": "yes"});
        assert!(bool_field(&body, "x", true));
    }

    #[test]
    fn closure_string_field_ignores_blank_and_non_string() {
        assert_eq!(
            string_field(&serde_json::json!({"name": " x "}), "name"),
            Some(" x ".to_string())
        );
        assert_eq!(string_field(&serde_json::json!({"name": "  "}), "name"), None);
        assert_eq!(string_field(&serde_json::json!({"name": 3}), "name"), None);
        assert_eq!(string_field(&serde_json::json!({}), "name"), None);
    }

    #[test]
    fn closure_int_list_parses_and_caps_level_list() {
        let body = serde_json::json!({"levelList": [1, 2, "x", null, 3]});
        assert_eq!(int_list(&body, "levelList").unwrap(), vec![1, 2, 3]);
        let over: Vec<i64> = (0..=ID_COUNT_LIMIT as i64).collect();
        let body = serde_json::json!({"levelList": over});
        assert!(int_list(&body, "levelList").is_err(), ">100 levels must be rejected");
    }

    #[test]
    fn closure_wrap_boolean_contract_matches_java() {
        // Java WrapBoolean 序列化为 {"value": bool}
        let v = wrap_bool(true);
        assert_eq!(v, serde_json::json!({"value": true}));
        assert_eq!(wrap_bool(false), serde_json::json!({"value": false}));
    }

    #[test]
    fn closure_batch_cap_boundary_still_enforced_for_new_keys() {
        let ids: Vec<String> = (0..=ID_COUNT_LIMIT).map(|i| format!("p{i}")).collect();
        assert!(crate::endpoints::capped(&ids).is_err());
        assert!(crate::endpoints::capped(&ids[..ID_COUNT_LIMIT]).is_ok());
    }

    #[test]
    fn closure_person_cols_pii_gating() {
        // PII 红线：默认列集不含 mobile/email；显式 includePii 才携带
        let plain = crate::endpoints::person_cols(false);
        let pii = crate::endpoints::person_cols(true);
        assert!(!plain.contains(&"mobile") && !plain.contains(&"email"));
        assert!(pii.contains(&"mobile") && pii.contains(&"email"));
    }

    #[test]
    fn closure_merged_flags_merges_single_and_list() {
        // Java Wi 兼容：{name, nameList} / {unit, unitList} 单值与数组合并去重
        let body = serde_json::json!({"unit": "u1", "unitList": ["u1", " u2 "], "name": "n1"});
        assert_eq!(
            crate::endpoints_duty2::merged_flags(&body, "unit", "unitList"),
            vec!["u1".to_string(), "u2".to_string()]
        );
        assert_eq!(
            crate::endpoints_duty2::merged_flags(&body, "name", "nameList"),
            vec!["n1".to_string()]
        );
    }

    #[test]
    fn closure_named_list_shape_contract() {
        // {key: [..]} 形状契约（identityList/groupList/roleList/nameList 等共用）
        let v = crate::endpoints::named_list("identityList", &["i1".to_string()]);
        assert_eq!(v, serde_json::json!({"identityList": ["i1"]}));
    }
}
