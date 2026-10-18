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
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/surface/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    // 第四十四批：加 admin 门后，无会话先于读库回 401（原无门时是假 pool 的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_surface_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/surface/get/surface-1")
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
                .uri("/api/query/assemble/surface/list/default")
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
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/surface/save/surface-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    // 第四十四批：加 admin 门后无会话先回 401（原无门时是假 pool 的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_surface_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/assemble/surface/delete/surface-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // 第四十四批：加 admin 门后无会话先回 401（原无门时是假 pool 的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
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
                .uri("/api/queryview/importmodel/uuid")
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
                .uri("/api/queryview/list")
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
                .uri("/api/queryview/stat/stat-1")
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
                .uri("/api/queryview/stat/list/query/query-1")
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
                .uri("/api/queryview/statement/st-1/format")
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
                .uri("/api/queryview/statement/execute/st-1/page/1/size/20")
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
                .uri("/api/queryview/search")
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
                .uri("/api/queryview/morelikethis")
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
                .uri("/api/queryview/table/row/delete/tbl-1/row-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // 第四十四批：加 admin 门后无会话先回 401（原无门时是假 pool 的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_u2_table_row_insert_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/queryview/table/row/insert/tbl-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    // 第五十九批：table_row_insert 收 admin 门后无会话返 401（原 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ── sqlparser 安全约束与工具函数（无 DB 依赖的纯单测）──

#[test]
fn test_u2_surface_validate_rejects_update() {
    let err =
        u2_closures::validate_single_select("UPDATE x_query_view SET name = 'x'").unwrap_err();
    assert!(
        err.contains("only SELECT"),
        "UPDATE must be rejected, got: {}",
        err
    );
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
    let (sql, values) = u2_closures::parameterize_statement_sql(
        "SELECT * FROM v WHERE flag = :flag AND n <> :limit0",
        &params,
    );
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
                .uri("/api/queryview/importmodel/record/delete/record-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // 第四十四批：加 admin 门后无会话先回 401（原无门时是假 pool 的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_u2_importmodel_reexecute_record_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/queryview/importmodel/execute/record/record-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // 收 admin 门后：无会话先于读库回 401（而非原先无门的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_query_get_writes_non_admin_forbidden() {
    use axum::extract::Extension;
    let pool = build_test_pool();
    let sess = shared::testing::test_session("query-cc-nonadmin");
    // 两把 GET 写面（table 批量 reload / importmodel 重执行）非 admin 必须 403
    //（is_admin 对空池 fail-closed）。
    assert!(
        matches!(
            crate::table_reload_dynamic(Extension(pool.clone()), Some(Extension(sess.clone())))
                .await,
            Err(shared::error::AppError::Forbidden)
        ),
        "table_reload_dynamic 非 admin 必须 403"
    );
    assert!(
        matches!(
            crate::u2_closures::importmodel_reexecute_record(
                Extension(pool.clone()),
                Some(Extension(sess.clone())),
                axum::extract::Path("record-1".to_string())
            )
            .await,
            Err(shared::error::AppError::Forbidden)
        ),
        "importmodel_reexecute_record 非 admin 必须 403"
    );
}

// ── 第四十四批：R7 含写收口（query_assemble_surface 真写面 7 把收 admin 门）──
// 真写面 = create/save/delete_surface（x_query_surface，creator 硬编码 system）、
// table_row_delete（x_query_table_data 按 id 删）、table_tableFlag_row_delete_all
// （整表清空）、importmodel_record_delete（x_query_import_model_record 按 id 删）、
// importmodel_id_execute（INSERT 执行记录）。其余 28 条 R7 多为 SELECT 读
// （importmodel list/record/status/id、stat/statement/view/table 的 list/paging/execute
// 等 mockputtopost / mockdeletetoget 形态），属「只读挂写方法」族，不在此批。
#[tokio::test]
async fn test_q44_write_writes_reject_without_session() {
    use axum::extract::{Extension, Json, Path};
    let pool = build_test_pool();
    let cases: Vec<(&str, Result<(), shared::error::AppError>)> = vec![
        (
            "create_surface",
            crate::create_surface(
                Extension(pool.clone()),
                None,
                Json(crate::CreateSurfaceRequest {
                    name: None,
                    category: None,
                    query: None,
                    template: None,
                }),
            )
            .await
            .map(|_| ()),
        ),
        (
            "save_surface",
            crate::save_surface(
                Extension(pool.clone()),
                None,
                Path("s-1".to_string()),
                Json(crate::CreateSurfaceRequest {
                    name: None,
                    category: None,
                    query: None,
                    template: None,
                }),
            )
            .await
            .map(|_| ()),
        ),
        (
            "delete_surface",
            crate::delete_surface(Extension(pool.clone()), None, Path("s-1".to_string()))
                .await
                .map(|_| ()),
        ),
        (
            "table_tableFlag_row_delete_all",
            crate::table_tableFlag_row_delete_all(
                Extension(pool.clone()),
                None,
                Path("tf-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "importmodel_id_execute",
            crate::importmodel_id_execute(Extension(pool.clone()), None, Path("im-1".to_string()))
                .await
                .map(|_| ()),
        ),
        (
            "u2::table_row_delete",
            crate::u2_closures::table_row_delete(
                Extension(pool.clone()),
                None,
                Path(("tf-1".to_string(), "r-1".to_string())),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u2::importmodel_record_delete",
            crate::u2_closures::importmodel_record_delete(
                Extension(pool.clone()),
                None,
                Path("rec-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
    ];
    let bad: Vec<&str> = cases
        .iter()
        .filter(|(_, r)| !matches!(r, Err(shared::error::AppError::Unauthorized)))
        .map(|(n, _)| *n)
        .collect();
    assert!(bad.is_empty(), "以下 R7 真写面无会话未回 401：{bad:?}");
    assert_eq!(cases.len(), 7, "本批 7 把必须逐把点名，少一把即漏门");
}

#[tokio::test]
async fn test_q44_write_writes_non_admin_forbidden() {
    use axum::extract::{Extension, Json, Path};
    let pool = build_test_pool();
    // 非 admin 会话：is_admin 对空池 fail-closed → false → 403（与 crate 既有门禁测试同口径）。
    let sess = shared::testing::test_session("q44-nonadmin");
    let some = Some(Extension(sess));
    let cases: Vec<(&str, Result<(), shared::error::AppError>)> = vec![
        (
            "create_surface",
            crate::create_surface(
                Extension(pool.clone()),
                some.clone(),
                Json(crate::CreateSurfaceRequest {
                    name: None,
                    category: None,
                    query: None,
                    template: None,
                }),
            )
            .await
            .map(|_| ()),
        ),
        (
            "save_surface",
            crate::save_surface(
                Extension(pool.clone()),
                some.clone(),
                Path("s-1".to_string()),
                Json(crate::CreateSurfaceRequest {
                    name: None,
                    category: None,
                    query: None,
                    template: None,
                }),
            )
            .await
            .map(|_| ()),
        ),
        (
            "delete_surface",
            crate::delete_surface(
                Extension(pool.clone()),
                some.clone(),
                Path("s-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "table_tableFlag_row_delete_all",
            crate::table_tableFlag_row_delete_all(
                Extension(pool.clone()),
                some.clone(),
                Path("tf-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "importmodel_id_execute",
            crate::importmodel_id_execute(
                Extension(pool.clone()),
                some.clone(),
                Path("im-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u2::table_row_delete",
            crate::u2_closures::table_row_delete(
                Extension(pool.clone()),
                some.clone(),
                Path(("tf-1".to_string(), "r-1".to_string())),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u2::importmodel_record_delete",
            crate::u2_closures::importmodel_record_delete(
                Extension(pool.clone()),
                some,
                Path("rec-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
    ];
    let bad: Vec<&str> = cases
        .iter()
        .filter(|(_, r)| !matches!(r, Err(shared::error::AppError::Forbidden)))
        .map(|(n, _)| *n)
        .collect();
    assert!(bad.is_empty(), "以下 R7 真写面非 admin 未回 403：{bad:?}");
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 v9 缺口闭合测试：o2server 精确路径/动词注册 + 纯函数契约
// 路由存在性口径：空 Config 池 → handler 执行到池获取失败 → 500（404 即路由缺失）
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_v9_importmodel_flag_query_legacy_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/importmodel/flag/im-1/query/qf-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_importmodel_record_family_legacy_paths_exist() {
    let pool = build_test_pool();

    let get = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/queryview/importmodel/record/r-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        get.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "GET /importmodel/record/{{id}}"
    );

    let status = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/queryview/importmodel/record/r-1/status")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        status.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "GET .../status"
    );

    let del = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/importmodel/record/r-1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        del.status(),
        StatusCode::UNAUTHORIZED,
        "DELETE /importmodel/record/{{id}} 动词补齐 (已加门, 无会话 401)"
    );
}

#[tokio::test]
async fn test_v9_importmodel_execute_legacy_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/importmodel/im-1/execute")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_v9_query_flag_and_list_legacy_paths_exist() {
    let pool = build_test_pool();

    let get_flag = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/queryview/query/qf-1")
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
                .uri("/api/queryview/query/list/key/kw")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list_key.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_v9_stat_execute_with_query_legacy_verb_route_exists() {
    let pool = build_test_pool();

    let put = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/queryview/stat/flag/s-1/query/qf-1/execute")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        put.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "PUT /stat/flag/{{flag}}/query/{{qf}}/execute"
    );

    let mock = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/stat/flag/s-1/query/qf-1/execute/mockputtopost")
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
async fn test_v9_statement_execute_legacy_path_route_exists() {
    let pool = build_test_pool();
    // o2server 精确段序：statement/{{flag}}/execute/page/{{page}}/size/{{size}}
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/statement/st-1/execute/page/1/size/20")
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
async fn test_v9_table_paging_post_legacy_verb_routes_exist() {
    let pool = build_test_pool();

    let paging = crate::router(pool.clone())
        .oneshot(
            Request::builder()
                .uri("/api/queryview/table/list/paging/1/size/20")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        paging.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "POST /table/list/paging/{{page}}/size/{{size}}"
    );

    let row_paging = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/table/list/table/tf-1/row/paging/1/size/20")
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
async fn test_v9_table_row_select_post_legacy_verb_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/table/list/tf-1/row/select")
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
async fn test_v9_view_bundle_excel_execute_legacy_verbs_exist() {
    for (uri, label) in [
        ("/api/queryview/view/v-1/bundle", "PUT /view/{{id}}/bundle"),
        ("/api/queryview/view/v-1/excel", "PUT /view/{{id}}/excel"),
        (
            "/api/queryview/view/v-1/execute",
            "PUT /view/{{id}}/execute",
        ),
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
        assert_eq!(
            response.status(),
            StatusCode::INTERNAL_SERVER_ERROR,
            "{} 动词补齐",
            label
        );
    }
}

#[tokio::test]
async fn test_v9_view_flag_query_bundle_mockputtopost_legacy_path_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/view/flag/v-1/query/qf-1/bundle/mockputtopost")
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
async fn test_v9_view_execute_v2_legacy_path_route_exists() {
    let pool = build_test_pool();
    let response = crate::router(pool)
        .oneshot(
            Request::builder()
                .uri("/api/queryview/view/flag/v-1/query/qf-1/execute/v2/page/1/size/20")
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
    assert!(
        formatted.to_uppercase().contains("SELECT"),
        "SELECT 必须保留"
    );
    assert!(
        formatted.to_uppercase().contains("ORDER BY"),
        "ORDER BY 必须保留"
    );
}
