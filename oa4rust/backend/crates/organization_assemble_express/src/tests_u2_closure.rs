// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
            "/api/person/list/login/after",
            "/api/person/list/login/after/object",
            "/api/person/list/login/recent",
            "/api/person/list/login/recent/object",
            "/api/person/list/pair/identity",
            "/api/person/detail/p1",
            "/api/person/list/group/object",
            "/api/person/list/identity/object",
            "/api/person/list/personattribute",
            "/api/person/list/personattribute/object",
            "/api/person/list/person/sub/direct",
            "/api/person/list/person/sub/direct/object",
            "/api/person/list/person/sub/nested",
            "/api/person/list/person/sub/nested/object",
            "/api/person/list/person/sup/direct",
            "/api/person/list/person/sup/direct/object",
            "/api/person/list/person/sup/nested",
            "/api/person/list/person/sup/nested/object",
            "/api/person/list/role/object",
            "/api/person/list/unit/sub/direct",
            "/api/person/list/unit/sub/direct/object",
            "/api/person/list/unit/sub/nested",
            "/api/person/list/unit/sub/nested/object",
            "/api/person/list/unit/sub/direct/like",
            "/api/person/list/unit/sub/direct/like/object",
            "/api/person/list/unit/sub/nested/like",
            "/api/person/list/unit/sub/nested/like/object",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn closure_unit_endpoints_registered() {
        for uri in [
            "/api/unit/identity/level",
            "/api/unit/identity/level/object",
            "/api/unit/identity/type",
            "/api/unit/identity/type/object",
            "/api/unit/list/identity",
            "/api/unit/list/identity/object",
            "/api/unit/list/identity/sup/nested",
            "/api/unit/list/identity/sup/nested/object",
            "/api/unit/list/level",
            "/api/unit/list/level/object",
            "/api/unit/list/level/name/object",
            "/api/unit/list/person",
            "/api/unit/list/person/object",
            "/api/unit/list/person/sup/nested",
            "/api/unit/list/person/sup/nested/object",
            "/api/unit/list/unitattribute",
            "/api/unit/list/unitattribute/object",
            "/api/unit/list/unitduty",
            "/api/unit/list/unitduty/object",
            "/api/unit/list/unit/sub/direct/object",
            "/api/unit/list/unit/sub/nested/object",
            "/api/unit/list/unit/sup/direct/object",
            "/api/unit/list/unit/sup/nested/object",
            "/api/unit/list/unit/tree",
            "/api/unit/check/unit/has/identity",
            "/api/unit/check/unit/has/unit",
            "/api/unit/list/types",
            "/api/unit/list/types/object",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn closure_identity_endpoints_registered() {
        for uri in [
            "/api/identity/list/person/object",
            "/api/identity/list/unit/sub/direct/object",
            "/api/identity/list/unit/sub/nested/object",
            "/api/identity/list/unit/person",
            "/api/identity/list/unit/person/object",
            "/api/identity/list/group",
            "/api/identity/list/group/object",
            "/api/identity/list/major/person",
            "/api/identity/list/major/person/object",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn closure_group_role_duty_endpoints_registered() {
        for uri in [
            "/api/group/has/role",
            "/api/group/list/group/sub/direct",
            "/api/group/list/group/sub/direct/object",
            "/api/group/list/group/sub/nested",
            "/api/group/list/group/sub/nested/object",
            "/api/group/list/group/sup/direct",
            "/api/group/list/group/sup/direct/object",
            "/api/group/list/group/sup/nested",
            "/api/group/list/group/sup/nested/object",
            "/api/group/list/person/object",
            "/api/group/list/identity",
            "/api/group/list/identity/object",
            "/api/group/list/group/tree",
            "/api/role/list/object",
            "/api/role/list/person/object",
            "/api/unitduty/list/identity/unit/name",
            "/api/unitduty/list/identity/unit/name/object",
            "/api/unitduty/list/name/identity",
            "/api/unitduty/list/unit/object",
            "/api/unitduty/find/by/unit/name",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn closure_attr_misc_endpoints_registered() {
        for uri in [
            "/api/personattribute/list/name/person",
            "/api/personattribute/list/attribute/person/name",
            "/api/personattribute/list/person/object",
            "/api/personattribute/set/person/name",
            "/api/personattribute/append/person/name",
            "/api/unitattribute/list/name/unit",
            "/api/unitattribute/list/attribute/unit/name",
            "/api/unitattribute/list/unit/object",
            "/api/unitattribute/set/unit/name",
            "/api/unitattribute/append/unit/name",
            "/api/empower/list/identity/object",
            "/api/empowerlog",
            "/api/distinguishedname/list",
        ] {
            assert_ne!(
                status_of("POST", uri).await,
                StatusCode::NOT_FOUND,
                "POST {uri}"
            );
        }
    }

    #[tokio::test]
    async fn closure_get_unit_type_endpoint_reachable() {
        // 唯一新增 GET 端点：命中路由后因无法建连返回 500（非 404）
        assert_eq!(
            status_of("GET", "/api/unit/list/type/dept/object").await,
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
        assert_eq!(
            normalize_flags(flags),
            vec!["a".to_string(), "b".to_string()]
        );
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
        assert_eq!(
            string_field(&serde_json::json!({"name": "  "}), "name"),
            None
        );
        assert_eq!(string_field(&serde_json::json!({"name": 3}), "name"), None);
        assert_eq!(string_field(&serde_json::json!({}), "name"), None);
    }

    #[test]
    fn closure_int_list_parses_and_caps_level_list() {
        let body = serde_json::json!({"levelList": [1, 2, "x", null, 3]});
        assert_eq!(int_list(&body, "levelList").unwrap(), vec![1, 2, 3]);
        let over: Vec<i64> = (0..=ID_COUNT_LIMIT as i64).collect();
        let body = serde_json::json!({"levelList": over});
        assert!(
            int_list(&body, "levelList").is_err(),
            ">100 levels must be rejected"
        );
    }

    #[test]
    fn closure_wrap_boolean_contract_matches_legacy() {
        // o2server WrapBoolean 序列化为 {"value": bool}
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
        // o2server Wi 兼容：{name, nameList} / {unit, unitList} 单值与数组合并去重
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
