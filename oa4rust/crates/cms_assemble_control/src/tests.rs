use super::*;
use shared::response::ActionResult;
use serde_json::json;

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_get_control_config_route() {
    use crate::routes::router;
    use axum::body::Body;
    use axum::http::{Request, Method};
    use tower::util::ServiceExt;

    let pool = Pool::builder(deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .build()
    .unwrap();
    let app = router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms/assemble/control/config/get")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_list_control_sections_route() {
    use crate::routes::router;
    use axum::body::Body;
    use axum::http::{Request, Method};
    use tower::util::ServiceExt;

    let pool = Pool::builder(deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .build()
    .unwrap();
    let app = router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms/assemble/control/sections")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_update_control_config_route() {
    use crate::routes::router;
    use axum::body::Body;
    use axum::http::{Request, Method};
    use tower::util::ServiceExt;

    let pool = Pool::builder(deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .build()
    .unwrap();
    let app = router(pool);

    let req_body = serde_json::to_string(&json!({"enabled": true, "maxCategoryCount": 300})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms/assemble/control/config/update")
                .method(Method::GET)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}
