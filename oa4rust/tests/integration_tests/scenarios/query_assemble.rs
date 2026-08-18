use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// query-assemble designer + surface real-DB verification (U2.4)
//
// Proves the realized query-assemble routed endpoints perform genuine DB
// reads/writes. For each crate we create a record and then list by its
// category, asserting the created row is returned inside a real `data` array
// read back from PostgreSQL. Runs through the real HTTP layer with the auth
// middleware active.
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
pub async fn query_assemble_flow() {
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

    // ── query_assemble_designer: create → list returns real DB data array ──
    let designer_category = "qa-it-designer-cat-001";
    let designer_name = "QA Designer IT 001";
    let designer_query = "SELECT 1";

    let d_create_resp = client
        .post(format!("{}/jaxrs/query/assemble/designer/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "name": designer_name,
            "category": designer_category,
            "query": designer_query
        }))
        .send()
        .await
        .expect("create designer request failed");

    assert_eq!(
        d_create_resp.status(),
        reqwest::StatusCode::OK,
        "create designer failed: {}",
        d_create_resp.text().await.unwrap_or_default()
    );
    let d_create_body: serde_json::Value = d_create_resp.json().await.expect("invalid create designer response");
    let designer_id = d_create_body["data"]["id"].as_str().expect("designer id missing").to_string();
    info!(designer_id = %designer_id, "designer created");

    let d_list_resp = client
        .get(format!("{}/jaxrs/query/assemble/designer/list/{}", base, designer_category))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list designer request failed");
    assert_eq!(d_list_resp.status(), reqwest::StatusCode::OK);
    let d_list_body: serde_json::Value = d_list_resp.json().await.expect("invalid list designer response");
    let designers = d_list_body["data"]["data"].as_array().expect("designer data array missing");
    assert!(
        designers.iter().any(|d| d["id"].as_str() == Some(designer_id.as_str())),
        "created designer not present in list data array"
    );
    info!(count = %d_list_body["data"]["count"], "designer listed with real DB data");

    // ── query_assemble_surface: create → list returns real DB data array ──
    let surface_category = "qa-it-surface-cat-001";
    let surface_name = "QA Surface IT 001";
    let surface_content = "SELECT * FROM x_query_surface";

    let s_create_resp = client
        .post(format!("{}/jaxrs/query/assemble/surface/create", base))
        .header("Authorization", &auth_header)
        .json(&json!({
            "name": surface_name,
            "category": surface_category,
            "query": surface_content
        }))
        .send()
        .await
        .expect("create surface request failed");

    assert_eq!(
        s_create_resp.status(),
        reqwest::StatusCode::OK,
        "create surface failed: {}",
        s_create_resp.text().await.unwrap_or_default()
    );
    let s_create_body: serde_json::Value = s_create_resp.json().await.expect("invalid create surface response");
    let surface_id = s_create_body["data"]["id"].as_str().expect("surface id missing").to_string();
    info!(surface_id = %surface_id, "surface created");

    let s_list_resp = client
        .get(format!("{}/jaxrs/query/assemble/surface/list/{}", base, surface_category))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("list surface request failed");
    assert_eq!(s_list_resp.status(), reqwest::StatusCode::OK);
    let s_list_body: serde_json::Value = s_list_resp.json().await.expect("invalid list surface response");
    let surfaces = s_list_body["data"]["data"].as_array().expect("surface data array missing");
    assert!(
        surfaces.iter().any(|d| d["id"].as_str() == Some(surface_id.as_str())),
        "created surface not present in list data array"
    );
    info!(count = %s_list_body["data"]["count"], "surface listed with real DB data");

    // ── preview_surface now reads the surface row from the DB ──
    let s_preview_resp = client
        .get(format!("{}/jaxrs/query/assemble/surface/preview/{}", base, surface_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("preview surface request failed");
    assert_eq!(s_preview_resp.status(), reqwest::StatusCode::OK);
    let s_preview_body: serde_json::Value = s_preview_resp.json().await.expect("invalid preview response");
    assert_eq!(s_preview_body["data"]["id"].as_str(), Some(surface_id.as_str()));
    assert_eq!(s_preview_body["data"]["name"].as_str(), Some(surface_name));
    info!(surface_id = %surface_id, "surface previewed from DB");

    server_handle.abort();
    let _ = server_handle.await;
}
