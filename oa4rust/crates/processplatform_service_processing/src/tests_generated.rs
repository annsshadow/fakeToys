#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_routes_batch_1() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/processplatform/service/processing/get/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/get/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/processplatform/service/processing/instance/{executionId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/instance/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/processplatform/service/processing/list/{category}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/processplatform/service/processing/process/{id}/complex
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/process/test-id/complex")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/processplatform/service/processing/work/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/work/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_2() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // POST /jaxrs/processplatform/service/processing/cancel/{executionId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/cancel/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/processplatform/service/processing/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/processplatform/service/processing/execute/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/execute/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/work/{id}/retract
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/retract")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/work/{id}/terminate
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/terminate")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_put_routes_batch_3() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // PUT /jaxrs/work/{id}/processing
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/processing")
                    .method(Method::PUT)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}