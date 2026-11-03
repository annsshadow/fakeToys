// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 全库级 arity 回归守卫：构建完整应用 Router（build_core_router，与生产同一装配），
//! 对 backend_routes.json 中登记的每条真实路由做 oneshot，断言无一返回 axum 运行时
//! 500「Wrong number of path arguments」——即 URL {param} 槽数与 handler Path 提取器元数不一致。
//! 该错误对所有调用者恒定 500，是「名义已注册、实际不可消费」的根因之一。
//!
//! # 探针卫生（scratch 库隔离）
//!
//! 本探针会对 4600+ 条真实路由发请求，其中包含写路由（占位 id「1」）。为不把瞬态
//! 写读留在共享库（`oa4rust_test` 被其它测试复用，垃圾行会触发 handler 解码 panic
//! 等假阳性），一律跑在专用 scratch 库 `oa_arity_probe`：每轮 DROP 重建 + 全量迁移，
//! 跑完 DROP。共享库只做「是否有 DB 可用」的探测。

use axum::body::Body;
use axum::http::{Method, Request};
use serde_json::Value;
use std::path::PathBuf;
use tower::util::ServiceExt;

/// 探针专用 scratch 库（每轮重建，跑完 DROP）
const SCRATCH_DB: &str = "oa_arity_probe";

/// 指向指定库的连接池（DATABASE_URL 解析后覆盖 dbname）
fn pool_to(dbname: &str) -> deadpool_postgres::Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(mgr)
        .max_size(8)
        .build()
        .unwrap()
}

/// 重建 scratch 库（存在则强删），全量迁移。
async fn fresh_scratch() {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
    c.execute(
        &format!("DROP DATABASE IF EXISTS {SCRATCH_DB} WITH (FORCE)"),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {SCRATCH_DB}"), &[])
        .await
        .expect("create scratch");
    let pool = pool_to(SCRATCH_DB);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 库迁移");
}

async fn drop_scratch() {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {SCRATCH_DB} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}

/// scratch 库累计插入行数（含所有写路由的瞬态写）
async fn scratch_inserted_rows(pool: &deadpool_postgres::Pool) -> i64 {
    let c = match pool.get().await {
        Ok(c) => c,
        Err(_) => return -1,
    };
    c.query_one(
        "SELECT COALESCE(sum(n_tup_ins), 0)::bigint FROM pg_stat_user_tables",
        &[],
    )
    .await
    .map(|r| r.get::<_, i64>(0))
    .unwrap_or(-1)
}

fn routes_json_path() -> PathBuf {
    // crates/mcp_server -> 仓库内 docs/audits/three-ends-2026-09-20/backend_routes.json
    // CARGO_MANIFEST_DIR = <repo>/oa4rust/backend/crates/mcp_server，仓库根 docs 需上溯 4 级。
    // （重构提交把 docs/audits 留在仓库根，原 3 级 depth 指向不存在的 oa4rust/docs。）
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../docs/audits/three-ends-2026-09-20/backend_routes.json")
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
    let mut unreachable: Vec<String> = Vec::new();

    // 探针卫生：所有请求落在 scratch 库，共享库零写入（见文件头说明）。
    if !shared::testing::is_db_available().await {
        eprintln!("SKIP: no database available");
        return;
    }
    // 穷举量级请求会打爆默认 300/分钟 限流——探针环境把阈值抬高
    // （限流行为本身由 shared 中间件测试覆盖）。
    std::env::set_var("OA4RUST_GENERAL_RATE_LIMIT", "1000000");
    std::env::set_var("OA4RUST_AUTH_RATE_LIMIT", "1000000");

    fresh_scratch().await;
    let probe_pool = pool_to(SCRATCH_DB);
    let rows_before = scratch_inserted_rows(&probe_pool).await;

    // 一次装配、逐请求 clone（axum Router: Clone），避免 4600+ 次重建路由的开销。
    let app = mcp_server::tool_bridge::build_core_router(
        probe_pool.clone(),
        shared::SessionManager::new(),
    )
    .await;

    let mut skipped_uri = 0usize;
    for (method, path) in &cases {
        let m = Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET);
        let req = match Request::builder()
            .method(m)
            .uri(path.as_str())
            .body(Body::empty())
        {
            Ok(r) => r,
            // 含非法 URI 字符（如 legacy 畸形空格路径）——正常客户端无法触达，跳过计数。
            Err(_) => {
                skipped_uri += 1;
                continue;
            }
        };
        let resp = app.clone().oneshot(req).await.unwrap();
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8_lossy(&bytes);
        if body.contains("Wrong number of path arguments") {
            arity_traps.push(format!("{method} {path}"));
        }
        // 已注册路由被自身注册路径命中却 404 = 不可达（路由树被字面段/同位异名参数遮蔽）。
        // 仅当响应体为空时才计入：axum 路由兜底 404 恒为空体；handler 主动返回的 404
        // （AppError::NotFound——占位 id「1」在测试库查无实体）带 JSON 错误体，是「路由可达、
        // 业务正常」，绝不能误判为路由不可达。405（方法不符）同样不计入。
        if status == axum::http::StatusCode::NOT_FOUND && bytes.is_empty() {
            unreachable.push(format!("{method} {path}"));
        }
    }
    eprintln!("probe 完成：跳过非法 URI {skipped_uri} 条");

    // 探针卫生收尾：报告 scratch 库实际写入量并 DROP，共享库全程未连。
    let rows_after = scratch_inserted_rows(&probe_pool).await;
    eprintln!(
        "probe 卫生：库={SCRATCH_DB} 本次写入行数={} （共享库零写入）",
        (rows_after - rows_before).max(0)
    );
    drop(probe_pool);
    drop_scratch().await;

    // 棘轮基线：两类「名义已注册、实际不可消费」缺陷各自一份基线，测试仅在出现基线之外的
    // 新回归时失败（防回归），修复推进时同步收缩基线文件。
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../docs/audits/three-ends-2026-09-20");
    let ratchet = |name: &str, found: &[String], label: &str| -> usize {
        let base_path = dir.join(name);
        let known: std::collections::HashSet<String> = std::fs::read_to_string(&base_path)
            .unwrap_or_default()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        let current: std::collections::HashSet<String> = found.iter().cloned().collect();
        // 落盘当前全集，便于修复后同步收缩基线。
        let mut sorted: Vec<&String> = current.iter().collect();
        sorted.sort();
        let _ = std::fs::write(
            dir.join(format!("_current_{name}")),
            sorted
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        );
        let new: Vec<&String> = current.difference(&known).collect();
        eprintln!(
            "{label}：当前 {} 条（基线 {}，剩余 {}，新回归 {}）",
            current.len(),
            known.len(),
            current.intersection(&known).count(),
            new.len()
        );
        for t in &new {
            eprintln!("  [新回归-{label}] {t}");
        }
        new.len()
    };
    let new_arity = ratchet("_arity_traps_baseline.txt", &arity_traps, "arity-500");
    let new_404 = ratchet(
        "_arity_unreachable_baseline.txt",
        &unreachable,
        "unreachable-404",
    );
    assert_eq!(
        new_arity + new_404,
        0,
        "出现基线之外的新缺陷：arity-500 新增 {new_arity} 条、unreachable-404 新增 {new_404} 条（见上方明细）"
    );
}
