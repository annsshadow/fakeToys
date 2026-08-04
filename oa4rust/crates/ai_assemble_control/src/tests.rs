#[cfg(test)]
mod tests {
    use crate::{ai_assemble_control_router, get_ai_control_config, list_ai_models, get_usage_stats, update_ai_control_config};
    use axum::extract::Extension;
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;

    fn mock_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_ai_control_config_returns_success() {
        let result = get_ai_control_config(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_list_ai_models_returns_success() {
        let result = list_ai_models(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_get_usage_stats_returns_success() {
        let result = get_usage_stats(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_update_ai_control_config_returns_success() {
        let body = axum::extract::Json(serde_json::json!({"maxTokens": 8192}));
        let result = update_ai_control_config(Extension(mock_pool()), body).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[test]
    fn test_ai_assemble_control_router_builds() {
        let pool = mock_pool();
        let _ = ai_assemble_control_router(pool);
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
    }
}
