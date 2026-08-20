use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// file_assemble_control + portal_assemble_designer real-DB happy path (U2.4)
//
// Proves the routed endpoints perform genuine DB reads/writes through the real
// HTTP layer with auth middleware active:
//   * file_assemble_control: create a file entity → list files in its folder
//     and assert the persisted metadata (id + name) is returned.
//   * portal_assemble_designer: create a portal design → list designs and
//     assert the persisted design (id + name) is returned.
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn file_portal_flow() {
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

    // ── file_assemble_control: create a file entity (unique folder + file id) ──
    let folder_id = "fp-folder-it-001";
    let file_name = "fp-file-it-001.txt";

    let create_file_resp = client
        .post(format!("{}/jaxrs/file/core/entity/file/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "name": file_name,
            "path": "/tmp/fp/fp-file-it-001.txt",
            "folderId": folder_id,
            "size": 1024i64
        }))
        .send()
        .await
        .expect("create file entity request failed");

    assert_eq!(
        create_file_resp.status(),
        reqwest::StatusCode::OK,
        "create file entity failed: {}",
        create_file_resp.text().await.unwrap_or_default()
    );
    let create_file_body: serde_json::Value =
        create_file_resp.json().await.expect("invalid create file response");
    let file_id = create_file_body["data"]["id"]
        .as_str()
        .expect("file id missing")
        .to_string();
    assert!(!file_id.is_empty(), "file id should not be empty");
    info!(file_id = %file_id, "file entity created");

    // ── file_assemble_control: list files in the folder → real DB data ──
    let list_file_resp = client
        .get(format!(
            "{}/jaxrs/file/assemble/control/file/list/{}",
            base, folder_id
        ))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list files request failed");
    assert_eq!(list_file_resp.status(), reqwest::StatusCode::OK);
    let list_file_body: serde_json::Value =
        list_file_resp.json().await.expect("invalid list files response");
    let files = list_file_body["data"]["data"]
        .as_array()
        .expect("data array missing");
    let found_file = files
        .iter()
        .any(|f| f["id"].as_str() == Some(file_id.as_str()));
    assert!(found_file, "created file not present in folder listing");
    let created_entry = files
        .iter()
        .find(|f| f["id"].as_str() == Some(file_id.as_str()))
        .expect("created file entry missing");
    assert_eq!(
        created_entry["name"].as_str(),
        Some(file_name),
        "persisted file name mismatch"
    );
    assert_eq!(
        created_entry["folderId"].as_str(),
        Some(folder_id),
        "persisted folder id mismatch"
    );
    info!(count = %list_file_body["data"]["count"], "files listed with real db data");

    // ── portal_assemble_designer: create a portal design (unique id) ──
    let design_name = "fp-design-it-001";

    let create_design_resp = client
        .post(format!("{}/jaxrs/portal/assemble/designer/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "name": design_name,
            "description": "Integration test portal design"
        }))
        .send()
        .await
        .expect("create design request failed");

    assert_eq!(
        create_design_resp.status(),
        reqwest::StatusCode::OK,
        "create design failed: {}",
        create_design_resp.text().await.unwrap_or_default()
    );
    let create_design_body: serde_json::Value =
        create_design_resp.json().await.expect("invalid create design response");
    let design_id = create_design_body["data"]["id"]
        .as_str()
        .expect("design id missing")
        .to_string();
    assert!(!design_id.is_empty(), "design id should not be empty");
    info!(design_id = %design_id, "portal design created");

    // ── portal_assemble_designer: list designs → real DB data ──
    let list_design_resp = client
        .get(format!("{}/jaxrs/portal/assemble/designer/list", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list designs request failed");
    assert_eq!(list_design_resp.status(), reqwest::StatusCode::OK);
    let list_design_body: serde_json::Value =
        list_design_resp.json().await.expect("invalid list designs response");
    let designs = list_design_body["data"]["data"]
        .as_array()
        .expect("data array missing");
    let found_design = designs
        .iter()
        .any(|d| d["id"].as_str() == Some(design_id.as_str()));
    assert!(found_design, "created design not present in design list");
    let design_entry = designs
        .iter()
        .find(|d| d["id"].as_str() == Some(design_id.as_str()))
        .expect("created design entry missing");
    assert_eq!(
        design_entry["name"].as_str(),
        Some(design_name),
        "persisted design name mismatch"
    );
    info!(count = %list_design_body["data"]["count"], "designs listed with real db data");

    server_handle.abort();
    let _ = server_handle.await;
}
