#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, Method, StatusCode},
    };
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

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"modelFlag": "test", "generating": true}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_router_builds() {
        let pool = build_test_pool();
        let _ = crate::router(pool);
    }

    #[tokio::test]
    async fn test_neural_generate_model_returns_internal_error_with_empty_pool() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/service/neural/generate/test-model")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_neural_list_model_returns_internal_error_with_empty_pool() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/service/neural/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
