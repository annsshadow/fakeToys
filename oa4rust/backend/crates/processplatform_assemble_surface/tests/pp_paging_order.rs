// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-B：`pp_c_snap` / `pp_c_review` 管理分页排序回归。
//!
//! 两张表的 `sequence` 列全仓无写入方（恒 NULL），旧 SQL
//! `ORDER BY "sequence" DESC NULLS LAST LIMIT/OFFSET` 等于无排序键、翻页边界不稳。
//! 本测试种 12 行、`xcreateTime` 含并列值、`id` 与插入顺序刻意与时间逆序，
//! 以 page size 1/2/3 三遍翻到底，断言：
//! 1. 并集 = 全量（无漏行）；2. 无重行；3. 顺序严格等于 `(xcreateTime DESC, id DESC)`。

use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use processplatform_assemble_surface::{
    review_u2_v2_search, snap_u2_manage_filter_paging, U2FilterWi, U2ReviewSearchWi,
};
use serde_json::Value;

const SCRATCH_SNAP: &str = "oa_o36_pp_snap";
const SCRATCH_REVIEW: &str = "oa_o36_pp_review";
const ADMIN: &str = "o36-pp-admin";

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
           VALUES ('o36-pp-admin', 'o36-pp-admin', 'o36 pp admin', 'x'); \
         INSERT INTO auth_role (id, name, deleted_at, disable) \
           VALUES ('o36-pp-role', 'admin', NULL, false); \
         INSERT INTO auth_person_role (person_id, role_id, unit_id) \
           VALUES ('o36-pp-admin', 'o36-pp-role', 'o36-unit');",
    )
    .await
    .expect("seed admin");
}

fn admin_session() -> shared::session::Session {
    shared::session::Session {
        token: "o36-pp-tok".to_string(),
        person_unique: ADMIN.to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
    }
}

/// 12 行、3 组并列时间；插入顺序 s01→s12，期望顺序为每组 id DESC：
/// s04,s03,s02,s01, s08,s07,s06,s05, s12,s11,s10,s09。
fn expected_ids() -> Vec<String> {
    vec![
        "s04", "s03", "s02", "s01", "s08", "s07", "s06", "s05", "s12", "s11", "s10", "s09",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn raw<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

fn collect_ids(data: &Value) -> Vec<String> {
    data.get("data")
        .and_then(|v| v.as_array())
        .expect("envelope.data 是数组")
        .iter()
        .map(|row| {
            row.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

#[tokio::test]
#[allow(non_snake_case)]
async fn snap_manage_paging_is_stable_with_duplicate_create_time() {
    let pool = fresh_scratch(SCRATCH_SNAP).await;
    seed_admin(&pool).await;
    let c = pool.get().await.expect("scratch 连接");
    let mut sql = String::from(
        "INSERT INTO \"pp_c_snap\" (\"id\", \"xid\", \"xtitle\", \"xperson\", \"xcreateTime\") VALUES ",
    );
    for i in 1..=12 {
        let t = match i {
            1..=4 => "2026-11-05 10:00:01",
            5..=8 => "2026-11-05 10:00:00",
            _ => "2026-11-05 09:59:59",
        };
        if i > 1 {
            sql.push(',');
        }
        sql.push_str(&format!(
            "('s{i:02}', 's{i:02}', 'o36 snap {i:02}', '{ADMIN}', '{t}')"
        ));
    }
    c.batch_execute(&sql).await.expect("seed pp_c_snap");
    drop(c);

    let expected = expected_ids();
    for size in [1i64, 2, 3] {
        let mut collected = Vec::new();
        let mut page = 1i64;
        loop {
            let wi = U2FilterWi {
                applicationList: vec![],
                processList: vec![],
                personList: vec![],
                key: None,
            };
            let res = snap_u2_manage_filter_paging(
                Extension(pool.clone()),
                Extension(admin_session()),
                Path((page, size)),
                Json(wi),
            )
            .await
            .expect("snap manage paging 不得 500");
            assert_eq!(
                res.0.r#type.as_deref(),
                Some("success"),
                "snap 分页失败: {}",
                raw(&res.0)
            );
            let data = res.0.data.expect("data");
            assert_eq!(
                data.get("count").and_then(|v| v.as_i64()),
                Some(12),
                "total 必须是 12（与翻页 size 无关）"
            );
            let ids = collect_ids(&data);
            if ids.is_empty() {
                break;
            }
            collected.extend(ids);
            page += 1;
        }
        assert_eq!(
            collected, expected,
            "size={size} 翻页出现漏行/重行/顺序错乱: {collected:?}"
        );
    }

    drop_scratch(SCRATCH_SNAP).await;
}

#[tokio::test]
async fn review_v2_search_paging_is_stable_with_duplicate_create_time() {
    let pool = fresh_scratch(SCRATCH_REVIEW).await;
    seed_admin(&pool).await;
    let c = pool.get().await.expect("scratch 连接");
    let mut sql = String::from(
        "INSERT INTO \"pp_c_review\" (\"id\", \"xid\", \"xtitle\", \"xperson\", \"xcreateTime\") VALUES ",
    );
    for i in 1..=12 {
        let t = match i {
            1..=4 => "2026-11-05 10:00:01",
            5..=8 => "2026-11-05 10:00:00",
            _ => "2026-11-05 09:59:59",
        };
        if i > 1 {
            sql.push(',');
        }
        sql.push_str(&format!(
            "('r{i:02}', 'r{i:02}', 'o36 review {i:02}', '{ADMIN}', '{t}')"
        ));
    }
    c.batch_execute(&sql).await.expect("seed pp_c_review");
    drop(c);

    let expected: Vec<String> = expected_ids()
        .into_iter()
        .map(|s| s.replacen('s', "r", 1))
        .collect();
    for size in [1i64, 2, 3] {
        let mut collected = Vec::new();
        let mut page = 1i64;
        loop {
            let res = review_u2_v2_search(
                Extension(pool.clone()),
                Extension(admin_session()),
                Json(U2ReviewSearchWi {
                    query: Some("o36".to_string()),
                    page: Some(page),
                    size: Some(size),
                    person: None,
                }),
            )
            .await
            .expect("review v2 search 不得 500");
            assert_eq!(
                res.0.r#type.as_deref(),
                Some("success"),
                "review 分页失败: {}",
                raw(&res.0)
            );
            let data = res.0.data.expect("data");
            assert_eq!(
                data.get("count").and_then(|v| v.as_i64()),
                Some(12),
                "total 必须是 12（与翻页 size 无关）"
            );
            let ids = collect_ids(&data);
            if ids.is_empty() {
                break;
            }
            collected.extend(ids);
            page += 1;
        }
        assert_eq!(
            collected, expected,
            "size={size} 翻页出现漏行/重行/顺序错乱: {collected:?}"
        );
    }

    drop_scratch(SCRATCH_REVIEW).await;
}
