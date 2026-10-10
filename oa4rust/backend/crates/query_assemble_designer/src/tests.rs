// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use serde_json::json;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
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
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/create")
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
                .uri("/api/query/assemble/designer/get/designer-1")
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
                .uri("/api/query/assemble/designer/list/default")
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
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/save/designer-1")
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
                .uri("/api/query/assemble/designer/delete/designer-1")
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
                .uri("/api/query/assemble/designer/search")
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
                .uri("/api/query/assemble/designer/input/compare")
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
                .uri("/api/query/assemble/designer/neural/model")
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
                .uri("/api/query/assemble/designer/output/list")
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
                .uri("/api/query/assemble/designer/list/all")
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
                .uri("/api/query/assemble/designer/table/list/manage")
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
                .uri("/api/query/assemble/designer/view/view-1")
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
                .uri("/api/query/assemble/designer/statement/st-1")
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
                .uri("/api/query/assemble/designer/statement/execute/st-1/page/1/size/20")
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
                .uri(
                    "/api/query/assemble/designer/statement/execute/st-1/mode/count/page/1/size/20",
                )
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
    assert!(
        err.contains("only SELECT"),
        "DELETE must be rejected, got: {}",
        err
    );
}

#[test]
fn test_u2_validate_single_select_rejects_multi_statement() {
    let err =
        u2_closures::validate_single_select("SELECT 1; DROP TABLE x_query_statement").unwrap_err();
    assert!(
        err.contains("single statement"),
        "multi-statement must be rejected"
    );
}

#[test]
fn test_u2_validate_single_select_rejects_empty() {
    assert!(u2_closures::validate_single_select("   ").is_err());
}

#[test]
fn test_u2_validate_single_select_accepts_select() {
    // o2server statement.data 为 JPQL 风格，Rust 侧存储可直接执行的 SQL；
    // 此处验证合法 SELECT 通过安全校验。
    assert!(u2_closures::validate_single_select(
        "SELECT id, name FROM x_query_table WHERE table_flag = 't1'"
    )
    .is_ok());
}

#[test]
fn test_u2_parameterize_statement_sql_binds_named_params() {
    let params = json!({"person": "张三@unit", "minAge": 18});
    let (sql, values) = u2_closures::parameterize_statement_sql(
        "SELECT * FROM t WHERE o.name = :person AND o.age > :minAge",
        &params,
    );
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
    assert!(
        !sql.contains("$1::"),
        "cast :: must not be treated as param"
    );
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

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 v9 缺口闭合测试：o2server 精确路径/动词注册 + 纯函数契约
// 路由存在性口径：空 Config 池 → handler 执行到池获取失败 → 500（404 即路由缺失）
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn test_v9_id_generate_clamps_count_and_generates_uuids() {
    // o2server ActionGet: 0 < count < 200 逐一生成，越界截断
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let resp = u2_closures::id_generate(axum::extract::Path(3i64))
            .await
            .unwrap();
        let ar = resp.0;
        let data = ar.data.as_ref().unwrap();
        assert_eq!(ar.count.unwrap(), 3, "count=3 应生成 3 个 id");
        assert_eq!(data.as_array().unwrap().len(), 3);

        let resp = u2_closures::id_generate(axum::extract::Path(500i64))
            .await
            .unwrap();
        assert_eq!(resp.0.count.unwrap(), 199, "count>200 截断为 199");

        let resp = u2_closures::id_generate(axum::extract::Path(0i64))
            .await
            .unwrap();
        assert_eq!(resp.0.count.unwrap(), 0, "count=0 不生成");
    });
}

#[tokio::test]
async fn test_v9_designer_search_legacy_path_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/designer/search")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"key":"test"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_importmodel_flag_crud_routes_exist() {
    let pool = build_test_pool();

    let get = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/importmodel/im-flag-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        get.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "GET /importmodel/{{flag}}"
    );

    let put = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/importmodel/im-flag-1")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"n1"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        put.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "PUT /importmodel/{{flag}}"
    );

    let del = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/importmodel/im-flag-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        del.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "DELETE /importmodel/{{flag}}"
    );
}

#[tokio::test]
async fn test_v9_importmodel_permission_legacy_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/importmodel/im-flag-1/permission")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"permissionList":[]}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_neural_model_reset_status_legacy_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/neural/model/m1/reset/status")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_query_put_delete_legacy_verb_routes_exist() {
    let pool = build_test_pool();

    let put = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/query/q-flag-1")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"q1"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        put.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "PUT /query/{{flag}} 动词补齐"
    );

    let del = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/query/q-flag-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        del.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "DELETE /query/{{flag}} 动词补齐"
    );
}

#[tokio::test]
async fn test_v9_query_list_all_legacy_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/query/list/all")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_stat_simulate_legacy_verb_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/stat/s1/simulate")
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
async fn test_v9_statement_execute_legacy_path_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    // o2server 精确段序：statement/{{flag}}/execute/page/{{page}}/size/{{size}}
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/statement/st-1/execute/page/1/size/20")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let mode_response = crate::router(build_test_pool())
        .oneshot(
            Request::builder()
                .uri(
                    "/api/query/assemble/designer/statement/st-1/execute/mode/count/page/1/size/20",
                )
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(mode_response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_table_row_legacy_paths_route_exists() {
    let pool = build_test_pool();

    let get_row = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/table/tf-1/row/r-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_row.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let insert = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/table/tf-1/row")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"k":"v"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(insert.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let count_where = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/table/tf-1/row/count/where/name='x'")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(count_where.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_view_bundle_simulate_legacy_verb_routes_exist() {
    let pool = build_test_pool();

    let bundle = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/view/v-1/bundle")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"grid":[]}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        bundle.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "PUT /view/{{id}}/bundle"
    );

    let simulate = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/designer/view/v-1/simulate")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        simulate.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "PUT /view/{{id}}/simulate"
    );
}

#[test]
fn test_v9_normalize_dedup_semantics_for_new_create_paths() {
    // v9 新增 create/edit 均以 normalize_identifier（trim+小写）做归一化查重：
    // 大小写与首尾空白差异必须视为同名。
    let a = u2_closures::normalize_identifier("  MyQuery ");
    let b = u2_closures::normalize_identifier("myquery");
    assert_eq!(a, b, "归一化后必须等价，查重才能命中");
}

#[test]
fn table_columns_contract_accepts_only_typed_safe_columns() {
    let columns = parse_table_columns(&json!({
        "columns": [
            {"name": "title", "type": "text", "nullable": false},
            {"name": "created_at", "type": "datetime"}
        ]
    }))
    .expect("valid typed columns");
    assert_eq!(columns[0].data_type, "text");
    assert!(!columns[0].nullable);
    assert!(columns[1].nullable);

    for unsafe_body in [
        json!({"columns": [{"name": "title; DROP TABLE users", "type": "text"}]}),
        json!({"columns": [{"name": "title", "type": "text; DROP TABLE users"}]}),
        json!({"columns": [{"name": "id", "type": "text"}]}),
        json!({"columns": [{"name": "title", "type": "text", "sql": "DROP TABLE users"}]}),
    ] {
        assert!(parse_table_columns(&unsafe_body).is_err());
    }
}

#[test]
fn table_ddl_is_generated_from_whitelisted_metadata() {
    let columns = vec![
        TableColumnDefinition {
            name: "title".to_string(),
            data_type: "text".to_string(),
            nullable: false,
        },
        TableColumnDefinition {
            name: "score".to_string(),
            data_type: "decimal".to_string(),
            nullable: true,
        },
    ];
    let ddl = create_table_ddl("t_safe_1", &columns).expect("safe DDL");
    assert_eq!(ddl, "CREATE TABLE \"x_query_data_t_safe_1\" (\"id\" UUID PRIMARY KEY DEFAULT gen_random_uuid(), \"title\" TEXT NOT NULL, \"score\" DOUBLE PRECISION)");
    assert!(physical_table_name("bad-name").is_err());
}

/// 软删泄漏回归（优化二轮 42）：importmodel_get_flag 原 WHERE 子句
/// `id=$1 OR model_flag=$1 AND deleted_at IS NULL` 因 AND 优先级高于 OR，实际为
/// `id=$1 OR (model_flag=$1 AND deleted_at IS NULL)`——按 id 命中的已删行照样返回。
/// 括号化修复后：已删导入模型按 id 查应返回「not found」，未删行正常返回。
#[tokio::test]
async fn importmodel_get_flag_hides_soft_deleted_by_id() {
    if !shared::testing::is_db_available().await {
        eprintln!("skipping importmodel_get_flag_hides_soft_deleted_by_id: DB not reachable");
        return;
    }
    let pool = shared::testing::test_pool();
    let c = pool.get().await.unwrap();
    c.execute(
        "CREATE TABLE IF NOT EXISTS x_query_import_model (id TEXT PRIMARY KEY, name TEXT, model_flag TEXT, query_flag TEXT, content TEXT, creator TEXT, creator_person TEXT, create_time TEXT, update_time TEXT, permission TEXT, deleted_at TIMESTAMP)",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "DELETE FROM x_query_import_model WHERE id IN ('u2-im-dead','u2-im-live')",
        &[],
    )
    .await
    .unwrap();
    // 已删行（按 id 命中）+ 未删行
    c.execute(
        "INSERT INTO x_query_import_model (id, name, deleted_at) VALUES ('u2-im-dead', 'dead-model', NOW())",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "INSERT INTO x_query_import_model (id, name, deleted_at) VALUES ('u2-im-live', 'live-model', NULL)",
        &[],
    )
    .await
    .unwrap();

    // 已删行按 id 查 → 必须 not found（修复前因 OR/AND 优先级会泄漏）
    let dead = crate::u2_closures::importmodel_get_flag(
        axum::Extension(pool.clone()),
        axum::extract::Path("u2-im-dead".to_string()),
    )
    .await
    .unwrap();
    let dead_json = serde_json::to_value(&dead.0).unwrap();
    assert_eq!(
        dead_json["type"], "error",
        "软删导入模型按 id 查不得返回记录"
    );

    // 未删行按 id 查 → 正常返回
    let live = crate::u2_closures::importmodel_get_flag(
        axum::Extension(pool.clone()),
        axum::extract::Path("u2-im-live".to_string()),
    )
    .await
    .unwrap();
    let live_json = serde_json::to_value(&live.0).unwrap();
    assert_eq!(live_json["type"], "success");
    assert_eq!(live_json["data"]["id"], "u2-im-live");

    c.execute(
        "DELETE FROM x_query_import_model WHERE id IN ('u2-im-dead','u2-im-live')",
        &[],
    )
    .await
    .unwrap();
}

/// NOW()→TEXT 列类型错配回归（优化二轮 49）：x_query_table.update_time 为 TEXT，
/// table_reload_dynamic 原 `SET update_time = NOW()`（timestamptz）在 PG 无 text 隐式
/// 赋值转换会 500；修为 to_char 字面量后应成功。同族 build/draft 写法已 to_char。
#[tokio::test]
async fn table_reload_dynamic_writes_text_update_time_ok() {
    if !shared::testing::is_db_available().await {
        eprintln!("skipping table_reload_dynamic_writes_text_update_time_ok: DB not reachable");
        return;
    }
    let pool = shared::testing::test_pool();
    let c = pool.get().await.unwrap();
    c.execute(
        "CREATE TABLE IF NOT EXISTS x_query_table (id TEXT, name TEXT, table_flag TEXT, creator TEXT, create_time TEXT, query_flag TEXT, update_time TEXT, status TEXT, reloaded BOOLEAN DEFAULT false)",
        &[],
    )
    .await
    .unwrap();
    for col in [
        "ALTER TABLE x_query_table ADD COLUMN IF NOT EXISTS status TEXT",
        "ALTER TABLE x_query_table ADD COLUMN IF NOT EXISTS reloaded BOOLEAN DEFAULT false",
        "ALTER TABLE x_query_table ADD COLUMN IF NOT EXISTS update_time TEXT",
    ] {
        c.execute(col, &[]).await.unwrap();
    }
    c.execute("DELETE FROM x_query_table WHERE id = 'u2-qt-reload'", &[])
        .await
        .unwrap();
    c.execute(
        "INSERT INTO x_query_table (id, table_flag, reloaded) VALUES ('u2-qt-reload','u2flag',false)",
        &[],
    )
    .await
    .unwrap();

    // 修复前：NOW() 赋给 TEXT 列 → PG 报错 → handler 返回 500；修复后成功
    let resp = crate::table_reload_dynamic(axum::Extension(pool.clone()))
        .await
        .expect("reload handler 不应因 NOW()→TEXT 类型错配而 500");
    let j = serde_json::to_value(&resp.0).unwrap();
    assert_eq!(j["type"], "success");

    let row = c
        .query_one(
            "SELECT reloaded, update_time FROM x_query_table WHERE id = 'u2-qt-reload'",
            &[],
        )
        .await
        .unwrap();
    assert!(row.get::<_, bool>("reloaded"), "reloaded 应被置 true");
    assert!(
        !row.get::<_, Option<String>>("update_time")
            .unwrap_or_default()
            .is_empty(),
        "update_time 应写入字面量时间串"
    );

    c.execute("DELETE FROM x_query_table WHERE id = 'u2-qt-reload'", &[])
        .await
        .unwrap();
}

/// NOW()→TEXT 类型错配回归（优化二轮 50，续轮49）：x_query_input 的 create_time
/// 为 TEXT，input_create 原 `VALUES (.., NOW())` 在 PG 无 timestamptz→text 隐式赋值
/// 转换会 500；修为 to_char 字面量后应成功落库。
#[tokio::test]
async fn input_create_writes_text_create_time_ok() {
    if !shared::testing::is_db_available().await {
        eprintln!("skipping input_create_writes_text_create_time_ok: DB not reachable");
        return;
    }
    let pool = shared::testing::test_pool();
    let c = pool.get().await.unwrap();
    c.execute(
        "CREATE TABLE IF NOT EXISTS x_query_input (id TEXT PRIMARY KEY, content TEXT, creator TEXT, create_time TEXT, update_time TEXT)",
        &[],
    )
    .await
    .unwrap();
    for col in [
        "ALTER TABLE x_query_input ADD COLUMN IF NOT EXISTS create_time TEXT",
        "ALTER TABLE x_query_input ADD COLUMN IF NOT EXISTS update_time TEXT",
    ] {
        c.execute(col, &[]).await.unwrap();
    }

    // 修复前：NOW() 赋 TEXT 列 → 报错 500；修复后 to_char 字面量成功
    let resp = crate::input_create(
        axum::Extension(pool.clone()),
        axum::Json(serde_json::json!({"content": "u2-input-body"})),
    )
    .await
    .expect("input_create 不应因 NOW()→TEXT 类型错配而 500");
    let j = serde_json::to_value(&resp.0).unwrap();
    assert_eq!(j["type"], "success");
    let id = j["data"]["id"].as_str().expect("new input id");

    let ct: Option<String> = c
        .query_one(
            "SELECT create_time FROM x_query_input WHERE id = $1",
            &[&id.to_string()],
        )
        .await
        .unwrap()
        .get("create_time");
    assert!(
        !ct.unwrap_or_default().is_empty(),
        "create_time 应写入字面量时间串"
    );
    c.execute(
        "DELETE FROM x_query_input WHERE id = $1",
        &[&id.to_string()],
    )
    .await
    .unwrap();
}
