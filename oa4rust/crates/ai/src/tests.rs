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
    async fn test_config_get_route_exists() {
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

        // Route exists - should not return 404
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_config_base_config_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/base/config")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_config_list_model_paging_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/list/model/paging/1/size/10")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_config_get_model_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/get/model/test-model")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_config_list_mcp_paging_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/list/mcp/paging/1/size/10")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_config_get_mcp_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/config/get/mcp/test-mcp")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_chat_list_paging_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/chat/list/paging/1/size/10")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_chat_list_completion_paging_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/chat/list/completion/test-clue/paging/1/size/10")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_chat_delete_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/chat/delete/test-clue")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_index_cms_doc_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/index/cms/doc/test-doc")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_index_cms_doc_with_app_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/index/cms/doc/with/app/test-app")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_index_delete_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/index/delete/test-flag")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_file_get_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/file/test-flag")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_file_download_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/file/test-id/download")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_file_download_scale_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/file/test-id/download/scale")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_file_delete_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/file/delete/test-flag")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_enable_model_route_exists() {
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

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_sync_to_knowledge_route_exists() {
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

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
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

    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<i32> = ActionResult::success(42);
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"], 42);
        assert_eq!(json["message"], serde_json::Value::Null);
    }

    #[test]
    fn test_action_result_error_serialization() {
        let result: ActionResult<&str> = ActionResult::error("something went wrong");
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "something went wrong");
        assert_eq!(json["data"], serde_json::Value::Null);
    }

    #[test]
    fn test_action_result_with_count() {
        let mut result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"items": []}));
        result.count = Some(10);
        result.size = Some(20);
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["count"], 10);
        assert_eq!(json["size"], 20);
    }

    #[test]
    fn test_action_result_count_zero() {
        let mut result: ActionResult<Vec<i32>> = ActionResult::success(vec![]);
        result.count = Some(0);
        result.size = Some(0);
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["count"], 0);
        assert_eq!(json["size"], 0);
    }

    #[tokio::test]
    async fn test_app_list_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/app/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_model_list_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/model/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_conversation_list_route_exists() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/conversation/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_unknown_route_returns_404() {
        let pool = build_test_pool();
        let app = crate::ai_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/nonexistent")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_action_result_error_serialization_json() {
        let result: ActionResult<String> = ActionResult::error("db connection failed");
        let json_str = serde_json::to_string(&result).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["type"], "error");
        assert_eq!(parsed["message"], "db connection failed");
    }

    #[test]
    fn test_action_result_success_roundtrip_json() {
        let original: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"id": "x", "count": 5}));
        let json_str = serde_json::to_string(&original).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["type"], "success");
        assert_eq!(parsed["data"]["id"], "x");
        assert_eq!(parsed["data"]["count"], 5);
    }

    #[test]
    fn test_action_result_error_with_message_and_none_data() {
        let result: ActionResult<String> = ActionResult::error("validation failed");
        assert!(result.data.is_none());
        assert_eq!(result.message, Some("validation failed".to_string()));
        assert_eq!(result.r#type, Some("error".to_string()));
    }
}
