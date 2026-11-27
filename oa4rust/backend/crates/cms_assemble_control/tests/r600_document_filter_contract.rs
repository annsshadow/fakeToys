// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R600 契约：document_filter_paging_u3 / document_filter_paging_manager_u3
//! 必须按 body 中的 type(status) / keyword(title ILIKE) 参数化过滤；无 body 仍兼容全量。

use axum::extract::Extension;
use axum::Json;
use serde_json::{json, Value};

const SCRATCH_DB: &str = "oa_r600_docfilter";

fn pool_to(dbname: &str) -> deadpool_postgres::Pool {
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

async fn fresh_scratch() -> deadpool_postgres::Pool {
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
        .expect("scratch 迁移");
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

async fn seed(p: &deadpool_postgres::Pool) {
    let admin = "r600-admin";
    let c = p.get().await.expect("scratch 连接");
    c.execute("INSERT INTO auth_role (id, name) VALUES ('r600-admin-role','admin') ON CONFLICT (id) DO NOTHING", &[]).await.expect("seed role");
    c.execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ($1,$1,'R600 Admin','r600-noop') ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id", &[&admin]).await.expect("seed person");
    c.execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1,'r600-admin-role','r600-unit') ON CONFLICT DO NOTHING", &[&admin]).await.expect("seed person role");
    c.execute(
        "DELETE FROM x_cms_data_document WHERE id IN ('r600-draft','r600-pub','r600-kw')",
        &[],
    )
    .await
    .expect("clean docs");
    c.execute("INSERT INTO x_cms_data_document (id, title, creator, status) VALUES ('r600-draft','R600 ordinary draft','r600-user','draft'),('r600-pub','R600 ordinary published','r600-user','published'),('r600-kw','R600_UNIQUE_NEEDLE_998877 keyword doc','r600-user','draft')", &[]).await.expect("seed docs");
}

async fn call(p: &deadpool_postgres::Pool, manager: bool, body: Option<Value>) -> Vec<String> {
    let b = body.map(Json);
    let resp = if manager {
        cms_assemble_control::document_filter_paging_manager_u3(
            Extension(p.clone()),
            Extension(shared::testing::test_session("r600-admin")),
            b,
        )
        .await
        .expect("manager handler")
    } else {
        cms_assemble_control::document_filter_paging_u3(Extension(p.clone()), b)
            .await
            .expect("non-manager handler")
    };
    let v = serde_json::to_value(&resp.0).expect("serialize");
    v["data"]
        .as_array()
        .expect("data array")
        .iter()
        .map(|r| r["id"].as_str().unwrap_or_default().to_string())
        .collect()
}

#[tokio::test]
async fn filter_type_and_keyword_work_in_both_variants() {
    let p = fresh_scratch().await;
    seed(&p).await;
    for manager in [false, true] {
        let all = call(&p, manager, None).await;
        assert!(
            all.contains(&"r600-draft".to_string()) && all.contains(&"r600-pub".to_string()),
            "无 body 必须兼容全量 manager={manager}"
        );

        let drafts = call(&p, manager, Some(json!({"type":"draft"}))).await;
        assert!(
            drafts.contains(&"r600-draft".to_string()) && drafts.contains(&"r600-kw".to_string()),
            "type=draft 必须含草稿 manager={manager}"
        );
        assert!(
            !drafts.contains(&"r600-pub".to_string()),
            "type=draft 必须排除 published manager={manager}"
        );

        let kw = call(&p, manager, Some(json!({"keyword":"UNIQUE_NEEDLE_998877"}))).await;
        assert_eq!(
            kw,
            vec!["r600-kw".to_string()],
            "keyword 必须只返标题匹配文档 manager={manager}"
        );

        let both = call(
            &p,
            manager,
            Some(json!({"type":"draft","keyword":"UNIQUE_NEEDLE_998877"})),
        )
        .await;
        assert_eq!(
            both,
            vec!["r600-kw".to_string()],
            "type+keyword 联合过滤 manager={manager}"
        );
    }
    drop_scratch().await;
}

#[tokio::test]
async fn source_guard_filters_and_admin_gate_are_present() {
    let src = include_str!("../src/lib.rs");
    assert!(
        src.contains("fn document_filter_where(body: Option<Json<Value>>)"),
        "过滤 helper 必须存在"
    );
    assert!(
        src.contains("title ILIKE $"),
        "keyword 必须参数化 title ILIKE"
    );
    assert!(src.contains("status = $"), "type 必须参数化 status");
    assert_eq!(
        src.matches("body: Option<Json<Value>>,").count(),
        2,
        "两变体都必须接收 body"
    );
    assert!(
        src.matches("u2_require_admin(&pool, &session).await?;")
            .count()
            >= 1,
        "manager 的管理员门必须保留"
    );
    assert!(
        src.contains("deleted_at IS NULL") && src.contains("ORDER BY create_time DESC LIMIT 200"),
        "原有软删/排序/上限语义必须保留"
    );
}
