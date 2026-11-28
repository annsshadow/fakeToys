// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R642: CMS file_id_upload 跨人覆盖文件正文真修复回归契约。
//! 修复前: file_id_upload 仅 require_session 丢弃身份, 任意会话可覆写他人 x_cms_file 正文。
//! 修复后: 保留 Session + u2_check_owner 属主门(Forbidden -> 403) + INSERT 写 creator 且 ON CONFLICT 不改属主。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use cms_assemble_control::cms_assemble_control_router;
use deadpool_postgres::{Manager, Pool};
use serde_json::{json, Value};
use tower::ServiceExt;

fn pg_pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config = url.parse().expect("DATABASE_URL");
    cfg.dbname(dbname);
    let mgr = Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pg_pool_to("postgres");
    let c = admin.get().await.expect("admin conn");
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
    let pool = pg_pool_to(dbname);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch migration");
    pool
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.batch_execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r642-alice','r642-alice','alice','x'),('r642-bob','r642-bob','bob','x'),('r642-admin','r642-admin','admin','x') ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed persons");
    c.batch_execute("INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r642-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed role");
    c.batch_execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r642-admin','r642-role','r642-unit') ON CONFLICT DO NOTHING;")
        .await.expect("seed admin role");
    c.batch_execute("INSERT INTO x_cms_file (id, name, content_base64, content_type, size, creator, create_time) VALUES ('r642-bob-file','bob.txt','Qk9CX09SSUc=','text/plain',8,'r642-bob', NOW());")
        .await.expect("seed bob file");
}

async fn call(app: &Router, who: Option<&str>, uri: &str, body: Value) -> (StatusCode, String) {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let resp = match who {
        Some(p) => {
            app.clone()
                .layer(Extension(shared::testing::test_session(p)))
                .oneshot(req)
                .await
        }
        None => app.clone().oneshot(req).await,
    }
    .expect("oneshot");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("body");
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

fn upload_body(name: &str, b64: &str) -> Value {
    json!({"name": name, "contentBase64": b64, "contentType": "text/plain", "size": 8})
}

#[tokio::test]
async fn r642_file_upload_owner_gate() {
    let pool = fresh_scratch("oa_r642_contract").await;
    seed(&pool).await;
    let app = cms_assemble_control_router(pool.clone());

    // 臂1: alice 跨人覆写 bob 文件 -> 403 且 DB 未改
    let (st, _) = call(
        &app,
        Some("r642-alice"),
        "/api/file/r642-bob-file/upload",
        upload_body("evil.txt", "RVZJTF9PUklH"),
    )
    .await;
    assert_eq!(
        st,
        StatusCode::FORBIDDEN,
        "R642 arm1: cross-user overwrite must be 403"
    );
    let c = pool.get().await.expect("conn");
    let r = c
        .query_opt(
            "SELECT content_base64, creator FROM x_cms_file WHERE id = 'r642-bob-file'",
            &[],
        )
        .await
        .expect("q1")
        .expect("bob row exists");
    assert_eq!(
        r.get::<_, String>("content_base64"),
        "Qk9CX09SSUc=",
        "R642 arm1: content unchanged"
    );
    assert_eq!(
        r.get::<_, Option<String>>("creator").unwrap_or_default(),
        "r642-bob",
        "R642 arm1: creator unchanged"
    );

    // 臂2: bob 覆写自己 -> 200, creator 保留, content 更新
    let (st2, _) = call(
        &app,
        Some("r642-bob"),
        "/api/file/r642-bob-file/upload",
        upload_body("bob2.txt", "Qk9CMi5UU1Q="),
    )
    .await;
    assert_eq!(
        st2,
        StatusCode::OK,
        "R642 arm2: owner overwrite must be 200"
    );
    let r2 = c
        .query_opt(
            "SELECT content_base64, creator FROM x_cms_file WHERE id = 'r642-bob-file'",
            &[],
        )
        .await
        .expect("q2")
        .expect("bob row exists");
    assert_eq!(
        r2.get::<_, String>("content_base64"),
        "Qk9CMi5UU1Q=",
        "R642 arm2: content updated"
    );
    assert_eq!(
        r2.get::<_, Option<String>>("creator").unwrap_or_default(),
        "r642-bob",
        "R642 arm2: creator retained"
    );

    // 臂3: alice 新建 -> 200, creator=alice
    let (st3, _) = call(
        &app,
        Some("r642-alice"),
        "/api/file/r642-new-file/upload",
        upload_body("new.txt", "TkVXX0ZJTEU="),
    )
    .await;
    assert_eq!(
        st3,
        StatusCode::OK,
        "R642 arm3: new file create must be 200"
    );
    let r3 = c
        .query_opt(
            "SELECT creator FROM x_cms_file WHERE id = 'r642-new-file'",
            &[],
        )
        .await
        .expect("q3")
        .expect("new row exists");
    assert_eq!(
        r3.get::<_, Option<String>>("creator").unwrap_or_default(),
        "r642-alice",
        "R642 arm3: creator = alice"
    );
    drop(c);

    // 臂4: 匿名无 session -> 401, 未写库
    let (st4, _) = call(
        &app,
        None,
        "/api/file/r642-anon-file/upload",
        upload_body("anon.txt", "QU5PTg=="),
    )
    .await;
    assert_eq!(
        st4,
        StatusCode::UNAUTHORIZED,
        "R642 arm4: anonymous must be 401"
    );
    let c2 = pool.get().await.expect("conn2");
    let anon = c2
        .query_opt("SELECT id FROM x_cms_file WHERE id = 'r642-anon-file'", &[])
        .await
        .expect("q4");
    assert!(anon.is_none(), "R642 arm4: anonymous must not write");
    drop(c2);

    // 臂5: 源码守卫(修复结构, 非仅行为)
    let src =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")).expect("src");
    let start = src.find("pub async fn file_id_upload").expect("fn marker");
    let tail = &src[start..];
    let end = tail.find("#[axum::debug_handler]").unwrap_or(tail.len());
    let f = &tail[..end];
    assert!(
        f.contains("let sess = require_session(session)?;"),
        "R642 arm5: session retained"
    );
    assert!(
        f.contains("u2_check_owner"),
        "R642 arm5: owner gate present"
    );
    assert!(f.contains("\"x_cms_file\""), "R642 arm5: table arg");
    assert!(f.contains("\"creator\""), "R642 arm5: owner col arg");
    assert!(
        f.contains("U2Gate::Forbidden"),
        "R642 arm5: forbidden branch"
    );
    let ins = f.find("INSERT INTO x_cms_file").expect("insert stmt");
    let iseg = &f[ins..];
    assert!(
        iseg.contains("creator"),
        "R642 arm5: INSERT must write creator"
    );
    let oc = iseg.find("ON CONFLICT").expect("upsert");
    assert!(
        !iseg[oc..].contains("creator ="),
        "R642 arm5: ON CONFLICT must not set creator"
    );

    println!("R642 five arms OK: cross-user 403+unchanged, owner 200+retained, new 200+creator, anon 401+no-write, source guards passed");
}
