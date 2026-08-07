#[cfg(test)]
mod tests {
    use crate::{ai_router, config_get, list_enable_model, sync_to_knowledge};
    use axum::extract::Extension;
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;

    fn mock_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_config_get_returns_success() {
        let result = config_get(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_list_enable_model_returns_success() {
        let result = list_enable_model(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_sync_to_knowledge_returns_success() {
        let result = sync_to_knowledge(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[test]
    fn test_ai_router_builds() {
        let pool = mock_pool();
        let _ = ai_router(pool);
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
