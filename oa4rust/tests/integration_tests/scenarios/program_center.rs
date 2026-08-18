use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// program_center routed endpoints (U2.4 real-ize)
//
// Verifies the routed program_center endpoints perform genuine DB reads/writes:
// application create (write) → applications list (read reflects the write) →
// modules_all (read, seeded row reflected) → config save (write) → config get
// (read reflects the write). Runs through the real HTTP layer with auth active.
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
pub async fn program_center_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server((*pool).clone())
        .await
        .expect("failed to start test server");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");

    let base = format!("http://{}", _addr);
    let auth_header = format!("Bearer {}", token);

    // Unique ids to avoid collisions across scenarios / re-runs.
    let app_id = "pc-it-app-001";
    let mod_id = "pc-it-mod-001";
    let cfg_key = "pc.it.flag.001";

    // Seed a program module directly so modules_all reflects a real row.
    {
        let db = pool.get().await.expect("seed: failed to get client");
        db.execute(
            "INSERT INTO x_program_module (id, name, entity, creator, create_time) \
             VALUES ($1, $2, $3, $4, NOW())",
            &[&mod_id, &"PC IT Module", &"Process", &"it-admin"],
        )
        .await
        .expect("seed: insert module failed");
        info!(module_id = %mod_id, "seeded program module");
    }

    // ── Step 1: Create an application (DB write) ──────────────────────────────
    let create_resp = client
        .post(format!("{}/jaxrs/program_center/application/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "name": "PC Integration App",
            "app_id": app_id,
            "description": "Created by program_center integration test",
            "creator": "it-admin"
        }))
        .send()
        .await
        .expect("create application request failed");

    assert_eq!(
        create_resp.status(),
        reqwest::StatusCode::OK,
        "create application failed: {}",
        create_resp.text().await.unwrap_or_default()
    );
    let create_body: serde_json::Value = create_resp.json().await.expect("invalid create response");
    let created_id = create_body["data"]["id"].as_str().expect("created id missing");
    assert_eq!(create_body["data"]["name"].as_str(), Some("PC Integration App"));
    assert_eq!(create_body["data"]["appId"].as_str(), Some(app_id));
    info!(created_id = %created_id, "application created");

    // ── Step 2: List applications — created app must be visible (DB read) ─────
    let list_resp = client
        .get(format!("{}/jaxrs/program/applications", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list applications request failed");
    assert_eq!(list_resp.status(), reqwest::StatusCode::OK);
    let list_body: serde_json::Value = list_resp.json().await.expect("invalid list response");
    let apps = list_body["data"]["data"].as_array().expect("data array missing");
    assert!(
        apps.iter().any(|a| a["id"].as_str() == Some(created_id)),
        "created application not present in list"
    );
    info!(count = %list_body["data"]["count"], "applications listed");

    // ── Step 3: modules_all reflects the seeded module (DB read) ──────────────
    let mod_resp = client
        .get(format!("{}/jaxrs/program/datastructure/modules/all", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("modules_all request failed");
    assert_eq!(mod_resp.status(), reqwest::StatusCode::OK);
    let mod_body: serde_json::Value = mod_resp.json().await.expect("invalid modules response");
    let modules = mod_body["data"]["data"].as_array().expect("modules array missing");
    assert!(
        modules.iter().any(|m| m["name"].as_str() == Some("PC IT Module")),
        "seeded module not present in modules_all"
    );
    info!(module_count = %mod_body["data"]["count"], "modules listed");

    // ── Step 4: Save a config (DB write) ─────────────────────────────────────
    let cfg_save_resp = client
        .post(format!("{}/jaxrs/program_center/config/save", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "key": cfg_key,
            "value": "enabled",
            "category": "pc-it",
            "creator": "it-admin"
        }))
        .send()
        .await
        .expect("config save request failed");
    assert_eq!(cfg_save_resp.status(), reqwest::StatusCode::OK);
    let cfg_save_body: serde_json::Value = cfg_save_resp.json().await.expect("invalid config save response");
    assert_eq!(cfg_save_body["data"]["key"].as_str(), Some(cfg_key));
    info!(config_key = %cfg_key, "config saved");

    // ── Step 5: Get config — reflects the write (DB read) ─────────────────────
    let cfg_get_resp = client
        .get(format!("{}/jaxrs/program_center/config/{}", base, cfg_key))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("config get request failed");
    assert_eq!(cfg_get_resp.status(), reqwest::StatusCode::OK);
    let cfg_get_body: serde_json::Value = cfg_get_resp.json().await.expect("invalid config get response");
    assert_eq!(cfg_get_body["data"]["key"].as_str(), Some(cfg_key));
    assert_eq!(cfg_get_body["data"]["value"].as_str(), Some("enabled"));
    info!(config_value = %cfg_get_body["data"]["value"], "config fetched");

    server_handle.abort();
    let _ = server_handle.await;
}
