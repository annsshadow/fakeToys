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

        // GET /jaxrs/program/applications
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/applications")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/program/appstyle/current/style
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/appstyle/current/style")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/program/datastructure/modules/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/datastructure/modules/all")
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

        // POST /jaxrs/program_center/agent/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/program_center/agent/save/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/program_center/application/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/program_center/application/save/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}