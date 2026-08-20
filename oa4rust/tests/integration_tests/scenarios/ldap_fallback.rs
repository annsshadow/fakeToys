use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::info;

use crate::integration_tests::db::TEST_DB;

// ──────────────────────────────────────────────────────────────────────────────
// LDAP fallback contract (U2.3)
//
// Verifies the graceful DB-fallback contract for the login flow:
//   (a) With no LDAP configured in the test environment, login via the auth
//       endpoint with seeded admin creds (it-admin / password123) falls back to
//       the database password check and returns 200 + a session token.
//   (b) GET /jaxrs/ldap/config reports LDAP as disabled (enabled == false) when
//       no LDAP configuration is present.
//
// Mirrors tests/integration_tests/scenarios/cms_document.rs in structure.
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn ldap_fallback_flow() {
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

    // (a) Login with admin creds — DB-fallback path (no LDAP configured).
    let login_resp = client
        .post(format!("{}/jaxrs/authentication", base))
        .json(&json!({
            "credential": "it-admin",
            "password": "password123"
        }))
        .send()
        .await
        .expect("login request failed");

    assert_eq!(
        login_resp.status(),
        reqwest::StatusCode::OK,
        "login failed: {}",
        login_resp.text().await.unwrap_or_default()
    );
    let login_body: serde_json::Value = login_resp.json().await.expect("invalid login response");
    let login_token = login_body["data"]["token"]
        .as_str()
        .unwrap_or("")
        .to_string();
    assert!(
        !login_token.is_empty(),
        "login should return a non-empty session token (DB-fallback path)"
    );
    info!(token_len = %login_token.len(), "admin login succeeded via DB fallback");

    // (b) LDAP config endpoint — must report enabled == false (no LDAP configured).
    let config_resp = client
        .get(format!("{}/jaxrs/ldap/config", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("ldap config request failed");

    assert_eq!(
        config_resp.status(),
        reqwest::StatusCode::OK,
        "ldap config request failed: {}",
        config_resp.text().await.unwrap_or_default()
    );
    let config_body: serde_json::Value = config_resp.json().await.expect("invalid config response");
    assert_eq!(
        config_body["data"]["enabled"].as_bool(),
        Some(false),
        "LDAP should be disabled when no LDAP config is present; body = {}",
        config_body
    );
    info!(enabled = %config_body["data"]["enabled"], "ldap config reported disabled");

    server_handle.abort();
    let _ = server_handle.await;
}
