// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! F2：考勤明细读面的越权访问回归。
//!
//! 改前：`attendancedetail/mobile/filter/list/...`、`attendancedetail/filter/list/{id}/next|prev/...`
//! 完全不过滤人员，任何已登录账号都能读到 `someone-else` 的记录。
//! 改后口径：管理员看全公司，其他人只看本人（`person_id = session.person_unique`）。
//!
//! 两个方向都必须为真：普通用户看不到他人；管理员看得到他人。
//! 破坏实验：把 `AND ($3::text IS NULL OR person_id = $3::text)` 去掉 → 本文件必须红。

use attendance_assemble_control::{
    attendancedetail_filter_list, attendancedetail_filter_list_id_next_count,
    attendancedetail_filter_list_id_prev_count,
    attendancedetail_mobile_filter_list_page_page_count_count,
};
use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_MOBILE: &str = "oa_f2_detail_mobile";
const SCRATCH_KEYSET: &str = "oa_f2_detail_keyset";

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

/// 种三种行：本人两条、他人一条。另种一个管理员账号。
async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(
        "INSERT INTO x_attendance_detail (id, person_id, date, status) VALUES \
           ('d-p1-old',   'p1',           '2026-01-01', 'normal'), \
           ('d-other',    'someone-else', '2026-01-02', 'normal'), \
           ('d-p1-new',   'p1',           '2026-01-03', 'normal'); \
         INSERT INTO auth_person (id, unique_id, name, password_hash) \
           VALUES ('f2-admin', 'f2-admin', 'f2 admin', 'x'); \
         INSERT INTO auth_role (id, name, deleted_at, disable) \
           VALUES ('f2-admin-role', 'admin', NULL, false); \
         INSERT INTO auth_person_role (person_id, role_id, unit_id) \
           VALUES ('f2-admin', 'f2-admin-role', 'f2-unit');",
    )
    .await
    .expect("seed");
}

fn session(person_unique: &str) -> shared::session::Session {
    shared::session::Session {
        token: format!("f2-tok-{person_unique}"),
        person_unique: person_unique.to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
    }
}

fn ids(v: &Value) -> Vec<String> {
    v.as_array()
        .expect("data 是数组")
        .iter()
        .map(|r| {
            r.get("id")
                .and_then(|x| x.as_str())
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

fn raw<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

#[tokio::test]
async fn mobile_list_scopes_to_session_person_and_admin_sees_all() {
    let pool = fresh_scratch(SCRATCH_MOBILE).await;
    seed(&pool).await;

    // 普通用户 p1：只看得到本人两条，他人的 `someone-else` 不可见。
    let mine = attendancedetail_mobile_filter_list_page_page_count_count(
        Extension(pool.clone()),
        Some(Extension(session("p1"))),
        Path((1, 10)),
    )
    .await
    .expect("mobile list 不得 500");
    assert_eq!(
        mine.0.r#type.as_deref(),
        Some("success"),
        "{}",
        raw(&mine.0)
    );
    assert_eq!(
        mine.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-p1-new", "d-p1-old"],
        "普通用户读到他人记录: {}",
        raw(&mine.0)
    );

    // 管理员：三条全见。
    let all = attendancedetail_mobile_filter_list_page_page_count_count(
        Extension(pool.clone()),
        Some(Extension(session("f2-admin"))),
        Path((1, 10)),
    )
    .await
    .expect("mobile list 不得 500");
    assert_eq!(
        all.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-p1-new", "d-other", "d-p1-old"],
        "管理员应看得到全公司: {}",
        raw(&all.0)
    );

    // 无分页的 filter/list（E1）同口径。
    let plain_mine =
        attendancedetail_filter_list(Extension(pool.clone()), Some(Extension(session("p1"))))
            .await
            .expect("filter/list 不得 500");
    assert_eq!(
        plain_mine.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-p1-new", "d-p1-old"],
        "普通用户 filter/list 读到他人记录: {}",
        raw(&plain_mine.0)
    );
    let plain_all = attendancedetail_filter_list(
        Extension(pool.clone()),
        Some(Extension(session("f2-admin"))),
    )
    .await
    .expect("filter/list 不得 500");
    assert_eq!(
        plain_all.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-p1-new", "d-other", "d-p1-old"],
        "管理员 filter/list 应看到全公司: {}",
        raw(&plain_all.0)
    );

    drop_scratch(SCRATCH_MOBILE).await;
}

#[tokio::test]
async fn keyset_next_prev_scope_to_session_person_and_admin_sees_all() {
    let pool = fresh_scratch(SCRATCH_KEYSET).await;
    seed(&pool).await;

    // next：游标 d-p1-old（2026-01-01）之后。普通用户只剩本人的 d-p1-new。
    let next_mine = attendancedetail_filter_list_id_next_count(
        Extension(pool.clone()),
        Some(Extension(session("p1"))),
        Path(("d-p1-old".to_string(), 10)),
    )
    .await
    .expect("next 不得 500");
    assert_eq!(
        next_mine.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-p1-new"],
        "普通用户 next 读到他人记录: {}",
        raw(&next_mine.0)
    );

    // next：管理员应看到 d-other（01-02）与 d-p1-new（01-03）。
    let next_all = attendancedetail_filter_list_id_next_count(
        Extension(pool.clone()),
        Some(Extension(session("f2-admin"))),
        Path(("d-p1-old".to_string(), 10)),
    )
    .await
    .expect("next 不得 500");
    assert_eq!(
        next_all.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-other", "d-p1-new"],
        "管理员 next 应看到全公司: {}",
        raw(&next_all.0)
    );

    // prev：游标 d-p1-new（01-03）之前。普通用户只剩本人的 d-p1-old。
    let prev_mine = attendancedetail_filter_list_id_prev_count(
        Extension(pool.clone()),
        Some(Extension(session("p1"))),
        Path(("d-p1-new".to_string(), 10)),
    )
    .await
    .expect("prev 不得 500");
    assert_eq!(
        prev_mine.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-p1-old"],
        "普通用户 prev 读到他人记录: {}",
        raw(&prev_mine.0)
    );

    // prev：管理员应看到 d-other（01-02）与 d-p1-old（01-01）。
    let prev_all = attendancedetail_filter_list_id_prev_count(
        Extension(pool.clone()),
        Some(Extension(session("f2-admin"))),
        Path(("d-p1-new".to_string(), 10)),
    )
    .await
    .expect("prev 不得 500");
    assert_eq!(
        prev_all.0.data.as_ref().map(ids).unwrap_or_default(),
        vec!["d-other", "d-p1-old"],
        "管理员 prev 应看到全公司: {}",
        raw(&prev_all.0)
    );

    drop_scratch(SCRATCH_KEYSET).await;
}
