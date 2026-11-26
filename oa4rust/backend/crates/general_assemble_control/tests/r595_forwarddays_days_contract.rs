// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R595：forwarddays 端点用 format!("{} + {} days", start, days) 作 TEXT 列日期上界，
//! '+' (0x2B) < 数字，导致 start+days 之后的日期行被 date <= 上界字典序排除。
//! 本契约 oneshot 真库造数：start=2026-01-01/days=30 应返 01-01/01-05/01-20 三条
//! （旧实现仅 1 条 → RED），超范围行不返；days=0 返当天；跨月 01-20+15=02-04 正确。
//! 并守卫 R594 Sakamoto helper/四端点/total_days/周末语义零回归、SQL 零改。

use axum::body::Body;
use axum::http::{Request, StatusCode};
use deadpool_postgres::Pool;
use general_assemble_control::general_assemble_control_router;
use serde_json::Value;
use tower::ServiceExt;

const SCRATCH_DB: &str = "oa_par_general_r595";
const SCRATCH_DB2: &str = "oa_par_general_r595b";
const SCRATCH_DB3: &str = "oa_par_general_r595c";
const BASE: &str = "/api/general/assemble/control/worktime";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL").expect("R595 需 DATABASE_URL 指向临时 PG");
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(mgr)
        .max_size(4)
        .build()
        .unwrap()
}

async fn fresh_scratch(scratch_db: &str) -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
    c.execute(
        &format!("DROP DATABASE IF EXISTS {scratch_db} WITH (FORCE)"),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {scratch_db}"), &[])
        .await
        .expect("create scratch");
    drop(c);
    let pool = pool_to(scratch_db);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 迁移");
    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM x_general_assemble_worktime", &[])
        .await
        .expect("清空 worktime");
    drop(c);
    pool
}

async fn drop_scratch(scratch_db: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {scratch_db} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}

async fn seed(pool: &Pool, id: &str, date: &str) {
    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_general_assemble_worktime \
         (id, date, is_holiday, is_workday, is_worktime, minutes, creator) \
         VALUES ($1, $2, false, true, true, 60, 'r595-probe')",
        &[&id, &date],
    )
    .await
    .expect("seed worktime");
}

async fn get_json(app: &axum::Router, uri: &str) -> Value {
    let r = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .expect("oneshot");
    assert_eq!(r.status(), StatusCode::OK, "uri={}", uri);
    let bytes = axum::body::to_bytes(r.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice(&bytes).expect("json")
}

fn dates_of(v: &Value) -> Vec<String> {
    v.get("data")
        .and_then(|x| x.as_array())
        .expect("data 数组")
        .iter()
        .map(|row| {
            row.get("date")
                .and_then(|d| d.as_str())
                .unwrap_or("")
                .to_string()
        })
        .collect()
}

#[tokio::test]
async fn forwarddays_returns_all_rows_within_calendar_upper_bound() {
    let pool = fresh_scratch(SCRATCH_DB).await;
    seed(&pool, "r595-1", "2026-01-01").await;
    seed(&pool, "r595-5", "2026-01-05").await;
    seed(&pool, "r595-20", "2026-01-20").await;
    seed(&pool, "r595-out", "2026-03-01").await;
    let app = general_assemble_control_router(pool.clone());
    let v = get_json(
        &app,
        &format!("{BASE}/forwarddays/start/2026-01-01/days/30"),
    )
    .await;
    let dates = dates_of(&v);
    assert_eq!(
        dates.len(),
        3,
        "旧上界 2026-01-01 + 30 days 字典序只返 1 条；正确上界 2026-01-31 应返 3 条: {dates:?}"
    );
    assert!(dates.contains(&"2026-01-01".to_string()));
    assert!(dates.contains(&"2026-01-05".to_string()));
    assert!(dates.contains(&"2026-01-20".to_string()));
    assert!(
        !dates.contains(&"2026-03-01".to_string()),
        "超范围行不得返回"
    );
    drop_scratch(SCRATCH_DB).await;

    // 核心源码守卫（真父旧上界会 fail，属核心用例）。
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("读 lib.rs");
    let count = |s: &str| src.matches(s).count();
    assert_eq!(
        count("fn worktime_forwarddays_end_date(start: &str, days: u32) -> String"),
        1,
        "日期加法 helper 只定义一次"
    );
    assert_eq!(
        count("let end_date = worktime_forwarddays_end_date(&start, days);"),
        1,
        "forwarddays 上界改用日期 helper"
    );
    assert_eq!(count("let end_date = format!("), 0, "TEXT 伪上界必须清零");
}

#[tokio::test]
async fn days_zero_and_r594_guards_hold() {
    let pool = fresh_scratch(SCRATCH_DB2).await;
    seed(&pool, "r595-z1", "2026-01-01").await;
    seed(&pool, "r595-z2", "2026-01-02").await;
    let app = general_assemble_control_router(pool.clone());
    let v = get_json(&app, &format!("{BASE}/forwarddays/start/2026-01-01/days/0")).await;
    assert_eq!(
        dates_of(&v),
        vec!["2026-01-01".to_string()],
        "days=0 上界应为 start 当天"
    );
    drop_scratch(SCRATCH_DB2).await;

    // 守卫：R594 成果零回归 + SQL 零改（真父本就 pass）。
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("读 lib.rs");
    let count = |s: &str| src.matches(s).count();
    assert_eq!(
        count("fn worktime_fallback_weekday(date: &str) -> Option<u8>"),
        1,
        "R594 helper 零回归"
    );
    assert_eq!(
        count("let weekday = worktime_fallback_weekday(&date);"),
        4,
        "R594 四端点调用零回归"
    );
    assert_eq!(count("total_days"), 0, "R594 伪星期算法清零");
    assert_eq!(
        count("weekday.map(|w| w >= 5).unwrap_or(false)"),
        4,
        "R594 周末语义零回归"
    );
    assert_eq!(
        count("let indefined = weekday.is_none();"),
        4,
        "R594 indefined 语义零回归"
    );
    assert!(
        count("date >= $1 AND date <= $2 ORDER BY date") >= 1,
        "forwarddays SQL 零改"
    );
}

#[tokio::test]
async fn forwarddays_cross_month_upper_bound() {
    let pool = fresh_scratch(SCRATCH_DB3).await;
    seed(&pool, "r595-c1", "2026-01-20").await;
    seed(&pool, "r595-c2", "2026-02-04").await;
    seed(&pool, "r595-c3", "2026-02-05").await;
    let app = general_assemble_control_router(pool.clone());
    let v = get_json(
        &app,
        &format!("{BASE}/forwarddays/start/2026-01-20/days/15"),
    )
    .await;
    let dates = dates_of(&v);
    assert_eq!(
        dates.len(),
        2,
        "2026-01-20 + 15 = 2026-02-04 跨月上界应返 2 条: {dates:?}"
    );
    assert!(dates.contains(&"2026-02-04".to_string()));
    assert!(!dates.contains(&"2026-02-05".to_string()));
    drop_scratch(SCRATCH_DB3).await;
}
