use super::*;
use axum::body::Body;
use axum::http::{Request, Method, StatusCode};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use serde_json::json;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        Config::new(),
        NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

#[test]
fn test_create_designer_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "id": "designer-1",
        "name": "My Designer",
        "category": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
}

#[test]
fn test_get_designer_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "designer-1",
        "name": "Query Designer",
        "category": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "designer-1");
}

#[test]
fn test_list_designers_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "designer-1", "category": "default"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_save_designer_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "designer-1",
        "saved": true,
        "updated_at": "2024-01-01T00:00:00Z"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["saved"], true);
}

#[tokio::test]
async fn test_create_designer_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "name": "My Designer",
        "query": "select * from test"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_designer_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/get/designer-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_designers_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/list/default")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_save_designer_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "name": "My Designer",
        "query": "select * from test"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/save/designer-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_delete_designer_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/delete/designer-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}


// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 缺口闭合测试：新注册路由存在性 + sqlparser 安全约束 + 参数化 + 归一化查重
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_u2_designer_search_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/search")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_input_compare_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/input/compare")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_neural_model_create_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/neural/model")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_output_list_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/output/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_query_list_all_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/list/all")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_table_list_manage_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/table/list/manage")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_view_get_by_id_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/view/view-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_statement_get_flag_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/statement/st-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_statement_execute_v2_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/statement/execute/st-1/page/1/size/20")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_statement_execute_mode_v2_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/designer/statement/execute/st-1/mode/count/page/1/size/20")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ── sqlparser 安全约束（无 DB 依赖的纯单测）──

#[test]
fn test_u2_validate_single_select_rejects_delete() {
    let err = u2_closures::validate_single_select("DELETE FROM x_query_table_data").unwrap_err();
    assert!(err.contains("only SELECT"), "DELETE must be rejected, got: {}", err);
}

#[test]
fn test_u2_validate_single_select_rejects_multi_statement() {
    let err = u2_closures::validate_single_select(
        "SELECT 1; DROP TABLE x_query_statement",
    )
    .unwrap_err();
    assert!(err.contains("single statement"), "multi-statement must be rejected");
}

#[test]
fn test_u2_validate_single_select_rejects_empty() {
    assert!(u2_closures::validate_single_select("   ").is_err());
}

#[test]
fn test_u2_validate_single_select_accepts_select() {
    // Java statement.data 为 JPQL 风格，Rust 侧存储可直接执行的 SQL；
    // 此处验证合法 SELECT 通过安全校验。
    assert!(u2_closures::validate_single_select("SELECT id, name FROM x_query_table WHERE table_flag = 't1'").is_ok());
}

#[test]
fn test_u2_parameterize_statement_sql_binds_named_params() {
    let params = json!({"person": "张三@unit", "minAge": 18});
    let (sql, values) =
        u2_closures::parameterize_statement_sql("SELECT * FROM t WHERE o.name = :person AND o.age > :minAge", &params);
    assert!(sql.contains("$1"), "named param should become $1: {}", sql);
    assert!(sql.contains("$2"), "named param should become $2: {}", sql);
    assert_eq!(values.len(), 2);
}

#[test]
fn test_u2_parameterize_statement_sql_skips_string_literals_and_casts() {
    let params = json!({"name": "x"});
    let (sql, values) = u2_closures::parameterize_statement_sql(
        "SELECT a::text FROM t WHERE s = 'lit:eral' AND n = :name",
        &params,
    );
    assert!(!sql.contains("$1::"), "cast :: must not be treated as param");
    assert!(sql.contains("'lit:eral'"), "string literal colon untouched");
    assert_eq!(values.len(), 1, "only the real named param is bound");
}

#[test]
fn test_u2_ensure_limit_injects_limit_once() {
    let out = u2_closures::ensure_limit("SELECT * FROM t", 500);
    assert_eq!(out.to_uppercase().matches("LIMIT").count(), 1);
    assert!(out.ends_with("LIMIT 500"));
    let kept = u2_closures::ensure_limit("SELECT * FROM t LIMIT 10", 500);
    assert!(kept.to_uppercase().contains("LIMIT 10"));
}

#[test]
fn test_u2_normalize_identifier_for_dedup() {
    // 归一化查重口径：trim + 小写后比较
    assert_eq!(
        u2_closures::normalize_identifier("  MyStatement "),
        "mystatement"
    );
    assert_eq!(u2_closures::normalize_identifier("ABC"), "abc");
}
