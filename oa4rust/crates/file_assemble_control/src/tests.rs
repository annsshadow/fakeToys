#[cfg(test)]
mod tests {
    use crate::{file_assemble_control_router, get_control_config, list_storage_pools, list_control_categories, update_control_config};
    use axum::extract::Extension;
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;

    fn mock_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_control_config_returns_success() {
        let result = get_control_config(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_list_storage_pools_returns_success() {
        let result = list_storage_pools(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_list_control_categories_returns_success() {
        let result = list_control_categories(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_update_control_config_returns_success() {
        let body = axum::extract::Json(serde_json::json!({"enabled": false}));
        let result = update_control_config(Extension(mock_pool()), body).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[test]
    fn test_file_assemble_control_router_builds() {
        let pool = mock_pool();
        let _ = file_assemble_control_router(pool);
    }

    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"enabled": true}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
    }

    #[test]
    fn test_action_result_error() {
        let result: ActionResult<String> = ActionResult::error("test error");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("test error".to_string()));
    }
}
