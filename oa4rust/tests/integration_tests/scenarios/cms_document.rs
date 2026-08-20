use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// CMS data_document CRUD lifecycle (U2.1 core)
//
// Verifies the realized document endpoints perform genuine DB reads/writes:
// create → list (visible) → get-by-id → update → soft-delete → list (gone) →
// get-by-id (404). Runs through the real HTTP layer with auth middleware active.
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn cms_document_crud_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    let doc_id = "cms-doc-it-001";

    // Step 1: Create (publish) a document
    let create_resp = client
        .post(format!("{}/jaxrs/cms_assemble_control/data/document/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "id": doc_id,
            "appId": "app-it-001",
            "categoryId": "cat-it-001",
            "title": "Integration Test Document",
            "content": "Body of the integration test document.",
            "authorId": "person-it-admin",
            "status": "published"
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
    assert_eq!(create_body["data"]["id"].as_str(), Some(doc_id));
    assert_eq!(create_body["data"]["status"].as_str(), Some("published"));
    info!(doc_id = %doc_id, "document created");

    // Step 2: List documents — created doc must be visible
    let list_resp = client
        .get(format!("{}/jaxrs/cms_assemble_control/data/document", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list document request failed");
    assert_eq!(list_resp.status(), reqwest::StatusCode::OK);
    let list_body: serde_json::Value = list_resp.json().await.expect("invalid list response");
    let docs = list_body["data"]["data"].as_array().expect("data array missing");
    assert!(
        docs.iter().any(|d| d["id"].as_str() == Some(doc_id)),
        "created document not present in list"
    );
    info!(count = %list_body["data"]["count"], "document listed");

    // Step 3: Get by id — must return real content
    let get_resp = client
        .get(format!("{}/jaxrs/cms_assemble_control/data/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get document request failed");
    assert_eq!(get_resp.status(), reqwest::StatusCode::OK);
    let get_body: serde_json::Value = get_resp.json().await.expect("invalid get response");
    assert_eq!(get_body["data"]["id"].as_str(), Some(doc_id));
    assert_eq!(get_body["data"]["title"].as_str(), Some("Integration Test Document"));
    assert_eq!(get_body["data"]["content"].as_str(), Some("Body of the integration test document."));
    info!(doc_id = %doc_id, "document fetched by id");

    // Step 4: Update title/status
    let update_resp = client
        .post(format!("{}/jaxrs/cms_assemble_control/data/document/{}/update", base, doc_id))
        .header("Authorization", &auth_header)
        .json(&json!({
            "title": "Integration Test Document (Updated)",
            "status": "archived"
        }))
        .send()
        .await
        .expect("update document request failed");
    assert_eq!(update_resp.status(), reqwest::StatusCode::OK);
    let update_body: serde_json::Value = update_resp.json().await.expect("invalid update response");
    assert_eq!(update_body["data"]["updated"].as_bool(), Some(true));

    let get2_resp = client
        .get(format!("{}/jaxrs/cms_assemble_control/data/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get after update failed");
    let get2_body: serde_json::Value = get2_resp.json().await.expect("invalid get2 response");
    assert_eq!(get2_body["data"]["title"].as_str(), Some("Integration Test Document (Updated)"));
    assert_eq!(get2_body["data"]["status"].as_str(), Some("archived"));
    info!(doc_id = %doc_id, "document updated");

    // Step 5: Soft-delete — must disappear from list and 404 on get
    let del_resp = client
        .post(format!("{}/jaxrs/cms_assemble_control/data/document/{}/delete", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("delete document request failed");
    assert_eq!(del_resp.status(), reqwest::StatusCode::OK);
    let del_body: serde_json::Value = del_resp.json().await.expect("invalid delete response");
    assert_eq!(del_body["data"]["deleted"].as_bool(), Some(true));

    let list2_resp = client
        .get(format!("{}/jaxrs/cms_assemble_control/data/document", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list after delete failed");
    let list2_body: serde_json::Value = list2_resp.json().await.expect("invalid list2 response");
    let docs2 = list2_body["data"]["data"].as_array().expect("data array missing");
    assert!(
        !docs2.iter().any(|d| d["id"].as_str() == Some(doc_id)),
        "soft-deleted document still present in list"
    );

    let get3_resp = client
        .get(format!("{}/jaxrs/cms_assemble_control/data/document/{}", base, doc_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get after delete failed");
    assert_eq!(
        get3_resp.status(),
        reqwest::StatusCode::NOT_FOUND,
        "soft-deleted document should 404 on get"
    );
    info!(doc_id = %doc_id, "document soft-deleted and verified gone");

    server_handle.abort();
    let _ = server_handle.await;
}
