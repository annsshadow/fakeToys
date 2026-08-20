//! OA4Rust 行为对比测试套件 (U3 / R3)
//!
//! 对比相同请求下 Rust 端点（localhost:3000）与 Java 端点（JAVA_SERVICE_URL）的
//! 响应结构（状态码 + JSON 字段名 + 类型），忽略允许列表中的已知命名差异。
//!
//! 运行方式：
//!   BEHAVIOR_COMPARE=1 cargo test --test behavior_compare
//!
//! 如果 Java 服务不可达，所有 Java 侧对比标记为 SKIP，测试仍通过。
//! 如果 Rust 服务不可达，测试失败。
//! 未设置 BEHAVIOR_COMPARE 环境变量时，测试自动跳过。

mod behavior_comparison;

use std::collections::HashMap;

use behavior_comparison::{ComparisonStatus, EndpointComparator, EndpointDef};

/// Rust 服务地址（CI 中通过 cargo test 启动，监听 3000 端口）。
const RUST_BASE_URL: &str = "http://localhost:3000";

/// 默认 Java 服务地址。
const DEFAULT_JAVA_BASE_URL: &str = "http://localhost:8080";

/// 默认登录凭证（行为对比专用测试账户，需两侧数据库均有此账户）。
const DEFAULT_CREDENTIAL: &str = "testadmin";
const DEFAULT_PASSWORD: &str = "testadmin";

/// 允许列表文件路径（相对于 tests/ 目录）。
const ALLOWLIST_PATH: &str = "tests/behavior_comparison/allowlist.yaml";

/// 报告输出路径。
const REPORT_PATH: &str = "target/debug/behavior-report.md";

// ──────────────────────────────────────────────────────────────────────────────
// 端点定义
// ──────────────────────────────────────────────────────────────────────────────

/// 合并自动生成端点清单，按 rust_path 去重。
fn all_endpoints() -> Vec<EndpointDef> {
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let mut result = Vec::new();
    for ep in behavior_comparison::endpoints::ENDPOINTS {
        if seen.insert(ep.rust_path) {
            result.push(ep.clone());
        }
    }
    result
}

// ──────────────────────────────────────────────────────────────────────────────
// 测试
// ──────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn behavior_compare_rust_vs_java() {
    if std::env::var("BEHAVIOR_COMPARE").is_err() {
        eprintln!("[behavior_compare] Skipping (set BEHAVIOR_COMPARE=1 to run)");
        return;
    }
    let java_url = std::env::var("JAVA_SERVICE_URL").unwrap_or_else(|_| DEFAULT_JAVA_BASE_URL.to_string());

    eprintln!("[behavior_compare] Rust base: {}", RUST_BASE_URL);
    eprintln!("[behavior_compare] Java base: {}", java_url);

    // ── 检查 Rust 服务可达性 ──────────────────────────────────────────────
    if !behavior_comparison::comparator::is_service_reachable(RUST_BASE_URL).await {
        eprintln!("[behavior_compare] Rust service unreachable at {} — aborting", RUST_BASE_URL);
        panic!("Rust service unreachable at {} — cannot run behavior comparison", RUST_BASE_URL);
    }
    eprintln!("[behavior_compare] Rust service reachable");

    // ── 检查 Java 服务可达性 ──────────────────────────────────────────────
    let java_reachable =
        behavior_comparison::comparator::is_service_reachable(&java_url).await;
    if !java_reachable {
        eprintln!("[behavior_compare] Java service unreachable at {} — all Java results will be SKIP", java_url);
    } else {
        eprintln!("[behavior_compare] Java service reachable");
    }

    // ── 加载允许列表 ──────────────────────────────────────────────────────
    let allowlist_path = std::env::var("BEHAVIOR_ALLOWLIST_PATH").unwrap_or_else(|_| ALLOWLIST_PATH.to_string());
    let comparator = match EndpointComparator::new(RUST_BASE_URL, &java_url)
        .with_allowlist(&allowlist_path)
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[behavior_compare] Failed to load allowlist from {}: {}", allowlist_path, e);
            eprintln!("[behavior_compare] Continuing with empty allowlist");
            EndpointComparator::new(RUST_BASE_URL, &java_url)
        }
    };
    let allowlist_entries = comparator.allowlist.entries.len();

    // ── 尝试登录获取认证令牌 ──────────────────────────────────────────────
    let credential = std::env::var("BEHAVIOR_TEST_CREDENTIAL").unwrap_or_else(|_| DEFAULT_CREDENTIAL.to_string());
    let password = std::env::var("BEHAVIOR_TEST_PASSWORD").unwrap_or_else(|_| DEFAULT_PASSWORD.to_string());

    let comparator = if java_reachable {
        match comparator.login(RUST_BASE_URL, &credential, &password).await {
            Some(rust_token) => {
                eprintln!("[behavior_compare] Rust login successful, token acquired");
                if let Some(java_token) = comparator.login(&java_url, &credential, &password).await {
                    eprintln!("[behavior_compare] Java login successful, token acquired");
                    comparator.with_auth_token(java_token)
                } else {
                    eprintln!("[behavior_compare] Java login failed — protected endpoints will be SKIP");
                    comparator.with_auth_token(rust_token)
                }
            }
            None => {
                eprintln!("[behavior_compare] Rust login failed — protected endpoints may be SKIP");
                comparator
            }
        }
    } else {
        eprintln!("[behavior_compare] Java unreachable — skipping all Java comparisons");
        comparator
    };

    // ── 执行对比 ──────────────────────────────────────────────────────────
    eprintln!("[behavior_compare] Comparing {} endpoints...", all_endpoints().len());
    let results = comparator.compare_all(&all_endpoints()).await;

    let passed = results.iter().filter(|r| r.status == ComparisonStatus::Pass).count();
    let failed = results.iter().filter(|r| r.status == ComparisonStatus::Fail).count();
    let skipped = results.iter().filter(|r| r.status == ComparisonStatus::Skip).count();

    eprintln!("[behavior_compare] Results: {} passed, {} failed, {} skipped", passed, failed, skipped);

    // ── 生成报告 ──────────────────────────────────────────────────────────
    let mut report = behavior_comparison::reporter::ComparisonReport::new(&java_url)
        .with_allowlist_count(allowlist_entries);

    for result in results {
        report.add_result(result);
    }

    let markdown = report.to_markdown();

    // 确保输出目录存在
    if let Some(parent) = std::path::Path::new(REPORT_PATH).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(REPORT_PATH, &markdown) {
        eprintln!("[behavior_compare] Failed to write report: {}", e);
    } else {
        eprintln!("[behavior_compare] Report written to {}", REPORT_PATH);
    }

    // ── 断言：无 FAIL（Java 不可达时不在此处失败，但记录 SKIP）───────────
    if failed > 0 {
        panic!(
            "behavior comparison: {} endpoint(s) FAILED ({} passed, {} skipped). See {}",
            failed, passed, skipped, REPORT_PATH
        );
    }

    eprintln!("[behavior_compare] All comparisons passed or skipped ({} SKIP due to Java unreachable)", skipped);
}
