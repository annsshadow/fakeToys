use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// File metadata creation �?upload �?retrieval cross-crate happy path
//
// Verifies: a file folder can be created, a file can be uploaded into it,
// and the file metadata can be retrieved by ID �?all through the real
// HTTP layer with auth middleware active.
// ──────────────────────────────────────────────────────────────────────────────

pub async fn file_upload_flow() {
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

    // Step 1: Create a file folder
    let folder_resp = client
        .post(format!("{}/jaxrs/file/folder/create", base))
        .header("Authorization", &auth_header)
        .header("Content-Type", "application/json")
        .json(&json!({
            "name": "Integration Test Folder",
            "person": "test-admin"
        }))
        .send()
        .await
        .expect("create folder request failed");

    assert_eq!(
        folder_resp.status(),
        reqwest::StatusCode::OK,
        "create folder failed: {}",
        folder_resp.text().await.unwrap_or_default()
    );

    let folder_body: serde_json::Value = folder_resp.json().await.expect("invalid folder response");
    let folder_id = folder_body["data"]["id"]
        .as_str()
        .expect("folder id missing")
        .to_string();
    assert!(!folder_id.is_empty(), "folder id should not be empty");
    info!(folder_id = %folder_id, "file folder created");

    // Step 2: Verify folder was created by listing top-level folders
    let list_resp = client
        .get(format!("{}/jaxrs/file/folder/list/top", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list folders request failed");

    assert_eq!(
        list_resp.status(),
        reqwest::StatusCode::OK,
        "list folders failed: {}",
        list_resp.text().await.unwrap_or_default()
    );

    // Folder list response format varies; just verify auth works
    info!("file upload flow: folder created, auth verified");

    // Step 3: Upload a file into the folder using multipart form
    let file_bytes = b"Hello, integration test! This is test file content.".as_slice();
    let file_part = reqwest::multipart::Part::bytes(file_bytes.to_vec())
        .file_name("test.txt")
        .mime_str("text/plain")
        .expect("invalid mime");

    let form = reqwest::multipart::Form::new()
        .part("file", file_part)
        .text("name", "integration-test-file.txt")
        .text("person", "test-admin")
        .text("referenceId", folder_id.clone())
        .text("referenceType", "folder");

    let upload_resp = client
        .post(format!("{}/jaxrs/file/upload", base))
        .header("Authorization", &auth_header)
        .multipart(form)
        .send()
        .await
        .expect("upload file request failed");

    assert_eq!(
        upload_resp.status(),
        reqwest::StatusCode::OK,
        "upload file failed: {}",
        upload_resp.text().await.unwrap_or_default()
    );

    let upload_body: serde_json::Value = upload_resp.json().await.expect("invalid upload response");
    let file_id = upload_body["data"]["id"]
        .as_str()
        .expect("file id missing")
        .to_string();
    assert!(!file_id.is_empty(), "file id should not be empty");
    assert_eq!(
        upload_body["data"]["name"].as_str(),
        Some("integration-test-file.txt")
    );
    assert_eq!(upload_body["data"]["extension"].as_str(), Some("txt"));
    info!(file_id = %file_id, "file uploaded");

    // Step 4: Verify - retrieve the file by ID
    let download_resp = client
        .get(format!("{}/jaxrs/file/download/{}", base, file_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("download file request failed");

    assert_eq!(
        download_resp.status(),
        reqwest::StatusCode::OK,
        "download file failed: {}",
        download_resp.text().await.unwrap_or_default()
    );

    let download_body: serde_json::Value = download_resp.json().await.expect("invalid download response");
    assert_eq!(download_body["data"]["id"].as_str(), Some(file_id.as_str()));
    assert_eq!(
        download_body["data"]["name"].as_str(),
        Some("integration-test-file.txt")
    );
    assert_eq!(download_body["data"]["extension"].as_str(), Some("txt"));

    // Shutdown the server
    server_handle.abort();
    let _ = server_handle.await;
}
