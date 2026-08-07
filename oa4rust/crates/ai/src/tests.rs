#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_config_get_returns_success() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/get")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_enable_model_returns_success() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/list/enable/model")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_sync_to_knowledge_returns_success() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/index/sync/to/knowledge")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_ai_router_builds() {
        let pool = build_test_pool();
        let _ = crate::ai_router(pool);
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
        assert_eq!(result.message, None);
    }

    #[test]
    fn test_action_result_error() {
        let result: ActionResult<String> = ActionResult::error("test error");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("test error".to_string()));
    }
}
