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

use behavior_comparison::{ComparisonResult, ComparisonStatus, EndpointComparator, EndpointDef};

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

/// CI 同款 Java 就绪探针（.github/workflows/ci.yml "Wait for o2server readiness"）：
/// POST /jaxrs/secret/set 返回任意非 502/503 的 HTTP 状态码即视为就绪。
/// （401/404 同样证明 HTTP 栈在正常应答；连接拒绝/超时才不可达。
/// 本机实测：o2server 镜像对未知裸 /jaxrs/* 直接 RST，故 CI 的第二探针
/// server/execute 在 reqwest 下恒为 Err，不能作为必要条件。）
async fn probe_java_readiness(base_url: &str) -> bool {    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    let secret_url = format!("{}/jaxrs/secret/set", base_url.trim_end_matches('/'));
    match client
        .post(&secret_url)
        .header("Content-Type", "application/json")
        .body(r#"{"secret":"o2oa@2022"}"#)
        .send()
        .await
    {
        Ok(resp) => {
            let code = resp.status().as_u16();
            code != 502 && code != 503
        }
        Err(_) => false,
    }
}

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

/// 幂等种子对比账户（Rust 库）。与 behavior_compare_sample 的 seed_testadmin
/// 同款 SQL，但直连 DATABASE_URL（主测试面向外部启动的服务，不持有测试池）。
/// 任一步失败仅告警：无 DB 凭据/库不可达时仍可跑匿名对比子集。
async fn seed_testadmin(credential: &str, password: &str) {
    let url = match std::env::var("DATABASE_URL") {
        Ok(u) if !u.is_empty() => u,
        _ => {
            eprintln!("[behavior_compare] DATABASE_URL not set — skip seeding {} (protected endpoints may 401)", credential);
            return;
        }
    };
    let config: deadpool_postgres::tokio_postgres::Config = match url.parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[behavior_compare] bad DATABASE_URL: {} — skip seeding", e);
            return;
        }
    };
    let mgr = deadpool_postgres::Manager::new(config, deadpool_postgres::tokio_postgres::NoTls);
    let pool = match deadpool_postgres::Pool::builder(mgr).max_size(1).build() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[behavior_compare] pool build failed: {} — skip seeding", e);
            return;
        }
    };
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[behavior_compare] db connect failed: {} — skip seeding", e);
            return;
        }
    };
    let hash = oa4rust::auth::password::hash_password(password);
    let sql = "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, deleted_at) \
               VALUES ($1, $2, $3, $4, false, NULL) \
               ON CONFLICT (unique_id) DO UPDATE SET password_hash = EXCLUDED.password_hash";
    match client
        .execute(sql, &[&"person-behavior-testadmin", &credential, &credential, &hash])
        .await
    {
        Ok(_) => eprintln!("[behavior_compare] seeded test account '{}' into Rust DB", credential),
        Err(e) => eprintln!("[behavior_compare] seed failed: {} — protected endpoints may 401", e),
    }
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
    // O2OA v9 无 /health 端点，/health 探测失败时回退 CI 就绪探针
    // （POST /jaxrs/secret/set + GET /jaxrs/server/execute）。
    let mut java_reachable =
        behavior_comparison::comparator::is_service_reachable(&java_url).await;
    if !java_reachable {
        java_reachable = probe_java_readiness(&java_url).await;
    }
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

    // Rust 库种子对比账户（幂等）：CI 的 postgres 服务容器与本地库默认都没有
    // testadmin 账户，缺种子会导致全部保护端点 401（实测 76 个 FAIL 中 ~60 个
    // 由此引起）。复用样例套件的种法；DATABASE_URL 未设置时跳过并告警。
    seed_testadmin(&credential, &password).await;

    let comparator = if java_reachable {
        match comparator.login(RUST_BASE_URL, &credential, &password).await {
            Some(rust_token) => {
                eprintln!("[behavior_compare] Rust login successful, token acquired");
                // Java 侧依次尝试：对比账户 → O2OA v9 内置管理员 xadmin
                // （密码为 /jaxrs/secret/set 初始化的密钥，CI 同款 o2oa@2022）。
                // Java 登录失败时其保护端点返回带 prompt 的错误信封，与 Rust
                // 成功信封逐条产生假差异（实测一次 1470 FAIL 中大多数属此类）。
                let java_login = match comparator
                    .login(&java_url, &credential, &password)
                    .await
                {
                    Some(t) => Some(("testadmin".to_string(), t)),
                    None => comparator
                        .login(&java_url, "xadmin", "o2oa@2022")
                        .await
                        .map(|t| ("xadmin".to_string(), t)),
                };
                if let Some((who, java_token)) = java_login {
                    eprintln!("[behavior_compare] Java login successful as '{}' — token acquired", who);
                    // 两侧 token 互不通用，必须按侧分发（此前误将 Java token 设为
                    // 全局，导致 Rust 侧全程 401 走错误信封）。
                    comparator.with_tokens(rust_token, java_token)
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

    // Fast path: when Java is unreachable, skip all Rust calls and emit SKIP report.
    // Without this, 4687 sequential requests with 45s timeout would take hours.
    let results = if !java_reachable {
        eprintln!("[behavior_compare] Java unreachable — skipping all endpoint comparisons (SKIP all)");
        all_endpoints()
            .iter()
            .map(|def| ComparisonResult {
                endpoint: def.rust_path.to_string(),
                method: def.method.to_string(),
                crate_name: def.crate_name.to_string(),
                rust_status: None,
                java_status: None,
                rust_response: None,
                java_response: None,
                is_equivalent: true,
                differences: vec![],
                status: ComparisonStatus::Skip,
            })
            .collect()
    } else {
        comparator.compare_all(&all_endpoints()).await
    };

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
