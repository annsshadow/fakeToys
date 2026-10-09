// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{u2_helpers, u2_router};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tower::ServiceExt;

const BASE: &str = "/api/organization/assemble/control";

fn u2_session() -> shared::session::Session {
    shared::session::Session {
        token: "u2-org-test-token".to_string(),
        person_unique: "tester@u2@P".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
    }
}

fn build_test_pool() -> deadpool_postgres::Pool {
    deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .build()
    .unwrap()
}

fn app() -> axum::Router {
    crate::router(build_test_pool())
}

async fn request(method: Method, uri: &str, body: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let req = builder
        .body(Body::from(body.unwrap_or_default().to_string()))
        .unwrap();
    app().oneshot(req).await.unwrap().status()
}

#[test]
fn normalize_key_collapses_whitespace() {
    assert_eq!(u2_helpers::normalize_key("  Zhang   San  "), "Zhang San");
    assert_eq!(u2_helpers::normalize_key("\tA\tB\n"), "A B");
    assert_eq!(u2_helpers::normalize_key("   "), "");
}

#[test]
fn batch_limit_allows_100_rejects_101() {
    assert!(u2_helpers::check_batch_len(100).is_ok());
    let err = u2_helpers::check_batch_len(101).unwrap_err();
    match err {
        shared::error::AppError::BadRequest(msg) => {
            assert!(msg.contains("exceeds limit"), "msg={msg}")
        }
        other => panic!("expected BadRequest, got {other:?}"),
    }
}

#[test]
fn password_policy_matches_legacy_defaults() {
    assert!(!u2_helpers::validate_password_policy("Abc12"));
    assert!(u2_helpers::validate_password_policy("Abc123"));
    assert!(!u2_helpers::validate_password_policy("123456"));
    assert!(!u2_helpers::validate_password_policy("abcdef"));
    assert!(!u2_helpers::validate_password_policy("Abc 123"));
    assert!(!u2_helpers::validate_password_policy(&"A".repeat(65)));
    assert!(u2_helpers::validate_password_policy(&"a1".repeat(32)));
}

#[test]
fn date_parsing_accepts_iso_and_rejects_garbage() {
    assert!(u2_helpers::is_parseable_date("2026-08-23"));
    assert!(u2_helpers::is_parseable_date("2026-08-23T10:00:00Z"));
    assert!(u2_helpers::is_parseable_date("2026-08-23 10:00:00"));
    assert!(!u2_helpers::is_parseable_date("23/08/2026"));
    assert!(!u2_helpers::is_parseable_date(""));
}

#[test]
fn camel_case_converts_snake_columns() {
    assert_eq!(u2_helpers::camel_case("parent_id"), "parentId");
    assert_eq!(u2_helpers::camel_case("id"), "id");
    assert_eq!(
        u2_helpers::camel_case("lock_expired_time"),
        "lockExpiredTime"
    );
}

#[tokio::test]
async fn idor_gate_fail_closed_without_db_session() {
    let pool = build_test_pool();
    let session = shared::session::Session {
        token: "t".to_string(),
        person_unique: "nobody@x".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        expires_at: chrono::Utc::now().naive_utc(),
    };
    let result = u2_helpers::require_admin(&pool, &session).await;
    match result {
        Err(shared::error::AppError::Forbidden) => {}
        other => panic!("expected Forbidden (fail-closed), got {other:?}"),
    }
}

#[test]
fn merged_router_builds_without_conflicts() {
    let _ = u2_router::router();
    let _ = crate::router(build_test_pool());
}

#[tokio::test]
async fn person_get_flag_route_registered() {
    assert_eq!(
        request(Method::GET, &format!("{BASE}/person/zhangsan"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_create_route_registered() {
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/person"),
            Some(r#"{"name":"zhangsan"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_edit_and_delete_routes_use_legacy_methods() {
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/person/p1"),
            Some(r#"{"mobile":"138"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::DELETE, &format!("{BASE}/person/p1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_wrong_method_rejected_with_405() {
    assert_eq!(
        request(Method::PATCH, &format!("{BASE}/person/p1"), Some("{}")).await,
        StatusCode::METHOD_NOT_ALLOWED
    );
    assert_eq!(
        request(
            Method::DELETE,
            &format!("{BASE}/person/check/password"),
            None
        )
        .await,
        StatusCode::METHOD_NOT_ALLOWED
    );
}

#[tokio::test]
async fn person_mock_aliases_registered() {
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/person/p1/mockputtopost"),
            Some("{}")
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/person/p1/mockdeletetoget"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_check_password_success_contract() {
    let resp = app()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("{BASE}/person/check/password"))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"password": "Abc123"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["value"], true);
}

#[tokio::test]
async fn person_check_password_weak_returns_false() {
    let resp = app()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("{BASE}/person/check/password"))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"password": "abc"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["data"]["value"], false);
}

#[tokio::test]
async fn person_status_and_icon_routes_registered() {
    assert_eq!(
        request(Method::POST, &format!("{BASE}/person/lock/p1"), Some("{}")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/person/unlock/p1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/person/ban/p1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/person/unban/p1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/person/p1/icon"),
            Some(r#"{"icon":"data:image/png;base64,x"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/person/p1/icon"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/person/p1/set/password/expired/time/2026-08-23"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_list_filter_and_delete_paging_registered() {
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/person/list/filter/1/size/20"),
            Some(r#"{"name":"san"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/person/list/delete/1/size/20"),
            Some("{}")
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_group_role_listing_routes_registered() {
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/person/list/group/g1/sub/direct"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/person/list/group/g1/sub/nested"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/person/list/role/r1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/person/list/0/next/20"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/person/list/0/prev/20"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_search_put_semantics_registered() {
    for path in [
        "/person/list/pinyininitial",
        "/person/list/like",
        "/person/list/like/pinyin",
    ] {
        assert_eq!(
            request(
                Method::PUT,
                &format!("{BASE}{path}"),
                Some(r#"{"key":"z"}"#)
            )
            .await,
            StatusCode::INTERNAL_SERVER_ERROR,
            "path={path}"
        );
        assert_eq!(
            request(Method::GET, &format!("{BASE}{path}"), None).await,
            StatusCode::METHOD_NOT_ALLOWED,
            "GET must be rejected as wrong method for {path}"
        );
    }
}

#[tokio::test]
async fn unit_crud_and_hierarchy_routes_registered() {
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/unit"),
            Some(r#"{"name":"hq"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/unit/u1"),
            Some(r#"{"name":"renamed"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::DELETE, &format!("{BASE}/unit/u1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/unit/get/root"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/unit/u1/sup/direct"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/unit/identity/i1/level/2"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/unit/identity/i1/type/company"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/unit/list/unit/type"),
            Some(r#"{"type":"company","unitList":["u1"]}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    // mockputtopost twin shares the (now single-query) unit_list_with_unit_type
    // handler; assert the POST verb is wired to the same collapsed path.
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/unit/list/unit/type/mockputtopost"),
            Some(r#"{"type":"company","unitList":["u1"]}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/unit/list"), Some("{}")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/unit/list/controller"),
            Some("{}")
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    for path in [
        "/unit/list/top",
        "/unit/list/control/top",
        "/unit/list/type",
    ] {
        assert_eq!(
            request(Method::GET, &format!("{BASE}{path}"), None).await,
            StatusCode::INTERNAL_SERVER_ERROR,
            "path={path}"
        );
    }
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/unit/list/top/type/company"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/unit/list/u1/sub/direct"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::GET,
            &format!("{BASE}/unit/list/u1/sub/direct/type/company"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/unit/list/u1/prev/10"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn identity_crud_routes_registered() {
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/identity"),
            Some(r#"{"name":"main","unitId":"u1"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/identity/i1"),
            Some(r#"{"name":"new"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::DELETE, &format!("{BASE}/identity/i1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/identity/i1/mockputtopost"),
            Some("{}")
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn group_member_management_uses_put_semantics() {
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/group"),
            Some(r#"{"name":"team"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/group/g1/add/member"),
            Some(r#"{"personList":["p1"]}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/group/g1/add/member/mockputtopost"),
            Some(r#"{"personList":["p1"]}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/group/g1/delete/member"),
            Some(r#"{"personList":["p1"]}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::DELETE, &format!("{BASE}/group/g1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/group/g1"),
            Some(r#"{"name":"t2"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn role_duty_permission_attribute_card_input_routes_registered() {
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/role"),
            Some(r#"{"name":"manager"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::DELETE, &format!("{BASE}/role/r1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/unitduty/update/member"),
            Some(r#"{"unit":"u1","unitDuty":"lead","identityList":["i1"]}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/unitduty"),
            Some(r#"{"name":"lead"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::PUT, &format!("{BASE}/unitduty/d1"), Some("{}")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/permissionsetting"),
            Some(r#"{"name":"ps"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::DELETE,
            &format!("{BASE}/permissionsetting/ps1"),
            None
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/unitattribute"),
            Some(r#"{"unitId":"u1","attributeKey":"k"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/personattribute"),
            Some(r#"{"personId":"p1","attributeKey":"k"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/personcard"),
            Some(r#"{"name":"card"}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/personcard/listpaging/page/1/size/20"),
            Some("{}")
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/personcard/listpagingwithgroup/page/1/size/20/mockputtopost"),
            Some("{}")
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/inputperson"),
            Some(r#"{"personList":[{"name":"lisi"}]}"#)
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

// ═══ KNOWN_BACKEND_GAPS 最后两条（/api/users/list、/api/departments/tree）实装回归 ═══
// 设计器数据源示例默认值所指端点；真 DB 往返（mock_pool 无表，无 DB 时跳过）。

#[tokio::test]
async fn users_list_returns_seeded_person() {
    if !shared::testing::is_db_available().await {
        return;
    }
    let pool = shared::testing::test_pool();
    let client = pool.get().await.unwrap();
    client
        .execute(
            "INSERT INTO x_org_person (id, name, mobile, email) VALUES ($1, $2, $3, $4)",
            &[
                &"u2-users-list-p1",
                &"张三",
                &"13800000000",
                &"zhang@u2.test",
            ],
        )
        .await
        .unwrap();
    let response = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/users/list")
                .extension(u2_session())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "success");
    let items = json["data"].as_array().expect("data is array");
    assert!(
        items
            .iter()
            .any(|it| it["id"] == "u2-users-list-p1" && it["name"] == "张三"),
        "seeded person must be listed"
    );
    client
        .execute(
            "DELETE FROM x_org_person WHERE id = $1",
            &[&"u2-users-list-p1"],
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn departments_tree_nests_child_under_parent() {
    if !shared::testing::is_db_available().await {
        return;
    }
    let pool = shared::testing::test_pool();
    let client = pool.get().await.unwrap();
    client
        .execute(
            "INSERT INTO x_org_unit (id, name, parent_id, level) VALUES ($1, $2, NULL, 0)",
            &[&"u2-unit-root", &"总公司"],
        )
        .await
        .unwrap();
    client
        .execute(
            "INSERT INTO x_org_unit (id, name, parent_id, level) VALUES ($1, $2, $3, 1)",
            &[&"u2-unit-child", &"研发部", &"u2-unit-root"],
        )
        .await
        .unwrap();
    let response = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/departments/tree")
                .extension(u2_session())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let roots = json["data"].as_array().expect("data is tree array");
    let root = roots
        .iter()
        .find(|n| n["id"] == "u2-unit-root")
        .expect("root unit present");
    let children = root["children"].as_array().expect("children array");
    assert!(
        children.iter().any(|c| c["id"] == "u2-unit-child"),
        "child unit must nest under its parent"
    );
    client
        .execute(
            "DELETE FROM x_org_unit WHERE id IN ($1, $2)",
            &[&"u2-unit-root", &"u2-unit-child"],
        )
        .await
        .unwrap();
}

// unit_list_with_unit_type N+1→single-query collapse (优化二轮 20): the batch
// path now filters `type=$1 AND (id=ANY($2) OR name=ANY($2))` in one query
// instead of resolving each flag id first. Verifies id-match, name-match, the
// type filter, and not-in-flags exclusion all hold against a live DB.
#[tokio::test]
async fn unit_list_with_unit_type_matches_by_id_and_name_respecting_type() {
    if !shared::testing::is_db_available().await {
        return;
    }
    let pool = shared::testing::test_pool();
    let client = pool.get().await.unwrap();
    // a: company matched by id; b: company matched by name; c: company NOT in
    // flags (must be excluded); d: in flags by id but wrong type (excluded).
    for (id, name, ty) in [
        ("u2-ult-a", "甲公司", "company"),
        ("u2-ult-b", "乙公司", "company"),
        ("u2-ult-c", "丙公司", "company"),
        ("u2-ult-d", "丁部门", "department"),
    ] {
        client
            .execute(
                "INSERT INTO x_org_unit (id, name, type, parent_id, level) VALUES ($1, $2, $3, NULL, 1)",
                &[&id, &name, &ty],
            )
            .await
            .unwrap();
    }
    let body = r#"{"type":"company","unitList":["u2-ult-a","乙公司","u2-ult-d"]}"#;
    let response = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/organization/assemble/control/unit/list/unit/type")
                .header("content-type", "application/json")
                .extension(u2_session())
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let items = json["data"].as_array().expect("data is array");
    let ids: Vec<&str> = items.iter().filter_map(|it| it["id"].as_str()).collect();
    assert!(
        ids.contains(&"u2-ult-a"),
        "id match must be listed: {ids:?}"
    );
    assert!(
        ids.contains(&"u2-ult-b"),
        "name match must be listed: {ids:?}"
    );
    assert!(
        !ids.contains(&"u2-ult-c"),
        "company not in flags must be excluded: {ids:?}"
    );
    assert!(
        !ids.contains(&"u2-ult-d"),
        "wrong-type flag must be excluded: {ids:?}"
    );
    client
        .execute(
            "DELETE FROM x_org_unit WHERE id IN ($1, $2, $3, $4)",
            &[&"u2-ult-a", &"u2-ult-b", &"u2-ult-c", &"u2-ult-d"],
        )
        .await
        .unwrap();
}
