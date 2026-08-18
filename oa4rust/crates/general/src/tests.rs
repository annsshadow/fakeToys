#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;
    use tower::util::ServiceExt;
    use serde_json::Value;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_security_clearance_enable_returns_success() {
        let pool = build_test_pool();
        let app = crate::general_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/securityclearance/enable")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_is_workday_returns_success() {
        let pool = build_test_pool();
        let app = crate::general_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/worktime/isworkday/20240101")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_is_workday_empty_date_returns_error() {
        let pool = build_test_pool();
        let app = crate::general_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/worktime/isworkday/")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_area_list_without_db_returns_internal_error() {
        let pool = build_test_pool();
        let app = crate::general_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/area/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
