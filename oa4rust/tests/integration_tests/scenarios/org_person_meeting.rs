use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// Org �?Person �?Meeting cross-crate happy path
//
// Verifies: admin user can create a meeting room, schedule a meeting,
// add an attendee, and retrieve the meeting by ID �?all through the
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
        .post(format!("{}/jaxrs/meeting/room/create", base))
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

    // Step 2: Create a meeting
    let meeting_resp = client
        .post(format!("{}/jaxrs/meeting/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "title": "Integration Test Meeting",
            "content": "Testing the meeting creation flow",
            "roomId": room_id,
            "startTime": "2026-12-01T10:00:00",
            "endTime": "2026-12-01T11:00:00",
            "creator": "test-admin"
        }))
        .send()
        .await
        .expect("create meeting request failed");

    assert_eq!(
        meeting_resp.status(),
        reqwest::StatusCode::OK,
        "create meeting failed: {}",
        meeting_resp.text().await.unwrap_or_default()
    );

    let meeting_body: serde_json::Value = meeting_resp.json().await.expect("invalid meeting response");
    let meeting_id = meeting_body["data"]["id"]
        .as_str()
        .expect("meeting id missing")
        .to_string();
    assert!(!meeting_id.is_empty(), "meeting id should not be empty");
    info!(meeting_id = %meeting_id, "meeting created");

    // Step 3: Add an attendee
    let invite_resp = client
        .post(format!("{}/jaxrs/meeting/{}/participant/add", base, meeting_id))
        .header("Authorization", &auth_header)
        .json(&json!({
            "invitee": "person-test-001"
        }))
        .send()
        .await
        .expect("add participant request failed");

    assert_eq!(
        invite_resp.status(),
        reqwest::StatusCode::OK,
        "add participant failed: {}",
        invite_resp.text().await.unwrap_or_default()
    );

    let invite_body: serde_json::Value = invite_resp.json().await.expect("invalid invite response");
    assert_eq!(
        invite_body["data"]["added"].as_bool(),
        Some(true),
        "participant not added"
    );
    info!(meeting_id = %meeting_id, "participant added");

    // Step 4: Verify - list participants
    let list_resp = client
        .get(format!("{}/jaxrs/meeting/{}/participant/list", base, meeting_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list participants request failed");

    assert_eq!(
        list_resp.status(),
        reqwest::StatusCode::OK,
        "list participants failed: {}",
        list_resp.text().await.unwrap_or_default()
    );

    let list_body: serde_json::Value = list_resp.json().await.expect("invalid list response");
    let count = list_body["data"]["count"]
        .as_i64()
        .expect("count missing");
    assert_eq!(count, 1, "expected 1 participant, got {}", count);

    let participants = list_body["data"]["data"]
        .as_array()
        .expect("data array missing");
    assert_eq!(participants[0]["invitee"].as_str(), Some("person-test-001"));

    // Step 5: Verify - get meeting by id
    let get_resp = client
        .get(format!("{}/jaxrs/meeting/{}", base, meeting_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("get meeting request failed");

    assert_eq!(
        get_resp.status(),
        reqwest::StatusCode::OK,
        "get meeting failed: {}",
        get_resp.text().await.unwrap_or_default()
    );

    let get_body: serde_json::Value = get_resp.json().await.expect("invalid get response");
    assert_eq!(get_body["data"]["id"].as_str(), Some(meeting_id.as_str()));
    assert_eq!(get_body["data"]["title"].as_str(), Some("Integration Test Meeting"));

    // Shutdown the server
    server_handle.abort();
    let _ = server_handle.await;
}
