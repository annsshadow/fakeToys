//! OA4Rust 性能基线测试 (Phase 4)
//!
//! 独立的集成测试目标，镜像 `tests/integration_runner.rs`：
//! 初始化一次性测试数据库、启动完整应用、然后对两个关键路径进行延迟压测：
//!
//!   (a) `POST /jaxrs/authentication`  (it-admin / password123) —— 认证 DB 密码校验路径
//!   (b) `GET  /jaxrs/cms_assemble_control/data/document`        —— 列表读取路径
//!
//! 使用 `std::time::Instant` 采集 N 次迭代的延迟，汇总 avg / p50 / p99。
//!
//! 运行方式（仅在安静机器上、带 `--ignored` 跑，避免被其它测试噪声干扰）：
//! ```bash
//! cargo test --test perf_baseline -- --ignored --nocapture
//! ```
//!
//! 环境变量：
//! - `DATABASE_URL` — 管理员连接 URL（默认 `postgres://o2server:password@localhost:5432/postgres`）
//!
//! 注意：本文件是一个独立的 `[[test]]` 目标，未修改 `integration_runner.rs`
//! 或 `integration_tests/scenarios/mod.rs`。

mod integration_tests;

use std::time::{Duration, Instant};

use reqwest::Client;
use serde_json::json;
use tracing::info;

/// 压测迭代次数（已排除 warm-up）。
const ITERATIONS: usize = 50;
/// warm-up 次数，不计入统计，仅用于预热连接池 / 查询计划缓存。
const WARMUP: usize = 5;

/// 性能基线主测试。
///
/// `#[ignore]` 与 `integration_runner` 保持一致：无 PostgreSQL 时 `cargo test`
/// 不会触碰它，只有显式 `--ignored` 才运行。
#[ignore = "requires a running PostgreSQL server"]
#[test]
fn perf_baseline() {
    // 同步测试函数内自建一个多线 tokio runtime 来驱动异步的
    // 数据库初始化与 HTTP 服务启动（与 helpers::setup_test_server 一致）。
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime for perf baseline");

    // 1) 同步初始化一次性测试数据库（init_test_database 内部自管 tokio runtime，
    //    必须在同步上下文调用，绝不能包进 runtime.block_on，否则触发
    //    "Cannot start a runtime from within a runtime" panic——与 integration_runner 一致）。
    let _ctx = integration_tests::db::init_test_database();
    let pool = _ctx.pool();

    // 关闭限流以测量成功路径延迟（仅本测试进程生效，生产不受影响）。
    std::env::set_var("OA4RUST_DISABLE_RATE_LIMIT", "1");

    runtime.block_on(async {
        // 2) 启动完整应用，拿到 addr / 后台句柄 / 已签发的 admin token
        let (addr, handle, token) = integration_tests::helpers::setup_test_server(pool.clone())
            .await
            .expect("failed to start test server for perf baseline");

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build reqwest client");

        let base = format!("http://{}", addr);
        let auth_header = format!("Bearer {}", token);

        // 3) warm-up —— 不计入统计
        for _ in 0..WARMUP {
            let _ = client
                .post(format!("{}/jaxrs/authentication", base))
                .json(&json!({ "credential": "it-admin", "password": "password123" }))
                .send()
                .await;
            let _ = client
                .get(format!("{}/jaxrs/cms_assemble_control/data/document", base))
                .header("Authorization", &auth_header)
                .send()
                .await;
        }

        // 4) 正式采集
        let mut auth_ms: Vec<f64> = Vec::with_capacity(ITERATIONS);
        let mut list_ms: Vec<f64> = Vec::with_capacity(ITERATIONS);

        for i in 0..ITERATIONS {
            // (a) 认证 DB 密码校验路径
            let start = Instant::now();
            let auth_resp = client
                .post(format!("{}/jaxrs/authentication", base))
                .json(&json!({ "credential": "it-admin", "password": "password123" }))
                .send()
                .await
                .expect("auth request failed");
            let auth_status = auth_resp.status();
            let _ = auth_resp.text().await;
            let auth_elapsed = start.elapsed().as_secs_f64() * 1000.0;
            if !auth_status.is_success() {
                println!(
                    "WARN: POST /jaxrs/authentication returned {} at iter {}",
                    auth_status, i
                );
            }
            auth_ms.push(auth_elapsed);

            // (b) 列表读取路径
            let start = Instant::now();
            let list_resp = client
                .get(format!("{}/jaxrs/cms_assemble_control/data/document", base))
                .header("Authorization", &auth_header)
                .send()
                .await
                .expect("list document request failed");
            let list_status = list_resp.status();
            let _ = list_resp.text().await;
            let list_elapsed = start.elapsed().as_secs_f64() * 1000.0;
            if !list_status.is_success() {
                println!(
                    "WARN: GET /jaxrs/cms_assemble_control/data/document returned {} at iter {}",
                    list_status, i
                );
            }
            list_ms.push(list_elapsed);
        }

        // 5) 汇总并打印
        println!(
            "\n=== OA4Rust perf baseline (iterations={}, warmup={}) ===",
            ITERATIONS, WARMUP
        );
        summarize("POST /jaxrs/authentication (DB password verify)", &auth_ms);
        summarize("GET  /jaxrs/cms_assemble_control/data/document (list)", &list_ms);

        // 关闭后台 HTTP 服务
        handle.abort();
    });
}

/// 计算并打印一组样本（毫秒）的 avg / p50 / p99。
fn summarize(name: &str, samples: &[f64]) {
    if samples.is_empty() {
        return;
    }
    let mut v = samples.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    let avg = v.iter().sum::<f64>() / n as f64;
    let pct = |p: f64| -> f64 {
        let idx = ((p / 100.0) * (n as f64 - 1.0)).round() as usize;
        v[idx]
    };
    let p50 = pct(50.0);
    let p99 = pct(99.0);

    println!(
        "[{:>8.3} ms avg | {:>8.3} ms p50 | {:>8.3} ms p99]  {}",
        avg, p50, p99, name
    );
    info!(
        target: "perf_baseline",
        n,
        avg_ms = avg,
        p50_ms = p50,
        p99_ms = p99,
        endpoint = name,
        "perf baseline summary"
    );
}
