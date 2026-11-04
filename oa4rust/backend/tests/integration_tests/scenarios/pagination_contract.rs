// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O17 分页契约：size/offset 越界必须 400，合法边界必须通过。
//!
//! 覆盖三类落点：
//! 1. `page_window` 端点（organization personcard）
//! 2. `page` 直接当 OFFSET 的旧式端点（processplatform draft）
//! 3. `from` 直接当 OFFSET 的图表端点（program_center bar）
//!
//! 所有越界断言都检查 400 + `ExceptionBadRequest` 错误体，避免以后有人把
//! 「显式拒绝」改回「静默钳位」而测试仍然绿。

#![allow(dead_code)]

use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TEST_DB;

const PERSONCARD: &str =
    "/api/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}";
const DRAFT: &str = "/api/processplatform/assemble/surface/draft/list/my/paging/{page}/size/{size}";
const BAR: &str = "/api/program_center/bar/create/mass/{from}/{count}";
const PERSON_FILTER: &str = "/api/person/list/filter/{page}/size/{size}";

async fn call(
    client: &Client,
    method: reqwest::Method,
    url: &str,
    auth: &str,
) -> (reqwest::StatusCode, String) {
    let resp = client
        .request(method, url)
        .header("Authorization", auth)
        .send()
        .await
        .expect("request failed");
    let status = resp.status();
    let body = resp.text().await.expect("read body failed");
    (status, body)
}

fn assert_bad_request(status: reqwest::StatusCode, body: &str, ctx: &str) {
    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "{ctx}: expected 400, body={body}"
    );
    let v: Value = serde_json::from_str(body)
        .unwrap_or_else(|_| panic!("{ctx}: 400 body is not JSON: {body}"));
    assert_eq!(
        v.get("prompt").and_then(Value::as_str),
        Some("com.x.base.core.project.exception.ExceptionBadRequest"),
        "{ctx}: unexpected prompt, body={body}"
    );
}

fn assert_ok_row_count(status: reqwest::StatusCode, body: &str, ctx: &str) -> usize {
    assert_eq!(status, reqwest::StatusCode::OK, "{ctx}: body={body}");
    let v: Value = serde_json::from_str(body)
        .unwrap_or_else(|_| panic!("{ctx}: 200 body is not JSON: {body}"));
    let data = v
        .get("data")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("{ctx}: missing data array, body={body}"));
    assert!(
        data.len() <= 200,
        "{ctx}: returned {} rows, expected <= 200",
        data.len()
    );
    data.len()
}

pub async fn pagination_contract_flow() {
    let pool = TEST_DB
        .get()
        .expect("test database not initialized; call init_test_database() first")
        .clone();

    let (addr, server_handle, token) =
        crate::integration_tests::helpers::setup_test_server(pool.clone())
            .await
            .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{addr}");
    let auth = format!("Bearer {token}");

    // ── 1. page_window 端点：size 边界 ─────────────────────────────────────
    let cases = [
        (1_000_000_000_i64, "size=1e9"),
        (201, "size=201"),
        (0, "size=0"),
        (-1, "size=-1"),
    ];
    for (size, ctx) in cases {
        let url = format!(
            "{base}{}",
            PERSONCARD
                .replace("{page}", "1")
                .replace("{size}", &size.to_string())
        );
        let (st, body) = call(&client, reqwest::Method::GET, &url, &auth).await;
        eprintln!("[pagination-contract] {ctx}: status={st} body={body}");
        assert_bad_request(st, &body, ctx);
    }

    // 非数字 size：axum 路径解析失败，同样是 400（响应体不是 JSON 信封）
    let url = format!(
        "{base}{}",
        PERSONCARD.replace("{page}", "1").replace("{size}", "abc")
    );
    let (st, body) = call(&client, reqwest::Method::GET, &url, &auth).await;
    eprintln!("[pagination-contract] size=abc: status={st} body={body}");
    assert_eq!(
        st,
        reqwest::StatusCode::BAD_REQUEST,
        "size=abc: body={body}"
    );

    // size=200 合法，返回行数不得超过 200
    let url = format!(
        "{base}{}",
        PERSONCARD.replace("{page}", "1").replace("{size}", "200")
    );
    let (st, body) = call(&client, reqwest::Method::GET, &url, &auth).await;
    eprintln!(
        "[pagination-contract] size=200: status={st} rows={}",
        body.len()
    );
    assert_ok_row_count(st, &body, "size=200");

    // 正常分页零回归
    let url = format!(
        "{base}{}",
        PERSONCARD.replace("{page}", "1").replace("{size}", "20")
    );
    let (st, body) = call(&client, reqwest::Method::GET, &url, &auth).await;
    eprintln!("[pagination-contract] normal page=1 size=20: status={st}");
    assert_ok_row_count(st, &body, "normal page=1 size=20");

    // page_window 的 offset 上界：1e18 页直接拒绝
    let url = format!(
        "{base}{}",
        PERSONCARD
            .replace("{page}", "1000000000000000000")
            .replace("{size}", "200")
    );
    let (st, body) = call(&client, reqwest::Method::GET, &url, &auth).await;
    eprintln!("[pagination-contract] page_window page=1e18: status={st} body={body}");
    assert_bad_request(st, &body, "page_window page=1e18");

    // ── 2. page 直接当 OFFSET 的旧式端点：offset 上界 ─────────────────────
    let draft = |page: &str, size: &str| {
        format!(
            "{base}{}",
            DRAFT.replace("{page}", page).replace("{size}", size)
        )
    };
    for (page, size, ctx) in [
        ("1000000000000000000", "20", "draft page=1e18"),
        ("1999801", "20", "draft page=1999801"),
    ] {
        let url = draft(page, size);
        let (st, body) = call(&client, reqwest::Method::POST, &url, &auth).await;
        eprintln!("[pagination-contract] {ctx}: status={st} body={body}");
        assert_bad_request(st, &body, ctx);
    }

    for (page, ctx) in [("1999800", "draft page=1999800"), ("0", "draft page=0")] {
        let url = draft(page, "20");
        let (st, body) = call(&client, reqwest::Method::POST, &url, &auth).await;
        eprintln!("[pagination-contract] {ctx}: status={st}");
        assert_ok_row_count(st, &body, ctx);
    }

    let url = draft("0", "201");
    let (st, body) = call(&client, reqwest::Method::POST, &url, &auth).await;
    eprintln!("[pagination-contract] draft size=201: status={st} body={body}");
    assert_bad_request(st, &body, "draft size=201");

    // ── 3. from 直接当 OFFSET 的图表端点：offset 上界 ─────────────────────
    for (from, count, ctx) in [
        ("1000000000000000000", "10", "bar from=1e18"),
        ("1999801", "10", "bar from=1999801"),
    ] {
        let url = format!(
            "{base}{}",
            BAR.replace("{from}", from).replace("{count}", count)
        );
        let (st, body) = call(&client, reqwest::Method::GET, &url, &auth).await;
        eprintln!("[pagination-contract] {ctx}: status={st} body={body}");
        assert_bad_request(st, &body, ctx);
    }

    let url = format!(
        "{base}{}",
        BAR.replace("{from}", "0").replace("{count}", "51")
    );
    let (st, _body) = call(&client, reqwest::Method::GET, &url, &auth).await;
    assert_eq!(
        st,
        reqwest::StatusCode::OK,
        "bar from=0 count=51 should be OK"
    );

    let url = format!(
        "{base}{}",
        BAR.replace("{from}", "0").replace("{count}", "201")
    );
    let (st, body) = call(&client, reqwest::Method::GET, &url, &auth).await;
    eprintln!("[pagination-contract] bar count=201: status={st} body={body}");
    assert_bad_request(st, &body, "bar count=201");

    // ── 4. 内联 clamp 端点（B1 补收的残余点）：size 越界同样 400 ──────────────
    for (size, ctx) in [
        (201_i64, "person_filter size=201"),
        (0, "person_filter size=0"),
    ] {
        let url = format!(
            "{base}{}",
            PERSON_FILTER
                .replace("{page}", "1")
                .replace("{size}", &size.to_string())
        );
        let resp = client
            .request(reqwest::Method::POST, &url)
            .header("Authorization", &auth)
            .json(&serde_json::json!({}))
            .send()
            .await
            .expect("request failed");
        let st = resp.status();
        let body = resp.text().await.expect("read body failed");
        eprintln!("[pagination-contract] {ctx}: status={st} body={body}");
        assert_bad_request(st, &body, ctx);
    }

    let url = format!(
        "{base}{}",
        PERSON_FILTER.replace("{page}", "1").replace("{size}", "20")
    );
    let resp = client
        .request(reqwest::Method::POST, &url)
        .header("Authorization", &auth)
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("request failed");
    let st = resp.status();
    let body = resp.text().await.expect("read body failed");
    eprintln!("[pagination-contract] person_filter size=20: status={st}");
    assert_eq!(
        st,
        reqwest::StatusCode::OK,
        "person_filter size=20: body={body}"
    );
    let v: Value = serde_json::from_str(&body).expect("person_filter body is not JSON");
    let rows = v
        .get("data")
        .and_then(|d| d.get("data"))
        .and_then(Value::as_array)
        .expect("person_filter missing data.data array");
    assert!(
        rows.len() <= 20,
        "person_filter returned {} rows",
        rows.len()
    );

    server_handle.abort();
}
