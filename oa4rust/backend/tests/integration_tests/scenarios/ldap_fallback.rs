// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
//   (b) GET /api/ldap/config reports LDAP as disabled (enabled == false) when
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

    // (a) Login with admin creds — DB-fallback path (no LDAP configured).
    let login_resp = client
        .post(format!("{}/api/authentication", base))
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
    // 新语义：会话经 HttpOnly cookie 下发，响应体不再返回 token
    // （依据 auth/src/lib.rs with_session_cookie；docs/security 会话 cookie 说明）。
    let set_cookie = login_resp
        .headers()
        .get(reqwest::header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert!(
        !set_cookie.is_empty(),
        "login should set a non-empty session cookie (DB-fallback path)"
    );
    let login_body: serde_json::Value = login_resp.json().await.expect("invalid login response");
    assert!(
        login_body["data"]["token"]
            .as_str()
            .unwrap_or("")
            .is_empty(),
        "login must not expose the session token in the response body"
    );
    info!(cookie_len = %set_cookie.len(), "admin login succeeded via DB fallback");

    // (b) LDAP config endpoint —— O20 定性 A：backend 全仓无此路由，但
    // tests/parity/corpus/oa4rust-ldap_config.json 明确要求 GET 返回 200
    // JSON（parity_runner.rs:157）。本 PR 不修，另开轮；此处保留红灯并给出
    // 可读失败信息，而不是 JSON 解析 panic。
    let config_resp = client
        .get(format!("{}/api/ldap/config", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .expect("ldap config request failed");

    assert_eq!(
        config_resp.status(),
        reqwest::StatusCode::OK,
        "A-class gap: GET /api/ldap/config must return 200 (O2OA parity)"
    );
    let config_text = config_resp.text().await.expect("ldap config body");
    let config_body: serde_json::Value = serde_json::from_str(&config_text).unwrap_or_else(|err| {
        panic!(
            "A-class gap: GET /api/ldap/config has no route (O2OA parity requires JSON 200); \
             raw body = {:?}, parse error = {}",
            config_text, err
        )
    });
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
