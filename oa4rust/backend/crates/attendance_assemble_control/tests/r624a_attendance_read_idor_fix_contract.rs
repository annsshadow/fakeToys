// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R624a: 考勤三读端点身份盲读真修复回归契约。
//! 真父: detail/list/{page}/size/{size}、detail/statistic/filter、detail/statistic/export/filter
//!       三端点仅取 pool+body，无 Session 参数；body.personId/person 自证，空 body 全员可读。
//! 修复: 三端点签名加 Extension<Session>，非 admin 强制 filter_person=session.person_unique，
//!       仅 admin 可用 body 传 personId/person；对齐 v2_leave_list_page_size 范式。

use attendance_assemble_control::attendance_assemble_control_router;
use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
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
    c.batch_execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r624a-alice','r624a-alice','alice','x'),('r624a-bob','r624a-bob','bob','x'),('r624a-admin','r624a-admin','admin','x') ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed persons");
    c.batch_execute("INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r624a-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed role");
    c.batch_execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r624a-admin','r624a-role','r624a-unit') ON CONFLICT DO NOTHING;")
        .await.expect("seed admin role");
    c.batch_execute("INSERT INTO x_attendance_detail (id, person_id, date, status, creator, unit_id, create_time, file_id) VALUES ('r624a-alice-1','r624a-alice','2026-01-01','normal','r624a-alice','u','2026-01-01 00:00:00','f-alice-1'),('r624a-bob-1','r624a-bob','2026-01-02','leave','r624a-bob','u','2026-01-02 00:00:00','f-bob-1'),('r624a-bob-2','r624a-bob','2026-01-03','normal','r624a-bob','u','2026-01-03 00:00:00','f-bob-2');")
        .await.expect("seed details");
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

fn detail_ids(body: &str) -> Vec<String> {
    let v: Value = serde_json::from_str(body).expect("json body");
    v.get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default()
        .iter()
        .filter_map(|r| r.get("id").and_then(|x| x.as_str()).map(|s| s.to_string()))
        .collect()
}

fn stat_counts(body: &str) -> Vec<i64> {
    let v: Value = serde_json::from_str(body).expect("json body");
    let arr = v
        .get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .or_else(|| {
            v.get("data")
                .and_then(|d| d.get("data"))
                .and_then(|d| d.as_array())
                .cloned()
        })
        .unwrap_or_default();
    arr.iter()
        .filter_map(|r| r.get("count").and_then(|x| x.as_i64()))
        .collect()
}

const LIST_URI: &str = "/api/attendance/assemble/control/v2/detail/list/1/size/100";
const STAT_URI: &str = "/api/attendance/assemble/control/v2/detail/statistic/filter";
const EXPORT_URI: &str = "/api/attendance/assemble/control/v2/detail/statistic/export/filter";

#[tokio::test]
async fn r624a_non_admin_read_must_be_scoped_to_self() {
    let pool = fresh_scratch("oa_r624a_contract").await;
    seed(&pool).await;
    let app = attendance_assemble_control_router(pool.clone());

    // 非 admin 空 body：真父可读取所有人的明细（alice/bob），修复后仅本人。
    let (st, body) = call(&app, Some("r624a-alice"), LIST_URI, json!({})).await;
    assert_eq!(st, StatusCode::OK, "list 请求应成功");
    let ids = detail_ids(&body);
    assert!(
        ids.contains(&"r624a-alice-1".to_string()),
        "本人明细必须可见"
    );
    assert!(
        !ids.contains(&"r624a-bob-1".to_string()),
        "他人明细不得可见"
    );
    assert!(
        !ids.contains(&"r624a-bob-2".to_string()),
        "他人明细不得可见"
    );

    // 非 admin 指定他人 personId：真父可越权读 bob；修复后仍仅本人。
    let (st, body) = call(
        &app,
        Some("r624a-alice"),
        LIST_URI,
        json!({"personId":"r624a-bob"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "list 请求应成功");
    let ids = detail_ids(&body);
    assert!(
        !ids.contains(&"r624a-bob-1".to_string()),
        "指定他人不得越权读取"
    );
    assert!(
        !ids.contains(&"r624a-bob-2".to_string()),
        "指定他人不得越权读取"
    );

    // 聚合 filter/export：真父可聚合全员，修复后非 admin 仅本人。
    let (st, body) = call(&app, Some("r624a-alice"), STAT_URI, json!({})).await;
    assert_eq!(st, StatusCode::OK, "stat 请求应成功");
    let counts = stat_counts(&body);
    assert_eq!(counts.iter().sum::<i64>(), 1, "非 admin 聚合应仅本人 1 条");
    let (st, body) = call(
        &app,
        Some("r624a-alice"),
        EXPORT_URI,
        json!({"person":"r624a-bob"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "export 请求应成功");
    let counts = stat_counts(&body);
    assert_eq!(counts.iter().sum::<i64>(), 1, "非 admin 导出不得聚合他人");

    // 裸请求 401 反臂：401 由全局 auth_middleware 提供，oneshot 直调 router 不经过该层。
    // 另建挂 auth_middleware 的 Router，无 token 必须在中间件层被拒（与端点参数无关）。
    let state = shared::middleware::SecurityState {
        session_manager: shared::SessionManager::new(),
        rate_limiter: shared::RateLimiter::new(),
        pool: pool.clone(),
    };
    let guarded = Router::new()
        .route(
            "/api/attendance/assemble/control/v2/detail/list/{page}/size/{size}",
            axum::routing::post(|| async { "ok" }),
        )
        .layer(axum::middleware::from_fn_with_state(
            state,
            shared::middleware::auth_middleware,
        ));
    let req = Request::builder()
        .method("POST")
        .uri(LIST_URI)
        .body(Body::empty())
        .unwrap();
    let resp = guarded.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "无 token 必须 401");
}

#[tokio::test]
async fn r624a_admin_can_query_any_and_source_guard() {
    let pool = fresh_scratch("oa_r624a_contract_admin").await;
    seed(&pool).await;
    let app = attendance_assemble_control_router(pool.clone());

    let (st, body) = call(
        &app,
        Some("r624a-admin"),
        LIST_URI,
        json!({"personId":"r624a-bob"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin list 应成功");
    let ids = detail_ids(&body);
    assert!(
        ids.contains(&"r624a-bob-1".to_string()),
        "admin 可查他人明细"
    );
    assert!(
        ids.contains(&"r624a-bob-2".to_string()),
        "admin 可查他人明细"
    );

    let (st, body) = call(
        &app,
        Some("r624a-admin"),
        STAT_URI,
        json!({"person":"r624a-bob"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin stat 应成功");
    assert_eq!(
        stat_counts(&body).iter().sum::<i64>(),
        2,
        "admin 可聚合他人 2 条"
    );
    let (st, body) = call(
        &app,
        Some("r624a-admin"),
        EXPORT_URI,
        json!({"person":"r624a-bob"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin export 应成功");
    assert_eq!(
        stat_counts(&body).iter().sum::<i64>(),
        2,
        "admin 可导出他人 2 条"
    );

    let source = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read lib.rs");
    // 精确定位三端点函数体,避免兄弟端点 v2_leave_list_page_size 的相同范式误满足。
    fn fn_body(source: &str, sig: &str) -> String {
        let start = source.find(sig).expect("fn signature");
        let rest = &source[start..];
        let open = rest.find('{').expect("fn open brace");
        let mut depth = 0i32;
        for (i, ch) in rest[open..].char_indices() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    return rest[..open + i + 1].to_string();
                }
            }
        }
        panic!("fn body not closed");
    }
    for sig in [
        "pub async fn v2_detail_list_page_size(",
        "pub async fn v2_detail_statistic_filter(",
        "pub async fn v2_detail_statistic_export_filter(",
    ] {
        let body = fn_body(&source, sig);
        assert!(
            body.contains("session: Extension<shared::session::Session>"),
            "{} 未接收会话身份",
            sig
        );
        assert!(
            body.contains("session.person_unique.clone()"),
            "{} 未按会话人钳制",
            sig
        );
    }
}
