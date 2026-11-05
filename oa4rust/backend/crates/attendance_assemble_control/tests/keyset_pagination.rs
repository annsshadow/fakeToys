// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O33：考勤统计展示的 keyset 分页缺陷回归。
//!
//! 旧实现形如 `WHERE id > $1 ORDER BY year DESC, month DESC LIMIT n`：游标键
//! （id）与排序键（year、month）不一致，跨页时会漏行并重行（见本文件断言与
//! 台账 O33 记录的实测输出）。本测试在专用 scratch 库 `oa_par_att_ks` 里造
//! 跨页数据（含并列值与 NULL），按「下一页」循环翻完全集，断言：
//! 1. 全集恰好等于按显示顺序排列的期望序列（不漏、不重、次序稳定）；
//! 2. `prev` 取到游标之前的最近一页，并按显示顺序返回；
//! 3. 升序端点（work_date）同样成立，NULL 落在末尾。

use attendance_assemble_control::{
    statisticshow_filter_personMonth_list_id_next_count,
    statisticshow_filter_personMonth_list_id_prev_count,
    statisticshow_filter_topUnitDay_list_id_next_count,
};
use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use std::collections::HashSet;

const SCRATCH_DESC: &str = "oa_par_att_ks_desc";
const SCRATCH_PREV: &str = "oa_par_att_ks_prev";
const SCRATCH_ASC: &str = "oa_par_att_ks_asc";
const SCRATCH_NUM_DESC: &str = "oa_par_att_ks_num_desc";
const SCRATCH_NUM_PREV: &str = "oa_par_att_ks_num_prev";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(mgr)
        .max_size(4)
        .build()
        .unwrap()
}

// 各用例并行跑，各自用独立库，避免 DROP/CREATE 互相踩（E23505 pg_database）。
async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
    c.execute(
        &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {dbname}"), &[])
        .await
        .expect("create scratch");
    drop(c);
    let pool = pool_to(dbname);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 库迁移");
    pool
}

async fn drop_scratch(dbname: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}

fn ids_of(data: &serde_json::Value) -> Vec<String> {
    data.as_array()
        .expect("list data 必须是数组")
        .iter()
        .map(|r| {
            r.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

/// 按 next 游标翻完整个列表：首屏哨兵 `(0)` → 上一页末条 id → … → 空页。
macro_rules! walk_next {
    ($fetch:expr) => {{
        let mut cursor = "(0)".to_string();
        let mut collected: Vec<String> = Vec::new();
        for _ in 0..16 {
            let page = $fetch(cursor.clone()).await;
            if page.is_empty() {
                break;
            }
            cursor = page.last().cloned().expect("页非空");
            collected.extend(page);
        }
        collected
    }};
}

async fn person_month_next(pool: Pool, cursor: String) -> Vec<String> {
    person_month_next_count(pool, cursor, 2).await
}

async fn person_month_next_count(pool: Pool, cursor: String, count: i64) -> Vec<String> {
    let res =
        statisticshow_filter_personMonth_list_id_next_count(Extension(pool), Path((cursor, count)))
            .await
            .expect("personMonth next 不应失败");
    ids_of(res.0.data.as_ref().expect("list 缺 data"))
}

async fn unit_day_next(pool: Pool, cursor: String) -> Vec<String> {
    let res =
        statisticshow_filter_topUnitDay_list_id_next_count(Extension(pool), Path((cursor, 2_i64)))
            .await
            .expect("topUnitDay next 不应失败");
    ids_of(res.0.data.as_ref().expect("list 缺 data"))
}

#[tokio::test]
async fn desc_year_month_keyset_walks_every_row_exactly_once() {
    let pool = fresh_scratch(SCRATCH_DESC).await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM x_attendance_statisticshow", &[])
        .await
        .expect("清空 statisticshow");
    // id 序（k1<k2<…）与排序键 (year,month) 刻意错位；k1/k5 并列，k7 为 NULL。
    c.execute(
        "INSERT INTO x_attendance_statisticshow (id, person_id, year, month, status, unit_id) \
         VALUES \
         ('k1', 'p', '2025', '11', 'ok', NULL), \
         ('k2', 'p', '2024', '01', 'ok', NULL), \
         ('k3', 'p', '2025', '12', 'ok', NULL), \
         ('k4', 'p', '2024', '12', 'ok', NULL), \
         ('k5', 'p', '2025', '11', 'ok', NULL), \
         ('k6', 'p', '2023', '06', 'ok', NULL), \
         ('k7', 'p', NULL,   NULL,   'ok', NULL)",
        &[],
    )
    .await
    .expect("seed statisticshow");
    drop(c);

    let actual = walk_next!(|cursor: String| person_month_next(pool.clone(), cursor));
    println!("漏行/重行实测（personMonth next）: {actual:?}");
    let expected: Vec<String> = ["k3", "k5", "k1", "k4", "k2", "k6", "k7"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        actual, expected,
        "year DESC, month DESC 的分页必须不漏不重、并列按 id DESC、NULL 落末"
    );

    drop_scratch(SCRATCH_DESC).await;
}

#[tokio::test]
async fn desc_prev_returns_window_before_cursor_in_display_order() {
    let pool = fresh_scratch(SCRATCH_PREV).await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM x_attendance_statisticshow", &[])
        .await
        .expect("清空 statisticshow");
    c.execute(
        "INSERT INTO x_attendance_statisticshow (id, person_id, year, month, status, unit_id) \
         VALUES \
         ('k1', 'p', '2025', '11', 'ok', NULL), \
         ('k2', 'p', '2024', '01', 'ok', NULL), \
         ('k3', 'p', '2025', '12', 'ok', NULL), \
         ('k4', 'p', '2024', '12', 'ok', NULL), \
         ('k5', 'p', '2025', '11', 'ok', NULL)",
        &[],
    )
    .await
    .expect("seed statisticshow");
    drop(c);

    let res = statisticshow_filter_personMonth_list_id_prev_count(
        Extension(pool.clone()),
        Path(("k2".to_string(), 2_i64)),
    )
    .await
    .expect("personMonth prev 不应失败");
    let actual = ids_of(res.0.data.as_ref().expect("list 缺 data"));
    println!("prev 实测（游标 k2）: {actual:?}");
    assert_eq!(
        actual,
        vec!["k1".to_string(), "k4".to_string()],
        "prev 必须取游标之前的最近两条，并按 year DESC, month DESC 返回"
    );

    drop_scratch(SCRATCH_PREV).await;
}

#[tokio::test]
async fn asc_work_date_keyset_walks_every_row_exactly_once() {
    let pool = fresh_scratch(SCRATCH_ASC).await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM x_attendance_statisticshow", &[])
        .await
        .expect("清空 statisticshow");
    c.execute(
        "INSERT INTO x_attendance_statisticshow (id, person_id, year, month, status, unit_id, work_date) \
         VALUES \
         ('w1', 'p', '2024', '01', 'ok', NULL, '2024-01-01'), \
         ('w2', 'p', '2024', '01', 'ok', NULL, '2024-01-03'), \
         ('w3', 'p', '2024', '01', 'ok', NULL, '2024-01-01'), \
         ('w4', 'p', '2024', '01', 'ok', NULL, NULL),         \
         ('w5', 'p', '2024', '01', 'ok', NULL, '2024-01-02')",
        &[],
    )
    .await
    .expect("seed statisticshow");
    drop(c);

    let actual = walk_next!(|cursor: String| unit_day_next(pool.clone(), cursor));
    println!("漏行/重行实测（topUnitDay next）: {actual:?}");
    let expected: Vec<String> = ["w1", "w3", "w5", "w2", "w4"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        actual, expected,
        "work_date ASC 的分页必须不漏不重、并列按 id ASC、NULL 落末"
    );

    drop_scratch(SCRATCH_ASC).await;
}

/// O34：month 是月份数字，TEXT 列未补零时字典序错误（'10' < '2'）。
/// 种子刻意混入未补零月份、并列 (year, month)、NULL 与短尾页，按 page size
/// 1 / 2 / 3 各走一遍到空页，证明「排序键与游标键」同步后不漏行、不重行。
#[tokio::test]
async fn desc_numeric_month_keyset_walks_every_row_exactly_once() {
    let pool = fresh_scratch(SCRATCH_NUM_DESC).await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM x_attendance_statisticshow", &[])
        .await
        .expect("清空 statisticshow");
    // k3/k5 在文本序下会与 k8/k1 相对位置颠倒；k1/k8 并列同月；
    // k2/k9 是跨位数边界的 1 vs 2；k7 全 NULL；9 行 = 4 整页 + 1 行尾页（page size 2）。
    c.execute(
        "INSERT INTO x_attendance_statisticshow (id, person_id, year, month, status, unit_id) \
         VALUES \
         ('k1', 'p', '2025', '2',  'ok', NULL), \
         ('k2', 'p', '2024', '1',  'ok', NULL), \
         ('k3', 'p', '2025', '11', 'ok', NULL), \
         ('k4', 'p', '2024', '12', 'ok', NULL), \
         ('k5', 'p', '2025', '10', 'ok', NULL), \
         ('k6', 'p', '2024', '9',  'ok', NULL), \
         ('k7', 'p', NULL,   NULL, 'ok', NULL), \
         ('k8', 'p', '2025', '2',  'ok', NULL), \
         ('k9', 'p', '2024', '2',  'ok', NULL)",
        &[],
    )
    .await
    .expect("seed statisticshow");
    drop(c);

    let expected: Vec<String> = ["k3", "k5", "k8", "k1", "k4", "k6", "k9", "k2", "k7"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    for size in [1_i64, 2, 3] {
        let mut cursor = "(0)".to_string();
        let mut collected: Vec<String> = Vec::new();
        for _ in 0..32 {
            let page = person_month_next_count(pool.clone(), cursor.clone(), size).await;
            if page.is_empty() {
                break;
            }
            cursor = page.last().cloned().expect("页非空");
            collected.extend(page);
        }
        println!("O34 跨页实测（month 数值序, page_size={size}）: {collected:?}");
        assert_eq!(
            collected.len(),
            9,
            "page_size={size} 行数不符：存在漏行或多页累积"
        );
        let uniq: HashSet<&String> = collected.iter().collect();
        assert_eq!(uniq.len(), 9, "page_size={size} 存在重复行");
        assert_eq!(
            collected, expected,
            "page_size={size}：year DESC、month 数值 DESC、并列 id DESC、NULL 落末"
        );
    }

    drop_scratch(SCRATCH_NUM_DESC).await;
}

/// O34：prev 窗口同样跨数值边界与并列块（游标 k4 之前的最近两条是 k8/k1）。
#[tokio::test]
async fn desc_numeric_month_prev_returns_window_before_cursor_in_display_order() {
    let pool = fresh_scratch(SCRATCH_NUM_PREV).await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM x_attendance_statisticshow", &[])
        .await
        .expect("清空 statisticshow");
    c.execute(
        "INSERT INTO x_attendance_statisticshow (id, person_id, year, month, status, unit_id) \
         VALUES \
         ('k1', 'p', '2025', '2',  'ok', NULL), \
         ('k2', 'p', '2024', '1',  'ok', NULL), \
         ('k3', 'p', '2025', '11', 'ok', NULL), \
         ('k4', 'p', '2024', '12', 'ok', NULL), \
         ('k5', 'p', '2025', '10', 'ok', NULL), \
         ('k6', 'p', '2024', '9',  'ok', NULL), \
         ('k7', 'p', NULL,   NULL, 'ok', NULL), \
         ('k8', 'p', '2025', '2',  'ok', NULL), \
         ('k9', 'p', '2024', '2',  'ok', NULL)",
        &[],
    )
    .await
    .expect("seed statisticshow");
    drop(c);

    let res = statisticshow_filter_personMonth_list_id_prev_count(
        Extension(pool.clone()),
        Path(("k4".to_string(), 2_i64)),
    )
    .await
    .expect("personMonth prev 不应失败");
    let actual = ids_of(res.0.data.as_ref().expect("list 缺 data"));
    println!("O34 prev 实测（游标 k4）: {actual:?}");
    assert_eq!(
        actual,
        vec!["k8".to_string(), "k1".to_string()],
        "prev 必须取数值序下游标之前的最近两条，并按显示顺序返回"
    );

    drop_scratch(SCRATCH_NUM_PREV).await;
}
