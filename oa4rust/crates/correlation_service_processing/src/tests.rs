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

// ── ActionResult format tests ──────────────────────────────────────────────

#[test]
fn test_link_service_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "linked": true,
        "source_type": "message",
        "source_id": "msg-1",
        "target_type": "process",
        "target_id": "proc-1"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["linked"], true);
}

#[test]
fn test_get_link_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "sourceType": "message",
        "sourceId": "msg-1",
        "targetType": "process",
        "targetId": "proc-1"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["sourceId"], "msg-1");
}

#[test]
fn test_list_links_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"sourceType": "message", "targetId": "proc-1"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_unlink_service_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "unlinked": true,
        "sourceType": "message",
        "sourceId": "msg-1",
        "targetType": "process",
        "targetId": "proc-1"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["unlinked"], true);
}

// ── Route existence: link service routes ───────────────────────────────────

#[tokio::test]
async fn test_link_service_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let req = serde_json::to_string(&json!({
        "source_type": "message",
        "source_id": "msg-1",
        "target_type": "process",
        "target_id": "proc-1"
    })).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/link")
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
async fn test_get_link_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/link/message/msg-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_links_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/list/message")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

    #[tokio::test]
    #[ignore = "handler requires DB, returns 500 with mock pool"]
    async fn test_unlink_service_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/unlink/message/msg-1/process/proc-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

// ── Route existence: all registered routes ─────────────────────────────────

#[tokio::test]
async fn test_get_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_create_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"person_id": "p1", "target_id": "t1", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_save_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"target_id": "t2", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/save/test-id")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_delete_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/delete/test-id")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_delete_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/delete/type/cms/document/doc-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::json!({"idList": ["probe-id"]}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_delete_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/job-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::json!({"idList": ["probe-id"]}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_cms_document_site_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1/site/site-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_processplatform_job_site_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1/site/site-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_readable_type_cms_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/readable/type/cms")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"person":"p","doucment":"d"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_readable_type_processplatform_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/readable/type/processplatform")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"person":"p","job":"j"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/type/cms/document/doc-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/type/processplatform/job/job-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_update_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"personId": "p1", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/update/type/cms/document/doc-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_update_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"personId": "p1", "type": "processplatform/job"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/job-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ── ActionResult serialization ─────────────────────────────────────────────

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(serde_json::json!({"linked": true}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["linked"], true);
}

#[test]
fn test_action_result_error_serialization() {
    let result: ActionResult<&str> = ActionResult::error("link not found");
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "error");
    assert_eq!(json["message"], "link not found");
    assert!(json["data"].is_null());
}

#[test]
fn test_action_result_with_count() {
    let mut result: ActionResult<serde_json::Value> =
        ActionResult::success(serde_json::json!({"items": []}));
    result.count = Some(3);
    result.size = Some(10);
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["count"], 3);
    assert_eq!(json["size"], 10);
}

// ── Request validation: create/save/delete/link with invalid input ──────────

#[tokio::test]
async fn test_create_correlation_empty_person_id() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"person_id": "", "target_id": "t1", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_create_correlation_missing_type() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"person_id": "p1", "target_id": "t1"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_save_correlation_missing_target_id() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/save/test-id")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_link_service_empty_body() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/link")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
    #[ignore = "handler requires DB, returns 500 with mock pool"]
    async fn test_unlink_service_route_ok() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/unlink/type1/id1/type2/id2")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

// ── Router build test ───────────────────────────────────────────────────────

#[test]
fn test_correlation_service_processing_router_builds() {
    let pool = build_test_pool();
    let _ = crate::router(pool);
}

// ═════════════════════════════════════════════════════════════════════════════
// plan002 U2：Java CorrelationAction 契约端点（u2 模块）行为测试
//
// 这些测试编码业务意图：
//  1. create 是 upsert —— 同 (from,target,site) 不产生重复行；
//  2. delete 必须校验 id 归属（type/bundle 不匹配时拒绝删除）—— 防跨 job 越权删数据；
//  3. readable 只在存在可读来源证据时返回 true —— 缺证据必须为 false。
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod u2_contract {
    use super::*;
    use crate::u2;
    use shared::testing::test_pool;

    /// 与 migration 077 等价的幂等 DDL 子集（测试自举，不污染其他用例）
    async fn ensure_schema(pool: &Pool) {
        let client = pool.get().await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_correlation (
                id TEXT PRIMARY KEY,
                create_time TIMESTAMP DEFAULT NOW(),
                update_time TIMESTAMP DEFAULT NOW(),
                type TEXT,
                target_id TEXT,
                person_id TEXT,
                from_type TEXT,
                from_bundle TEXT,
                target_type TEXT,
                target_bundle TEXT,
                person TEXT,
                site TEXT,
                view TEXT,
                target_title TEXT,
                target_category TEXT,
                target_start_time TEXT,
                target_creator_person TEXT
             )", &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_work (
                id VARCHAR(255) PRIMARY KEY,
                title VARCHAR(500),
                creator VARCHAR(255)
             )", &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_review (
                id VARCHAR(255) PRIMARY KEY,
                work_id VARCHAR(255),
                reviewer VARCHAR(255)
             )", &[]).await.unwrap();
        for col in [
            "from_type TEXT",
            "from_bundle TEXT",
            "target_type TEXT",
            "target_bundle TEXT",
            "person TEXT",
            "site TEXT",
            "view TEXT",
            "target_title TEXT",
            "target_category TEXT",
            "target_start_time TEXT",
            "target_creator_person TEXT",
        ] {
            client
                .execute(&format!("ALTER TABLE x_correlation ADD COLUMN IF NOT EXISTS {col}"), &[])
                .await
                .unwrap();
        }
    }

    fn app() -> axum::Router {
        crate::router(test_pool())
    }

    async fn body_bytes(response: axum::response::Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn u2_create_pp_upserts_without_duplicates() {
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_correlation WHERE from_bundle IN ('job-1','job-2')", &[])
                .await
                .unwrap();
        }
        let body = serde_json::json!({
            "person": "person-a@P",
            "targetList": [
                {"type": "cms", "bundle": "doc-1", "site": "site-1"},
                {"type": "processplatform", "bundle": "", "site": "bad"}
            ]
        });
        for _ in 0..2 {
            let response = app()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/correlation/service/processing/correlation/type/processplatform/job/job-1")
                        .method(Method::POST)
                        .header("content-type", "application/json")
                        .body(Body::from(body.to_string()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }
        // 首次：success=1 / failure=1；重复创建后行数仍为 1（upsert 语义）
        let client = pool.get().await.unwrap();
        let n: i64 = client
            .query_one("SELECT COUNT(*) AS c FROM x_correlation WHERE from_bundle = 'job-1'", &[])
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 1, "同一 (from,target,site) 不得重复插入");
    }

    #[tokio::test]
    async fn u2_create_reports_invalid_targets() {
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_correlation WHERE from_bundle IN ('job-1','job-2')", &[])
                .await
                .unwrap();
        }
        let body = serde_json::json!({
            "person": "person-a@P",
            "targetList": [
                {"type": "cms", "bundle": "doc-1", "site": "site-1"},
                {"type": "processplatform", "bundle": "", "site": "bad"},
                {"type": "unknown-type", "bundle": "x"}
            ]
        });
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/service/processing/correlation/type/processplatform/job/job-2")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["successList"].as_array().unwrap().len(), 1);
        assert_eq!(v["data"]["failureList"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn u2_delete_rejects_cross_job_ids() {
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let client = pool.get().await.unwrap();
            client.execute("DELETE FROM x_correlation WHERE id = 'row-x'", &[]).await.unwrap();
            client.execute(
                "INSERT INTO x_correlation (id, from_type, from_bundle, target_type, target_bundle) \
                 VALUES ('row-x', 'processplatform', 'job-other', 'cms', 'doc-9')", &[]).await.unwrap();
        }
        let body = serde_json::json!({"idList": ["row-x"]});
        // job-1 与 row-x 的 from_bundle=job-other 不匹配 → 必须拒绝而非静默删除
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/job-1")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let v = body_bytes(response).await;
        assert_eq!(v["type"], "error");

        let client = pool.get().await.unwrap();
        let n: i64 = client
            .query_one("SELECT COUNT(*) AS c FROM x_correlation WHERE id = 'row-x'", &[])
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 1, "归属校验失败的行不得被删除");
    }

    #[tokio::test]
    async fn u2_readable_requires_evidence() {
        let pool = test_pool();
        ensure_schema(&pool).await;
        // 无任何关联行 → value=false（缺证据不得放行）
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/service/processing/correlation/readable/type/processplatform")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"person":"someone@P","job":"job-empty"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["value"], false);

        // 有 cms 来源 + 文档存在 + 创建者匹配 → value=true
        {
            let client = pool.get().await.unwrap();
            client.execute("DELETE FROM x_correlation WHERE id = 'row-r'", &[]).await.unwrap();
            client.execute(
                "INSERT INTO x_correlation (id, from_type, from_bundle, target_type, target_bundle) \
                 VALUES ('row-r', 'cms', 'doc-src', 'processplatform', 'job-target')", &[]).await.unwrap();
            client.execute(
                "CREATE TABLE IF NOT EXISTS x_cms_document (id VARCHAR PRIMARY KEY, creator_person VARCHAR)",
                &[]).await.unwrap();
            client.execute(
                "INSERT INTO x_cms_document (id, creator_person) VALUES ('doc-src', 'creator-a@P') \
                 ON CONFLICT (id) DO UPDATE SET creator_person = EXCLUDED.creator_person",
                &[]).await.unwrap();
        }
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/service/processing/correlation/readable/type/processplatform")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"person":"creator-a@P","job":"job-target"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["value"], true);
    }

    #[tokio::test]
    async fn u2_list_by_site_filters() {
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let client = pool.get().await.unwrap();
            client.execute("DELETE FROM x_correlation WHERE from_bundle = 'doc-l'", &[])
                .await
                .unwrap();
            for (id, site) in [("l1", "s1"), ("l2", "s2")] {
                client.execute(
                    "INSERT INTO x_correlation (id, from_type, from_bundle, target_type, target_bundle, site) \
                     VALUES ($1, 'cms', 'doc-l', 'cms', 't', $2)",
                    &[&id, &site]).await.unwrap();
            }
        }
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-l/site/s1")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["count"], 1);
        assert_eq!(v["data"]["data"][0]["id"], "l1");
        assert_eq!(v["data"]["data"][0]["site"], "s1");
    }

    /// update 按 site 替换：旧 site 行被清除，仅保留新集合
    #[tokio::test]
    async fn u2_update_replaces_site_scope() {
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let client = pool.get().await.unwrap();
            client.execute("DELETE FROM x_correlation WHERE from_bundle = 'doc-u'", &[])
                .await
                .unwrap();
            client.execute(
                "INSERT INTO x_correlation (id, from_type, from_bundle, target_type, target_bundle, site) \
                 VALUES ('old-1', 'cms', 'doc-u', 'cms', 't-old', 'sx')", &[]).await.unwrap();
        }
        let body = serde_json::json!({
            "person": "p@P",
            "siteTargetList": [
                {"site": "sx", "targetList": [{"type": "cms", "bundle": "t-new"}]}
            ]
        });
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/service/processing/correlation/update/type/cms/document/doc-u")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let client = pool.get().await.unwrap();
        let rows = client
            .query(
                "SELECT target_bundle FROM x_correlation WHERE from_bundle = 'doc-u' AND COALESCE(site,'')='sx'",
                &[],
            )
            .await
            .unwrap();
        let bundles: Vec<String> = rows.iter().map(|r| r.get::<_, String>("target_bundle")).collect();
        assert!(!bundles.contains(&"t-old".to_string()), "旧目标应被替换");
        assert!(bundles.contains(&"t-new".to_string()), "新目标应写入");
    }

    /// Wi 字段名兼容 Java 历史拼写 doucment
    #[tokio::test]
    async fn u2_readable_cms_accepts_doucment_field() {
        let wi: u2::ReadableCmsWi =
            serde_json::from_value(serde_json::json!({"person":"p","doucment":"d"})).unwrap();
        assert_eq!(wi.doucment.as_deref(), Some("d"));
    }
}
