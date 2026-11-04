// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3：`x_hotpic` 的「全局热点图」模型（`(application, info_id)` 两列空串 = 部署级全局）。
//!
//! 生产库 51 行空键此前「写成功、读不到」——三条读路径都只做精确匹配，全局行被丢弃。
//! 本用例钉住：
//! 1. 5 条写端点缺省时落 `('' ,'')`，写后按业务键读回同值；
//! 2. 半截参数（只给 application 或只给 infoId）→ 400；
//! 3. 全局行在任意 application 视图下可见；
//! 4. 特定 application 行只在自身视图可见，不出现在别的视图；
//! 5. 写错 `(application, info_id)` 的行仍然读不到（写侧错误不被读侧全局兜底掩盖）；
//! 6. 51 行全局数据在任意视图下全部可见（hotpic crate 列表返回 51 条）。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const VIEW_A: (&str, &str) = ("it-view-a", "it-info-a");
const VIEW_B: (&str, &str) = ("it-view-b", "it-info-b");

pub async fn hotpic_global_scope_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    reset(&pool).await;
    case_five_writes_are_global(&client, &base, &auth, &pool).await;
    case_half_scope_is_bad_request(&client, &base, &auth).await;
    case_specific_scope_is_isolated(&client, &base, &auth, &pool).await;
    case_wrong_key_stays_invisible(&client, &base, &auth, &pool).await;
    case_fifty_one_global_rows_visible(&client, &base, &auth, &pool).await;
    reset(&pool).await;

    server.abort();
    let _ = server.await;
}

/// 5 条写端点缺省写 `('' ,'')`：逐条写后按业务键读回，断言同值 + 全局可见。
async fn case_five_writes_are_global(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    // 1) POST /api/hotpic/create/hotpic
    let body = assert_success_json(
        send(
            client,
            auth,
            reqwest::Method::POST,
            &format!("{base}/api/hotpic/create/hotpic"),
            Some(json!({ "title": "it-global-post", "imageUrl": "it-img-post" })),
        )
        .await,
        "POST create/hotpic",
    );
    let id = body["data"]["id"].as_str().expect("data.id").to_string();
    assert_global_row(pool, &id, "it-global-post", "it-img-post").await;

    // 2) POST /api/hotpic/assemble/control/user/hotpic（variant 路径，同一 handler）
    let body = assert_success_json(
        send(
            client,
            auth,
            reqwest::Method::POST,
            &format!("{base}/api/hotpic/assemble/control/user/hotpic"),
            Some(json!({ "title": "it-global-user", "imageUrl": "it-img-user" })),
        )
        .await,
        "POST assemble user/hotpic",
    );
    let id = body["data"]["id"].as_str().expect("data.id").to_string();
    assert_global_row(pool, &id, "it-global-user", "it-img-user").await;

    // 3) POST /api/hotpic/assemble/control/update/control/config
    let body = assert_success_json(
        send(
            client,
            auth,
            reqwest::Method::POST,
            &format!("{base}/api/hotpic/assemble/control/update/control/config"),
            Some(json!({ "name": "it-global-config-post" })),
        )
        .await,
        "POST update/control/config",
    );
    assert_eq!(body["data"]["updated"], true, "body={body}");
    assert_global_row_by_title(pool, "it-global-config-post").await;

    // 4) GET /api/hotpic_assemble_control/create/hotpic（GET 变体 + body）
    let body = assert_success_json(
        send(
            client,
            auth,
            reqwest::Method::GET,
            &format!("{base}/api/hotpic_assemble_control/create/hotpic"),
            Some(json!({ "title": "it-global-get-create", "imageUrl": "it-img-get" })),
        )
        .await,
        "GET create/hotpic",
    );
    let id = body["data"]["id"].as_str().expect("data.id").to_string();
    assert_global_row(pool, &id, "it-global-get-create", "it-img-get").await;

    // 5) GET /api/hotpic_assemble_control/update/control/config（GET 变体 + body）
    let body = assert_success_json(
        send(
            client,
            auth,
            reqwest::Method::GET,
            &format!("{base}/api/hotpic_assemble_control/update/control/config"),
            Some(json!({ "name": "it-global-config-get" })),
        )
        .await,
        "GET update/control/config",
    );
    assert_eq!(body["data"]["updated"], true, "body={body}");
    assert_global_row_by_title(pool, "it-global-config-get").await;

    // 写后读：5 行全部在任意视图可见，且读回值就是空键
    let total = count_our_rows(pool).await;
    assert_eq!(total, 5, "5 条写端点各应落 1 行，实际 {total}");

    for (app, info) in [VIEW_A, VIEW_B] {
        let body = get_json(
            client,
            auth,
            &format!("{base}/api/hotpic/user/hotpic/{app}/{info}"),
            app,
        )
        .await;
        let data = body["data"].as_array().expect("data must be array");
        assert_eq!(data.len(), 5, "视图 ({app},{info}) 应看到全部 5 行全局数据");
        assert_eq!(body["count"].as_i64(), Some(5), "body={body}");
        for row in data {
            assert_eq!(
                row["application"], "",
                "全局行 application 应为空串，row={row}"
            );
            assert_eq!(row["infoId"], "", "全局行 infoId 应为空串，row={row}");
        }
    }

    // 另两条读路径同样认全局：core_entity 列表/存在性、assemble 单对象
    let body = get_json(
        client,
        auth,
        &format!(
            "{base}/api/hotpic/core/entity/list/by/{}/{}",
            VIEW_A.0, VIEW_A.1
        ),
        "core entity list",
    )
    .await;
    assert_eq!(
        body["data"]["data"].as_array().map(|a| a.len()),
        Some(5),
        "core_entity 列表应返回 5 行全局数据，body={body}"
    );
    let body = get_json(
        client,
        auth,
        &format!(
            "{base}/api/hotpic/core/entity/exists/check/{}/{}",
            VIEW_A.0, VIEW_A.1
        ),
        "core entity exists",
    )
    .await;
    assert_eq!(body["data"]["allExists"], true, "body={body}");
    assert_eq!(body["data"]["count"].as_i64(), Some(5), "body={body}");
    let body = get_json(
        client,
        auth,
        &format!(
            "{base}/api/hotpic_assemble_control/user/hotpic/{}/{}",
            VIEW_A.0, VIEW_A.1
        ),
        "assemble single",
    )
    .await;
    assert_eq!(body["data"]["application"], "", "body={body}");
    assert_eq!(body["data"]["infoId"], "", "body={body}");
}

/// 半截参数：只给 application 或只给 infoId → 400（不许把半截当全局）。
async fn case_half_scope_is_bad_request(client: &Client, base: &str, auth: &str) {
    let cases: [(&str, reqwest::Method, &str); 3] = [
        (
            "POST create/hotpic",
            reqwest::Method::POST,
            "/api/hotpic/create/hotpic",
        ),
        (
            "POST update/control/config",
            reqwest::Method::POST,
            "/api/hotpic/assemble/control/update/control/config",
        ),
        (
            "GET create/hotpic",
            reqwest::Method::GET,
            "/api/hotpic_assemble_control/create/hotpic",
        ),
    ];
    for (label, method, path) in cases {
        for body in [
            json!({ "title": "it-global-half", "application": "it-view-a" }),
            json!({ "title": "it-global-half", "infoId": "it-info-a" }),
        ] {
            let (status, raw) = send(
                client,
                auth,
                method.clone(),
                &format!("{base}{path}"),
                Some(body),
            )
            .await;
            assert_eq!(
                status,
                reqwest::StatusCode::BAD_REQUEST,
                "{label} 半截参数应 400，实际 {status}，raw={raw}"
            );
            assert!(
                raw.contains("application"),
                "400 文案应点名 application，raw={raw}"
            );
        }
    }
}

/// 全局行任意视图可见；特定行只在本视图可见。
async fn case_specific_scope_is_isolated(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    reset(pool).await;
    let body = assert_success_json(
        send(
            client,
            auth,
            reqwest::Method::POST,
            &format!("{base}/api/hotpic/create/hotpic"),
            Some(json!({
                "title": "it-global-specific",
                "application": VIEW_A.0,
                "infoId": VIEW_A.1,
            })),
        )
        .await,
        "specific create",
    );
    let id = body["data"]["id"].as_str().expect("data.id").to_string();

    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT application, info_id FROM x_hotpic WHERE id = $1",
            &[&id],
        )
        .await
        .expect("read specific row failed");
    assert_eq!(row.get::<_, String>("application"), VIEW_A.0);
    assert_eq!(row.get::<_, String>("info_id"), VIEW_A.1);

    let body = get_json(
        client,
        auth,
        &format!("{base}/api/hotpic/user/hotpic/{}/{}", VIEW_A.0, VIEW_A.1),
        "own view",
    )
    .await;
    let data = body["data"].as_array().expect("data must be array");
    assert_eq!(data.len(), 1, "本视图应只看到这条专属行，body={body}");
    assert_eq!(data[0]["id"], id, "body={body}");
    assert_eq!(data[0]["application"], VIEW_A.0);
    assert_eq!(data[0]["infoId"], VIEW_A.1);

    let body = get_json(
        client,
        auth,
        &format!("{base}/api/hotpic/user/hotpic/{}/{}", VIEW_B.0, VIEW_B.1),
        "other view",
    )
    .await;
    assert_eq!(
        body["data"].as_array().map(|a| a.len()),
        Some(0),
        "别的 application 视图不得看到这条专属行，body={body}"
    );

    let body = get_json(
        client,
        auth,
        &format!(
            "{base}/api/hotpic/core/entity/exists/check/{}/{}",
            VIEW_B.0, VIEW_B.1
        ),
        "other view exists",
    )
    .await;
    assert_eq!(body["data"]["allExists"], false, "body={body}");
    assert_eq!(body["data"]["count"].as_i64(), Some(0), "body={body}");

    let (status, raw) = send(
        client,
        auth,
        reqwest::Method::GET,
        &format!(
            "{base}/api/hotpic_assemble_control/user/hotpic/{}/{}",
            VIEW_B.0, VIEW_B.1
        ),
        None,
    )
    .await;
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "读不到应是业务错误信封（HTTP 200 + type=error），实际 {status}，raw={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("body must be JSON");
    assert_eq!(body["type"], "error", "别的视图应读不到，body={raw}");
    assert_eq!(body["data"], Value::Null, "body={raw}");
}

/// 写错 `(application, info_id)` 的行仍然读不到：写侧错误不能被读侧全局兜底掩盖。
async fn case_wrong_key_stays_invisible(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let db = pg_client(pool).await;
    db.execute(
        "INSERT INTO x_hotpic (id, application, info_id, title, image_url, creator, create_time, deleted_at) \
         VALUES ('it-hotpic-wrong-key', 'default', '0', 'it-global-wrong', '', 'system', NOW(), NULL)",
        &[],
    )
    .await
    .expect("seed wrong-key hotpic failed");

    let body = get_json(
        client,
        auth,
        &format!("{base}/api/hotpic/user/hotpic/{}/{}", VIEW_B.0, VIEW_B.1),
        "wrong key view",
    )
    .await;
    assert_eq!(
        body["data"].as_array().map(|a| a.len()),
        Some(0),
        "写错键的行不得被全局兜底读出来，body={body}"
    );

    let body = get_json(
        client,
        auth,
        &format!(
            "{base}/api/hotpic/core/entity/exists/check/{}/{}",
            VIEW_A.0, VIEW_A.1
        ),
        "wrong key exists",
    )
    .await;
    assert_eq!(
        body["data"]["count"].as_i64(),
        Some(1),
        "错键行不得混进任何真实视图（该视图只有 1 条专属行），body={body}"
    );

    let body = get_json(
        client,
        auth,
        &format!("{base}/api/hotpic/user/hotpic/{}/{}", VIEW_A.0, VIEW_A.1),
        "wrong key own view",
    )
    .await;
    let data = body["data"].as_array().expect("data must be array");
    assert_eq!(data.len(), 1, "body={body}");
    assert!(
        !data.iter().any(|r| r["id"] == "it-hotpic-wrong-key"),
        "错键行不得出现在 (it-view-a,it-info-a) 视图，body={body}"
    );
}

/// 51 行空键（= 生产库现状规模）在任意视图全部可见。
async fn case_fifty_one_global_rows_visible(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    reset(pool).await;
    let db = pg_client(pool).await;
    for i in 0..51 {
        db.execute(
            "INSERT INTO x_hotpic (id, application, info_id, title, image_url, creator, create_time, deleted_at) \
             VALUES ($1, '', '', $2, '', 'system', NOW(), NULL)",
            &[&format!("it-hotpic-51-{i:02}"), &format!("it-global-51-{i:02}")],
        )
        .await
        .expect("seed 51 global hotpics failed");
    }

    for (app, info) in [VIEW_A, VIEW_B] {
        let body = get_json(
            client,
            auth,
            &format!("{base}/api/hotpic/user/hotpic/{app}/{info}"),
            app,
        )
        .await;
        let data = body["data"].as_array().expect("data must be array");
        assert_eq!(
            data.len(),
            51,
            "视图 ({app},{info}) 应看到全部 51 行全局数据，body count={}",
            body["count"]
        );
        assert_eq!(body["count"].as_i64(), Some(51), "body={body}");
        assert!(
            data.iter()
                .all(|r| r["application"] == "" && r["infoId"] == ""),
            "51 行都应读回空键，body={body}"
        );
    }

    let body = get_json(
        client,
        auth,
        &format!(
            "{base}/api/hotpic/core/entity/exists/check/{}/{}",
            VIEW_A.0, VIEW_A.1
        ),
        "51 exists",
    )
    .await;
    assert_eq!(body["data"]["count"].as_i64(), Some(51), "body={body}");

    let body = get_json(
        client,
        auth,
        &format!(
            "{base}/api/hotpic/core/entity/list/by/{}/{}",
            VIEW_A.0, VIEW_A.1
        ),
        "51 list",
    )
    .await;
    assert_eq!(
        body["data"]["data"].as_array().map(|a| a.len()),
        Some(20),
        "core_entity 列表有 limit(20)，应返回 20 条全局行，body={body}"
    );
}

async fn assert_global_row(pool: &Arc<TestPool>, id: &str, title: &str, image_url: &str) {
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT application, info_id, title, image_url FROM x_hotpic \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .expect("read global hotpic failed");
    assert_eq!(
        row.get::<_, String>("application"),
        "",
        "缺省写应落全局行（application 空串）"
    );
    assert_eq!(
        row.get::<_, String>("info_id"),
        "",
        "缺省写应落全局行（info_id 空串）"
    );
    assert_eq!(row.get::<_, String>("title"), title);
    assert_eq!(
        row.get::<_, String>("image_url"),
        image_url,
        "写后必读：image_url 应与请求同值"
    );
}

async fn assert_global_row_by_title(pool: &Arc<TestPool>, title: &str) {
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT application, info_id FROM x_hotpic \
             WHERE title = $1 AND deleted_at IS NULL",
            &[&title],
        )
        .await
        .expect("read config hotpic failed");
    assert_eq!(row.get::<_, String>("application"), "", "title={title}");
    assert_eq!(row.get::<_, String>("info_id"), "", "title={title}");
}

async fn count_our_rows(pool: &Arc<TestPool>) -> i64 {
    let db = pg_client(pool).await;
    db.query_one(
        "SELECT COUNT(*) AS cnt FROM x_hotpic WHERE title LIKE 'it-global%' AND deleted_at IS NULL",
        &[],
    )
    .await
    .expect("count hotpic rows failed")
    .get("cnt")
}

async fn reset(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_hotpic WHERE title LIKE 'it-global%' OR id LIKE 'it-hotpic-%'",
        &[],
    )
    .await
    .expect("reset x_hotpic failed");
}

async fn get_json(client: &Client, auth: &str, url: &str, label: &str) -> Value {
    let (status, raw) = send(client, auth, reqwest::Method::GET, url, None).await;
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "{label} 应为 200，实际 {status}，raw={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "{label} body={raw}");
    body
}

fn assert_success_json((status, raw): (reqwest::StatusCode, String), label: &str) -> Value {
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "{label} 应为 200（缺省 = 全局热点图），实际 {status}，raw={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "{label} body={raw}");
    body
}

async fn send(
    client: &Client,
    auth: &str,
    method: reqwest::Method,
    url: &str,
    body: Option<Value>,
) -> (reqwest::StatusCode, String) {
    let mut builder = client.request(method, url).header("Authorization", auth);
    if let Some(b) = body {
        builder = builder.json(&b);
    }
    let resp = builder.send().await.expect("request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("body");
    (status, raw)
}

async fn pg_client(pool: &Arc<TestPool>) -> deadpool_postgres::Object {
    pool.as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client")
}
