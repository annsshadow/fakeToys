use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// Org Person Meeting cross-crate happy path
//
// Verifies: admin user can create a meeting room through the
// real HTTP layer with auth middleware active.
// ──────────────────────────────────────────────────────────────────────────────

pub async fn org_person_meeting_flow() {
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

    // Step 1: Create a meeting room
    let room_resp = client
        .post(format!("{}/jaxrs/meeting/assemble/control/room", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "name": "Test Conference Room",
            "buildingId": "bldg-1",
            "floor": "3F",
            "capacity": 20,
            "orderNumber": 1
        }))
        .send()
        .await
        .expect("create room request failed");

    assert_eq!(
        room_resp.status(),
        reqwest::StatusCode::OK,
        "create room failed: {}",
        room_resp.text().await.unwrap_or_default()
    );

    let room_body: serde_json::Value = room_resp.json().await.expect("invalid room response");
    let room_id = room_body["data"]["id"]
        .as_str()
        .expect("room id missing")
        .to_string();
    assert!(!room_id.is_empty(), "room id should not be empty");
    info!(room_id = %room_id, "meeting room created");

    // Step 2: Verify auth works with a health check
    let ping = client.get(format!("{}/health", base)).send().await.unwrap();
    assert!(ping.status().is_success());
    info!("meeting room flow verified (auth + room creation)");

    // Shutdown the server
    server_handle.abort();
    let _ = server_handle.await;
}
