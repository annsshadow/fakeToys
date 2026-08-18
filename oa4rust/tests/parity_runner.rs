//! OA4Rust parity-regression suite (Phase 4 / U4.1)
//!
//! Two modes, both run as `#[ignore]` integration tests (require a live
//! PostgreSQL server, hence `--ignored`):
//!
//! 1. **RECORD** (`parity_record`): starts a real server against the test
//!    database and records oa4rust endpoint responses into
//!    `tests/parity/corpus/oa4rust-<name>.json`. This is the Rust-side
//!    baseline corpus.
//!
//! 2. **VERIFY** (`parity_verify`): loads the recorded oa4rust baseline and,
//!    if an o2server corpus dir `tests/parity/corpus-o2server/` exists
//!    (external TODO — o2server is NOT available in this environment), replays
//!    a field-by-field contract diff via [`diff`]. Endpoints missing on either
//!    side, status mismatches, and value divergences are collected and
//!    reported. When the o2server corpus is absent the test skips cleanly with
//!    a clear TODO, so it is safe to run in CI without o2server.
//!
//! The `diff` routine is the real, compilable comparison primitive; the only
//! missing piece for a full takeover verification is the o2server corpus.
//!
//! Run:
//! ```bash
//! cargo test --test parity_runner -- --ignored --nocapture
//! ```

mod integration_tests;

use integration_tests::db::init_test_database;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;

/// Compare two response corpora (oa4rust vs o2server) field-by-field.
///
/// Returns a list of human-readable difference descriptions. The o2server
/// side is currently a TODO on the caller's side; this function is the real,
/// compilable comparison primitive.
pub fn diff(a: &Value, b: &Value) -> Vec<String> {
    let mut out = Vec::new();
    diff_impl(a, b, "", &mut out);
    out
}

fn diff_impl(a: &Value, b: &Value, path: &str, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(ao), Value::Object(bo)) => {
            let keys: std::collections::BTreeSet<&String> =
                ao.keys().chain(bo.keys()).collect();
            for k in keys {
                let child = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", path, k)
                };
                match (ao.get(k), bo.get(k)) {
                    (Some(av), Some(bv)) => diff_impl(av, bv, &child, out),
                    (Some(_), None) => out.push(format!("{}: present in oa4rust, missing in o2server", child)),
                    (None, Some(_)) => out.push(format!("{}: missing in oa4rust, present in o2server", child)),
                    (None, None) => {}
                }
            }
        }
        (Value::Array(aa), Value::Array(ba)) => {
            if aa.len() != ba.len() {
                out.push(format!(
                    "{}: array length differs (oa4rust={}, o2server={})",
                    path,
                    aa.len(),
                    ba.len()
                ));
            }
            for (i, (av, bv)) in aa.iter().zip(ba.iter()).enumerate() {
                diff_impl(av, bv, &format!("{}[{}]", path, i), out);
            }
        }
        (av, bv) => {
            if av != bv {
                out.push(format!("{}: value differs (oa4rust={:?}, o2server={:?})", path, av, bv));
            }
        }
    }
}

/// Load a corpus directory (`<prefix>-<name>.json`) into a name→record map.
/// Returns an empty map if the directory does not exist (so callers can decide
/// whether to skip or fail).
fn load_corpus(dir: &Path) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    if !dir.exists() {
        return map;
    }
    let entries = fs::read_dir(dir).expect("read corpus dir");
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let text = match fs::read_to_string(&p) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let v: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
            map.insert(name.to_string(), v);
        }
    }
    map
}

#[ignore = "requires a running PostgreSQL server"]
#[test]
fn parity_record() {
    let _ctx = init_test_database();
    let pool = _ctx.pool();

    let rt = tokio::runtime::Runtime::new().expect("failed to build runtime");
    rt.block_on(async {
        let (_addr, server_handle, token) = integration_tests::helpers::setup_test_server((*pool).clone())
            .await
            .expect("failed to start test server");

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build reqwest client");

        let base = format!("http://{}", _addr);
        let auth_header = format!("Bearer {}", token);

        // 跨模块已知可用（无路径参数）的只读端点，录制 oa4rust 基线响应。
        // 带路径参数的端点（如 /jaxrs/ai/file/{id}/download）需种子数据，留待 o2server 语料齐备后扩展。
        let endpoints: Vec<(&str, &str, &str)> = vec![
            ("cms_document_list", "GET", "/jaxrs/cms_assemble_control/data/document"),
            ("program_applications", "GET", "/jaxrs/program/applications"),
            ("ldap_config", "GET", "/jaxrs/ldap/config"),
            ("portalcategory_list", "GET", "/jaxrs/portalcategory/list"),
            ("console_status", "GET", "/jaxrs/console/status"),
            ("console_metric_cpu", "GET", "/jaxrs/console/metric/cpu"),
            ("control_config", "GET", "/jaxrs/component_assemble_control/get/control/config"),
            ("ai_mcp_config_list", "GET", "/jaxrs/ai/config/list/mcp/paging/1/size/10"),
        ];

        let out_dir = Path::new("tests/parity/corpus");
        fs::create_dir_all(out_dir).expect("failed to create corpus dir");

        for (name, method, path) in endpoints {
            let url = format!("{}{}", base, path);
            let builder = match method {
                "POST" => client.post(&url),
                _ => client.get(&url),
            };
            let resp = builder
                .header("Authorization", &auth_header)
                .send()
                .await
                .unwrap_or_else(|e| panic!("request to {} failed: {}", path, e));

            let status = resp.status().as_u16();
            let body: Value = resp.json().await.unwrap_or(Value::Null);

            let record = json!({
                "name": name,
                "method": method,
                "path": path,
                "status": status,
                "body": body,
            });

            let file = out_dir.join(format!("oa4rust-{}.json", name));
            fs::write(&file, serde_json::to_string_pretty(&record).expect("serialize"))
                .unwrap_or_else(|e| panic!("write {} failed: {}", file.display(), e));
            println!("recorded {} -> {} (status {})", name, file.display(), status);
        }

        server_handle.abort();
        let _ = server_handle.await;
    });
}

/// Verify the recorded oa4rust baseline against an o2server corpus.
///
/// This is the contract-diff half of U4.1. It does NOT need a running server
/// (the oa4rust baseline is already recorded). It loads
/// `tests/parity/corpus/oa4rust-<name>.json` and, when
/// `tests/parity/corpus-o2server/` exists, replays a field-by-field diff per
/// endpoint via [`diff`].
///
/// o2server is NOT available in this environment, so the o2server corpus is an
/// external TODO. When it is absent the test skips cleanly (no false failure in
/// CI) and prints the exact next step for a human to provide the corpus.
#[ignore = "requires a running PostgreSQL server + o2server corpus"]
#[test]
fn parity_verify() {
    let baseline_dir = Path::new("tests/parity/corpus");
    let o2_dir = Path::new("tests/parity/corpus-o2server");

    let baseline = load_corpus(baseline_dir);
    assert!(
        !baseline.is_empty(),
        "oa4rust baseline corpus missing — run `parity_record` first"
    );

    if !o2_dir.exists() {
        println!(
            "SKIP parity_verify: o2server corpus dir {:?} not found.\n\
             \x20  External TODO — provide o2server responses under \
             tests/parity/corpus-o2server/oa4rust-<name>.json (same schema as \
             the oa4rust baseline) to enable contract diff.",
            o2_dir
        );
        return;
    }

    let o2 = load_corpus(o2_dir);
    let mut findings: Vec<String> = Vec::new();

    for (name, base_rec) in &baseline {
        let o2_rec = match o2.get(name) {
            Some(r) => r,
            None => {
                findings.push(format!("{}: present in oa4rust, missing in o2server", name));
                continue;
            }
        };
        let base_status = base_rec.get("status").and_then(|v| v.as_u64()).unwrap_or(0);
        let o2_status = o2_rec.get("status").and_then(|v| v.as_u64()).unwrap_or(0);
        if base_status != o2_status {
            findings.push(format!(
                "{}: status differs (oa4rust={}, o2server={})",
                name, base_status, o2_status
            ));
        }
        let base_body = base_rec.get("body").unwrap_or(&Value::Null);
        let o2_body = o2_rec.get("body").unwrap_or(&Value::Null);
        findings.extend(diff(base_body, o2_body));
    }
    for name in o2.keys() {
        if !baseline.contains_key(name) {
            findings.push(format!("{}: present in o2server, missing in oa4rust", name));
        }
    }

    if findings.is_empty() {
        println!(
            "parity_verify: OK — {} endpoints contract-matched between oa4rust and o2server",
            baseline.len()
        );
    } else {
        println!("parity_verify: {} finding(s):", findings.len());
        for f in &findings {
            println!("  - {}", f);
        }
        panic!(
            "parity_verify failed with {} finding(s) — see output above",
            findings.len()
        );
    }
}
