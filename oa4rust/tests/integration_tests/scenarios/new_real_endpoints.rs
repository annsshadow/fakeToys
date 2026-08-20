use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::{Client, StatusCode};
use serde_json::json;
use serde_json::Value;
use tracing::info;

use crate::integration_tests::db::TEST_DB;
use deadpool_postgres::Pool;

// ──────────────────────────────────────────────────────────────────────────────
// Real-PostgreSQL integration verification for newly-realized DB-touching endpoints.
//
// Each scenario spins up the full app on a random port, obtains the seeded admin
// token, seeds prerequisite rows directly via the deadpool Pool, then drives each
// endpoint through reqwest with auth. Goal: prove HTTP 200 + parseable ActionResult
// and genuine DB interaction (real rows read / written) — never a 500.
// ──────────────────────────────────────────────────────────────────────────────

fn uniq(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{}-{}-{}", prefix, std::process::id(), nanos)
}

async fn get_ok(base: &str, client: &Client, auth: &str, path: &str) -> Value {
    let resp = client
        .get(format!("{}{}", base, path))
        .header("Authorization", auth)
        .send()
        .await
        .unwrap_or_else(|e| panic!("GET {} failed: {}", path, e));
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "GET {} -> non-200: {}",
        path,
        resp.text().await.unwrap_or_default()
    );
    let v: Value = resp.json().await.unwrap_or_else(|e| panic!("GET {} bad json: {}", path, e));
    assert!(v.get("type").is_some(), "GET {}: no ActionResult.type", path);
    v
}

async fn post_ok(base: &str, client: &Client, auth: &str, path: &str, body: Value) -> Value {
    let resp = client
        .post(format!("{}{}", base, path))
        .header("Authorization", auth)
        .json(&body)
        .send()
        .await
        .unwrap_or_else(|e| panic!("POST {} failed: {}", path, e));
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "POST {} -> non-200: {}",
        path,
        resp.text().await.unwrap_or_default()
    );
    let v: Value = resp.json().await.unwrap_or_else(|e| panic!("POST {} bad json: {}", path, e));
    assert!(v.get("type").is_some(), "POST {}: no ActionResult.type", path);
    v
}

// ── AI: file download/scale/delete, mcp config list/get, index delete ──────────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn ai_endpoints_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    // Seed an x_ai_file row so the download endpoints return real data.
    let file_id = uniq("ai-file");
    let file_name = format!("{}.txt", file_id);
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_ai_file (id, xid, xname, xlength, xcreator) VALUES ($1, $2, $3, $4, $5)",
            &[&file_id, &file_id, &file_name, &(1024i64), &"it-admin"],
        )
        .await
        .expect("seed x_ai_file");
    }

    let dl = get_ok(&base, &client, &auth, &format!("/jaxrs/ai/file/{}/download", file_id)).await;
    assert_eq!(dl["data"]["id"].as_str(), Some(file_id.as_str()));
    info!(file_id = %file_id, "file_download returned real row");

    let dls = get_ok(&base, &client, &auth, &format!("/jaxrs/ai/file/{}/download/scale", file_id)).await;
    assert_eq!(dls["data"]["id"].as_str(), Some(file_id.as_str()));
    info!(file_id = %file_id, "file_download_scale returned real row");

    // DELETE then verify the row is gone (side effect).
    let del = get_ok(&base, &client, &auth, &format!("/jaxrs/ai/file/delete/{}", file_id)).await;
    assert_eq!(del["data"]["id"].as_str(), Some(file_id.as_str()));
    let c = pool.get().await.expect("pool");
    let remaining = c
        .query_opt("SELECT 1 FROM x_ai_file WHERE xid = $1", &[&file_id])
        .await
        .expect("check");
    assert!(remaining.is_none(), "file_delete did not remove the row");
    info!(file_id = %file_id, "file_delete removed the row");

    // Seed an x_ai_mcp_config row.
    let mcp_id = uniq("ai-mcp");
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_ai_mcp_config (id, name, url, default_model, enabled, is_base, is_extended, max_tokens) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &mcp_id,
                &format!("mcp-{}", mcp_id),
                &"http://example.test/mcp",
                &"gpt",
                &true,
                &false,
                &false,
                &(1000i64),
            ],
        )
        .await
        .expect("seed x_ai_mcp_config");
    }

    let list = get_ok(&base, &client, &auth, "/jaxrs/ai/config/list/mcp/paging/1/size/10").await;
    let mcp_items = list["data"]["data"].as_array().expect("mcp data array");
    assert!(
        mcp_items.iter().any(|r| r["id"].as_str() == Some(mcp_id.as_str())),
        "seeded mcp config not returned in list"
    );
    info!(mcp_id = %mcp_id, "config_list_mcp_paging returned real row");

    let get_mcp = get_ok(&base, &client, &auth, &format!("/jaxrs/ai/config/get/mcp/{}", mcp_id)).await;
    assert_eq!(get_mcp["data"]["id"].as_str(), Some(mcp_id.as_str()));
    info!(mcp_id = %mcp_id, "config_get_mcp returned real row");

    // Seed an x_ai_index row then delete it (side effect).
    let idx_id = uniq("ai-idx");
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_ai_index (id, doc_id, title) VALUES ($1, $2, $3)",
            &[&idx_id, &format!("doc-{}", idx_id), &"title"],
        )
        .await
        .expect("seed x_ai_index");
    }
    let del_idx = get_ok(&base, &client, &auth, &format!("/jaxrs/ai/index/delete/{}", idx_id)).await;
    assert_eq!(del_idx["data"]["id"].as_str(), Some(idx_id.as_str()));
    let c = pool.get().await.expect("pool");
    let idx_remaining = c
        .query_opt("SELECT 1 FROM x_ai_index WHERE id = $1", &[&idx_id])
        .await
        .expect("check");
    assert!(idx_remaining.is_none(), "index_delete did not remove the row");
    info!(idx_id = %idx_id, "index_delete removed the row");

    server_handle.abort();
    let _ = server_handle.await;
}

// ── query_core_express: get_cache_status (read-by-id + 404-safe; never 500) ────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn query_endpoints_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    let q_id = uniq("q");
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_query (id, name, query_type, count) VALUES ($1, $2, $3, $4)",
            &[&q_id, &"Q", &"sql", &"5"],
        )
        .await
        .expect("seed x_query");
    }

    let cached = get_ok(
        &base,
        &client,
        &auth,
        &format!("/jaxrs/query/core/express/cache/status/{}", q_id),
    ).await;
    assert_eq!(cached["data"]["queryId"].as_str(), Some(q_id.as_str()));
    assert_eq!(cached["data"]["cached"].as_bool(), Some(true));
    info!(q_id = %q_id, "get_cache_status returned cached=true for real row");

    // Non-existent id must NOT 500 — returns cached=false.
    let missing = get_ok(
        &base,
        &client,
        &auth,
        &format!("/jaxrs/query/core/express/cache/status/{}", uniq("q-missing")),
    ).await;
    assert_eq!(missing["data"]["cached"].as_bool(), Some(false));
    info!("get_cache_status returned cached=false for missing id (no 500)");

    server_handle.abort();
    let _ = server_handle.await;
}

// ── bbs_core_entity: create_reply (write → row persisted) ──────────────────────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn bbs_endpoints_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    let topic_id = uniq("topic");
    let resp = post_ok(
        &base,
        &client,
        &auth,
        "/jaxrs/bbs/core/entity/reply",
        json!({ "topicId": topic_id, "content": "hello reply", "creator": "it-admin" }),
    ).await;
    let reply_id = resp["data"]["id"].as_str().expect("reply id missing").to_string();
    assert!(!reply_id.is_empty());

    let c = pool.get().await.expect("pool");
    let row = c
        .query_opt(
            "SELECT id, topic_id, content FROM x_bbs_reply WHERE id = $1",
            &[&reply_id],
        )
        .await
        .expect("check reply");
    assert!(row.is_some(), "create_reply did not persist the row");
    info!(reply_id = %reply_id, "create_reply persisted real row");

    server_handle.abort();
    let _ = server_handle.await;
}

// ── portal: list_portal_category (real category read) ─────────────────────────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn portal_endpoints_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    let cat = uniq("portal-cat");
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_portal (id, name, alias, description, portal_category) VALUES ($1, $2, $3, $4, $5)",
            &[&uniq("portal"), &"Portal", &"alias", &"desc", &cat],
        )
        .await
        .expect("seed x_portal");
    }

    let list = get_ok(&base, &client, &auth, "/jaxrs/portalcategory/list").await;
    let cats = list["data"]["data"].as_array().expect("category data array");
    assert!(
        cats.iter().any(|r| r["id"].as_str() == Some(cat.as_str())),
        "seeded portal category not returned"
    );
    info!(cat = %cat, "list_portal_category returned real category");

    server_handle.abort();
    let _ = server_handle.await;
}

// ── component_assemble_control: update_control_config + get_control_config ─────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn component_assemble_control_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    // update_control_config is registered as GET but reads a JSON body.
    let upd = client
        .get(format!("{}/jaxrs/component_assemble_control/update/control/config", base))
        .header("Authorization", &auth)
        .json(&json!({ "enabled": true, "maxComponentCount": 42, "allowCustomComponents": true }))
        .send()
        .await
        .expect("update_control_config request");
    assert_eq!(
        upd.status(),
        StatusCode::OK,
        "update_control_config -> non-200: {}",
        upd.text().await.unwrap_or_default()
    );
    let upd_body: Value = upd.json().await.expect("update_control_config json");
    assert_eq!(upd_body["data"]["updated"].as_bool(), Some(true));

    // Side effect: config row 'default' exists with enabled=true.
    let c = pool.get().await.expect("pool");
    let cfg = c
        .query_opt(
            "SELECT enabled FROM x_component_assemble_control_config WHERE id = 'default'",
            &[],
        )
        .await
        .expect("check config");
    assert!(cfg.is_some(), "update_control_config did not persist the row");
    info!("update_control_config persisted config row");

    // get_control_config reads the real row back.
    let get_cfg = get_ok(&base, &client, &auth, "/jaxrs/component_assemble_control/get/control/config").await;
    assert_eq!(get_cfg["data"]["enabled"].as_bool(), Some(true));
    assert_eq!(get_cfg["data"]["maxComponentCount"].as_i64(), Some(42));
    info!("get_control_config returned real config");

    server_handle.abort();
    let _ = server_handle.await;
}

// ── correlation_service_processing: unlink_service (write → row removed) ───────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn correlation_endpoints_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    let corr_id = uniq("corr");
    let source_type = "topic";
    let source_id = uniq("src");
    let target_type = "doc";
    let target_id = uniq("tgt");
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_correlation (id, \"type\", person_id, target_id) VALUES ($1, $2, $3, $4)",
            &[&corr_id, &source_type, &source_id, &target_id],
        )
        .await
        .expect("seed x_correlation");
    }

    let unlink = post_ok(
        &base,
        &client,
        &auth,
        &format!(
            "/jaxrs/correlation/service/processing/unlink/{}/{}/{}/{}",
            source_type, source_id, target_type, target_id
        ),
        json!({}),
    ).await;
    assert_eq!(unlink["data"]["unlinked"].as_bool(), Some(true));

    let c = pool.get().await.expect("pool");
    let remaining = c
        .query_opt("SELECT 1 FROM x_correlation WHERE id = $1", &[&corr_id])
        .await
        .expect("check");
    assert!(remaining.is_none(), "unlink_service did not remove the row");
    info!(corr_id = %corr_id, "unlink_service removed the row");

    server_handle.abort();
    let _ = server_handle.await;
}

// ── organization_assemble_control: 5 realized read handlers (real DB) ──────────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn org_assemble_control_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    let role_id = uniq("role");
    let unit_id = uniq("unit");
    let idn_id = uniq("idn");
    let person_id = uniq("person");
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_org_role (id, name) VALUES ($1, $2)",
            &[&role_id, &"Role"],
        )
        .await
        .expect("seed x_org_role");
        c.execute(
            "INSERT INTO x_org_unit (id, name) VALUES ($1, $2)",
            &[&unit_id, &"Unit"],
        )
        .await
        .expect("seed x_org_unit");
        c.execute(
            "INSERT INTO x_org_identity (id, name) VALUES ($1, $2)",
            &[&idn_id, &"Identity"],
        )
        .await
        .expect("seed x_org_identity");
        c.execute(
            "INSERT INTO x_org_person (id, name) VALUES ($1, $2)",
            &[&person_id, &person_id],
        )
        .await
        .expect("seed x_org_person");
    }

    let role = get_ok(
        &base,
        &client,
        &auth,
        &format!("/jaxrs/organization/assemble/control/role/{}", role_id),
    ).await;
    assert_eq!(role["data"]["id"].as_str(), Some(role_id.as_str()));
    info!(role_id = %role_id, "organization_assemble_control_role_flag returned real row");

    let unit = get_ok(
        &base,
        &client,
        &auth,
        &format!("/jaxrs/organization/assemble/control/unit/{}", unit_id),
    ).await;
    assert_eq!(unit["data"]["id"].as_str(), Some(unit_id.as_str()));
    info!(unit_id = %unit_id, "organization_assemble_control_unit_flag returned real row");

    let idn = get_ok(
        &base,
        &client,
        &auth,
        &format!("/jaxrs/organization/assemble/control/identity/{}", idn_id),
    ).await;
    assert_eq!(idn["data"]["id"].as_str(), Some(idn_id.as_str()));
    info!(idn_id = %idn_id, "identity_flag returned real row");

    let person = get_ok(
        &base,
        &client,
        &auth,
        &format!("/jaxrs/organization/assemble/control/personcard/{}", person_id),
    ).await;
    assert_eq!(person["data"]["id"].as_str(), Some(person_id.as_str()));
    info!(person_id = %person_id, "personcard_flag returned real row");

    let like = post_ok(
        &base,
        &client,
        &auth,
        "/jaxrs/organization/assemble/control/person/list/like",
        json!({ "name": person_id }),
    ).await;
    let people = like["data"]["data"].as_array().expect("person list data");
    assert!(
        people.iter().any(|p| p["id"].as_str() == Some(person_id.as_str())),
        "seeded person not returned by list/like"
    );
    info!(person_id = %person_id, "person_list_like returned real row");

    server_handle.abort();
    let _ = server_handle.await;
}

// ── console / jpush / organization_assemble_express status endpoints ───────────
#[tokio::test]
#[ignore = "requires a running database server"]
pub async fn console_jpush_express_flow() {
    let pool = (**TEST_DB.get().expect("db not initialized")).clone();
    let (_addr, server_handle, token) = crate::integration_tests::helpers::setup_test_server(pool.clone())
        .await
        .expect("start server");
    let client = Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
    let base = format!("http://{}", _addr);
    let auth = format!("Bearer {}", token);

    let status = get_ok(&base, &client, &auth, "/jaxrs/console/status").await;
    assert!(status["data"]["status"].as_str().is_some());
    info!("console get_status returned real status");

    // Seed a real metric row so get_metric returns genuine value/unit (proves xname column fix).
    let m_id = uniq("metric");
    {
        let c = pool.as_pg().unwrap().get().await.expect("pool");
        c.execute(
            "INSERT INTO x_console_metric (id, xname, xvalue, xunit) VALUES ($1, 'cpu', '42', '%')",
            &[&m_id],
        )
        .await
        .expect("seed x_console_metric");
    }
    let metric = get_ok(&base, &client, &auth, "/jaxrs/console/metric/cpu").await;
    assert_eq!(metric["data"]["name"].as_str(), Some("cpu"));
    assert_eq!(metric["data"]["value"].as_i64(), Some(42), "get_metric returned real xvalue");
    assert_eq!(metric["data"]["unit"].as_str(), Some("%"), "get_metric returned real xunit");
    info!("console get_metric returned real metric entry");

    let hello = get_ok(&base, &client, &auth, "/hello/world").await;
    assert_eq!(hello["data"]["status"].as_str(), Some("ok"));
    info!("jpush hello returned ok + db counts");

    let express = get_ok(
        &base,
        &client,
        &auth,
        "/jaxrs/organization/assemble/express/status/get",
    ).await;
    assert!(express["data"]["status"].as_str().is_some());
    info!("organization_assemble_express get_express_status returned real status");

    server_handle.abort();
    let _ = server_handle.await;
}
