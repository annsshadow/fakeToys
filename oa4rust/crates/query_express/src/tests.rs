use super::*;
use axum::body::Body;
use axum::http::{Request, Method, StatusCode};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use serde_json::json;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        Config::new(),
        NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

#[tokio::test]
async fn test_query_list_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_query_express_router_builds() {
    let pool = build_test_pool();
    let _ = crate::query_express_router(pool);
}
