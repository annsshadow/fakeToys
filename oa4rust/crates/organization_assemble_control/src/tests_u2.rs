use super::{u2_helpers, u2_router};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tower::ServiceExt;

const BASE: &str = "/jaxrs/organization/assemble/control";

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
fn password_policy_matches_java_defaults() {
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
    assert_eq!(u2_helpers::camel_case("lock_expired_time"), "lockExpiredTime");
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
        request(Method::POST, &format!("{BASE}/person"), Some(r#"{"name":"zhangsan"}"#)).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_edit_and_delete_routes_use_java_methods() {
    assert_eq!(
        request(Method::PUT, &format!("{BASE}/person/p1"), Some(r#"{"mobile":"138"}"#)).await,
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
        request(Method::DELETE, &format!("{BASE}/person/check/password/x"), None).await,
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
        request(Method::GET, &format!("{BASE}/person/p1/mockdeletetoget"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn person_check_password_success_contract() {
    let resp = app()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("{BASE}/person/check/password/Abc123"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["value"], true);
}

#[tokio::test]
async fn person_check_password_weak_returns_false() {
    let resp = app()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("{BASE}/person/check/password/abc"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
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
        request(Method::GET, &format!("{BASE}/person/list/group/g1/sub/direct"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/person/list/group/g1/sub/nested"), None).await,
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
            request(Method::PUT, &format!("{BASE}{path}"), Some(r#"{"key":"z"}"#)).await,
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
        request(Method::POST, &format!("{BASE}/unit"), Some(r#"{"name":"hq"}"#)).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::PUT, &format!("{BASE}/unit/u1"), Some(r#"{"name":"renamed"}"#)).await,
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
        request(Method::GET, &format!("{BASE}/unit/identity/i1/level/2"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/unit/identity/i1/type/company"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/unit/list/unit/type"),
            Some(r#"{"type":"company","unitList":["u1"]}"#
        )
        )
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/unit/list"), Some("{}")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/unit/list/controller"), Some("{}")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    for path in ["/unit/list/top", "/unit/list/control/top", "/unit/list/type"] {
        assert_eq!(
            request(Method::GET, &format!("{BASE}{path}"), None).await,
            StatusCode::INTERNAL_SERVER_ERROR,
            "path={path}"
        );
    }
    assert_eq!(
        request(Method::GET, &format!("{BASE}/unit/list/top/type/company"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/unit/list/u1/sub/direct"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::GET, &format!("{BASE}/unit/list/u1/sub/direct/type/company"), None).await,
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
        request(Method::PUT, &format!("{BASE}/identity/i1"), Some(r#"{"name":"new"}"#)).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::DELETE, &format!("{BASE}/identity/i1"), None).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/identity/i1/mockputtopost"), Some("{}")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn group_member_management_uses_put_semantics() {
    assert_eq!(
        request(Method::POST, &format!("{BASE}/group"), Some(r#"{"name":"team"}"#)).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/group/g1/add/member"),
            Some(r#"{"personList":["p1"]}"#
        ))
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::POST,
            &format!("{BASE}/group/g1/add/member/mockputtopost"),
            Some(r#"{"personList":["p1"]}"#
        ))
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(
            Method::PUT,
            &format!("{BASE}/group/g1/delete/member"),
            Some(r#"{"personList":["p1"]}"#
        ))
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
        request(Method::POST, &format!("{BASE}/role"), Some(r#"{"name":"manager"}"#)).await,
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
            Some(r#"{"unit":"u1","unitDuty":"lead","identityList":["i1"]}"#
        ))
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/unitduty"), Some(r#"{"name":"lead"}"#)).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::PUT, &format!("{BASE}/unitduty/d1"), Some("{}")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::POST, &format!("{BASE}/permissionsetting"), Some(r#"{"name":"ps"}"#)).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        request(Method::DELETE, &format!("{BASE}/permissionsetting/ps1"), None).await,
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
        request(Method::POST, &format!("{BASE}/personcard"), Some(r#"{"name":"card"}"#)).await,
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
            Some(r#"{"personList":[{"name":"lisi"}]}"#
        ))
        .await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}
