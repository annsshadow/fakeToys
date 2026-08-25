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
fn test_create_flow_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "id": "flow-1",
        "name": "My Flow",
        "description": "A process flow",
        "category": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
}

#[test]
fn test_get_flow_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "flow-1",
        "name": "Process Flow",
        "nodes": [],
        "edges": []
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "flow-1");
}

#[test]
fn test_list_flows_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "flow-1", "category": "processplatform"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_save_flow_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "flow-1",
        "saved": true,
        "updated_at": "2024-01-01T00:00:00Z"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["saved"], true);
}

#[test]
fn test_delete_flow_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "flow-1",
        "deleted": true
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["deleted"], true);
}

#[test]
fn test_preview_flow_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "flow-1",
        "preview_url": "/preview/flow/flow-1",
        "nodes": [],
        "edges": []
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["preview_url"], "/preview/flow/flow-1");
}

#[tokio::test]
async fn test_create_flow_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "name": "My Flow",
        "description": "A process flow",
        "category": "default"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/designer/create")
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
async fn test_get_flow_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/designer/get/flow-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_flows_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/designer/list/processplatform")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_save_flow_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "nodes": [],
        "edges": []
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/designer/save/flow-1")
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
async fn test_delete_flow_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/designer/delete/flow-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_preview_flow_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/designer/preview/flow-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 新增：Java 端点缺口闭合（51 端点）的测试
// 路由存在性用空配置池（pool.get() 必败 → 500），断言"非 404"即证明注册成功；
// 归一化查重与 IDOR 门禁的关键决策用纯函数单测固化。
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn test_normalize_name_key_collapses_whitespace_and_case() {
    // 查重口径：首尾空白、内部空白宽度、大小写差异都不构成新名字
    assert_eq!(normalize_name_key("  请假  申请 "), "请假 申请");
    assert_eq!(normalize_name_key("Leave\tRequest"), "leave request");
    assert_eq!(normalize_name_key("ABC"), normalize_name_key("abc"));
}

#[test]
fn test_normalize_name_key_distinguishes_different_names() {
    // 反例守护：不同语义的名字不能被归一化吞掉，否则查重会误杀合法创建
    assert_ne!(normalize_name_key("请假申请"), "出差申请");
    assert_ne!(normalize_name_key("a b"), "ab");
}

#[test]
fn test_application_update_envelope_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "app-1",
        "updated": true
    }));
    let v = serde_json::to_value(&result).unwrap();
    assert_eq!(v["type"], "success");
    assert_eq!(v["data"]["updated"], true);
}

async fn designer_u2_status(method: Method, uri: &str, body: Option<String>) -> StatusCode {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let mut builder = Request::builder().uri(uri).method(method);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let req = match body {
        Some(b) => builder.body(Body::from(b)).unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    };
    app.oneshot(req).await.unwrap().status()
}

#[tokio::test]
async fn test_designer_put_application_registered() {
    // Java PUT /application/{id}：此前仅有 GET，PUT 属主门禁端点必须可达
    let status = designer_u2_status(Method::PUT, "/jaxrs/processplatform/assemble/designer/application/app-1", Some("{\"name\":\"x\"}".into())).await;
    assert_ne!(status, StatusCode::NOT_FOUND, "PUT /application/{{id}} 应已注册");
}

#[tokio::test]
async fn test_designer_put_application_icon_exact_shape() {
    // Java 精确路径 /application/{id}/icon（区别于遗留 /application/icon/{id}）
    let status = designer_u2_status(Method::PUT, "/jaxrs/processplatform/assemble/designer/application/app-1/icon", Some("{\"icon\":\"mood\"}".into())).await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_designer_post_application_permission_registered() {
    let status = designer_u2_status(Method::POST, "/jaxrs/processplatform/assemble/designer/application/app-1/permission", Some("{\"view\":[\"xadmin\"]}".into())).await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_designer_applicationcategory_list_registered() {
    let status = designer_u2_status(Method::GET, "/jaxrs/processplatform/assemble/designer/applicationcategory/list", None).await;
    assert_ne!(status, StatusCode::NOT_FOUND, "applicationcategory/list handler 已存在但此前从未路由");
}

#[tokio::test]
async fn test_designer_item_access_family_registered() {
    let base = "/jaxrs/processplatform/assemble/designer/item-access";
    for (method, uri) in [
        (Method::POST, base.to_string()),
        (Method::POST, format!("{}/bach/save", base)),
        (Method::GET, format!("{}/item-1", base)),
        (Method::GET, format!("{}/path/p1", base)),
        (Method::GET, format!("{}/process/pr1", base)),
        (Method::DELETE, format!("{}/delete/process/pr1/path/p1", base)),
    ] {
        let status = designer_u2_status(method.clone(), &uri, if method == Method::POST { Some("{\"items\":[]}".into()) } else { None }).await;
        assert_ne!(status, StatusCode::NOT_FOUND, "item-access 端点应注册: {}", uri);
    }
}

#[tokio::test]
async fn test_designer_mapping_verbs_registered() {
    let status_get = designer_u2_status(Method::GET, "/jaxrs/processplatform/assemble/designer/mapping/m-1/execute", None).await;
    assert_ne!(status_get, StatusCode::NOT_FOUND, "/mapping/{{flag}}/execute Java 精确形态");
    let status_delete = designer_u2_status(Method::DELETE, "/jaxrs/processplatform/assemble/designer/mapping/m-1", None).await;
    assert_ne!(status_delete, StatusCode::NOT_FOUND, "DELETE /mapping/{{flag}} 应已补齐动词");
}

#[tokio::test]
async fn test_designer_mergeitemplan_verbs_and_paging() {
    let base = "/jaxrs/processplatform/assemble/designer";
    assert_ne!(designer_u2_status(Method::GET, &format!("{}/mergeitemplan/mp-1", base), None).await, StatusCode::NOT_FOUND);
    assert_ne!(designer_u2_status(Method::GET, &format!("{}/mergeitemplan/list/paging/1/size/20", base), None).await, StatusCode::NOT_FOUND, "分页精确形态 paging/{{page}}/size/{{size}}");
    assert_ne!(designer_u2_status(Method::GET, &format!("{}/mergeitemplan/list/application/app-1/paging/1/size/20", base), None).await, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_designer_process_edition_family_registered() {
    let base = "/jaxrs/processplatform/assemble/designer";
    for (method, uri) in [
        (Method::GET, format!("{}/process/application/app-1/disable/edition", base)),
        (Method::GET, format!("{}/process/application/app-1/edition/v1.0", base)),
        (Method::GET, format!("{}/process/upgrade/all", base)),
        (Method::GET, format!("{}/process/pr-1/disable", base)),
        (Method::GET, format!("{}/process/pr-1/enabled", base)),
        (Method::POST, format!("{}/process/pr-1/execute/projection", base)),
        (Method::GET, format!("{}/process/pr-1/lead/out", base)),
        (Method::POST, format!("{}/process/pr-1/list/element", base)),
        (Method::POST, format!("{}/process/pr-1/permission", base)),
        (Method::GET, format!("{}/process/pr-1/process", base)),
        (Method::POST, format!("{}/process/pr-1/upgrade", base)),
        (Method::DELETE, format!("{}/process/pr-1/false/edition", base)),
    ] {
        let status = designer_u2_status(method.clone(), &uri, None).await;
        assert_ne!(status, StatusCode::NOT_FOUND, "process 端点应注册: {}", uri);
    }
}

#[tokio::test]
async fn test_designer_script_by_name_and_workcompleted_merge_registered() {
    let base = "/jaxrs/processplatform/assemble/designer";
    assert_ne!(
        designer_u2_status(Method::GET, &format!("{}/script/application/app-1/name/init", base), None).await,
        StatusCode::NOT_FOUND,
        "script/application/{{id}}/name/{{name}} Java 精确形态"
    );
    assert_ne!(
        designer_u2_status(Method::GET, &format!("{}/workcompleted/application/app-1/merge/data", base), None).await,
        StatusCode::NOT_FOUND
    );
    assert_ne!(
        designer_u2_status(Method::GET, &format!("{}/workcompleted/process/pr-1/merge/data", base), None).await,
        StatusCode::NOT_FOUND
    );
}
