// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// CMS data_document CRUD lifecycle (U2.1 core)
//
// 新语义（对齐 plan002 U2 实装的 canonical 路由）：
//   POST   /api/document                    创建（服务端生成 id，初始 status=draft）
//   PUT    /api/document/publish/{id}       发布（status=published）
//   GET    /api/document/{id}               按 id 读取真实行
//   POST   /api/document/{id}/update        更新 title/content
//   DELETE /api/document/{id}               软删
//   POST   /api/document/list/document      按 ids 列取（legacy 顶层 data 数组）
//
// 出处：cms_assemble_control/src/routes.rs plan002 U2 段（/api/document*），
// 提交 b63d277b8（plan002 U2 行为对齐）。旧测试的
// `/api/cms_assemble_control/data/document/...` 在实装路由表中不存在。
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn cms_document_crud_flow() {
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

    // Step 1: Create — 服务端生成 id，初始状态固定 draft（请求体 status 不参与创建）
    let create_resp = client
        .post(format!("{}/api/document", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "appId": "app-it-001",
            "categoryId": "cat-it-001",
            "title": "Integration Test Document",
            "content": "Body of the integration test document."
        }))
        .send()
        .await
        .expect("create document request failed");

    assert_eq!(
        create_resp.status(),
        reqwest::StatusCode::OK,
        "create document failed: {}",
        create_resp.text().await.unwrap_or_default()
    );
    let create_body: serde_json::Value = create_resp.json().await.expect("invalid create response");
    let doc_id = create_body["data"]["id"]
        .as_str()
        .expect("create did not return a document id")
        .to_string();
    assert_eq!(create_body["data"]["status"].as_str(), Some("draft"));
    info!(doc_id = %doc_id, "document created");

    // Step 2: Publish — canonical 发布端点
    let publish_resp = client
        .put(format!("{}/api/document/publish/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("publish document request failed");
    assert_eq!(publish_resp.status(), reqwest::StatusCode::OK);
    let publish_body: serde_json::Value =
        publish_resp.json().await.expect("invalid publish response");
    assert_eq!(publish_body["data"]["published"].as_bool(), Some(true));

    // Step 3: Get by id — must return real content and published status
    let get_resp = client
        .get(format!("{}/api/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get document request failed");
    assert_eq!(get_resp.status(), reqwest::StatusCode::OK);
    let get_body: serde_json::Value = get_resp.json().await.expect("invalid get response");
    assert_eq!(get_body["data"]["id"].as_str(), Some(doc_id.as_str()));
    assert_eq!(
        get_body["data"]["title"].as_str(),
        Some("Integration Test Document")
    );
    assert_eq!(
        get_body["data"]["content"].as_str(),
        Some("Body of the integration test document.")
    );
    assert_eq!(get_body["data"]["status"].as_str(), Some("published"));
    info!(doc_id = %doc_id, "document fetched by id");

    // Step 4: Update title — canonical update 端点
    let update_resp = client
        .post(format!("{}/api/document/{}/update", base, doc_id))
        .header("Authorization", &auth_header)
        .json(&json!({ "title": "Integration Test Document (Updated)" }))
        .send()
        .await
        .expect("update document request failed");
    assert_eq!(update_resp.status(), reqwest::StatusCode::OK);
    let update_body: serde_json::Value = update_resp.json().await.expect("invalid update response");
    assert_eq!(update_body["data"]["updated"].as_bool(), Some(true));

    let get2_resp = client
        .get(format!("{}/api/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get after update failed");
    let get2_body: serde_json::Value = get2_resp.json().await.expect("invalid get2 response");
    assert_eq!(
        get2_body["data"]["title"].as_str(),
        Some("Integration Test Document (Updated)")
    );
    assert_eq!(get2_body["data"]["status"].as_str(), Some("published"));
    info!(doc_id = %doc_id, "document updated");

    // Step 5: Soft-delete — 列取消失、按 id 读取返回错误信封
    let del_resp = client
        .delete(format!("{}/api/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete document request failed");
    assert_eq!(del_resp.status(), reqwest::StatusCode::OK);
    let del_body: serde_json::Value = del_resp.json().await.expect("invalid delete response");
    assert_eq!(del_body["data"]["deleted"].as_bool(), Some(true));

    let list2_resp = client
        .post(format!("{}/api/document/list/document", base))
        .header("Authorization", &auth_header)
        .json(&json!({ "ids": [doc_id] }))
        .send()
        .await
        .expect("list after delete failed");
    assert_eq!(list2_resp.status(), reqwest::StatusCode::OK);
    let list2_body: serde_json::Value = list2_resp.json().await.expect("invalid list2 response");
    let docs2 = list2_body["data"]
        .as_array()
        .expect("legacy data array missing");
    assert!(
        !docs2
            .iter()
            .any(|d| d["id"].as_str() == Some(doc_id.as_str())),
        "soft-deleted document still present in list"
    );

    let get3_resp = client
        .get(format!("{}/api/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get after delete failed");
    assert_eq!(get3_resp.status(), reqwest::StatusCode::OK);
    let get3_body: serde_json::Value = get3_resp.json().await.expect("invalid get3 response");
    assert_eq!(get3_body["type"].as_str(), Some("error"));
    assert_eq!(get3_body["message"].as_str(), Some("document not found"));
    info!(doc_id = %doc_id, "document soft-deleted and verified gone");

    server_handle.abort();
    let _ = server_handle.await;
}
