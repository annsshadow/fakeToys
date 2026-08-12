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

        // GET /jaxrs/processplatform/assemble/surface/get/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/get/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/processplatform/assemble/surface/list/{category}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/processplatform/assemble/surface/preview/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/preview/test-id")
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

        // POST /jaxrs/processplatform/assemble/surface/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/processplatform/assemble/surface/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/processplatform/assemble/surface/publish/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/publish/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/processplatform/assemble/surface/save/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}