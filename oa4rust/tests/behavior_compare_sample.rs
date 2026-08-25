//! 小样本行为对比首次实测驱动 (plan002 U9a→U3 前置验证)
//!
//! 与 tests/behavior_compare.rs 的区别：
//! - 只跑手工挑选的 7 个两端均已实现的只读端点
//!   （自动生成的 ENDPOINTS 目前 java_war/java_action 全为空，全量跑无意义）
//! - 在测试进程内启动 Rust 服务（随机端口），不要求外部服务监听 3000
//! - 分别持有 Rust/Java 登录 token，按目标服务发送对应 token
//!   （O2OA v9 需要 x-token header，见 comparator.rs 修正说明）
//!
//! 运行方式（需 PostgreSQL 与 Java o2server 容器在跑，且 Java 库已创建 testadmin 账户）：
//!   $env:BEHAVIOR_COMPARE_SAMPLE = "1"
//!   $env:JAVA_SERVICE_URL = "http://localhost:18080"
//!   cargo test --test behavior_compare_sample -- --nocapture
//!
//! 报告输出：target/debug/behavior-report-sample.md
//! 首次实测目的为记录基线，端点 FAIL 不导致测试失败。

mod behavior_comparison;
mod integration_tests;

use std::sync::Arc;

use anyhow::Context as _;

use behavior_comparison::comparator::{ComparisonStatus, EndpointComparator, EndpointDef};
use behavior_comparison::reporter::ComparisonReport;

/// Java o2server v9 组织管理 war。
const JAVA_ORG_WAR: &str = "x_organization_assemble_control";
/// Java o2server v9 认证 war。
const JAVA_AUTH_WAR: &str = "x_organization_assemble_authentication";

const REPORT_PATH: &str = "target/debug/behavior-report-sample.md";

/// 对比专用测试账户（两侧数据库均需存在，密码相同）。
const CREDENTIAL: &str = "testadmin";
const PASSWORD: &str = "testadmin";

/// 首次实测样本：7 个只读端点。
///
/// java_war/java_action 为 O2OA v9.5.2 实测路径（curl 200/500 验证过）。
/// rust_path 中 {flag} 等占位符以实际值 "(0)" 填充。
fn sample_endpoints() -> Vec<EndpointDef> {
    vec![
        EndpointDef {
            crate_name: "control",
            method: "GET",
            rust_path: "/jaxrs/unit/list/(0)/next/20",
            java_war: JAVA_ORG_WAR,
            java_action: "unit/list/(0)/next/20",
            body: None,
            requires_auth: true,
        },
        EndpointDef {
            crate_name: "control",
            method: "GET",
            rust_path: "/jaxrs/person/list/(0)/next/20",
            java_war: JAVA_ORG_WAR,
            java_action: "person/list/(0)/next/20",
            body: None,
            requires_auth: true,
        },
        EndpointDef {
            crate_name: "control",
            method: "GET",
            rust_path: "/jaxrs/role/list/(0)/next/20",
            java_war: JAVA_ORG_WAR,
            java_action: "role/list/(0)/next/20",
            body: None,
            requires_auth: true,
        },
        EndpointDef {
            crate_name: "control",
            method: "GET",
            rust_path: "/jaxrs/group/list/(0)/next/20",
            java_war: JAVA_ORG_WAR,
            java_action: "group/list/(0)/next/20",
            body: None,
            requires_auth: true,
        },
        // 已知数据不对齐风险项：Java 库有 xadmin person 记录，Rust 测试库没有。
        EndpointDef {
            crate_name: "control",
            method: "GET",
            rust_path: "/jaxrs/person/xadmin",
            java_war: JAVA_ORG_WAR,
            java_action: "person/xadmin",
            body: None,
            requires_auth: true,
        },
        // 已知路由歧义风险项：Java 把 unit/list 当作 unit/{flag}=list 处理(500)，
        // Rust 有独立 /jaxrs/unit/list 路由。
        EndpointDef {
            crate_name: "control",
            method: "GET",
            rust_path: "/jaxrs/unit/list",
            java_war: JAVA_ORG_WAR,
            java_action: "unit/list",
            body: None,
            requires_auth: true,
        },
        // whoami：Rust auth crate 与 Java authentication war 各自实现。
        EndpointDef {
            crate_name: "auth",
            method: "GET",
            rust_path: "/jaxrs/authentication",
            java_war: JAVA_AUTH_WAR,
            java_action: "authentication",
            body: None,
            requires_auth: true,
        },
    ]
}

/// 在 Rust 测试库中 seed 对比账户 testadmin/testadmin。
async fn seed_testadmin(pool: &Arc<integration_tests::db::TestPool>) -> anyhow::Result<()> {
    let pg_pool = pool
        .as_pg()
        .expect("seed_testadmin requires PostgreSQL pool");
    let client = pg_pool.get().await?;
    let hash = oa4rust::auth::password::hash_password(PASSWORD);
    client
        .execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, deleted_at) \
             VALUES ($1, $2, $3, $4, false, NULL) \
             ON CONFLICT (unique_id) DO UPDATE SET password_hash = EXCLUDED.password_hash",
            &[
                &"person-behavior-testadmin",
                &CREDENTIAL,
                &CREDENTIAL,
                &hash,
            ],
        )
        .await
        .context("insert behavior-compare person failed")?;
    Ok(())
}

#[tokio::test]
async fn behavior_compare_sample_first_run() {
    if std::env::var("BEHAVIOR_COMPARE_SAMPLE").is_err() {
        eprintln!("[behavior_compare_sample] Skipping (set BEHAVIOR_COMPARE_SAMPLE=1 to run)");
        return;
    }
    let java_url = std::env::var("JAVA_SERVICE_URL").unwrap_or_else(|_| "http://localhost:18080".to_string());

    // ── 启动 Rust 测试服务 ────────────────────────────────────────────────
    let ctx = integration_tests::db::init_test_database_async().await;
    let (addr, handle, _admin_token) =
        integration_tests::helpers::setup_test_server(ctx.pool()).await.expect("failed to start rust test server");
    let rust_url = format!("http://{}", addr);
    eprintln!("[behavior_compare_sample] Rust server at {}", rust_url);
    eprintln!("[behavior_compare_sample] Java base: {}", java_url);

    seed_testadmin(&ctx.pool()).await.expect("failed to seed testadmin");

    // ── 两侧登录 ─────────────────────────────────────────────────────────
    let base_comparator = EndpointComparator::new(&rust_url, &java_url);
    let rust_token = base_comparator
        .login(&rust_url, CREDENTIAL, PASSWORD)
        .await
        .expect("Rust login failed with testadmin/testadmin");
    eprintln!("[behavior_compare_sample] Rust login OK");
    let java_token = base_comparator
        .login(&java_url, CREDENTIAL, PASSWORD)
        .await
        .expect("Java login failed with testadmin/testadmin");
    eprintln!("[behavior_compare_sample] Java login OK");

    // ── 构造 comparator（allowlist 失败则空表继续）──────────────────────
    let mut comparator = base_comparator
        .with_tokens(rust_token.clone(), java_token.clone());
    match behavior_comparison::DiffAllowlist::from_yaml("tests/behavior_comparison/allowlist.yaml") {
        Ok(list) => comparator.allowlist = list,
        Err(e) => eprintln!("[behavior_compare_sample] allowlist load failed: {} — continuing empty", e),
    }

    // ── 执行对比 ─────────────────────────────────────────────────────────
    let endpoints = sample_endpoints();
    eprintln!("[behavior_compare_sample] Comparing {} endpoints...", endpoints.len());
    let results = comparator.compare_all(&endpoints).await;

    let passed = results.iter().filter(|r| r.status == ComparisonStatus::Pass).count();
    let failed = results.iter().filter(|r| r.status == ComparisonStatus::Fail).count();
    let skipped = results.iter().filter(|r| r.status == ComparisonStatus::Skip).count();

    for r in &results {
        eprintln!(
            "[behavior_compare_sample] {:?} {} => rust={:?} java={:?} status={:?}{}",
            r.method,
            r.endpoint,
            r.rust_status,
            r.java_status,
            r.status,
            if r.differences.is_empty() { String::new() } else { format!(" diffs={:?}", r.differences) }
        );
    }
    eprintln!("[behavior_compare_sample] Results: {} passed, {} failed, {} skipped", passed, failed, skipped);

    // ── 报告 ─────────────────────────────────────────────────────────────
    let mut report = ComparisonReport::new(&java_url);
    for r in results {
        report.add_result(r);
    }
    let markdown = report.to_markdown();
    if let Some(parent) = std::path::Path::new(REPORT_PATH).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::write(REPORT_PATH, &markdown) {
        Ok(_) => eprintln!("[behavior_compare_sample] Report written to {}", REPORT_PATH),
        Err(e) => eprintln!("[behavior_compare_sample] Failed to write report: {}", e),
    }

    handle.abort();
}
