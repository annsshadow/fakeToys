// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O25：`x_timer_job` 的真实行为契约（once 任务）。
//!
//! 修复前的三个既有缺陷：
//! 1. 写入口把 `fire_at` / `created_at` 按 String 绑定 TIMESTAMP → 真实 POST 500；
//! 2. ticker 每 30s 重复 fire，且不检查 `fired_at` / `cancelled_at`；
//! 3. `restore` 从未接线，重启后未完成 job 彻底失联。
//!
//! 本文件按节点拆成独立入口，由 integration_runner 分别调用。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use tokio::time::sleep;

use crate::integration_tests::db::{TestContext, TestPool};
use crate::integration_tests::helpers::{seed_test_data, setup_test_server};
use processplatform_service_processing::timer::TimerRegistry;
use shared::session::SessionManager;

fn uniq(prefix: &str) -> String {
    format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

/// O25 节点 1：真实 POST `timer/start` → 200，且回库行存在、字段类型与值正确。
pub async fn timer_start_write_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    let work_id = uniq("tw");
    let pg = pool.as_pg().expect("postgres pool");
    {
        let c = pg.get().await.expect("pool client");
        c.execute(
            "INSERT INTO x_work (id, title, process, creator) VALUES ($1, 'Timer Work', 'proc-timer', 'it-admin')",
            &[&work_id],
        )
        .await
        .expect("seed x_work for timer job FK");
    }

    // 未来 1 小时，避免 30s ticker 在用例内提前触发。
    let fire_at = Utc::now().naive_utc() + chrono::Duration::hours(1);
    let fire_at_str = fire_at.format("%Y-%m-%d %H:%M:%S").to_string();

    let resp = client
        .post(format!(
            "{base}/api/processplatform/service/processing/timer/start"
        ))
        .header("Authorization", &auth)
        .json(&json!({ "workId": work_id, "fireAt": fire_at_str }))
        .send()
        .await
        .expect("POST timer/start failed");
    let status = resp.status();
    let body: Value = resp.json().await.expect("timer/start non-json body");
    assert_eq!(
        status,
        StatusCode::OK,
        "POST timer/start 应为 200（修前 TIMESTAMP 按 String 绑定是 500）：{body}"
    );
    let job_id = body["data"]["jobId"]
        .as_str()
        .expect("timer/start 响应应含 data.jobId")
        .to_string();
    assert_eq!(body["data"]["workId"].as_str(), Some(work_id.as_str()));

    let row = pg
        .get()
        .await
        .expect("pool client")
        .query_opt(
            "SELECT fire_at, created_at, cron, kind, fired_at, cancelled_at \
             FROM x_timer_job WHERE id = $1",
            &[&job_id],
        )
        .await
        .expect("read back x_timer_job")
        .unwrap_or_else(|| panic!("timer/start 返回 200 但库里没有 job {job_id}"));

    let stored_fire_at: NaiveDateTime = row.get("fire_at");
    let stored_created_at: NaiveDateTime = row.get("created_at");
    assert_eq!(
        stored_fire_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        fire_at_str,
        "fire_at 应原样落库（NaiveDateTime 绑定，不被截断/改写）"
    );
    assert!(
        stored_created_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str()
            > "2020-01-01 00:00:00",
        "created_at 应落真实时间，实际 {stored_created_at}"
    );
    assert_eq!(row.get::<_, Option<String>>("cron"), None);
    assert_eq!(row.get::<_, String>("kind"), "once");
    assert_eq!(
        row.get::<_, Option<NaiveDateTime>>("fired_at"),
        None,
        "刚创建的 job 不应已 fired"
    );
    assert_eq!(
        row.get::<_, Option<NaiveDateTime>>("cancelled_at"),
        None,
        "刚创建的 job 不应已取消"
    );

    println!("timer/start 写后回库: job={job_id} fire_at={stored_fire_at}");

    server.abort();
    let _ = server.await;
}

async fn seed_work(pg: &deadpool_postgres::Pool, work_id: &str) {
    pg.get()
        .await
        .expect("pool client")
        .execute(
            "INSERT INTO x_work (id, title, process, creator) VALUES ($1, 'Timer Work', 'proc-timer', 'it-admin')",
            &[&work_id],
        )
        .await
        .expect("seed x_work for timer job FK");
}

async fn read_fired_at(pg: &deadpool_postgres::Pool, job_id: &str) -> Option<NaiveDateTime> {
    pg.get()
        .await
        .expect("pool client")
        .query_one("SELECT fired_at FROM x_timer_job WHERE id = $1", &[&job_id])
        .await
        .expect("read fired_at")
        .get("fired_at")
}

/// O25 节点 2：到期的 once job 只执行一次——连续 ≥3 个 tick 后 `fired_at`
/// 仍只被写一次，且该 job 已从内存待执行集合移除。
pub async fn timer_once_fires_once_flow(pool: Arc<TestPool>) {
    let pg = pool.as_pg().expect("postgres pool").clone();
    let work_id = uniq("tw");
    seed_work(&pg, &work_id).await;

    let registry = TimerRegistry::with_pool(pg.clone());
    let fire_at = Utc::now().naive_utc() - chrono::Duration::seconds(1);
    let job_id = registry
        .start(work_id, None, fire_at, None)
        .await
        .expect("timer.start 应写入 x_timer_job");

    // 200ms 一个 tick：等待 900ms = ≥4 个 tick 周期。
    registry.start_background_with_interval(Duration::from_millis(200));
    sleep(Duration::from_millis(900)).await;

    let first = read_fired_at(&pg, &job_id)
        .await
        .expect("到期 job 应在首个 tick 被 fire（fired_at 非空）");
    assert!(
        registry.jobs.read().await.get(&job_id).is_none(),
        "fire 后必须从待执行集合移除，否则下一个 tick 会再次执行"
    );

    // 再等 700ms（≥3 个 tick）：fired_at 必须未被覆盖。
    sleep(Duration::from_millis(700)).await;
    let second = read_fired_at(&pg, &job_id)
        .await
        .expect("fired_at 不应被清空");
    assert_eq!(
        first, second,
        "fired_at 只能被写一次；再等 3 个 tick 后不应变化（实际 {first} → {second}）"
    );

    println!("timer once: job={job_id} fired_at={first}（≥3 tick 后未变）");
}

async fn seed_job(
    pg: &deadpool_postgres::Pool,
    job_id: &str,
    work_id: &str,
    fire_at: NaiveDateTime,
    fired_at: Option<NaiveDateTime>,
) {
    pg.get()
        .await
        .expect("pool client")
        .execute(
            "INSERT INTO x_timer_job (id, work_id, task_id, fire_at, cron, created_at, fired_at, kind) \
             VALUES ($1, $2, NULL, $3, NULL, $4, $5, 'once')",
            &[&job_id, &work_id, &fire_at, &Utc::now().naive_utc(), &fired_at],
        )
        .await
        .expect("seed x_timer_job");
}

/// O25 节点 3：`router()` 启动时接线 `restore`，三种情况都必须成立。
/// - 未到期（A）：恢复后重新排期，到期时触发一次；
/// - 已到期未执行（B）：恢复后的首个 tick 立即执行一次；
/// - 已执行过（C）：restore 不加载，重启后绝不重放。
pub async fn timer_restore_cases_flow(pool: Arc<TestPool>) {
    let pg = pool.as_pg().expect("postgres pool").clone();
    let now = Utc::now().naive_utc();

    let work_id = uniq("tw");
    seed_work(&pg, &work_id).await;

    let job_a = uniq("tj"); // 未到期
    let job_b = uniq("tj"); // 已到期未执行
    let job_c = uniq("tj"); // 已执行过
    seed_job(
        &pg,
        &job_a,
        &work_id,
        now + chrono::Duration::seconds(2),
        None,
    )
    .await;
    seed_job(
        &pg,
        &job_b,
        &work_id,
        now - chrono::Duration::seconds(5),
        None,
    )
    .await;
    let c_fired_at = now - chrono::Duration::seconds(10);
    seed_job(
        &pg,
        &job_c,
        &work_id,
        now - chrono::Duration::seconds(5),
        Some(c_fired_at),
    )
    .await;

    // 走真实启动路径：create_app → processplatform router → restore → ticker。
    let (_addr, server, _token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");

    // ticker 默认 30s；A 在 +2s 到期，最迟在恢复后的第二个 tick 触发。
    let deadline = tokio::time::Instant::now() + Duration::from_secs(35);
    loop {
        let a = read_fired_at(&pg, &job_a).await;
        let b = read_fired_at(&pg, &job_b).await;
        if a.is_some() && b.is_some() {
            println!("timer restore: A={a:?} B={b:?}（未到期/已到期未执行均已触发）");
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "35s 内 A/B 未被触发，说明 router() 没有接线 restore（A={a:?} B={b:?}）"
        );
        sleep(Duration::from_millis(500)).await;
    }

    let c_after = read_fired_at(&pg, &job_c)
        .await
        .expect("已执行 job 的 fired_at 不应被清空");
    // PostgreSQL TIMESTAMP 精度是微秒，内存值带纳秒；按微秒口径比较。
    assert_eq!(
        micro(c_fired_at),
        micro(c_after),
        "已执行过的 job 重启后绝不能重放（fired_at 必须保持原值）"
    );

    server.abort();
    let _ = server.await;
}

/// 真进程重启的子进程守卫：用例结束/panic 时都要杀掉，避免残留占端口。
struct ChildServer(std::process::Child);

impl Drop for ChildServer {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn free_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("local_addr").port()
}

fn spawn_server(
    db_url: &str,
    port: u16,
    log_path: &std::path::Path,
    tick_seconds: u64,
) -> ChildServer {
    let stdout = std::fs::File::create(log_path).expect("create child log");
    let stderr = stdout.try_clone().expect("clone child log handle");
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_oa4rust"));
    cmd.env("DATABASE_URL", db_url)
        .env("OA4RUST_BIND_ADDR", format!("127.0.0.1:{port}"))
        .env("TIMER_TICK_SECONDS", tick_seconds.to_string())
        .env("RUST_LOG", "warn")
        .stdout(std::process::Stdio::from(stdout))
        .stderr(std::process::Stdio::from(stderr));
    let child = cmd.spawn().expect("spawn oa4rust binary");
    ChildServer(child)
}

async fn wait_listening(
    port: u16,
    child: &mut ChildServer,
    deadline: tokio::time::Instant,
    what: &str,
) {
    loop {
        if std::net::TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        if let Some(status) = child.0.try_wait().expect("try_wait") {
            panic!("{what} 进程提前退出（{status}），未监听 127.0.0.1:{port}");
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "{what} 未在期限内监听 127.0.0.1:{port}"
        );
        sleep(Duration::from_millis(200)).await;
    }
}

fn micro(dt: NaiveDateTime) -> String {
    dt.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
}

/// O25 节点 4：真实进程重启回归。
/// P1（**真实进程**）通过 POST 创建 job → 杀掉 P1 → 启动**新进程** P2
/// （同一个隔离库）→ 到期任务被触发；已触发后再等 ≥3 个 tick 不重放。
pub async fn timer_process_restart_flow(ctx: Arc<TestContext>) {
    let pg = ctx.pool.as_pg().expect("postgres pool").clone();
    let work_id = uniq("tw");
    seed_work(&pg, &work_id).await;

    let base = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/postgres".to_string());
    let mut url = url::Url::parse(&base).expect("invalid DATABASE_URL");
    url.set_path(&format!("/{}", ctx.db_name));

    // 子进程从库里加载会话；用同一个 SessionManager 口径种下 admin + session。
    let session_manager = SessionManager::with_pool(pg.clone());
    let token = seed_test_data(&ctx.pool, &session_manager)
        .await
        .expect("seed admin/session for child process");

    // ── P1：真实进程，真实 POST 创建 job ───────────────────────────
    let p1_port = free_port();
    let p1_log = std::env::temp_dir().join(format!(
        "oa4rust-timer-p1-{}-{}.log",
        std::process::id(),
        work_id
    ));
    // P1 用生产默认 30s tick：创建 job 后立即被杀，不会在用例内提前触发。
    let mut p1 = spawn_server(url.as_str(), p1_port, &p1_log, 30);
    wait_listening(
        p1_port,
        &mut p1,
        tokio::time::Instant::now() + Duration::from_secs(20),
        "P1",
    )
    .await;

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let fire_at = Utc::now().naive_utc() - chrono::Duration::seconds(1);
    let fire_at_str = fire_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let resp = client
        .post(format!(
            "http://127.0.0.1:{}/api/processplatform/service/processing/timer/start",
            p1_port
        ))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({ "workId": work_id, "fireAt": fire_at_str }))
        .send()
        .await
        .expect("POST timer/start to P1 failed");
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "P1 创建 job 应 200（写入口 TIMESTAMP 绑定）"
    );
    let body: Value = resp.json().await.expect("timer/start json");
    let job_id = body["data"]["jobId"].as_str().expect("jobId").to_string();
    assert_eq!(
        read_fired_at(&pg, &job_id).await,
        None,
        "P1 的 30s ticker 不应在创建后立即触发"
    );

    // ── 重启：杀掉 P1 整个进程（内存 registry 随之消失）──────────────
    drop(p1);
    let p1_log_text = std::fs::read_to_string(&p1_log).unwrap_or_default();
    assert!(
        p1_log_text.is_empty() || !p1_log_text.contains("error"),
        "P1 日志含错误：\n{p1_log_text}"
    );
    let _ = std::fs::remove_file(&p1_log);

    // ── P2：真正的新进程，同一库，1s tick ────────────────────────────
    let p2_port = free_port();
    let p2_log = std::env::temp_dir().join(format!(
        "oa4rust-timer-p2-{}-{}.log",
        std::process::id(),
        job_id
    ));
    let mut p2 = spawn_server(url.as_str(), p2_port, &p2_log, 1);

    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    let fired = loop {
        if let Some(fired) = read_fired_at(&pg, &job_id).await {
            break fired;
        }
        if let Some(status) = p2.0.try_wait().expect("try_wait") {
            let log = std::fs::read_to_string(&p2_log).unwrap_or_default();
            panic!("重启后的进程 P2 提前退出（{status}），到期 job 未触发；日志：\n{log}");
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "真实进程重启后 30s 内到期 job 未被触发（restore 未接线？）"
        );
        sleep(Duration::from_millis(300)).await;
    };
    println!("timer restart: job={job_id} 在新进程 P2 触发 fired_at={fired}");

    // 已触发后再等 ≥3 个 tick（1s tick），不得重放/覆盖 fired_at。
    sleep(Duration::from_secs(3)).await;
    let after = read_fired_at(&pg, &job_id).await.expect("fired_at 仍在");
    assert_eq!(
        micro(fired),
        micro(after),
        "已执行 job 在重启后绝不能被重放（fired_at 必须保持原值）"
    );

    drop(p2);
    let _ = std::fs::remove_file(&p2_log);
}

/// O25 第二档（cron）：parity corpus / 文档 / 前端都没有 cron 的书面承诺，
/// 因此 `timer/start` 入口直接 400 拒收，且不得落任何 job 行。
pub async fn timer_cron_rejected_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let work_id = uniq("tw");
    seed_work(&pool.as_pg().expect("postgres pool").clone(), &work_id).await;
    let fire_at_str = (Utc::now().naive_utc() + chrono::Duration::hours(1))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    let resp = client
        .post(format!(
            "http://{}/api/processplatform/service/processing/timer/start",
            addr
        ))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "workId": work_id,
            "cron": "0 */5 * * * *",
            "fireAt": fire_at_str,
        }))
        .send()
        .await
        .expect("POST timer/start with cron failed");
    let status = resp.status();
    let raw = resp.text().await.expect("timer/start body");
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "cron 应被 400 拒收，实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("400 body must be JSON");
    assert_eq!(body["type"], "error", "body={raw}");
    assert!(
        raw.contains("cron") && raw.contains("暂不支持"),
        "400 文案必须明确“cron 暂不支持”，body={raw}"
    );

    // 拒收必须发生在写库之前：不能留一条永不触发的 job。
    let count: i64 = pool
        .as_pg()
        .expect("postgres pool")
        .get()
        .await
        .expect("pool client")
        .query_one(
            "SELECT COUNT(*) FROM x_timer_job WHERE work_id = $1",
            &[&work_id],
        )
        .await
        .expect("count timer jobs")
        .get(0);
    assert_eq!(count, 0, "cron 被拒收后不应落 x_timer_job 行");

    server.abort();
    let _ = server.await;
}
