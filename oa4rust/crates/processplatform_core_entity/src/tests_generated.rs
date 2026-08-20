#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_work_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/work/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list route should be registered");
    }

    #[tokio::test]
    async fn test_work_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/work/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_get route should be registered");
    }

    #[tokio::test]
    async fn test_task_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/task/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list route should be registered");
    }

    #[tokio::test]
    async fn test_task_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/task/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_get route should be registered");
    }

    #[tokio::test]
    async fn test_ticket_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/ticket/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "ticket_list route should be registered");
    }

    // SKIPPED: workcompleted_list not accessible
}