// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R314 D1：任务完成后已办列表同步回归。
//! 后端 task_complete 只写 x_task / x_record / x_work，从不写 pp_c_taskcompleted；
//! 桌面已办页读取 taskcompleted/list 端点直接查该表，导致审批完成后已办 count=0。
//! 本测试在 scratch DB 直接复现写后读：完成后必须能查到一条已办记录。

use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::json;

const SCRATCH_DB: &str = "oa_r314_taskcompleted_sync";

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

async fn fresh_scratch() -> Pool {
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
    drop(c);
    let pool = pool_to(SCRATCH_DB);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 库迁移");
    pool
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

async fn seed(pool: &Pool, work_id: &str, task_id: &str, person: &str) {
    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_work (id, title, process, application, work_status, creator) VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &work_id.to_string(),
            &"R314 已办同步验证".to_string(),
            &"r314-proc".to_string(),
            &"r314-app".to_string(),
            &"active".to_string(),
            &person.to_string(),
        ],
    )
    .await
    .expect("seed x_work");
    c.execute(
        "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[
            &task_id.to_string(),
            &"审批任务".to_string(),
            &work_id.to_string(),
            &"approval".to_string(),
            &"token-r314".to_string(),
            &person.to_string(),
            &"active".to_string(),
        ],
    )
    .await
    .expect("seed x_task");
}

async fn completed_count(pool: &Pool, person: &str) -> i64 {
    let c = pool.get().await.expect("scratch 连接");
    c.query_one(
        "SELECT COUNT(*)::bigint AS n FROM pp_c_taskcompleted WHERE xperson = $1",
        &[&person],
    )
    .await
    .expect("查已办")
    .get::<_, i64>("n")
}

#[tokio::test]
async fn task_complete_writes_taskcompleted_entry() {
    let pool = fresh_scratch().await;
    let person = "r314-person";
    let work_id = "r314-work-1";
    let task_id = "r314-task-1";
    seed(&pool, work_id, task_id, person).await;
    assert_eq!(completed_count(&pool, person).await, 0, "起始无已办");

    let session = shared::testing::test_session(person);
    let result = processplatform_service_processing::task_complete(
        Extension(pool.clone()),
        Some(Extension(session)),
        Path(task_id.to_string()),
        Some(Json(json!({"opinion": "R314 已办同步验证"}))),
    )
    .await
    .expect("task_complete 不得 500");
    assert_eq!(
        result.0.r#type.as_deref(),
        Some("success"),
        "task_complete 返回失败"
    );

    let count = completed_count(&pool, person).await;
    assert!(
        count >= 1,
        "审批完成后已办列表必须有记录（pp_c_taskcompleted count），实收 {}",
        count
    );
    drop_scratch().await;
}
