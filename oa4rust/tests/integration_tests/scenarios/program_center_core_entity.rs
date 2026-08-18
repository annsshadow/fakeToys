use std::time::Duration;

use reqwest::Client;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// Program Center Core Entity — create → list happy path
//
// Verifies: an application record can be inserted into x_application
// and retrieved via the list endpoint through the real HTTP layer.
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
pub async fn program_center_core_entity_application_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (_addr, server_handle, _token) = crate::integration_tests::helpers::setup_test_server((*pool).clone())
        .await
        .expect("failed to start test server");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);

    // Step 1: Insert a test application record directly into the database
    {
        let db_client = pool.get().await.expect("failed to get pool client");
        db_client
            .execute(
                "INSERT INTO x_application (id, name, category, sub_category, version, publisher, creator_person) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7) \
                 ON CONFLICT (id) DO NOTHING",
                &[
                    &"app-pc-integration-test-001",
                    &"Integration Test App",
                    &"Office",
                    &"Productivity",
                    &"1.0.0",
                    &"Test Publisher",
                    &"it-admin",
                ],
            )
            .await
            .expect("insert x_application failed");
    }

    // Step 2: Call the list endpoint and verify the record is returned
    let list_resp = client
        .get(format!("{}/jaxrs/program_center/application/list", base))
        .send()
        .await
        .expect("list application request failed");

    assert_eq!(
        list_resp.status(),
        reqwest::StatusCode::OK,
        "application list failed: {}",
        list_resp.text().await.unwrap_or_default()
    );

    let body: serde_json::Value = list_resp.json().await.expect("invalid list response");
    let data = body["data"]["data"]
        .as_array()
        .expect("data array missing");

    let found = data.iter().any(|app| {
        app["id"].as_str() == Some("app-pc-integration-test-001")
            && app["name"].as_str() == Some("Integration Test App")
            && app["category"].as_str() == Some("Office")
    });
    assert!(found, "expected integration test application not found in list response");

    server_handle.abort();
    let _ = server_handle.await;
}
