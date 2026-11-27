// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R597b contract test: appeal list status domain alignment (RED -> GREEN).
//! Zero source changes. Deterministic source guard + DB-gated real-handler cases.
use axum::extract::Extension;
use deadpool_postgres::tokio_postgres::Client as PgClient;
use deadpool_postgres::Pool;
use serde_json::Value;
use std::path::PathBuf;

const SCRATCH_DB: &str = "oa_r597b_scratch";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL").expect("R597b needs DATABASE_URL");
    let mut cfg: deadpool_postgres::tokio_postgres::Config = url.parse().expect("bad DATABASE_URL");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(mgr)
        .max_size(4)
        .build()
        .unwrap()
}

fn as_value<T: serde::Serialize>(v: &T) -> Value {
    serde_json::to_value(v).expect("serialize")
}

fn lib_source() -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    std::fs::read_to_string(p).expect("read lib.rs")
}

fn list_fn_body(src: &str) -> String {
    let i = src
        .find("pub async fn list_appeal_records")
        .expect("list fn");
    let tail = &src[i..];
    let end = tail
        .find("pub async fn submit_appeal")
        .unwrap_or(tail.len());
    tail[..end].to_string()
}

#[test]
fn r597b_source_guard_list_coalesces_audit_status() {
    let body = list_fn_body(&lib_source());
    assert!(
        body.contains("COALESCE"),
        "list SELECT must COALESCE audit_status/appeal_status: RED before fix"
    );
    assert!(
        body.contains("NULLIF(audit_status"),
        "list SELECT must NULLIF-blank audit_status before coalescing"
    );
    assert!(
        body.contains("AS appeal_status"),
        "coalesced expr must alias to appeal_status"
    );
}

async fn fresh_scratch() {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin conn");
    c.execute(
        &format!("DROP DATABASE IF EXISTS {} WITH (FORCE)", SCRATCH_DB),
        &[],
    )
    .await
    .unwrap();
    c.execute(&format!("CREATE DATABASE {}", SCRATCH_DB), &[])
        .await
        .unwrap();
    drop(c);
    let pool = pool_to(SCRATCH_DB);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch migrations");
    pool.get()
        .await
        .unwrap()
        .execute("DELETE FROM x_attendance_appeal_info", &[])
        .await
        .unwrap();
}

async fn seed(client: &PgClient, id: &str, appeal: &str, audit: Option<&str>) {
    let id = id.to_string();
    let person = "r597b-person".to_string();
    let appeal = appeal.to_string();
    let audit: Option<String> = audit.map(|s| s.to_string());
    client
        .execute(
            "INSERT INTO x_attendance_appeal_info (id, person_id, appeal_status, creator, create_time, audit_status) VALUES ($1,$2,$3,$4,NOW(),$5)",
            &[&id, &person, &appeal, &person, &audit],
        )
        .await
        .unwrap();
}

async fn listed_status(pool: &Pool, id: &str) -> String {
    let r = attendance::list_appeal_records(Extension(pool.clone()))
        .await
        .expect("list handler");
    let v = as_value(&r.0);
    let arr = v.get("data").and_then(|x| x.as_array()).unwrap();
    let row = arr
        .iter()
        .find(|x| x.pointer("/id").and_then(|s| s.as_str()) == Some(id))
        .expect("row present");
    row.pointer("/status")
        .and_then(|s| s.as_str())
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn r597b_real_handler_three_cases() {
    if !shared::testing::is_db_available().await {
        println!("R597B_SKIP: no database (set DATABASE_URL); source guard still covers RED/GREEN");
        return;
    }
    fresh_scratch().await;
    let pool = pool_to(SCRATCH_DB);
    let c = pool.get().await.unwrap();

    // Case A: legacy audit wrote audit_status=approved, appeal_status stays appealed.
    seed(&c, "r597b-A", "appealed", Some("approved")).await;
    // Case B: V2 audit wrote audit_status=rejected.
    seed(&c, "r597b-B", "appealed", Some("rejected")).await;
    // Case C: no audit_status -> fall back to appeal_status (DB-direct legacy write).
    seed(&c, "r597b-C", "approved", None).await;
    let diag = c
        .query(
            "SELECT id, appeal_status, audit_status FROM x_attendance_appeal_info ORDER BY id",
            &[],
        )
        .await
        .unwrap();
    for r in &diag {
        let did: String = r.get(0);
        let dap: String = r.get(1);
        let dau: Option<String> = r.get(2);
        println!("R597B_DIAG id={} appeal={} audit={:?}", did, dap, dau);
    }
    drop(c);

    let a = listed_status(&pool, "r597b-A").await;
    let b = listed_status(&pool, "r597b-B").await;
    let d = listed_status(&pool, "r597b-C").await;
    println!("R597B_CASE_A={} CASE_B={} CASE_C={}", a, b, d);
    assert_eq!(a, "approved", "legacy audit_status must surface in list");
    assert_eq!(b, "rejected", "V2 audit_status must surface in list");
    assert_eq!(
        d, "approved",
        "blank audit_status falls back to appeal_status"
    );
}
