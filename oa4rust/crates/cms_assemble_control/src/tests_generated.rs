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
        let app = crate::cms_assemble_control_router(pool);

        // GET /jaxrs/application/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/application/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/cms_assemble_control/get/control/config
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/get/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/cms_assemble_control/list/control/sections
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/list/control/sections")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/cms_assemble_control/update/control/config
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/update/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/commend/list/paging/{docId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/commend/list/paging/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/queryview/flag/{view}/definition/{queryFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/queryview/flag/test-id/definition/test-id")
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
        let app = crate::cms_assemble_control_router(pool);

        // POST /jaxrs/document/{id}/view/count
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/document/test-id/view/count")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}