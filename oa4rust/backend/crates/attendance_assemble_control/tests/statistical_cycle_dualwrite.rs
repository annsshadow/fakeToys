// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36：`x_attendance_statistical_cycle` 读写列不一致回归。
//!
//! 写侧只写 `cycle_year/cycle_month`，读侧却按 `year/month` 过滤/排序，导致
//! 新建周期在 `cycleDetail` 中查不到。本测试在独立 scratch 库里同时覆盖三种
//! 存量形态：
//! 1. 双写新行（修复后 write 同时写 `year/month`）；
//! 2. 历史导入行（只有 `year/month`）；
//! 3. 旧 handler 行（只有 `cycle_year/cycle_month`）。
//!
//! 读取侧统一 `COALESCE(year, cycle_year)` / `COALESCE(month, cycle_month)`。
//! 排序按数字语义（`'10' > '2'`），并以 `id DESC` 作并列裁决。

use attendance_assemble_control::{
    attendancestatisticalcycle_create, attendancestatisticalcycle_cycleDetail_year_month,
    attendancestatisticalcycle_id, attendancestatisticalcycle_list_all,
};
use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::{json, Value};

const SCRATCH_CREATE: &str = "oa_o36_cycle_create";
const SCRATCH_LEGACY: &str = "oa_o36_cycle_legacy";
const SCRATCH_OLD: &str = "oa_o36_cycle_old";
const SCRATCH_ORDER: &str = "oa_o36_cycle_order";

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

async fn seed_admin(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(
        "INSERT INTO auth_person (id, unique_id, name, password_hash) \
           VALUES ('o36-admin', 'o36-admin', 'o36 admin', 'x'); \
         INSERT INTO auth_role (id, name, deleted_at, disable) \
           VALUES ('o36-admin-role', 'admin', NULL, false); \
         INSERT INTO auth_person_role (person_id, role_id, unit_id) \
           VALUES ('o36-admin', 'o36-admin-role', 'o36-unit');",
    )
    .await
    .expect("seed admin");
}

fn admin_session() -> shared::session::Session {
    shared::session::Session {
        token: "o36-tok".to_string(),
        person_unique: "o36-admin".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
    }
}

fn field<'a>(data: &'a Value, key: &str) -> &'a str {
    data.get(key).and_then(|v| v.as_str()).unwrap_or_default()
}

fn raw<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

/// 改前基线（未修 write/read 列时跑本用例）：create 成功但 cycleDetail 返回
/// `statistical cycle not found`。改后：同一周期 200 + id/year/month。
#[tokio::test]
async fn new_cycle_is_visible_through_cycle_detail() {
    let pool = fresh_scratch(SCRATCH_CREATE).await;
    seed_admin(&pool).await;

    let created = attendancestatisticalcycle_create(
        Extension(pool.clone()),
        Extension(admin_session()),
        Json(json!({
            "topUnitName": "T",
            "unitName": "U",
            "cycleYear": "2026",
            "cycleMonth": "10",
            "description": "o36"
        })),
    )
    .await
    .expect("create handler 不得 500");
    assert_eq!(
        created.0.r#type.as_deref(),
        Some("success"),
        "create 未成功: {}",
        raw(&created.0)
    );
    let id = created
        .0
        .data
        .as_ref()
        .and_then(|d| d.get("id"))
        .and_then(|v| v.as_str())
        .expect("create 返回 id")
        .to_string();

    let detail = attendancestatisticalcycle_cycleDetail_year_month(
        Extension(pool.clone()),
        Path(("2026".to_string(), "10".to_string())),
    )
    .await
    .expect("cycleDetail handler 不得 500");
    assert_eq!(
        detail.0.r#type.as_deref(),
        Some("success"),
        "写 cycle_* 却按 year/month 查，新周期查不到；原始响应={}",
        raw(&detail.0)
    );
    let data = detail.0.data.expect("cycleDetail 成功必有 data");
    assert_eq!(field(&data, "id"), id.as_str());
    assert_eq!(field(&data, "year"), "2026");
    assert_eq!(field(&data, "month"), "10");

    // 读写兜底（COALESCE）会让「只写 cycle_*」的读路径仍然成功，因此写侧
    // 双写必须由物理列断言守：去掉 year/month 的写入 → 这里必须红。
    let c = pool.get().await.expect("scratch 连接");
    let row = c
        .query_one(
            "SELECT cycle_year, cycle_month, year, month FROM x_attendance_statistical_cycle WHERE id = $1",
            &[&id],
        )
        .await
        .expect("回读新行");
    for (col, want) in [
        ("cycle_year", "2026"),
        ("cycle_month", "10"),
        ("year", "2026"),
        ("month", "10"),
    ] {
        assert_eq!(
            row.get::<_, Option<String>>(col).as_deref(),
            Some(want),
            "写侧必须双写 {col}（去掉 year/month 写入即红）"
        );
    }
    drop(c);

    drop_scratch(SCRATCH_CREATE).await;
}

/// 历史导入行只有 `year/month`：detail、单条详情、列表三处都必须可读。
#[tokio::test]
async fn legacy_year_month_only_row_is_readable() {
    let pool = fresh_scratch(SCRATCH_LEGACY).await;
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(
        "INSERT INTO x_attendance_statistical_cycle \
           (id, year, month, cycle_status, creator, create_time, update_time) VALUES \
           ('o36-legacy', '2026', '10', 'published', 'p', NOW(), NOW()), \
           ('o36-legacy2025', '2025', '2', 'published', 'p', NOW(), NOW());",
    )
    .await
    .expect("seed legacy rows");
    drop(c);

    let detail = attendancestatisticalcycle_cycleDetail_year_month(
        Extension(pool.clone()),
        Path(("2026".to_string(), "10".to_string())),
    )
    .await
    .expect("cycleDetail handler 不得 500");
    assert_eq!(
        detail.0.r#type.as_deref(),
        Some("success"),
        "历史行（仅 year/month）必须可读: {}",
        raw(&detail.0)
    );
    let data = detail.0.data.expect("data");
    assert_eq!(field(&data, "id"), "o36-legacy");

    let by_id =
        attendancestatisticalcycle_id(Extension(pool.clone()), Path("o36-legacy".to_string()))
            .await
            .expect("id handler 不得 500");
    assert_eq!(
        by_id.0.r#type.as_deref(),
        Some("success"),
        "{}",
        raw(&by_id.0)
    );
    let data = by_id.0.data.expect("data");
    assert_eq!(field(&data, "year"), "2026");
    assert_eq!(field(&data, "month"), "10");

    let list = attendancestatisticalcycle_list_all(Extension(pool.clone()))
        .await
        .expect("list handler 不得 500");
    assert_eq!(
        list.0.r#type.as_deref(),
        Some("success"),
        "{}",
        raw(&list.0)
    );
    let rows = list.0.data.expect("data");
    let ids: Vec<&str> = rows
        .as_array()
        .expect("list data 是数组")
        .iter()
        .map(|r| field(r, "id"))
        .collect();
    // 2026/10 数字语义大于 2025/2。
    assert_eq!(ids, vec!["o36-legacy", "o36-legacy2025"], "{rows:?}");

    drop_scratch(SCRATCH_LEGACY).await;
}

/// 旧 handler 行只有 `cycle_year/cycle_month`：COALESCE 反向兜底后仍可读。
#[tokio::test]
async fn old_cycle_year_only_row_is_readable() {
    let pool = fresh_scratch(SCRATCH_OLD).await;
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(
        "INSERT INTO x_attendance_statistical_cycle \
           (id, cycle_year, cycle_month, cycle_status, creator, create_time, update_time) VALUES \
           ('o36-oldwrite', '2026', '10', 'published', 'p', NOW(), NOW());",
    )
    .await
    .expect("seed old rows");
    drop(c);

    let detail = attendancestatisticalcycle_cycleDetail_year_month(
        Extension(pool.clone()),
        Path(("2026".to_string(), "10".to_string())),
    )
    .await
    .expect("cycleDetail handler 不得 500");
    assert_eq!(
        detail.0.r#type.as_deref(),
        Some("success"),
        "旧 write 行（仅 cycle_*）必须可读: {}",
        raw(&detail.0)
    );
    let data = detail.0.data.expect("data");
    assert_eq!(field(&data, "id"), "o36-oldwrite");

    drop_scratch(SCRATCH_OLD).await;
}

/// 列表排序：month 走数字语义（`'10' > '2'`），同键以 `id DESC` 裁决。
#[tokio::test]
async fn list_orders_months_numerically_with_id_tiebreak() {
    let pool = fresh_scratch(SCRATCH_ORDER).await;
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(
        "INSERT INTO x_attendance_statistical_cycle \
           (id, year, month, cycle_status, creator, create_time, update_time) VALUES \
           ('o36-m2',   '2026', '2',  'published', 'p', NOW(), NOW()), \
           ('o36-m10',  '2026', '10', 'published', 'p', NOW(), NOW()), \
           ('o36-tieA', '2026', '10', 'published', 'p', NOW(), NOW()), \
           ('o36-tieB', '2026', '10', 'published', 'p', NOW(), NOW());",
    )
    .await
    .expect("seed order rows");
    drop(c);

    let list = attendancestatisticalcycle_list_all(Extension(pool.clone()))
        .await
        .expect("list handler 不得 500");
    assert_eq!(
        list.0.r#type.as_deref(),
        Some("success"),
        "{}",
        raw(&list.0)
    );
    let rows = list.0.data.expect("data");
    let ids: Vec<&str> = rows
        .as_array()
        .expect("list data 是数组")
        .iter()
        .map(|r| field(r, "id"))
        .collect();
    assert_eq!(
        ids,
        vec!["o36-tieB", "o36-tieA", "o36-m10", "o36-m2"],
        "数字语义 + id DESC 裁决失败: {rows:?}"
    );

    drop_scratch(SCRATCH_ORDER).await;
}
