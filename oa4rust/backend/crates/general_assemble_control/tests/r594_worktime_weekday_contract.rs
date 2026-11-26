// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R594：worktime 四端点在「无配置行」回退分支必须用真日历星期判周末，
//! 禁用 legacy 的 (year-2000)*365+(month-1)*30+day 伪算法。
//! 通过 oneshot 路由真讨 DATABASE_URL 指向的临时 scratch 库（无 worktime 行），
//! 断言 2026-01-03(周六)/2026-01-04(周日)/2024-02-29(周四)/2000-01-01(周六)/2025-06-01(周日)
//! 的真日历结果，并验证周一反例、非法日期 indefined、DB 命中行优先。

use axum::body::Body;
use axum::http::{Request, StatusCode};
use deadpool_postgres::Pool;
use general_assemble_control::general_assemble_control_router;
use serde_json::Value;
use tower::ServiceExt;

const SCRATCH_DB: &str = "oa_par_general_r594";
const SCRATCH_DB2: &str = "oa_par_general_r594b";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL").expect("R594 需 DATABASE_URL 指向临时 PG");
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
        .expect("清空 worktime 配置行");
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

fn flag(v: &Value, key: &str) -> bool {
    v.get(key)
        .and_then(|x| x.as_bool())
        .unwrap_or_else(|| panic!("缺字段 {key}"))
}

#[tokio::test]
async fn fallback_weekday_uses_real_calendar_on_all_four_endpoints() {
    let pool = fresh_scratch(SCRATCH_DB).await;
    let app = general_assemble_control_router(pool.clone());
    let base = "/api/general/assemble/control/worktime";

    // 周六 2026-01-03：旧伪算法把 (26*365+0+3)%7=4 当周四 → isHoliday=false（错）。
    assert!(flag(
        &get_json(&app, &format!("{base}/indefined/holiday/2026-01-03")).await,
        "isHoliday"
    ));
    assert!(flag(
        &get_json(&app, &format!("{base}/is/holiday/2026-01-03")).await,
        "isHoliday"
    ));
    assert!(!flag(
        &get_json(&app, &format!("{base}/indefined/workday/2026-01-03")).await,
        "isWorkday"
    ));
    assert!(!flag(
        &get_json(&app, &format!("{base}/is/workday/2026-01-03")).await,
        "isWorkday"
    ));

    // 周日 2026-01-04：旧算法 (26*365+0+4)%7=5 → 误判周末。
    assert!(flag(
        &get_json(&app, &format!("{base}/indefined/holiday/2026-01-04")).await,
        "isHoliday"
    ));
    assert!(flag(
        &get_json(&app, &format!("{base}/is/holiday/2026-01-04")).await,
        "isHoliday"
    ));

    // 闰日 2024-02-29 是周四：旧算法 (24*365+29)%7=6 → 误判周末。
    assert!(!flag(
        &get_json(&app, &format!("{base}/indefined/holiday/2024-02-29")).await,
        "isHoliday"
    ));
    assert!(flag(
        &get_json(&app, &format!("{base}/indefined/workday/2024-02-29")).await,
        "isWorkday"
    ));

    // 跨越世纪/年首样本。
    assert!(flag(
        &get_json(&app, &format!("{base}/indefined/holiday/2000-01-01")).await,
        "isHoliday"
    ));
    assert!(flag(
        &get_json(&app, &format!("{base}/indefined/holiday/2025-06-01")).await,
        "isHoliday"
    ));

    // 周一反例：四个端点均不得误判周末。
    assert!(!flag(
        &get_json(&app, &format!("{base}/indefined/holiday/2026-01-05")).await,
        "isHoliday"
    ));
    assert!(!flag(
        &get_json(&app, &format!("{base}/is/holiday/2026-01-05")).await,
        "isHoliday"
    ));
    assert!(flag(
        &get_json(&app, &format!("{base}/indefined/workday/2026-01-05")).await,
        "isWorkday"
    ));
    assert!(flag(
        &get_json(&app, &format!("{base}/is/workday/2026-01-05")).await,
        "isWorkday"
    ));

    // 非法日期走 indefined=true，不进入伪星期分支。
    let bad = get_json(&app, &format!("{base}/is/holiday/not-a-date")).await;
    assert!(flag(&bad, "indefined"), "非法日期必须 indefined=true");
    assert!(!flag(&bad, "isHoliday"));

    drop_scratch(SCRATCH_DB).await;
}

#[tokio::test]
async fn db_configured_row_takes_precedence_and_fallback_is_removed_from_source() {
    let pool = fresh_scratch(SCRATCH_DB2).await;
    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_general_assemble_worktime (id, date, is_holiday, is_workday, creator) VALUES ('r594-seed', '2026-01-03', false, true, 'probe')",
        &[],
    ).await.expect("seed worktime");
    drop(c);
    let app = general_assemble_control_router(pool.clone());
    let base = "/api/general/assemble/control/worktime";
    let v = get_json(&app, &format!("{base}/indefined/holiday/2026-01-03")).await;
    assert!(!flag(&v, "isHoliday"), "DB 命中行必须优先于回退");
    assert!(!flag(&v, "indefined"), "DB 命中行 indefined=false");
    drop_scratch(SCRATCH_DB2).await;

    // 源码守卫：四处理换为 helper，伪算法清零，下游语义保留。
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("读 lib.rs");
    let count = |s: &str| src.matches(s).count();
    assert_eq!(
        count("fn worktime_fallback_weekday(date: &str) -> Option<u8>"),
        1,
        "helper 只定义一次"
    );
    assert_eq!(
        count("let weekday = worktime_fallback_weekday(&date);"),
        4,
        "四端点均用 helper"
    );
    assert_eq!(count("total_days"), 0, "伪算法必须清零");
    assert_eq!(
        count("weekday.map(|w| w >= 5).unwrap_or(false)"),
        4,
        "周末判定语义不变"
    );
    assert_eq!(
        count("let indefined = weekday.is_none();"),
        4,
        "indefined 语义不变"
    );
}
