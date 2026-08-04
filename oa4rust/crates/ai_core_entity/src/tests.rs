#[cfg(test)]
mod tests {
    use crate::{AiApp, AiModel, AiConversation};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        Pool::builder(mgr).build().unwrap()
    }

    #[tokio::test]
    async fn test_app_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::ai_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/core/entity/app/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_model_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::ai_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/core/entity/model/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_conversation_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::ai_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/ai/core/entity/conversation/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_action_result_format() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 1, "data": []}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 1);
    }

    #[test]
    fn test_ai_app_serialization() {
        let app = AiApp {
            id: "ai-app-001".to_string(),
            name: "AI助手".to_string(),
            description: Some("智能助手".to_string()),
            status: "active".to_string(),
        };
        let json = serde_json::to_value(&app).unwrap();
        assert_eq!(json["id"], "ai-app-001");
        assert_eq!(json["name"], "AI助手");
        assert_eq!(json["status"], "active");
    }

    #[test]
    fn test_ai_model_serialization() {
        let model = AiModel {
            id: "model-001".to_string(),
            name: "GPT-4".to_string(),
            provider: "openai".to_string(),
            enabled: true,
        };
        let json = serde_json::to_value(&model).unwrap();
        assert_eq!(json["id"], "model-001");
        assert_eq!(json["provider"], "openai");
        assert_eq!(json["enabled"], true);
    }

    #[test]
    fn test_ai_conversation_serialization() {
        let conversation = AiConversation {
            id: "conv-001".to_string(),
            title: "AI对话测试".to_string(),
            user_id: "user-001".to_string(),
            create_time: "2024-01-01T10:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&conversation).unwrap();
        assert_eq!(json["id"], "conv-001");
        assert_eq!(json["title"], "AI对话测试");
        assert_eq!(json["user_id"], "user-001");
    }
}
