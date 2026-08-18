#[cfg(test)]
mod tests {
    use crate::hotpic_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::Pool;
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn mock_pool() -> Pool {
        let mgr = deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        deadpool_postgres::Pool::builder(mgr).build().unwrap()
    }

    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 2, "data": []}));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[tokio::test]
    async fn test_exists_check_returns_internal_error() {
        let pool = mock_pool();
        let app = hotpic_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/user/hotpic/exists/check")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_by_id_returns_internal_error() {
        let pool = mock_pool();
        let app = hotpic_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/user/hotpic/hotpic-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_by_application_and_info_id_returns_internal_error() {
        let pool = mock_pool();
        let app = hotpic_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/user/hotpic/CMS/doc-123")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
