#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::response::ActionResult;
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use serde_json::json;
    use shared::testing::{is_db_available, test_pool};
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

#[test]
fn test_create_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "id": "proc-1",
        "name": "My Process",
        "status": "draft"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
}

#[test]
fn test_get_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "proc-1",
        "name": "Process Flow",
        "status": "active"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "proc-1");
}

#[test]
fn test_list_processes_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "proc-1", "status": "active"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_cancel_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "proc-1",
        "cancelled": true
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["cancelled"], true);
}

#[tokio::test]
async fn test_create_process_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "name": "My Process",
        "description": "A new process",
        "category": "default"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/create")
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
async fn test_get_process_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/get/proc-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_processes_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/list/default")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_execute_process_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/execute/proc-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_cancel_process_instance_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/cancel/proc-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_process_instance_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/instance/proc-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
    #[tokio::test]
    async fn test_get_jaxrs_processplatform_service_processing() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/get/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_processplatform_service_processing() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/cancel/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_work_id_retract() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/retract")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_work_id_terminate() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/terminate")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_put_jaxrs_work_id_processing() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/processing")
                    .method(Method::PUT)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_work_start_status_transition_db_connected() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_work_start_status_transition: DATABASE_URL not reachable");
            return;
        }

        let pool = test_pool();
        let work_id = "work-it-start-test";
        let client = pool.get().await.ok();

        // Seed a work record with pending status
        if let Some(c) = &client {
            let _ = c
                .execute(
                    "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time, start_time) \
                     VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW()) \
                     ON CONFLICT (id) DO UPDATE SET work_status = EXCLUDED.work_status",
                    &[&work_id, &"Test Work Start", &"default", &"default", &"pending", &"system"],
                )
                .await;
        }

        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/jaxrs/work/{}/start", work_id))
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["status"], "processing");
    }
}

// ═══════════════════════════════════════════════════════════════════
// plan002 U2：Java jaxrs 契约端点（u2 模块）行为测试
//
// 这些测试编码业务意图而非仅行为：
//  1. 归一化查重——同主键/同内容重复创建不产生新行（review/read/record/attachment/documentversion）；
//  2. IDOR 门禁——跨引用参数不匹配归属时必须拒绝而非静默删除（attachment/taskcompleted）；
//  3. 状态机事务性——read processing 迁移、snap suspend、task expire、v3 retract 均在事务内迁移状态；
//  4. JSONB 路径寻址——applicationdict/data 的 create/update/delete 遵循 Java 异常语义。
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod u2_contract {
    use super::*;
    use shared::testing::is_db_available;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::Pool;
    use serde_json::{json, Value};
    use shared::response::ActionResult;
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    /// 与 migration 079/020 等价的幂等 DDL 子集（测试自举，不污染其他用例）
    async fn ensure_schema(pool: &Pool) {
        let c = pool.get().await.unwrap();
        let ddl = [
            "CREATE TABLE IF NOT EXISTS x_application_dict (id VARCHAR(255) PRIMARY KEY, name VARCHAR(255), category VARCHAR(255), data JSONB DEFAULT '{}'::jsonb, creator VARCHAR(255), create_time TIMESTAMP DEFAULT NOW(), update_time TIMESTAMP DEFAULT NOW())",
            "CREATE TABLE IF NOT EXISTS x_data (scope VARCHAR(50) NOT NULL, bundle VARCHAR(255) NOT NULL, data JSONB DEFAULT '{}'::jsonb, create_time TIMESTAMP DEFAULT NOW(), update_time TIMESTAMP DEFAULT NOW(), PRIMARY KEY (scope, bundle))",
            "CREATE TABLE IF NOT EXISTS x_work (id VARCHAR(255) PRIMARY KEY, title VARCHAR(500) NOT NULL DEFAULT '', process VARCHAR(255) NOT NULL DEFAULT '', application VARCHAR(255), work_status VARCHAR(50) DEFAULT 'pending', creator VARCHAR(255), create_time TIMESTAMP DEFAULT NOW(), start_time TIMESTAMP, end_time TIMESTAMP, deleted_at TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS x_task (id VARCHAR(255) PRIMARY KEY, title VARCHAR(500), work VARCHAR(255) NOT NULL DEFAULT '', activity VARCHAR(255), activity_token VARCHAR(255), person VARCHAR(255), start_time TIMESTAMP, end_time TIMESTAMP, task_status VARCHAR(50) DEFAULT 'pending', create_time TIMESTAMP DEFAULT NOW(), deleted_at TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS x_review (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', reviewer VARCHAR(255), comment TEXT, status VARCHAR(50) DEFAULT 'pending', create_time TIMESTAMP DEFAULT NOW(), deleted_at TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS x_snap (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', snap_type VARCHAR(50) NOT NULL DEFAULT 'snap', snap_data JSONB, create_time TIMESTAMP DEFAULT NOW())",
            "CREATE TABLE IF NOT EXISTS x_record (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255), task_id VARCHAR(255), record_type VARCHAR(50), content TEXT, creator VARCHAR(255), create_time TIMESTAMP DEFAULT NOW())",
            "CREATE TABLE IF NOT EXISTS x_workcompleted (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', completed_time TIMESTAMP DEFAULT NOW(), creator VARCHAR(255), create_time TIMESTAMP DEFAULT NOW())",
            "CREATE TABLE IF NOT EXISTS x_draft (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', content JSONB, creator VARCHAR(255), create_time TIMESTAMP DEFAULT NOW(), deleted_at TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS x_read (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', person VARCHAR(255) NOT NULL DEFAULT '', scope VARCHAR(50) NOT NULL DEFAULT 'work', read_time TIMESTAMP DEFAULT NOW(), deleted_at TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS x_readcompleted (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', person VARCHAR(255) NOT NULL DEFAULT '', completed_time TIMESTAMP DEFAULT NOW())",
            "CREATE TABLE IF NOT EXISTS x_attachment (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255), workcompleted_id VARCHAR(255), name VARCHAR(255), content TEXT, creator VARCHAR(255), create_time TIMESTAMP DEFAULT NOW(), deleted_at TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS x_document_version (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', version INTEGER DEFAULT 1, content JSONB, creator VARCHAR(255), create_time TIMESTAMP DEFAULT NOW())",
            "CREATE TABLE IF NOT EXISTS x_job (id VARCHAR(255) PRIMARY KEY, work_id VARCHAR(255) NOT NULL DEFAULT '', person VARCHAR(255) NOT NULL DEFAULT '', activity_token VARCHAR(255), job_status VARCHAR(50) DEFAULT 'pending', create_time TIMESTAMP DEFAULT NOW(), start_time TIMESTAMP, end_time TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS x_process_definition (id VARCHAR(255) PRIMARY KEY, name VARCHAR(255) NOT NULL DEFAULT '', category VARCHAR(255), process_definition TEXT, version INTEGER DEFAULT 1, creator VARCHAR(255) DEFAULT 'system', status VARCHAR(50) DEFAULT 'disabled', create_time TIMESTAMP DEFAULT NOW(), update_time TIMESTAMP DEFAULT NOW())",
        ];
        for sql in ddl {
            c.execute(sql, &[]).await.unwrap();
        }
        c.execute("ALTER TABLE x_read ADD COLUMN IF NOT EXISTS scope VARCHAR(50) NOT NULL DEFAULT 'work'", &[]).await.unwrap();
        c.execute("ALTER TABLE x_task ADD COLUMN IF NOT EXISTS next_task_identity VARCHAR(255)", &[]).await.unwrap();
    }

    fn app() -> axum::Router {
        crate::router(test_pool())
    }

    async fn send(method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
        let req = match body {
            Some(v) => Request::builder()
                .uri(uri)
                .method(method)
                .header("content-type", "application/json")
                .body(Body::from(v.to_string()))
                .unwrap(),
            None => Request::builder().uri(uri).method(method).body(Body::empty()).unwrap(),
        };
        let resp = app().oneshot(req).await.unwrap();
        let status = resp.status();
        let raw = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap().to_vec();
        assert_eq!(status, StatusCode::OK, "business errors are HTTP 200 per ActionResult contract; body={}", String::from_utf8_lossy(&raw));
        let v: Value = serde_json::from_slice(&raw).unwrap();
        // 9-field envelope contract: type/data/message must exist at top level
        assert!(v.get("type").is_some(), "missing envelope field: type");
        assert!(v.get("message").is_some(), "missing envelope field: message");
        assert!(v.get("date").is_some(), "missing envelope field: date");
        (status, v)
    }

    async fn count(pool: &Pool, sql: &str) -> i64 {
        let c = pool.get().await.unwrap();
        c.query_one(sql, &[]).await.unwrap().get::<_, i64>("c")
    }



    #[tokio::test]
    async fn u2_dict_path_set_then_delete_roundtrip() {
        if !is_db_available().await {
            eprintln!("skipping u2_dict_path_set_then_delete_roundtrip: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_dict_path_set_then_delete_roundtrip: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        let base = "/jaxrs/processplatform/service/processing/applicationdict";
        let body = json!({"k1": {"k2": "v"}});
        let (_, v) = send(Method::PUT, &format!("{base}/dic-rt/p0/data"), Some(body)).await;
        assert_eq!(v["type"], "success");
        assert_eq!(v["data"]["value"]["p0"]["k1"]["k2"], "v", "\u{8def}\u{5f84}\u{5bfb}\u{5740}\u{5199}\u{5165}\u{5e94}\u{6302}\u{5728} p0 \u{952e}\u{4e0b}");
        let (_, v) = send(Method::PUT, &format!("{base}/dic-rt"), Some(json!({"name": "n"}))).await;
        assert_eq!(v["data"]["name"], "n");
        let (_, v) = send(Method::DELETE, &format!("{base}/dic-rt/p0/data"), None).await;
        assert_eq!(v["type"], "success");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_application_dict WHERE id='dic-rt' AND data::text LIKE '%p0%' OR data::text LIKE '%k1%'").await;
        assert_eq!(n, 0, "删除后键 k1 不得残留");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_application_dict WHERE id='dic-rt'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_data_work_create_then_update_conflict_semantics() {
        if !is_db_available().await {
            eprintln!("skipping u2_data_work_create_then_update_conflict_semantics: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_data_work_create_then_update_conflict_semantics: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_data WHERE scope='work' AND bundle='dw-1'", &[]).await.unwrap();
            c.execute("DELETE FROM x_work WHERE id='dw-1'", &[]).await.unwrap();
            c.execute("INSERT INTO x_work (id,title,process,creator) VALUES ('dw-1','t','p','system')", &[]).await.unwrap();
        }
        let base = "/jaxrs/processplatform/service/processing/data/work/dw-1";
        let (_, v) = send(Method::POST, base, Some(json!({"a":1}))).await;
        assert_eq!(v["type"], "success");
        let (_, v) = send(Method::POST, base, Some(json!({"a":2}))).await;
        assert_eq!(v["type"], "error", "Java ExceptionDataAlreadyExist 语义");
        let (_, v) = send(Method::POST, &format!("{base}/b"), Some(json!(true))).await;
        assert_eq!(v["type"], "success", "新键创建应成功");
        let (_, v) = send(Method::PUT, &format!("{base}/a"), Some(json!(9))).await;
        assert_eq!(v["data"]["value"]["a"], 9);
        let (_, v) = send(Method::POST, &format!("{base}/delete"), None).await;
        assert_eq!(v["type"], "success");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_data WHERE scope='work' AND bundle='dw-1'").await;
        assert_eq!(n, 0);
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_work WHERE id='dw-1'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_attachment_copy_dedups_by_name() {
        if !is_db_available().await {
            eprintln!("skipping u2_attachment_copy_dedups_by_name: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_attachment_copy_dedups_by_name: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_attachment WHERE work_id IN ('att-src-w','att-dst-w') OR id LIKE 'att-copy-%'", &[]).await.unwrap();
            for w in ["att-src-w", "att-dst-w"] {
                c.execute("DELETE FROM x_work WHERE id=$1", &[&w]).await.unwrap();
                c.execute("INSERT INTO x_work (id,title,process,creator) VALUES ($1,'t','p','system')", &[&w]).await.unwrap();
            }
            c.execute("INSERT INTO x_attachment (id, work_id, name) VALUES ('att-src-a','att-src-w','doc.pdf')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/attachment/copy/work/att-dst-w";
        let body = json!({"attachmentList": ["att-src-a"]});
        let (_, v1) = send(Method::POST, url, Some(body.clone())).await;
        assert_eq!(v1["data"]["successList"][0]["copied"], true);
        let (_, v2) = send(Method::POST, url, Some(body)).await;
        assert_eq!(v2["data"]["successList"][0]["copied"], false);
        assert_eq!(v2["data"]["successList"][0]["reason"], "already exist");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_attachment WHERE work_id='att-dst-w' AND name='doc.pdf'").await;
        assert_eq!(n, 1, "目标 work 下同名附件不得重复");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_attachment WHERE work_id IN ('att-src-w','att-dst-w')", &[]).await.unwrap();
        c.execute("DELETE FROM x_work WHERE id IN ('att-src-w','att-dst-w')", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_attachment_delete_with_wrong_work_is_idor_rejected() {
        if !is_db_available().await {
            eprintln!("skipping u2_attachment_delete_with_wrong_work_is_idor_rejected: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_attachment_delete_with_wrong_work_is_idor_rejected: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_attachment WHERE id='att-x'", &[]).await.unwrap();
            c.execute("INSERT INTO x_attachment (id, work_id, name) VALUES ('att-x','w-owner','f.txt')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/attachment/att-x/work/w-other";
        let (_, v) = send(Method::DELETE, url, None).await;
        assert_eq!(v["type"], "error", "附件不属于该 work 时必须拒绝");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_attachment WHERE id='att-x' AND deleted_at IS NULL").await;
        assert_eq!(n, 1, "被拒绝的删除不得落盘");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_attachment WHERE id='att-x'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_review_create_dedups_same_reviewer_per_work() {
        if !is_db_available().await {
            eprintln!("skipping u2_review_create_dedups_same_reviewer_per_work: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_review_create_dedups_same_reviewer_per_work: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_review WHERE work_id='rev-w'", &[]).await.unwrap();
            c.execute("DELETE FROM x_work WHERE id='rev-w'", &[]).await.unwrap();
            c.execute("INSERT INTO x_work (id,title,process,creator) VALUES ('rev-w','t','p','system')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/review/create/work";
        let body = json!({"work": "rev-w", "personList": ["alice@P"]});
        let (_, v1) = send(Method::POST, url, Some(body.clone())).await;
        assert_eq!(v1["data"]["successList"].as_array().unwrap().len(), 1);
        let (_, v2) = send(Method::POST, url, Some(body)).await;
        assert_eq!(v2["data"]["failureList"][0]["reason"], "already exist");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_review WHERE work_id='rev-w' AND reviewer='alice@P'").await;
        assert_eq!(n, 1, "同 (work,reviewer) 不得重复建评");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_review WHERE work_id='rev-w'", &[]).await.unwrap();
        c.execute("DELETE FROM x_work WHERE id='rev-w'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_read_processing_moves_row_into_completed_exactly_once() {
        if !is_db_available().await {
            eprintln!("skipping u2_read_processing_moves_row_into_completed_exactly_once: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_read_processing_moves_row_into_completed_exactly_once: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_read WHERE id='read-mv'", &[]).await.unwrap();
            c.execute("DELETE FROM x_readcompleted WHERE work_id='read-w' AND person='bob@P'", &[]).await.unwrap();
            c.execute("INSERT INTO x_read (id, work_id, person) VALUES ('read-mv','read-w','bob@P')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/read/read-mv/processing";
        let (_, v) = send(Method::PUT, url, None).await;
        assert_eq!(v["type"], "success");
        assert!(v["data"]["readCompletedId"].as_str().is_some());
        let active = count(&pool, "SELECT COUNT(*) AS c FROM x_read WHERE id='read-mv' AND deleted_at IS NULL").await;
        assert_eq!(active, 0, "已读后源记录必须退出待阅");
        let done = count(&pool, "SELECT COUNT(*) AS c FROM x_readcompleted WHERE work_id='read-w' AND person='bob@P'").await;
        assert_eq!(done, 1);
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_read WHERE id='read-mv'", &[]).await.unwrap();
        c.execute("DELETE FROM x_readcompleted WHERE work_id='read-w' AND person='bob@P'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_record_create_is_content_deduped() {
        if !is_db_available().await {
            eprintln!("skipping u2_record_create_is_content_deduped: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_record_create_is_content_deduped: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_record WHERE work_id='rec-job' AND record_type='info'", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/record/job/rec-job";
        let body = json!({"recordType": "Info", "content": 424242});
        let (_, v1) = send(Method::POST, url, Some(body.clone())).await;
        assert_ne!(v1["data"]["duplicated"], true);
        let (_, v2) = send(Method::POST, url, Some(body)).await;
        assert_eq!(v2["data"]["duplicated"], true, "同内容重复提交必须去重");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_record WHERE work_id='rec-job' AND record_type='info' AND content = '424242'").await;
        assert_eq!(n, 1);
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_record WHERE work_id='rec-job' AND record_type='info'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_documentversion_versions_increment_monotonically() {
        if !is_db_available().await {
            eprintln!("skipping u2_documentversion_versions_increment_monotonically: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_documentversion_versions_increment_monotonically: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_document_version WHERE work_id='dv-w'", &[]).await.unwrap();
            c.execute("DELETE FROM x_work WHERE id='dv-w'", &[]).await.unwrap();
            c.execute("INSERT INTO x_work (id,title,process,creator) VALUES ('dv-w','t','p','system')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/documentversion/work/dv-w";
        let (_, v1) = send(Method::POST, url, Some(json!({}))).await;
        let (_, v2) = send(Method::POST, url, Some(json!({}))).await;
        let s1 = v1["data"]["version"].as_i64().unwrap();
        let s2 = v2["data"]["version"].as_i64().unwrap();
        assert_eq!(s2, s1 + 1, "版本号必须严格递增");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_document_version WHERE work_id='dv-w'", &[]).await.unwrap();
        c.execute("DELETE FROM x_work WHERE id='dv-w'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_snap_suspend_snapshots_and_sets_status() {
        if !is_db_available().await {
            eprintln!("skipping u2_snap_suspend_snapshots_and_sets_status: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_snap_suspend_snapshots_and_sets_status: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_snap WHERE work_id='sus-w'", &[]).await.unwrap();
            c.execute("DELETE FROM x_work WHERE id='sus-w'", &[]).await.unwrap();
            c.execute("INSERT INTO x_work (id,title,process,work_status,creator) VALUES ('sus-w','t','p','processing','system')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/snap/work/sus-w/type/suspend";
        let (_, v) = send(Method::GET, url, None).await;
        assert_eq!(v["type"], "success");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_snap WHERE work_id='sus-w' AND snap_type='suspend'").await;
        assert_eq!(n, 1, "挂起必须产生快照");
        let st: String = {
            let c = pool.get().await.unwrap();
            c.query_one("SELECT work_status FROM x_work WHERE id='sus-w'", &[]).await.unwrap().get("work_status")
        };
        assert_eq!(st, "suspended");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_snap WHERE work_id='sus-w'", &[]).await.unwrap();
        c.execute("DELETE FROM x_work WHERE id='sus-w'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_task_expire_contract_route_updates_state() {
        if !is_db_available().await {
            eprintln!("skipping u2_task_expire_contract_route_updates_state: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_task_expire_contract_route_updates_state: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_task WHERE id='tk-exp'", &[]).await.unwrap();
            c.execute("INSERT INTO x_task (id,title,work,person,task_status) VALUES ('tk-exp','t','tk-work','p@P','active')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/task/tk-exp/expire";
        let (_, v) = send(Method::GET, url, None).await;
        assert_eq!(v["type"], "success");
        let st: String = {
            let c = pool.get().await.unwrap();
            c.query_one("SELECT task_status FROM x_task WHERE id='tk-exp'", &[]).await.unwrap().get("task_status")
        };
        assert_eq!(st, "expired", "GET /task/{{id}}/expire 契约形状必须真实迁移状态");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_task WHERE id='tk-exp'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_taskcompleted_press_cross_work_rejected_then_dedup() {
        if !is_db_available().await {
            eprintln!("skipping u2_taskcompleted_press_cross_work_rejected_then_dedup: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_taskcompleted_press_cross_work_rejected_then_dedup: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_readcompleted WHERE id='tc-press'", &[]).await.unwrap();
            c.execute("DELETE FROM x_record WHERE work_id='tc-w' AND record_type='press'", &[]).await.unwrap();
            c.execute("INSERT INTO x_readcompleted (id, work_id, person) VALUES ('tc-press','tc-w','p@P')", &[]).await.unwrap();
        }
        let wrong = "/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-other";
        let (_, v) = send(Method::GET, wrong, None).await;
        assert_eq!(v["type"], "error", "IDOR：记录不属于该 work 时必须拒绝催办");
        let right = "/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-w";
        let (_, v1) = send(Method::GET, right, None).await;
        assert_ne!(v1["data"]["duplicated"], true);
        let (_, v2) = send(Method::GET, right, None).await;
        assert_eq!(v2["data"]["duplicated"], true);
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_record WHERE work_id='tc-w' AND record_type='press'").await;
        assert_eq!(n, 1);
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_readcompleted WHERE id='tc-press'", &[]).await.unwrap();
        c.execute("DELETE FROM x_record WHERE work_id='tc-w' AND record_type='press'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_touch_clean_event_removes_only_stale_events() {
        if !is_db_available().await {
            eprintln!("skipping u2_touch_clean_event_removes_only_stale_events: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_touch_clean_event_removes_only_stale_events: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_record WHERE id IN ('ev-old','ev-new')", &[]).await.unwrap();
            c.execute("INSERT INTO x_record (id, work_id, record_type, content, create_time) VALUES ('ev-old','w','event','x', NOW() - INTERVAL '48 hours')", &[]).await.unwrap();
            c.execute("INSERT INTO x_record (id, work_id, record_type, content) VALUES ('ev-new','w','event','x')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/touch/cleanevent";
        let (_, v) = send(Method::GET, url, None).await;
        assert_eq!(v["type"], "success");
        let old_gone = count(&pool, "SELECT COUNT(*) AS c FROM x_record WHERE id='ev-old'").await;
        let new_kept = count(&pool, "SELECT COUNT(*) AS c FROM x_record WHERE id='ev-new'").await;
        assert_eq!(old_gone, 0, "超 24h 的陈旧事件必须清理");
        assert_eq!(new_kept, 1, "24h 内的事件不得误删");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_record WHERE id IN ('ev-old','ev-new')", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_work_v3_retract_cancels_active_tasks_transactionally() {
        if !is_db_available().await {
            eprintln!("skipping u2_work_v3_retract_cancels_active_tasks_transactionally: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_task WHERE work='ret-w'", &[]).await.unwrap();
            c.execute("DELETE FROM x_record WHERE work_id='ret-w' AND record_type='retract'", &[]).await.unwrap();
            c.execute("DELETE FROM x_work WHERE id='ret-w'", &[]).await.unwrap();
            c.execute("INSERT INTO x_work (id,title,process,creator) VALUES ('ret-w','t','p','system')", &[]).await.unwrap();
            c.execute("INSERT INTO x_task (id,title,work,person,task_status) VALUES ('ret-t1','t','ret-w','p@P','active')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/work/v3/retract";
        let (_, v) = send(Method::POST, url, Some(json!({"work": "ret-w"}))).await;
        assert_eq!(v["type"], "success");
        let cancelled = count(&pool, "SELECT COUNT(*) AS c FROM x_task WHERE work='ret-w' AND task_status='cancelled'").await;
        assert_eq!(cancelled, 1, "撤回必须取消活动任务");
        let rec = count(&pool, "SELECT COUNT(*) AS c FROM x_record WHERE work_id='ret-w' AND record_type='retract'").await;
        assert_eq!(rec, 1, "撤回必须留痕");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_task WHERE work='ret-w'", &[]).await.unwrap();
        c.execute("DELETE FROM x_record WHERE work_id='ret-w' AND record_type='retract'", &[]).await.unwrap();
        c.execute("DELETE FROM x_work WHERE id='ret-w'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_wc_merge_keeps_flag_collapses_duplicates() {
        if !is_db_available().await {
            eprintln!("skipping u2_wc_merge_keeps_flag_collapses_duplicates: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_wc_merge_keeps_flag_collapses_duplicates: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_workcompleted WHERE work_id='mrg-w'", &[]).await.unwrap();
            for id in ["mrg-keep", "mrg-drop"] {
                c.execute("INSERT INTO x_workcompleted (id, work_id) VALUES ($1,'mrg-w')", &[&id]).await.unwrap();
            }
        }
        let url = "/jaxrs/processplatform/service/processing/workcompleted/mrg-keep/merge";
        let (_, v) = send(Method::GET, url, None).await;
        assert_eq!(v["type"], "success");
        assert_eq!(v["data"]["merged"], 1);
        let kept = count(&pool, "SELECT COUNT(*) AS c FROM x_workcompleted WHERE id='mrg-keep'").await;
        let dropped = count(&pool, "SELECT COUNT(*) AS c FROM x_workcompleted WHERE id='mrg-drop'").await;
        assert_eq!((kept, dropped), (1, 0), "合并后仅保留 flag 指定项");
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_workcompleted WHERE work_id='mrg-w'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_taskcompleted_next_identity_updates_task() {
        if !is_db_available().await {
            eprintln!("skipping u2_taskcompleted_next_identity_updates_task: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_taskcompleted_next_identity_updates_task: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_task WHERE id='nti-t'", &[]).await.unwrap();
            c.execute("INSERT INTO x_task (id,title,work,person,task_status) VALUES ('nti-t','t','nti-w','p@P','active')", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/taskcompleted/next/task/identity";
        let (_, v) = send(Method::PUT, url, Some(json!({"id": "nti-t", "nextTaskIdentity": "manager@I"}))).await;
        assert_eq!(v["type"], "success");
        let ident: Option<String> = {
            let c = pool.get().await.unwrap();
            c.query_one("SELECT next_task_identity FROM x_task WHERE id='nti-t'", &[]).await.unwrap().get("next_task_identity")
        };
        assert_eq!(ident.as_deref(), Some("manager@I"));
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_task WHERE id='nti-t'", &[]).await.unwrap();
    }

    #[tokio::test]
    async fn u2_work_create_via_process_path_bootstraps_instance() {
        if !is_db_available().await {
            eprintln!("skipping u2_work_create_via_process_path_bootstraps_instance: DATABASE_URL not reachable");
            return;
        }
        if !is_db_available().await {
            eprintln!("skipping u2_work_create_via_process_path_bootstraps_instance: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        ensure_schema(&pool).await;
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_task WHERE work IN (SELECT id FROM x_work WHERE process='pd-boot')", &[]).await.unwrap();
            c.execute("DELETE FROM x_work WHERE process='pd-boot'", &[]).await.unwrap();
            c.execute("DELETE FROM x_process_definition WHERE id='pd-boot'", &[]).await.unwrap();
            c.execute("INSERT INTO x_process_definition (id,name) VALUES ('pd-boot','pd-boot-name') ON CONFLICT (id) DO NOTHING", &[]).await.unwrap();
        }
        let url = "/jaxrs/processplatform/service/processing/work/process/pd-boot/name/boot/serial";
        let (_, v) = send(Method::POST, url, Some(json!({}))).await;
        assert_eq!(v["type"], "success");
        assert!(v["data"]["taskId"].as_str().is_some(), "创建工作必须同步产生首个任务");
        let n = count(&pool, "SELECT COUNT(*) AS c FROM x_work WHERE process='pd-boot' AND title LIKE 'boot-%'").await;
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_task WHERE work IN (SELECT id FROM x_work WHERE process='pd-boot')", &[]).await.unwrap();
        c.execute("DELETE FROM x_work WHERE process='pd-boot'", &[]).await.unwrap();
        c.execute("DELETE FROM x_process_definition WHERE id='pd-boot'", &[]).await.unwrap();
    }
}
