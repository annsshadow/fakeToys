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
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/processplatform/assemble/designer/create")
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
                .uri("/api/processplatform/assemble/designer/get/flow-1")
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
                .uri("/api/processplatform/assemble/designer/list/processplatform")
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
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/processplatform/assemble/designer/save/flow-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    // 第五十七批：save_flow 收 admin 门后无会话返 401（原 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_flow_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/processplatform/assemble/designer/delete/flow-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // 第五十七批：delete_flow 收 admin 门后无会话返 401（原 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_preview_flow_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/processplatform/assemble/designer/preview/flow-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 新增：o2server 端点缺口闭合（51 端点）的测试
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
    // o2server PUT /application/{id}：此前仅有 GET，PUT 属主门禁端点必须可达
    let status = designer_u2_status(
        Method::PUT,
        "/api/processplatform/assemble/designer/application/app-1",
        Some("{\"name\":\"x\"}".into()),
    )
    .await;
    assert_ne!(
        status,
        StatusCode::NOT_FOUND,
        "PUT /application/{{id}} 应已注册"
    );
}

#[tokio::test]
async fn test_designer_put_application_icon_exact_shape() {
    // o2server 精确路径 /application/{id}/icon（区别于遗留 /application/icon/{id}）
    let status = designer_u2_status(
        Method::PUT,
        "/api/processplatform/assemble/designer/application/app-1/icon",
        Some("{\"icon\":\"mood\"}".into()),
    )
    .await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_designer_post_application_permission_registered() {
    let status = designer_u2_status(
        Method::POST,
        "/api/processplatform/assemble/designer/application/app-1/permission",
        Some("{\"view\":[\"xadmin\"]}".into()),
    )
    .await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_designer_applicationcategory_list_registered() {
    let status = designer_u2_status(
        Method::GET,
        "/api/processplatform/assemble/designer/applicationcategory/list",
        None,
    )
    .await;
    assert_ne!(
        status,
        StatusCode::NOT_FOUND,
        "applicationcategory/list handler 已存在但此前从未路由"
    );
}

#[tokio::test]
async fn test_designer_item_access_family_registered() {
    let base = "/api/processplatform/assemble/designer/item-access";
    for (method, uri) in [
        (Method::POST, base.to_string()),
        (Method::POST, format!("{}/bach/save", base)),
        (Method::GET, format!("{}/item-1", base)),
        (Method::GET, format!("{}/path/p1", base)),
        (Method::GET, format!("{}/process/pr1", base)),
        (
            Method::DELETE,
            format!("{}/delete/process/pr1/path/p1", base),
        ),
    ] {
        let status = designer_u2_status(
            method.clone(),
            &uri,
            if method == Method::POST {
                Some("{\"items\":[]}".into())
            } else {
                None
            },
        )
        .await;
        assert_ne!(
            status,
            StatusCode::NOT_FOUND,
            "item-access 端点应注册: {}",
            uri
        );
    }
}

#[tokio::test]
async fn test_designer_mapping_verbs_registered() {
    let status_get = designer_u2_status(
        Method::GET,
        "/api/processplatform/assemble/designer/mapping/m-1/execute",
        None,
    )
    .await;
    assert_ne!(
        status_get,
        StatusCode::NOT_FOUND,
        "/mapping/{{flag}}/execute o2server 精确形态"
    );
    let status_delete = designer_u2_status(
        Method::DELETE,
        "/api/processplatform/assemble/designer/mapping/m-1",
        None,
    )
    .await;
    assert_ne!(
        status_delete,
        StatusCode::NOT_FOUND,
        "DELETE /mapping/{{flag}} 应已补齐动词"
    );
}

#[tokio::test]
async fn test_designer_mergeitemplan_verbs_and_paging() {
    let base = "/api/processplatform/assemble/designer";
    assert_ne!(
        designer_u2_status(Method::GET, &format!("{}/mergeitemplan/mp-1", base), None).await,
        StatusCode::NOT_FOUND
    );
    assert_ne!(
        designer_u2_status(
            Method::GET,
            &format!("{}/mergeitemplan/list/paging/1/size/20", base),
            None
        )
        .await,
        StatusCode::NOT_FOUND,
        "分页精确形态 paging/{{page}}/size/{{size}}"
    );
    assert_ne!(
        designer_u2_status(
            Method::GET,
            &format!(
                "{}/mergeitemplan/list/application/app-1/paging/1/size/20",
                base
            ),
            None
        )
        .await,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn test_designer_process_edition_family_registered() {
    let base = "/api/processplatform/assemble/designer";
    for (method, uri) in [
        (
            Method::GET,
            format!("{}/process/application/app-1/disable/edition", base),
        ),
        (
            Method::GET,
            format!("{}/process/application/app-1/edition/v1.0", base),
        ),
        (Method::GET, format!("{}/process/upgrade/all", base)),
        (Method::GET, format!("{}/process/pr-1/disable", base)),
        (Method::GET, format!("{}/process/pr-1/enabled", base)),
        (
            Method::POST,
            format!("{}/process/pr-1/execute/projection", base),
        ),
        (Method::GET, format!("{}/process/pr-1/lead/out", base)),
        (Method::POST, format!("{}/process/pr-1/list/element", base)),
        (Method::POST, format!("{}/process/pr-1/permission", base)),
        (Method::GET, format!("{}/process/pr-1/process", base)),
        (Method::POST, format!("{}/process/pr-1/upgrade", base)),
        (
            Method::DELETE,
            format!("{}/process/pr-1/false/edition", base),
        ),
    ] {
        let status = designer_u2_status(method.clone(), &uri, None).await;
        assert_ne!(status, StatusCode::NOT_FOUND, "process 端点应注册: {}", uri);
    }
}

#[tokio::test]
async fn test_designer_script_by_name_and_workcompleted_merge_registered() {
    let base = "/api/processplatform/assemble/designer";
    assert_ne!(
        designer_u2_status(
            Method::GET,
            &format!("{}/script/application/app-1/name/init", base),
            None
        )
        .await,
        StatusCode::NOT_FOUND,
        "script/application/{{id}}/name/{{name}} o2server 精确形态"
    );
    assert_ne!(
        designer_u2_status(
            Method::GET,
            &format!("{}/workcompleted/application/app-1/merge/data", base),
            None
        )
        .await,
        StatusCode::NOT_FOUND
    );
    assert_ne!(
        designer_u2_status(
            Method::GET,
            &format!("{}/workcompleted/process/pr-1/merge/data", base),
            None
        )
        .await,
        StatusCode::NOT_FOUND
    );
}

// 回归（四轮审计 GET 写面队列 · pp_designer 3 把 process 生命周期写）：
// process_id_disable / process_id_enable / process_id_upgrade 按 id 改 PP_E_PROCESS，
// 无归属门 ⇒ 收 guard_write（进程属主或 admin，coalesce xcreatorPerson/creator_person）。
#[tokio::test]
async fn test_pp_process_writes_require_owner() {
    use axum::extract::Extension;
    if !shared::testing::is_db_available().await {
        eprintln!("skipping test_pp_process_writes_require_owner: DATABASE_URL not reachable");
        return;
    }
    let pool = shared::testing::test_pool();
    const OWNER: &str = "pp-gate-owner";
    const OTHER: &str = "pp-gate-other";
    const ADMIN: &str = "pp-gate-admin";
    const ROLE: &str = "pp-gate-role-admin";
    let sess = |p: &str| Extension(shared::testing::test_session(p));
    let pid = "pp-gate-1".to_string();

    // 造 admin + 归属进程行（xcreatorPerson=OWNER）。
    {
        let c = pool.get().await.unwrap();
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) \
             ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
            &[&ROLE.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1,$1,'pp gate admin','x') ON CONFLICT (id) DO NOTHING",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1,$2,'pp-gate-unit') ON CONFLICT DO NOTHING",
            &[&ADMIN.to_string(), &ROLE.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO pp_e_process (id, xid, xname, \"xcreatorPerson\", create_time) \
             VALUES ($1,$1,'p',$2,NOW()::text) ON CONFLICT (id) DO NOTHING",
            &[&pid, &OWNER.to_string()],
        )
        .await
        .unwrap();
    }

    // 非属主/非 admin → 403 且不落笔（xversion 不 bump）。
    assert!(
        matches!(
            crate::process_id_disable(Extension(pool.clone()), sess(OTHER), axum::extract::Path(pid.clone())).await,
            Err(shared::error::AppError::Forbidden)
        ),
        "disable 他人必须 403"
    );
    assert!(
        matches!(
            crate::process_id_enable(Extension(pool.clone()), sess(OTHER), axum::extract::Path(pid.clone())).await,
            Err(shared::error::AppError::Forbidden)
        ),
        "enable 他人必须 403"
    );
    assert!(
        matches!(
            crate::process_id_upgrade(Extension(pool.clone()), sess(OTHER), axum::extract::Path(pid.clone())).await,
            Err(shared::error::AppError::Forbidden)
        ),
        "upgrade 他人必须 403"
    );
    {
        let c = pool.get().await.unwrap();
        let v: i64 = c
            .query_one("SELECT COALESCE(xversion,0) FROM pp_e_process WHERE xid=$1", &[&pid])
            .await
            .unwrap()
            .get(0);
        assert_eq!(v, 0, "非属主 403 侧不得 bump xversion");
    }

    // 属主放行：disable + upgrade。
    assert!(
        crate::process_id_disable(Extension(pool.clone()), sess(OWNER), axum::extract::Path(pid.clone()))
            .await
            .is_ok(),
        "属主 disable 应放行"
    );
    assert!(
        crate::process_id_upgrade(Extension(pool.clone()), sess(OWNER), axum::extract::Path(pid.clone()))
            .await
            .is_ok(),
        "属主 upgrade 应放行"
    );
    {
        let c = pool.get().await.unwrap();
        let v: i64 = c
            .query_one("SELECT COALESCE(xversion,0) FROM pp_e_process WHERE xid=$1", &[&pid])
            .await
            .unwrap()
            .get(0);
        assert_eq!(v, 1, "属主 upgrade 应 bump xversion");
        let s: String = c
            .query_one("SELECT xstatus::text FROM pp_e_process WHERE xid=$1", &[&pid])
            .await
            .unwrap()
            .get(0);
        assert_eq!(s, "disabled", "属主 disable 应置 disabled");
    }

    // admin 放行：enable（admin 旁路）。
    assert!(
        crate::process_id_enable(Extension(pool.clone()), sess(ADMIN), axum::extract::Path(pid.clone()))
            .await
            .is_ok(),
        "admin enable 应放行（admin 旁路）"
    );
    {
        let c = pool.get().await.unwrap();
        let s: String = c
            .query_one("SELECT xstatus::text FROM pp_e_process WHERE xid=$1", &[&pid])
            .await
            .unwrap()
            .get(0);
        assert_eq!(s, "enabled", "admin enable 应置 enabled");
    }

    // 收口：清造数，零残留。
    let c = pool.get().await.unwrap();
    c.execute("DELETE FROM pp_e_process WHERE xid=$1", &[&pid])
        .await
        .unwrap();
    c.execute("DELETE FROM auth_person_role WHERE person_id=$1", &[&ADMIN.to_string()])
        .await
        .unwrap();
    c.execute("DELETE FROM auth_person WHERE id=$1", &[&ADMIN.to_string()])
        .await
        .unwrap();
    c.execute("DELETE FROM auth_role WHERE id=$1", &[&ROLE.to_string()])
        .await
        .unwrap();
}

/// 第 37 批回归臂：设计器元数据写（dict / form / xform 各 create/save/delete，共 9 把）
/// 收 admin 门（A3-N9）。离线：无会话 401（先于读库）；非 admin（mock pool fail-closed）403。
#[tokio::test]
async fn pp_designer_meta_writes_require_admin() {
    use shared::error::AppError;
    let dead = build_test_pool();
    let body = || axum::Json(json!({}));
    let path = || axum::extract::Path("x".to_string());

    // 无会话 → 九把全 401（门先于 crud 的 pool.get()）
    let unauth = [
        crate::dict_create(Extension(dead.clone()), None, body()).await,
        crate::dict_save(Extension(dead.clone()), None, path(), body()).await,
        crate::dict_delete(Extension(dead.clone()), None, path()).await,
        crate::form_create(Extension(dead.clone()), None, body()).await,
        crate::form_save(Extension(dead.clone()), None, path(), body()).await,
        crate::form_delete(Extension(dead.clone()), None, path()).await,
        crate::xform_create(Extension(dead.clone()), None, body()).await,
        crate::xform_save(Extension(dead.clone()), None, path(), body()).await,
        crate::xform_delete(Extension(dead.clone()), None, path()).await,
    ];
    for (i, r) in unauth.iter().enumerate() {
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "第 {i} 把（dict/form/xform 序）无会话必须 401"
        );
    }

    // 非 admin（mock pool 下 is_admin fail-closed）→ 九把全 403
    let s = || Some(Extension(shared::testing::test_session("u-ppd-nobody")));
    let forbidden = [
        crate::dict_create(Extension(dead.clone()), s(), body()).await,
        crate::dict_save(Extension(dead.clone()), s(), path(), body()).await,
        crate::dict_delete(Extension(dead.clone()), s(), path()).await,
        crate::form_create(Extension(dead.clone()), s(), body()).await,
        crate::form_save(Extension(dead.clone()), s(), path(), body()).await,
        crate::form_delete(Extension(dead.clone()), s(), path()).await,
        crate::xform_create(Extension(dead.clone()), s(), body()).await,
        crate::xform_save(Extension(dead.clone()), s(), path(), body()).await,
        crate::xform_delete(Extension(dead.clone()), s(), path()).await,
    ];
    for (i, r) in forbidden.iter().enumerate() {
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "第 {i} 把（dict/form/xform 序）非 admin 必须 403"
        );
    }
}
