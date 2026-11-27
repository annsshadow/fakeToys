// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R605a 契约：v2 申诉状态机三端点必须带当前状态前置谓词。
//! 核心用例：非法跃迁（status2 start / status0 reset / status0 end）必须被拒（真父必 RED）。
//! 守卫用例：合法跃迁、owner gate、start_check 语义真父也通过。

use std::path::PathBuf;

use attendance_assemble_control::{
    v2_appeal_end_process, v2_appeal_reset_status, v2_appeal_start_check, v2_appeal_start_process,
};
use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::response::ActionResult;

const SCRATCH: &str = "oa_r605a_state";
const SCRATCH_GUARD: &str = "oa_r605a_guard";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(mgr)
        .max_size(4)
        .build()
        .unwrap()
}

async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
    c.execute(
        &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {dbname}"), &[])
        .await
        .expect("create scratch");
    drop(c);
    let pool = pool_to(dbname);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 迁移");
    pool
}

async fn drop_scratch(dbname: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(concat!(
        "INSERT INTO x_attendance_v2_appeal_info (id, user_id, status, creator_person, send_status) VALUES ",
        "('a0', 'alice', 0, 'alice', false), ",
        "('a1', 'alice', 1, 'alice', false), ",
        "('a1b', 'alice', 1, 'alice', false), ",
        "('a2', 'alice', 2, 'alice', false), ",
        "('b0', 'bob', 0, 'bob', false);",
    )).await.expect("seed");
}

fn session(person: &str) -> shared::session::Session {
    shared::session::Session {
        token: format!("r605a-{person}"),
        person_unique: person.to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
    }
}

fn raw<T: serde::Serialize>(v: &ActionResult<T>) -> String {
    serde_json::to_string(v).unwrap_or_default()
}
fn is_error<T>(v: &ActionResult<T>) -> bool {
    v.r#type.as_deref() == Some("error")
}
fn is_success<T>(v: &ActionResult<T>) -> bool {
    v.r#type.as_deref() == Some("success")
}
fn msg<T>(v: &ActionResult<T>) -> &str {
    v.message.as_deref().unwrap_or("")
}
fn status_of(v: &ActionResult<Value>) -> Option<i64> {
    v.data.as_ref()?.get("status")?.as_i64()
}

// 核心用例：非法跃迁被拒（真父必 RED）。
#[tokio::test]
async fn illegal_transitions_are_rejected() {
    let pool = fresh_scratch(SCRATCH).await;
    seed(&pool).await;

    // 非法1：status=2（已通过）不可再 start/process。
    let r = v2_appeal_start_process(
        Extension(pool.clone()),
        Extension(session("alice")),
        Path("a2".to_string()),
        Json(json!({"jobId": "job-illegal"})),
    )
    .await
    .expect("start/process 不得 500");
    assert!(is_error(&r.0), "status2 start 应被拒，实际: {}", raw(&r.0));
    assert!(
        msg(&r.0).contains("expected state"),
        "错误文案: {}",
        raw(&r.0)
    );

    // 非法2：status=0（待处理）不可 reset/status。
    let r = v2_appeal_reset_status(
        Extension(pool.clone()),
        Extension(session("alice")),
        Path("a0".to_string()),
    )
    .await
    .expect("reset/status 不得 500");
    assert!(is_error(&r.0), "status0 reset 应被拒，实际: {}", raw(&r.0));
    assert!(
        msg(&r.0).contains("expected state"),
        "错误文案: {}",
        raw(&r.0)
    );

    // 非法3：status=0（未发起）不可 end/process 直接写通过。
    let r = v2_appeal_end_process(
        Extension(pool.clone()),
        Extension(session("alice")),
        Path("a0".to_string()),
        Json(json!({"status": 2})),
    )
    .await
    .expect("end/process 不得 500");
    assert!(is_error(&r.0), "status0 end 应被拒，实际: {}", raw(&r.0));
    assert!(
        msg(&r.0).contains("expected state"),
        "错误文案: {}",
        raw(&r.0)
    );

    // 源码守卫：三端点 SQL 均带 AND status 前置；manager_status UPDATE 保持零改。
    let src = std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"))
        .expect("读 lib.rs");
    assert!(
        src.contains("job_id = $3, update_time = NOW() WHERE id = $1 AND status = $4"),
        "start/process 缺 status 谓词"
    );
    assert!(
        src.contains("send_status = FALSE, update_time = NOW() WHERE id = $1 AND status = $3"),
        "reset/status 缺 status 谓词"
    );
    assert!(
        src.contains("COALESCE($3, job_id), update_time = NOW() WHERE id = $1 AND status = $4"),
        "end/process 缺 status 谓词"
    );
    assert!(
        src.contains("update_status_admin_person = $3, update_time = NOW() WHERE id = $1\","),
        "manager_status UPDATE 零改守卫"
    );

    drop_scratch(SCRATCH).await;
}

// 守卫用例：合法跃迁与既有语义不受影响（真父也通过）。
#[tokio::test]
async fn legal_transitions_and_owner_guards_unchanged() {
    let pool = fresh_scratch(SCRATCH_GUARD).await;
    seed(&pool).await;

    // 合法1：status=0 → start/process 成功写入 status=1。
    let r = v2_appeal_start_process(
        Extension(pool.clone()),
        Extension(session("alice")),
        Path("a0".to_string()),
        Json(json!({"jobId": "job-ok"})),
    )
    .await
    .expect("start/process 不得 500");
    assert!(is_success(&r.0), "合法 start 应成功: {}", raw(&r.0));
    assert_eq!(status_of(&r.0), Some(1));

    // 合法2：status=1 → reset/status 成功回退 status=0。
    let r = v2_appeal_reset_status(
        Extension(pool.clone()),
        Extension(session("alice")),
        Path("a1".to_string()),
    )
    .await
    .expect("reset/status 不得 500");
    assert!(is_success(&r.0), "合法 reset 应成功: {}", raw(&r.0));
    assert_eq!(status_of(&r.0), Some(0));

    // 合法3：status=1 → end/process{status:2} 成功写入 status=2。
    let r = v2_appeal_end_process(
        Extension(pool.clone()),
        Extension(session("alice")),
        Path("a1b".to_string()),
        Json(json!({"status": 2})),
    )
    .await
    .expect("end/process 不得 500");
    assert!(is_success(&r.0), "合法 end 应成功: {}", raw(&r.0));
    assert_eq!(status_of(&r.0), Some(2));

    // owner gate：bob 非 owner 读 alice 私有申诉仍 Forbidden（零改语义）。
    let bob = v2_appeal_start_process(
        Extension(pool.clone()),
        Extension(session("bob")),
        Path("a0".to_string()),
        Json(json!({"jobId": "job-bob"})),
    )
    .await;
    assert!(bob.is_err(), "bob 非 owner 应被 Forbidden 拒绝");

    // start_check：status=2 不可重复发起（设计意图不变）。
    let chk = v2_appeal_start_check(
        Extension(pool.clone()),
        Extension(session("alice")),
        Path("a2".to_string()),
    )
    .await
    .expect("start/check 不得 500");
    let chk_val = chk
        .0
        .data
        .as_ref()
        .and_then(|d| d.get("value"))
        .and_then(|v| v.as_bool());
    assert_eq!(chk_val, Some(false), "status2 start_check 应不可发起");

    drop_scratch(SCRATCH_GUARD).await;
}
