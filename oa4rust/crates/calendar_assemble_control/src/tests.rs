use super::*;
use shared::response::ActionResult;
use serde_json::json;
use axum::body::Body;
use axum::http::{Request, Method, StatusCode};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        Config::new(),
        NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({"count": 2, "data": []}));
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
                .uri("/jaxrs/calendar_assemble_control/get/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_control_calendars_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/calendar_assemble_control/list/control/calendars")
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

    let req_body = serde_json::to_string(&json!({"enabled": true, "defaultTimeZone": "UTC+0"})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/calendar_assemble_control/update/control/config")
                .method(Method::GET)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ───────────── plan002 U2：新端点路由存在性（mock pool，断言非 404）─────────────

async fn hit_route(method: Method, uri: &str, body: Option<&str>) -> StatusCode {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let req = builder
        .body(Body::from(body.unwrap_or("").to_string()))
        .unwrap();
    app.oneshot(req).await.unwrap().status()
}

fn assert_registered(status: StatusCode) {
    // 路由已注册（mock pool 下非 404；无会话的受保护路由返回 500，亦非 404）
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn route_calendar_create() {
    assert_registered(hit_route(Method::POST, "/jaxrs/calendar_assemble_control/calendar", Some("{}")).await);
}
#[tokio::test]
async fn route_calendar_follow_get() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/calendar/follow/x", None).await);
}
#[tokio::test]
async fn route_calendar_follow_cancel() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/calendar/follow/x/cancel", None).await);
}
#[tokio::test]
async fn route_calendar_ismanager_calendar() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/calendar/ismanager/calendar/x", None).await);
}
#[tokio::test]
async fn route_calendar_list_filter() {
    assert_registered(hit_route(Method::PUT, "/jaxrs/calendar_assemble_control/calendar/list/filter", Some("{}")).await);
}
#[tokio::test]
async fn route_calendar_manager_list_with_person() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/calendar/manager/list/with/person/x", None).await);
}
#[tokio::test]
async fn route_calendar_delete() {
    assert_registered(hit_route(Method::DELETE, "/jaxrs/calendar_assemble_control/calendar/x", None).await);
}
#[tokio::test]
async fn route_event_create() {
    assert_registered(hit_route(Method::POST, "/jaxrs/calendar_assemble_control/event", Some("{}")).await);
}
#[tokio::test]
async fn route_event_delete_after() {
    assert_registered(hit_route(Method::DELETE, "/jaxrs/calendar_assemble_control/event/after/x", None).await);
}
#[tokio::test]
async fn route_event_delete_all() {
    assert_registered(hit_route(Method::DELETE, "/jaxrs/calendar_assemble_control/event/all/x", None).await);
}
#[tokio::test]
async fn route_event_list_filter() {
    assert_registered(hit_route(Method::PUT, "/jaxrs/calendar_assemble_control/event/list/filter", Some("{}")).await);
}
#[tokio::test]
async fn route_event_rfc() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/event/rfc/x", None).await);
}
#[tokio::test]
async fn route_message_create() {
    assert_registered(hit_route(Method::POST, "/jaxrs/calendar_assemble_control/message", Some("{}")).await);
}
#[tokio::test]
async fn route_setting_create() {
    assert_registered(hit_route(Method::POST, "/jaxrs/calendar_assemble_control/setting", Some("{}")).await);
}
#[tokio::test]
async fn route_setting_get_by_code() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/setting/code/x", None).await);
}
#[tokio::test]
async fn route_setting_get() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/setting/x", None).await);
}
#[tokio::test]
async fn route_test_1() {
    assert_registered(hit_route(Method::GET, "/jaxrs/calendar_assemble_control/test/1", None).await);
}

// ───────────── plan002 U2：真实 DB 集成测试（DB 可用时执行，含清理）─────────────

#[tokio::test]
async fn real_calendar_get_and_setting_by_code() {
    if !shared::testing::is_db_available().await {
        return;
    }
    let pool = shared::testing::test_pool();
    let client = pool.get().await.expect("db");

    let cal_id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO cal_calendar (id, name, type, is_public, status, createor, create_time) \
             VALUES ($1, 'u2test', 'PERSONAL', false, 'OPEN', 'u2tester', NOW())",
            &[&cal_id],
        )
        .await
        .unwrap();

    let code = format!("u2code_{}", &cal_id[..8]);
    client
        .execute(
            "INSERT INTO cal_setting (id, code, name, value, order_no, create_time) \
             VALUES ($1, $2, 'u2s', 'v', 0, NOW())",
            &[&cal_id, &code],
        )
        .await
        .unwrap();

    let app = crate::router(pool);
    // 读日历
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/jaxrs/calendar_assemble_control/calendar/{}", cal_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["data"]["id"], cal_id);

    // 按 code 查设置
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/jaxrs/calendar_assemble_control/setting/code/{}", code))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 清理
    let _ = client.execute("DELETE FROM cal_setting WHERE id = $1", &[&cal_id]).await;
    let _ = client.execute("DELETE FROM cal_calendar WHERE id = $1", &[&cal_id]).await;
}
