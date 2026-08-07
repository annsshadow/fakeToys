use super::*;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::extract::Path;
use deadpool_postgres::{Manager, Pool};
use shared::response::ActionResult;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    Pool::builder(mgr).build().unwrap()
}

#[test]
fn test_portal_router_builds() {
    let pool = build_test_pool();
    let _ = router(pool);
}

#[test]
fn test_portal_get_returns_error_without_db() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = build_test_pool();
        let result: Result<Json<ActionResult<Value>>, AppError> =
            portal_get(Extension(pool), Path("test-id".to_string())).await;
        match result {
            Ok(_) => panic!("expected error without DB"),
            Err(AppError::Internal) => {}
            Err(_) => panic!("expected Internal error"),
        }
    });
}

#[test]
fn test_portal_list_returns_error_without_db() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = build_test_pool();
        let result: Result<Json<ActionResult<Value>>, AppError> =
            portal_list(Extension(pool)).await;
        match result {
            Ok(_) => panic!("expected error without DB"),
            Err(AppError::Internal) => {}
            Err(_) => panic!("expected Internal error"),
        }
    });
}

#[tokio::test]
async fn test_portal_get_returns_internal_error() {
    let pool = build_test_pool();
    let app = router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/test-id")
                .method(axum::http::Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_portal_list_returns_internal_error() {
    let pool = build_test_pool();
    let app = router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/list")
                .method(axum::http::Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
