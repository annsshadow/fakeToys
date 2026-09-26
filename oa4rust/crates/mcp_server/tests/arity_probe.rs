// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 全库级 arity 回归守卫：构建完整应用 Router（build_core_router，与生产同一装配），
//! 对 backend_routes.json 中登记的每条真实路由做 oneshot，断言无一返回 axum 运行时
//! 500「Wrong number of path arguments」——即 URL {param} 槽数与 handler Path 提取器元数不一致。
//! 该错误对所有调用者恒定 500，是「名义已注册、实际不可消费」的根因之一。

use axum::body::Body;
use axum::http::{Method, Request};
use serde_json::Value;
use std::path::PathBuf;
use tower::util::ServiceExt;

fn routes_json_path() -> PathBuf {
    // crates/mcp_server -> 仓库内 docs/audits/three-ends-2026-09-20/backend_routes.json
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../docs/audits/three-ends-2026-09-20/backend_routes.json")
}

#[tokio::test]
async fn every_registered_route_has_matching_path_arity() {
    let raw = std::fs::read_to_string(routes_json_path())
        .expect("backend_routes.json 可读（arity 探针数据源）");
    let by_crate: serde_json::Map<String, Value> = serde_json::from_str(&raw).unwrap();

    let mut cases: Vec<(String, String)> = Vec::new();
    for (_crate, routes) in &by_crate {
        for r in routes.as_array().unwrap() {
            let method = r["method"].as_str().unwrap().to_string();
            let path = r["path"].as_str().unwrap();
            if path.contains("mock") {
                continue;
            }
            // {param} / {*rest} 槽填占位值 "1"
            let mut filled = String::new();
            let mut depth = 0;
            for ch in path.chars() {
                match ch {
                    '{' => {
                        depth += 1;
                        if depth == 1 {
                            filled.push('1');
                        }
                    }
                    '}' => depth -= 1,
                    _ if depth == 0 => filled.push(ch),
                    _ => {}
                }
            }
            cases.push((method, filled));
        }
    }

    let mut arity_traps: Vec<String> = Vec::new();
    // 一次装配、逐请求 clone（axum Router: Clone），避免 4600+ 次重建路由的开销。
    let app = mcp_server::tool_bridge::build_core_router(
        shared::testing::test_pool(),
        shared::SessionManager::new(),
    )
    .await;

    let mut skipped_uri = 0usize;
    for (method, path) in &cases {
        let m = Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET);
        let req = match Request::builder().method(m).uri(path.as_str()).body(Body::empty()) {
            Ok(r) => r,
            // 含非法 URI 字符（如 legacy 畸形空格路径）——正常客户端无法触达，跳过计数。
            Err(_) => {
                skipped_uri += 1;
                continue;
            }
        };
        let resp = app.clone().oneshot(req).await.unwrap();
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8_lossy(&bytes);
        if body.contains("Wrong number of path arguments") {
            arity_traps.push(format!("{method} {path}"));
        }
    }
    eprintln!("probe 完成：跳过非法 URI {skipped_uri} 条");

    // 棘轮基线：已登记在案的 arity 陷阱（逐 crate 修复中）。测试只在出现「基线之外的新
    // 陷阱」时失败（防回归），并报告剩余进度。修复推进时同步收缩 _arity_traps_baseline.txt。
    let baseline_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../docs/audits/three-ends-2026-09-20/_arity_traps_baseline.txt");
    let known: std::collections::HashSet<String> = std::fs::read_to_string(&baseline_path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let current: std::collections::HashSet<String> = arity_traps.iter().cloned().collect();
    let new_traps: Vec<&String> = current.difference(&known).collect();
    let remaining = current.intersection(&known).count();
    eprintln!(
        "arity 陷阱：当前 {} 条（基线 {}，剩余待修 {}，新回归 {}）",
        current.len(),
        known.len(),
        remaining,
        new_traps.len()
    );
    if !new_traps.is_empty() {
        eprintln!("== 基线之外的新 arity 回归 ==");
        for t in &new_traps {
            eprintln!("  {t}");
        }
    }
    assert!(
        new_traps.is_empty(),
        "{} 条基线之外的新 arity 陷阱（URL 槽数≠handler Path 元数的运行时 500），见上方明细",
        new_traps.len()
    );
}
