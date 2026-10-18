// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use serde_json::json;
use shared::response::ActionResult;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
    Pool::builder(mgr).max_size(1).build().unwrap()
}

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_get_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush_assemble_control/get/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_control_apps_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush_assemble_control/list/control/apps")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_update_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req_body = serde_json::to_string(&json!({"enabled": true, "maxPushCount": 5000})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush_assemble_control/update/control/config")
                .method(Method::GET)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();

    // 极光推送广播写收 admin 门：无会话先于读库回 401（而非原先无门的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_control_config_non_admin_forbidden() {
    use axum::extract::Extension;
    let pool = build_test_pool();
    let sess = shared::testing::test_session("jpush-cc-nonadmin");
    let r = crate::update_control_config(
        Extension(pool),
        Some(Extension(sess)),
        axum::extract::Json(json!({"enabled": true, "maxPushCount": 5000})),
    )
    .await;
    assert!(
        matches!(r, Err(shared::error::AppError::Forbidden)),
        "非 admin 写极光推送广播配置必须 403"
    );
}

#[tokio::test]
async fn test_jpush_get_by_id_legacy_path() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush/get/jpush-001")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_jpush_create_legacy_path() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let body = serde_json::to_string(&json!({"title": "test", "content": "hello"})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    // 广播推送创建收 admin 门：无会话先于读库回 401（而非原先无门的 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_jpush_control_config_legacy_path() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush/assemble/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_device_bind_legacy_path() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let body = serde_json::to_string(&json!({"person": "test", "deviceName": "dev1"})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush/assemble/control/device/bind")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    // 第五十五批：device_bind 收会话门后无会话返 401（原 500）。
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_u2_post_message_send_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body =
        serde_json::to_string(&json!({"title": "t", "content": "c", "target": "u"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush_assemble_control/message/send")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_u2_post_device_admin_unbind_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush_assemble_control/device/admin/unbind/all/person")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_u2_delete_device_unbind_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/jpush_assemble_control/device/unbind/a/b")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

// 回归（四轮审计 GET 写面队列 · jpush 3 把）：create_jpush / device_admin_unbind_all_person
// 收 admin 门；device_unbind_new 收归属门（解绑前取设备绑定 creator）。
#[tokio::test]
async fn test_jpush_get_writes_owner_admin_gate() {
    use axum::extract::Extension;
    // 无会话离线臂：三把全 401（先于读库）。
    let dead = {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    };
    assert!(
        matches!(
            crate::create_jpush(
                Extension(dead.clone()),
                None,
                axum::extract::Json(crate::JpushRequest {
                    title: Some("t".to_string()),
                    content: Some("c".to_string()),
                    target: Some("all".to_string()),
                })
            )
            .await,
            Err(shared::error::AppError::Unauthorized)
        ),
        "create_jpush 无会话必须 401"
    );
    assert!(
        matches!(
            crate::device_admin_unbind_all_person(
                Extension(dead.clone()),
                None,
                axum::extract::Json(json!({"personId": "p"}))
            )
            .await,
            Err(shared::error::AppError::Unauthorized)
        ),
        "device_admin_unbind 无会话必须 401"
    );
    assert!(
        matches!(
            crate::device_unbind_new_deviceName_deviceType_pushType(
                Extension(dead.clone()),
                None,
                axum::extract::Path(("dn".to_string(), "dt".to_string(), "pt".to_string()))
            )
            .await,
            Err(shared::error::AppError::Unauthorized)
        ),
        "device_unbind_new 无会话必须 401"
    );

    if !shared::testing::is_db_available().await {
        eprintln!(
            "skipping test_jpush_get_writes_owner_admin_gate(real-db arm): DATABASE_URL not reachable"
        );
        return;
    }
    let pool = shared::testing::test_pool();
    const OWNER: &str = "jpush-gate-owner";
    const OTHER: &str = "jpush-gate-other";
    const ADMIN: &str = "jpush-gate-admin";
    const ROLE: &str = "jpush-gate-role-admin";
    let sess = |p: &str| Some(Extension(shared::testing::test_session(p)));
    let unbind_path = || {
        axum::extract::Path((
            "jpush-gate-dn".to_string(),
            "jpush-gate-dt".to_string(),
            "jpush-gate-pt".to_string(),
        ))
    };

    // 造 admin + 归属设备绑定行（creator=OWNER）。
    {
        let c = pool.get().await.unwrap();
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) \
             ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
            &[&ROLE.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1,$1,'jpush gate admin','x') ON CONFLICT (id) DO NOTHING",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1,$2,'jpush-gate-unit') ON CONFLICT DO NOTHING",
            &[&ADMIN.to_string(), &ROLE.to_string()],
        )
        .await
        .unwrap();
        // 幂等造数：x_jpush.id 无唯一约束（schema 缺陷，见账本 §五），不能用 ON CONFLICT (id)；
        // 先按 id 清后插，保证可重复跑。
        c.execute("DELETE FROM x_jpush WHERE id = 'jpush-gate-dev'", &[])
            .await
            .unwrap();
        c.execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) \
             VALUES ('jpush-gate-dev','jpush-gate-dn','jpush-gate-dt','jpush-gate-pt',$1,NOW())",
            &[&OWNER.to_string()],
        )
        .await
        .unwrap();
    }

    // 非 admin → create_jpush / device_admin_unbind 403 且不落笔；非属主 → device_unbind_new 403 不落笔。
    assert!(
        matches!(
            crate::create_jpush(
                Extension(pool.clone()),
                sess(OTHER),
                axum::extract::Json(crate::JpushRequest {
                    title: Some("t".to_string()),
                    content: Some("c".to_string()),
                    target: Some("all".to_string()),
                })
            )
            .await,
            Err(shared::error::AppError::Forbidden)
        ),
        "create_jpush 非 admin 必须 403"
    );
    assert!(
        matches!(
            crate::device_admin_unbind_all_person(
                Extension(pool.clone()),
                sess(OTHER),
                axum::extract::Json(json!({"personId": OWNER}))
            )
            .await,
            Err(shared::error::AppError::Forbidden)
        ),
        "device_admin_unbind 非 admin 必须 403"
    );
    assert!(
        matches!(
            crate::device_unbind_new_deviceName_deviceType_pushType(
                Extension(pool.clone()),
                sess(OTHER),
                unbind_path()
            )
            .await,
            Err(shared::error::AppError::Forbidden)
        ),
        "device_unbind_new 他人必须 403"
    );
    {
        let c = pool.get().await.unwrap();
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) FROM x_jpush WHERE id='jpush-gate-dev' AND deleted_at IS NULL",
                &[],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(n, 1, "非属主 403 侧不得解绑设备");
    }

    // 属主放行：device_unbind_new 解绑成功（软删）。
    assert!(
        crate::device_unbind_new_deviceName_deviceType_pushType(
            Extension(pool.clone()),
            sess(OWNER),
            unbind_path()
        )
        .await
        .is_ok(),
        "属主 device_unbind_new 应放行"
    );
    {
        let c = pool.get().await.unwrap();
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) FROM x_jpush WHERE id='jpush-gate-dev' AND deleted_at IS NULL",
                &[],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(n, 0, "属主解绑后设备应软删");
    }

    // admin 放行：create_jpush 建广播；device_admin_unbind 解绑属主。
    assert!(
        crate::create_jpush(
            Extension(pool.clone()),
            sess(ADMIN),
            axum::extract::Json(crate::JpushRequest {
                title: Some("jpush-gate-bc".to_string()),
                content: Some("c".to_string()),
                target: Some("all".to_string()),
            })
        )
        .await
        .is_ok(),
        "admin create_jpush 应放行"
    );
    assert!(
        crate::device_admin_unbind_all_person(
            Extension(pool.clone()),
            sess(ADMIN),
            axum::extract::Json(json!({"personId": OWNER}))
        )
        .await
        .is_ok(),
        "admin device_admin_unbind 应放行"
    );

    // 收口：清造数，零残留。
    let c = pool.get().await.unwrap();
    c.execute(
        "DELETE FROM x_jpush WHERE id LIKE 'jpush-gate-%' OR title LIKE 'jpush-gate-%'",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "DELETE FROM auth_person_role WHERE person_id=$1",
        &[&ADMIN.to_string()],
    )
    .await
    .unwrap();
    c.execute("DELETE FROM auth_person WHERE id=$1", &[&ADMIN.to_string()])
        .await
        .unwrap();
    c.execute("DELETE FROM auth_role WHERE id=$1", &[&ROLE.to_string()])
        .await
        .unwrap();
}
