// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::{TestPool, TEST_DB};

// ──────────────────────────────────────────────────────────────────────────────
// BPMN / workflow execution semantics (U2.2 core)
//
// O20 词表修正（B）：原先这些用例打的是 `/work/create`、`/task/{id}/agree|
// disagree|return` 与 `/execute/{id}`，全仓既无这些路由，前端 ProcessWork.vue
// 也从未调用（它用的是下面的 canonical 那套）。证据见台账 O20 节。
//
// 本文件改打前端真实调用面：
//   POST /api/processplatform/service/processing/work   → 建 work + 首个任务
//   PUT  /api/work/{id}/processing                       → work pending → processing
//   POST /api/task/{id}/claim                            → pending → active
//   POST /api/task/{id}/complete                         → active → completed
//   POST /api/task/{id}/reject                           → active → rejected
//   POST /api/task/{id}/transfer/{person}                → 改派负责人
//   GET  /api/work/{id}                                  → work + tasks
//
// 会签（agree/disagree/return/countersign）无消费者、无文档承诺、无实现，
// 原 `bpmn_countersign_flow` 已按台账 O20 的 C 类删除（能力缺口，已确认无人使用）。
// ──────────────────────────────────────────────────────────────────────────────

/// 种一条可用的流程定义；work_create 会校验 `id OR name` 存在。
async fn seed_process(pool: &TestPool, process_id: &str) -> bool {
    let client = pool
        .as_pg()
        .expect("pg pool")
        .get()
        .await
        .expect("pg client");
    let affected = client
        .execute(
            "INSERT INTO x_process_definition (id, name, category, status, creator) \
             VALUES ($1, $2, 'integration', 'enabled', 'it-admin') \
             ON CONFLICT (id) DO UPDATE SET status = 'enabled'",
            &[&process_id, &format!("name-{process_id}")],
        )
        .await
        .expect("seed process definition");
    affected > 0
}

async fn create_work(client: &Client, base: &str, auth: &str, process_id: &str) -> (String, String) {
    let response = client
        .post(format!(
            "{}/api/processplatform/service/processing/work",
            base
        ))
        .header("Authorization", auth)
        .json(&json!({"processId": process_id, "title": format!("work-{process_id}")}))
        .send()
        .await
        .expect("work create request failed");
    assert_eq!(
        response.status(),
        reqwest::StatusCode::OK,
        "work create failed: {}",
        response.text().await.unwrap_or_default()
    );
    let body: serde_json::Value = response.json().await.expect("invalid work create response");
    (
        body["data"]["id"]
            .as_str()
            .expect("work create missing id")
            .to_string(),
        body["data"]["taskId"]
            .as_str()
            .expect("work create missing taskId")
            .to_string(),
    )
}

async fn db_task_status(pool: &TestPool, task_id: &str) -> String {
    let client = pool
        .as_pg()
        .expect("pg pool")
        .get()
        .await
        .expect("pg client");
    let row = client
        .query_one("SELECT task_status FROM x_task WHERE id = $1", &[&task_id])
        .await
        .expect("task status query failed");
    row.get("task_status")
}

async fn db_work_status(pool: &TestPool, work_id: &str) -> String {
    let client = pool
        .as_pg()
        .expect("pg pool")
        .get()
        .await
        .expect("pg client");
    let row = client
        .query_one("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
        .await
        .expect("work status query failed");
    row.get("work_status")
}

async fn db_record(pool: &TestPool, task_id: &str, record_type: &str) -> Option<String> {
    let client = pool
        .as_pg()
        .expect("pg pool")
        .get()
        .await
        .expect("pg client");
    client
        .query_opt(
            "SELECT content FROM x_record WHERE task_id = $1 AND record_type = $2",
            &[&task_id, &record_type],
        )
        .await
        .expect("record query failed")
        .map(|row| row.get::<_, Option<String>>("content").unwrap_or_default())
}

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn bpmn_process_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (_addr, server_handle, token) =
        crate::integration_tests::helpers::setup_test_server(pool.clone())
            .await
            .expect("failed to start test server");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    let process_id = format!("bpmn-it-process-{}", std::process::id());
    assert!(seed_process(&pool, &process_id).await, "seed process failed");

    // Step 1: 原生 work create —— 建 x_work（pending）+ 首个 active 任务。
    let (work_id, task_id) = create_work(&client, &base, &auth_header, &process_id).await;
    assert_eq!(db_work_status(&pool, &work_id).await, "pending");
    assert_eq!(db_task_status(&pool, &task_id).await, "active");
    info!(work_id = %work_id, task_id = %task_id, "work created");

    // Step 2: PUT work/{id}/processing —— 前端「进入办理」走这条。
    let processing = client
        .put(format!("{}/api/work/{}/processing", base, work_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("work processing request failed");
    assert_eq!(
        processing.status(),
        reqwest::StatusCode::OK,
        "work processing failed: {}",
        processing.text().await.unwrap_or_default()
    );
    assert_eq!(db_work_status(&pool, &work_id).await, "processing");

    // Step 3: 完成首个任务 —— 无后续任务，work 收尾。canonical 没有单独
    // 的 GET /api/work/{id}（只有 /api/work/{id}/{processing|start|complete}），
    // 故结果一律回 SQL 现场取证。
    let complete = client
        .post(format!("{}/api/task/{}/complete", base, task_id))
        .header("Authorization", &auth_header)
        .json(&json!({"opinion": "approved in integration run"}))
        .send()
        .await
        .expect("task complete request failed");
    assert_eq!(
        complete.status(),
        reqwest::StatusCode::OK,
        "task complete failed: {}",
        complete.text().await.unwrap_or_default()
    );
    let complete_body: serde_json::Value = complete.json().await.expect("invalid complete response");
    assert_eq!(
        complete_body["data"]["task_status"].as_str(),
        Some("completed")
    );
    assert_eq!(db_task_status(&pool, &task_id).await, "completed");
    assert_eq!(db_work_status(&pool, &work_id).await, "completed");
    assert_eq!(
        db_record(&pool, &task_id, "complete").await.as_deref(),
        Some("approved in integration run")
    );
    info!("task completed and work archived");

    server_handle.abort();
    let _ = server_handle.await;
}

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn bpmn_task_actions_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (_addr, server_handle, token) =
        crate::integration_tests::helpers::setup_test_server(pool.clone())
            .await
            .expect("failed to start test server");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    async fn set_task_status(pool: &TestPool, task_id: &str, status: &str) {
        let client = pool
            .as_pg()
            .expect("pg pool")
            .get()
            .await
            .expect("pg client");
        client
            .execute(
                "UPDATE x_task SET task_status = $1 WHERE id = $2",
                &[&status, &task_id],
            )
            .await
            .expect("set task status");
    }

    // ── Part A: claim（pending → active）────────────────────────────────────
    let pid_a = format!("bpmn-it-claim-{}", std::process::id());
    assert!(seed_process(&pool, &pid_a).await, "seed claim process failed");
    let (_work_a, task_a) = create_work(&client, &base, &auth_header, &pid_a).await;
    // canonical 语义：只有 pending 任务可认领；create 出来的是 active，先置 pending。
    set_task_status(&pool, &task_a, "pending").await;
    let claim = client
        .post(format!("{}/api/task/{}/claim", base, task_a))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("claim failed");
    assert_eq!(
        claim.status(),
        reqwest::StatusCode::OK,
        "claim failed: {}",
        claim.text().await.unwrap_or_default()
    );
    let claim_body: serde_json::Value = claim.json().await.expect("invalid claim response");
    assert_eq!(claim_body["data"]["task_status"].as_str(), Some("active"));
    assert_eq!(db_task_status(&pool, &task_a).await, "active");
    assert_eq!(
        db_record(&pool, &task_a, "claim").await.as_deref(),
        Some("task claimed")
    );
    info!(task = %task_a, "task claimed");

    // ── Part B: transfer（改派）─────────────────────────────────────────────
    let pid_b = format!("bpmn-it-xfer-{}", std::process::id());
    assert!(seed_process(&pool, &pid_b).await, "seed transfer process failed");
    let (_work_b, task_b) = create_work(&client, &base, &auth_header, &pid_b).await;
    let xfer = client
        .post(format!("{}/api/task/{}/transfer/{}", base, task_b, "bob"))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("transfer failed");
    assert_eq!(
        xfer.status(),
        reqwest::StatusCode::OK,
        "transfer failed: {}",
        xfer.text().await.unwrap_or_default()
    );
    let xfer_body: serde_json::Value = xfer.json().await.expect("invalid transfer response");
    assert_eq!(xfer_body["data"]["newPerson"].as_str(), Some("bob"));
    let rec = pool
        .as_pg()
        .expect("pg pool")
        .get()
        .await
        .expect("pg client")
        .query_one(
            "SELECT content FROM x_record WHERE task_id = $1 AND record_type = 'transfer'",
            &[&task_b],
        )
        .await
        .expect("transfer record query failed");
    assert_eq!(
        rec.get::<_, Option<String>>("content").as_deref(),
        Some("transferred from it-admin to bob")
    );
    info!(task = %task_b, "task transferred");

    // ── Part C: reject（active → rejected，work 不受影响）────────────────────
    let pid_c = format!("bpmn-it-reject-{}", std::process::id());
    assert!(seed_process(&pool, &pid_c).await, "seed reject process failed");
    let (work_c, task_c) = create_work(&client, &base, &auth_header, &pid_c).await;
    let processing = client
        .put(format!("{}/api/work/{}/processing", base, work_c))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("work processing failed");
    assert_eq!(processing.status(), reqwest::StatusCode::OK);
    let reject = client
        .post(format!("{}/api/task/{}/reject", base, task_c))
        .header("Authorization", &auth_header)
        .json(&json!({"opinion": "rejected by reviewer"}))
        .send()
        .await
        .expect("reject failed");
    assert_eq!(
        reject.status(),
        reqwest::StatusCode::OK,
        "reject failed: {}",
        reject.text().await.unwrap_or_default()
    );
    let reject_body: serde_json::Value = reject.json().await.expect("invalid reject response");
    assert_eq!(
        reject_body["data"]["task_status"].as_str(),
        Some("rejected")
    );
    assert_eq!(db_task_status(&pool, &task_c).await, "rejected");
    // reject 只改任务，不推进 work（canonical 语义，见 task_reject 实现）。
    assert_eq!(db_work_status(&pool, &work_c).await, "processing");
    assert_eq!(
        db_record(&pool, &task_c, "reject").await.as_deref(),
        Some("rejected by reviewer")
    );
    info!(task = %task_c, "task rejected");

    server_handle.abort();
    let _ = server_handle.await;
}
