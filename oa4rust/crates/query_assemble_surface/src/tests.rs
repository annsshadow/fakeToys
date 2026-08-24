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
fn test_create_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "id": "surface-1",
        "name": "My Surface",
        "category": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
}

#[test]
fn test_get_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "name": "Query Surface",
        "category": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "surface-1");
}

#[test]
fn test_list_surfaces_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "surface-1", "category": "default"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_save_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "saved": true,
        "updated_at": "2024-01-01T00:00:00Z"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["saved"], true);
}

#[tokio::test]
async fn test_create_surface_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "name": "My Surface",
        "query": "select * from test",
        "template": "default"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/create")
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
async fn test_get_surface_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/get/surface-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_surfaces_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/list/default")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_save_surface_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "name": "My Surface",
        "query": "select * from test"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/save/surface-1")
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
async fn test_delete_surface_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/delete/surface-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}


// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 缺口闭合测试：新注册路由存在性 + sqlparser 安全约束 + 格式化/参数化
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_u2_importmodel_uuid_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/uuid")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // uuid 生成端点不依赖数据库，直接成功返回
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_u2_query_list_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_stat_get_id_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/stat/stat-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_stat_list_with_query_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/stat/list/query/query-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_statement_get_format_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/statement/st-1/format")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_statement_execute_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/statement/execute/st-1/page/1/size/20")
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
async fn test_u2_search_post_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/search")
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
async fn test_u2_morelikethis_post_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/morelikethis")
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
async fn test_u2_table_row_delete_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/table/row/delete/tbl-1/row-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_table_row_insert_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/table/row/insert/tbl-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ── sqlparser 安全约束与工具函数（无 DB 依赖的纯单测）──

#[test]
fn test_u2_surface_validate_rejects_update() {
    let err = u2_closures::validate_single_select("UPDATE x_query_view SET name = 'x'").unwrap_err();
    assert!(err.contains("only SELECT"), "UPDATE must be rejected, got: {}", err);
}

#[test]
fn test_u2_surface_validate_rejects_multi_statement() {
    let err = u2_closures::validate_single_select("SELECT 1; DELETE FROM t").unwrap_err();
    assert!(err.contains("single statement"));
}

#[test]
fn test_u2_surface_validate_accepts_select_with_join() {
    assert!(u2_closures::validate_single_select(
        "SELECT a.id FROM x_query_design a JOIN x_query_view b ON a.id = b.id"
    )
    .is_ok());
}

#[test]
fn test_u2_surface_parameterize_named_params() {
    let params = json!({"flag": "v1", "limit0": 5});
    let (sql, values) =
        u2_closures::parameterize_statement_sql("SELECT * FROM v WHERE flag = :flag AND n <> :limit0", &params);
    assert!(sql.contains("$1") && sql.contains("$2"));
    assert_eq!(values.len(), 2);
}

#[test]
fn test_u2_surface_format_sql_inserts_newlines() {
    let out = u2_closures::format_sql("SELECT id FROM t WHERE id = 1 ORDER BY id");
    assert!(out.contains('\n'), "keywords should break lines: {}", out);
    assert!(out.to_uppercase().contains("ORDER BY"));
}


#[tokio::test]
async fn test_u2_importmodel_record_delete_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/record/delete/record-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_u2_importmodel_reexecute_record_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/execute/record/record-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}


// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 v9 缺口闭合测试：Java 精确路径/动词注册 + 纯函数契约
// 路由存在性口径：空 Config 池 → handler 执行到池获取失败 → 500（404 即路由缺失）
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_v9_importmodel_flag_query_java_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/flag/im-1/query/qf-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_importmodel_record_family_java_paths_exist() {
    let pool = build_test_pool();

    let get = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/record/r-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get.status(), StatusCode::INTERNAL_SERVER_ERROR, "GET /importmodel/record/{{id}}");

    let status = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/record/r-1/status")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(status.status(), StatusCode::INTERNAL_SERVER_ERROR, "GET .../status");

    let del = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/record/r-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del.status(), StatusCode::INTERNAL_SERVER_ERROR, "DELETE /importmodel/record/{{id}} 动词补齐");
}

#[tokio::test]
async fn test_v9_importmodel_execute_java_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/importmodel/im-1/execute")
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
async fn test_v9_query_flag_and_list_java_paths_exist() {
    let pool = build_test_pool();

    let get_flag = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/query/qf-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_flag.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let list_key = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/query/list/key/kw")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_key.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_stat_execute_with_query_java_verb_route_exists() {
    let pool = build_test_pool();

    let put = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/stat/flag/s-1/query/qf-1/execute")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(put.status(), StatusCode::INTERNAL_SERVER_ERROR, "PUT /stat/flag/{{flag}}/query/{{qf}}/execute");

    let mock = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/stat/flag/s-1/query/qf-1/execute/mockputtopost")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(mock.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_statement_execute_java_path_route_exists() {
    let pool = build_test_pool();
    // Java 精确段序：statement/{{flag}}/execute/page/{{page}}/size/{{size}}
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/statement/st-1/execute/page/1/size/20")
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
async fn test_v9_table_paging_post_java_verb_routes_exist() {
    let pool = build_test_pool();

    let paging = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/table/list/paging/1/size/20")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(paging.status(), StatusCode::INTERNAL_SERVER_ERROR, "POST /table/list/paging/{{page}}/size/{{size}}");

    let row_paging = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/table/list/table/tf-1/row/paging/1/size/20")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(row_paging.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_table_row_select_post_java_verb_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/table/list/tf-1/row/select")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"where":"k"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_view_bundle_excel_execute_java_verbs_exist() {
    for (uri, label) in [
        ("/jaxrs/queryview/view/v-1/bundle", "PUT /view/{{id}}/bundle"),
        ("/jaxrs/queryview/view/v-1/excel", "PUT /view/{{id}}/excel"),
        ("/jaxrs/queryview/view/v-1/execute", "PUT /view/{{id}}/execute"),
    ] {
        let response = crate::router(build_test_pool())
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .method(Method::PUT)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"excelData":""}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR, "{} 动词补齐", label);
    }
}

#[tokio::test]
async fn test_v9_view_flag_query_bundle_mockputtopost_java_path_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/view/flag/v-1/query/qf-1/bundle/mockputtopost")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"grid":[]}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_view_execute_v2_java_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/jaxrs/queryview/view/flag/v-1/query/qf-1/execute/v2/page/1/size/20")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_v9_format_sql_contract_for_statement_format_endpoint() {
    // statement/{{id}}/format 输出契约：主要关键字前换行，保持语义不变
    let formatted = u2_closures::format_sql("SELECT a FROM t WHERE x = 1 ORDER BY a");
    assert!(formatted.contains('\n'), "关键字前应换行: {}", formatted);
    assert!(formatted.to_uppercase().contains("SELECT"), "SELECT 必须保留");
    assert!(formatted.to_uppercase().contains("ORDER BY"), "ORDER BY 必须保留");
}
