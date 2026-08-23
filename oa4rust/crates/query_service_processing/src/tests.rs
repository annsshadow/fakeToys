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
fn test_process_query_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "q-1",
        "name": "Test Query",
        "queryType": "sql",
        "count": 10,
        "processed": true,
        "params": {}
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["processed"], true);
    assert_eq!(json["data"]["count"], 10);
}

#[test]
fn test_batch_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "total": 2,
        "results": [
            {"id": "q-1", "name": "Query 1", "queryType": "sql", "count": 5, "processed": true},
            {"id": "q-2", "name": "Query 2", "queryType": "rest", "count": 3, "processed": true}
        ]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["total"], 2);
}

#[test]
fn test_get_service_status_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "status": "running",
        "activeConnections": 1,
        "queuedRequests": 0,
        "processedCount": 10
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["status"], "running");
    assert!(json["data"]["activeConnections"].is_number());
    assert!(json["data"]["queuedRequests"].is_number());
}

#[test]
fn test_reset_service_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "reset": true,
        "resetAt": "2024-06-01T00:00:00Z",
        "clearedCache": true,
        "processedCount": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["reset"], true);
    assert!(json["data"]["resetAt"].is_string());
}

#[tokio::test]
async fn test_process_query_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "query_type": "sql",
        "params": {},
        "options": {}
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/process")
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
async fn test_batch_process_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "queries": [
            {"query_type": "sql", "params": {}},
            {"query_type": "rest", "params": {}}
        ]
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/batch")
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
async fn test_get_service_status_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/status")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_reset_service_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/reset")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}


// ═════════════════════════════════════════════════════════════════════════════
// plan002 U2：Java x_query_service_processing 契约端点（u2 模块）行为测试
//
// 这些测试编码业务意图：
//  1. touch/reset 是真实任务状态机 —— 状态落库可查，reset 清除错误并回到 idle；
//  2. neural 推算必须以「已完成学习」为前提 —— 缺学习记录时拒绝（对齐 Java
//     ExceptionModelNotReady），不得凭空给出计算结果；
//  3. 动态表行按 bundle 定位 upsert —— 同 bundle 反复更新不得产生重复行。
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod u2_contract {
    use super::*;
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    fn app() -> axum::Router {
        let pool = test_pool();
        crate::router(pool)
    }

    async fn client() -> deadpool_postgres::Client {
        test_pool().get().await.unwrap()
    }

    /// 与 migration 077 等价的幂等 DDL 子集（含既有表缺列补齐）
    async fn ensure_schema() {
        let c = client().await;
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_query_index_state (
                id VARCHAR(64) PRIMARY KEY,
                entity_type TEXT NOT NULL,
                freq TEXT,
                node TEXT NOT NULL DEFAULT '0',
                status TEXT NOT NULL DEFAULT 'idle',
                error_message TEXT,
                last_touch_time TIMESTAMP,
                last_reset_time TIMESTAMP,
                update_time TIMESTAMP NOT NULL DEFAULT NOW()
             )",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS uq_x_query_index_state \
             ON x_query_index_state (entity_type, freq, node)",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_query_index_extra (
                id VARCHAR(64) PRIMARY KEY,
                type TEXT NOT NULL,
                key TEXT NOT NULL,
                doc_id TEXT NOT NULL,
                data JSONB NOT NULL DEFAULT '{}'::jsonb,
                create_time TIMESTAMP NOT NULL DEFAULT NOW(),
                update_time TIMESTAMP NOT NULL DEFAULT NOW()
             )",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS uq_x_query_index_extra_doc \
             ON x_query_index_extra (type, key, doc_id)",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_query_neural_job (
                id VARCHAR(36) PRIMARY KEY,
                model_flag TEXT NOT NULL,
                action TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'running',
                message TEXT,
                create_time TIMESTAMP NOT NULL DEFAULT NOW(),
                update_time TIMESTAMP NOT NULL DEFAULT NOW()
             )",
            &[],
        )
        .await
        .unwrap();
        // 动态表数据行（既有表可能无 bundle 列）
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_query_table_data (
                id TEXT PRIMARY KEY,
                table_flag TEXT,
                data TEXT,
                create_time TIMESTAMP DEFAULT NOW(),
                update_time TIMESTAMP DEFAULT NOW()
             )",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "ALTER TABLE x_query_table_data ADD COLUMN IF NOT EXISTS bundle TEXT",
            &[],
        )
        .await
        .unwrap();
        // 设计搜索 / 文档计数依赖的基表（缺列补齐，保证查询不因 schema 缺口而失败）
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_cms_script (
                id VARCHAR(255) PRIMARY KEY,
                app_id VARCHAR(255),
                name VARCHAR(255),
                script_content TEXT
             )",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_cms_document (
                id VARCHAR PRIMARY KEY,
                creator_person VARCHAR
             )",
            &[],
        )
        .await
        .unwrap();
    }

    async fn get(router: axum::Router, path: &str) -> serde_json::Value {
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri(path)
                    .method("GET")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        serde_json::from_slice(&bytes).unwrap()
    }

    async fn post(router: axum::Router, path: &str, body: String) -> serde_json::Value {
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri(path)
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        assert_eq!(
            status,
            axum::http::StatusCode::OK,
            "POST {path} 应返回 200, body={}",
            String::from_utf8_lossy(&bytes)
        );
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn u2_touch_state_machine_persists() {
        ensure_schema().await;
        let v = get(app(), "/jaxrs/query/service/processing/touch/high/freq/work/node/u2node/touch").await;
        assert_eq!(v["data"]["status"], "touched");
        let c = client().await;
        let row = c
            .query_one(
                "SELECT status FROM x_query_index_state WHERE entity_type='work' AND freq='high' AND node='u2node'",
                &[],
            )
            .await
            .unwrap();
        assert_eq!(row.get::<_, String>("status"), "touched");

        let v = get(app(), "/jaxrs/query/service/processing/touch/high/freq/work/node/u2node/reset").await;
        assert_eq!(v["data"]["status"], "idle");
        let row = c
            .query_one(
                "SELECT status, (last_reset_time IS NOT NULL) AS has_reset \
                 FROM x_query_index_state WHERE entity_type='work' AND freq='high' AND node='u2node'",
                &[],
            )
            .await
            .unwrap();
        assert_eq!(row.get::<_, String>("status"), "idle");
        assert!(row.get::<_, bool>("has_reset"), "reset 应记录重置时间");
    }

    #[tokio::test]
    async fn u2_optimize_and_reload_markers() {
        ensure_schema().await;
        let v = get(app(), "/jaxrs/query/service/processing/touch/optimize/index/n0/touch").await;
        assert_eq!(v["data"]["value"], true);
        let c = client().await;
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) AS c FROM x_query_index_state WHERE entity_type='optimize' AND node='n0'",
                &[],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 1);

        let v = get(app(), "/jaxrs/query/service/processing/table/reload/dynamic").await;
        assert_eq!(v["type"], "success");
    }

    #[tokio::test]
    async fn u2_extra_document_validates_required_fields() {
        ensure_schema().await;
        let p = "/jaxrs/query/service/processing/index/update/extra/document";
        // 缺 type/key/id/createTime/updateTime 必须被拒（对齐 Java ExceptionEmptyField）
        let v = post(app(), p, r#"{"key":"k","id":"i"}"#.into()).await;
        assert_eq!(v["type"], "error");
        assert!(v["message"].as_str().unwrap().contains("required"));

        let v = post(app(), p, r#"{"type":"cms","key":"u2key","id":"doc-e","createTime":"2026-08-24 10:00:00","updateTime":"2026-08-24 10:00:00","title":"t"}"#.into()).await;
        assert_eq!(v["data"]["value"], true);
        let c = client().await;
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) AS c FROM x_query_index_extra WHERE type='cms' AND key='u2key' AND doc_id='doc-e'",
                &[],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 1, "附加文档应落库");
    }

    #[tokio::test]
    async fn u2_directory_count_reflects_documents() {
        ensure_schema().await;
        let v = post(
            app(),
            "/jaxrs/query/service/processing/index/directory/document/count",
            "{}".into(),
        )
        .await;
        // category/key 全空 → 返回文档库真实计数
        assert_eq!(v["data"]["exists"], v["data"]["count"].as_i64().unwrap() > 0);
        // 带 category 过滤时（非全量口径）→ 计数 0 且 exists=false
        let v = post(
            app(),
            "/jaxrs/query/service/processing/index/directory/document/count",
            r#"{"category":"custom","key":"x"}"#.into(),
        )
        .await;
        assert_eq!(v["data"]["count"], 0);
        assert_eq!(v["data"]["exists"], false);
    }

    #[tokio::test]
    async fn u2_design_search_finds_seeded_script() {
        ensure_schema().await;
        {
            let c = client().await;
            c.execute("DELETE FROM x_cms_script WHERE name LIKE 'U2SEARCH%'", &[])
                .await
                .unwrap();
            c.execute(
                "INSERT INTO x_cms_script (id, app_id, name, script_content) \
                 VALUES ('u2-script-1', 'app-u2', 'U2SEARCH目标脚本', 'print(1)')",
                &[],
            )
            .await
            .unwrap();
        }
        let v = post(
            app(),
            "/jaxrs/query/service/processing/design/search",
            r#"{"keyword":"U2SEARCH目标","moduleList":[{"moduleType":"cms"}]}"#.into(),
        )
        .await;
        assert_eq!(v["type"], "success");
        let items = v["data"]["data"]["cms"].as_array().unwrap();
        assert!(
            items.iter().any(|i| i["id"] == "u2-script-1"),
            "应能在 cms 模块中检索到目标脚本"
        );
    }

    #[tokio::test]
    async fn u2_design_search_requires_keyword() {
        ensure_schema().await;
        let v = post(
            app(),
            "/jaxrs/query/service/processing/design/search",
            r#"{"keyword":"   ","moduleList":[]}"#.into(),
        )
        .await;
        assert_eq!(v["type"], "error");
    }

    #[tokio::test]
    async fn u2_neural_calculate_requires_completed_learn() {
        ensure_schema().await;
        let c = client().await;
        c.execute("DELETE FROM x_query_neural_job WHERE model_flag = 'u2model'", &[])
            .await
            .unwrap();

        let base = "/jaxrs/query/service/processing/neural";
        // 未学习：推算必须拒绝（对齐 Java ExceptionModelNotReady）
        let v = get(app(), &format!("{base}/list/calculate/model/u2model/work/w1")).await;
        assert_eq!(v["type"], "error");
        assert!(v["message"].as_str().unwrap().contains("not ready"));

        // 学习 → 可停止；停止后仍无 completed 学习 → 推算依旧拒绝
        let v = get(app(), &format!("{base}/learn/model/u2model")).await;
        assert_eq!(v["data"]["status"], "running");
        let v = get(app(), &format!("{base}/stop/learning/model/u2model")).await;
        assert_eq!(v["data"]["stopped"], 1);
        let v = get(app(), &format!("{base}/stop/learning/model/u2model")).await;
        assert_eq!(v["type"], "error", "无可停止任务时应报错");

        // 落一条 completed 学习记录 → 推算放行
        c.execute(
            "INSERT INTO x_query_neural_job (id, model_flag, action, status) \
             VALUES ('u2-job-done', 'u2model', 'learn', 'completed')",
            &[],
        )
        .await
        .unwrap();
        let v = get(app(), &format!("{base}/list/calculate/model/u2model/work/w1")).await;
        assert_eq!(v["type"], "success");
        assert_eq!(v["data"]["workId"], "w1");
    }

    #[tokio::test]
    async fn u2_generate_then_stop_generating() {
        ensure_schema().await;
        let c = client().await;
        c.execute("DELETE FROM x_query_neural_job WHERE model_flag = 'u2gen'", &[])
            .await
            .unwrap();
        let base = "/jaxrs/query/service/processing/neural";
        let v = get(app(), &format!("{base}/generate/model/u2gen")).await;
        assert_eq!(v["data"]["action"], "generate");
        // 幂等启动：重复 generate 不新增运行中任务
        let _ = get(app(), &format!("{base}/generate/model/u2gen")).await;
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) AS c FROM x_query_neural_job WHERE model_flag='u2gen' AND status='running'",
                &[],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 1);
        let v = get(app(), &format!("{base}/stop/generating/model/u2gen")).await;
        assert_eq!(v["data"]["value"], true);
    }

    #[tokio::test]
    async fn u2_table_row_bundle_upsert_roundtrip() {
        ensure_schema().await;
        let c = client().await;
        c.execute(
            "DELETE FROM x_query_table_data WHERE table_flag = 'u2table'",
            &[],
        )
        .await
        .unwrap();

        let ins = "/jaxrs/query/service/processing/table/u2table/insert";
        let v = post(app(), ins, r#"{"name":"row-a"}"#.into()).await;
        let bundle = v["data"]["bundle"].as_str().unwrap().to_string();

        let upd = "/jaxrs/query/service/processing/table/u2table/update";
        for round in 0..3 {
            let v = post(
                app(),
                &format!("{upd}/{bundle}"),
                format!(r#"{{"name":"row-a","round":{round}}}"#),
            )
            .await;
            assert_eq!(v["data"]["updated"], true);
            let n: i64 = c
                .query_one(
                    "SELECT COUNT(*) AS c FROM x_query_table_data WHERE table_flag='u2table' AND bundle=$1",
                    &[&bundle],
                )
                .await
                .unwrap()
                .get("c");
            assert_eq!(n, 1, "同 bundle 更新不得产生重复行");
        }
    }
}
