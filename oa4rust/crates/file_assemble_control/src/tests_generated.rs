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
        let app = crate::file_assemble_control_router(pool);

        // GET /jaxrs/anonymous/file/{id}/download/stream
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/file/test-id/download/stream")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attachment/download/{attid}/stream
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attachment/download/test-id/stream")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/file/assemble/control/attachment2/{id}/office/preview/type/{type}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/attachment2/test-id/office/preview/type/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/file/assemble/control/file/list/{folderId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/file/assemble/control/file/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/file/{id}/download/stream
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/test-id/download/stream")
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
        let app = crate::file_assemble_control_router(pool);

        // POST /jaxrs/file/assemble/control/file/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/file/assemble/control/file/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/file/assemble/control/file/upload
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/upload")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/file/core/entity/file/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/file/core/entity/file/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/file/core/entity/file/update/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/update/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}