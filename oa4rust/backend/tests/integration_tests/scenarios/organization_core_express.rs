// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24：`organization/core/express` 的 `read_sync_enabled` 行为级用例。
//!
//! 三条都必须走真实 HTTP + 真实 PostgreSQL：
//! 1. `x_org_config` 有合法 `sync_enabled=true` 行 → `/config` 200，且断 body 具体内容；
//! 2. 有行但值非法（`maybe`）→ 500，且捕获日志里能读到**原始值**；
//! 3. 无行 → 503，文案指出缺 `x_org_config.sync_enabled`。
//!
//! 破坏实验锚点：把 `parse::<bool>()` 改回 `row.get::<bool>()`，第 1 条必须红。
//!
//! 本模块也会被 parity_runner / perf_baseline 等测试二进制编译（未被调用），
//! 与同目录其他 scenario 文件一样允许 dead_code。
#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::error_capture;
use crate::integration_tests::helpers::setup_test_server;

pub async fn organization_express_config_contract_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    // 先删干净，保证每条用例的起点可预期。
    set_sync_enabled_row(&pool, None).await;

    case_valid_row_returns_200(&client, &base, &auth, &pool).await;
    case_invalid_value_returns_500_with_raw_log(&client, &base, &auth, &pool).await;
    case_missing_row_returns_503(&client, &base, &auth, &pool).await;

    server.abort();
    let _ = server.await;
}

/// 有合法配置行 → 200。断言到 body 字段级，而不是只看状态码。
async fn case_valid_row_returns_200(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    set_sync_enabled_row(pool, Some("true")).await;

    let resp = client
        .get(format!("{base}/api/organization/core/express/config"))
        .header("Authorization", auth)
        .send()
        .await
        .expect("config request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("config body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "有合法 sync_enabled=true 行时 /config 应为 200，实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("config body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["message"], "", "body={raw}");
    assert_eq!(body["size"], 0, "body={raw}");
    assert_eq!(body["count"], 0, "body={raw}");
    assert_eq!(body["data"]["enabled"], true, "body={raw}");
    assert_eq!(body["data"]["syncInterval"], 300, "body={raw}");
    assert_eq!(body["data"]["maxRecords"], 10000, "body={raw}");

    // /status 走同一读取路径，也必须在 200 分支可执行。
    let resp = client
        .get(format!("{base}/api/organization/core/express/status"))
        .header("Authorization", auth)
        .send()
        .await
        .expect("status request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("status body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "有合法 sync_enabled=true 行时 /status 应为 200，实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("status body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["status"], "running", "body={raw}");
    assert_eq!(body["data"]["enabled"], true, "body={raw}");
    assert!(
        body["data"]["personCount"].is_number(),
        "personCount 应为数字，body={raw}"
    );
    assert!(
        body["data"]["groupCount"].is_number(),
        "groupCount 应为数字，body={raw}"
    );
}

/// 有行但值非法 → 500，日志必须带原始值（诊断要能定位到是哪条脏配置）。
async fn case_invalid_value_returns_500_with_raw_log(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    set_sync_enabled_row(pool, Some("maybe")).await;
    let _ = error_capture::drain();

    let resp = client
        .get(format!("{base}/api/organization/core/express/config"))
        .header("Authorization", auth)
        .send()
        .await
        .expect("config request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("config body");
    assert_eq!(
        status,
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        "非法值必须是 500（不是 503，也不是 200），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("config body must be JSON");
    assert_eq!(body["message"], "internal server error", "body={raw}");
    assert_eq!(
        body["prompt"], "com.x.base.core.project.exception.ExceptionInternal",
        "body={raw}"
    );

    let logs = error_capture::drain().join("\n");
    // tracing 会把 `{raw:?}` 的引号转义成 \"，所以这里按日志里的真实形态断言。
    assert!(
        logs.contains("x_org_config.sync_enabled=\\\"maybe\\\""),
        "日志必须带原始配置值 `maybe`，实际捕获日志：\n{logs}"
    );
    assert!(
        logs.contains("source="),
        "日志必须带 source 链（file:line + 原因），实际捕获日志：\n{logs}"
    );
}

/// 无行 → 503，文案指出缺哪一项。
async fn case_missing_row_returns_503(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    set_sync_enabled_row(pool, None).await;

    for path in ["config", "status"] {
        let resp = client
            .get(format!("{base}/api/organization/core/express/{path}"))
            .header("Authorization", auth)
            .send()
            .await
            .expect("express request failed");
        let status = resp.status();
        let raw = resp.text().await.expect("express body");
        assert_eq!(
            status,
            reqwest::StatusCode::SERVICE_UNAVAILABLE,
            "缺 sync_enabled 行时 /{path} 应为 503，实际 {status}，body={raw}"
        );
        let body: Value = serde_json::from_str(&raw).expect("express body must be JSON");
        assert_eq!(
            body["message"], "service not configured: missing x_org_config.sync_enabled",
            "/{path} 文案必须指出缺哪一项，body={raw}"
        );
        assert_eq!(
            body["prompt"], "com.x.base.core.project.exception.ExceptionInternal",
            "body={raw}"
        );
    }
}

/// `value=None` 表示删除该行；否则写入 `sync_enabled` 行。
async fn set_sync_enabled_row(pool: &Arc<TestPool>, value: Option<&str>) {
    let client = pool
        .as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client");
    client
        .execute(
            "DELETE FROM x_org_config WHERE config_key = 'sync_enabled'",
            &[],
        )
        .await
        .expect("delete sync_enabled row failed");
    if let Some(value) = value {
        client
            .execute(
                "INSERT INTO x_org_config (id, config_key, config_value) \
                 VALUES ('it-sync-enabled', 'sync_enabled', $1)",
                &[&value],
            )
            .await
            .expect("insert sync_enabled row failed");
    }
}
