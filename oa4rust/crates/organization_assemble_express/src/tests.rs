#[cfg(test)]
mod tests {
    use crate::{organization_assemble_express_router, get_express_config, list_organization_units, sync_organization_data, get_express_status};
    use axum::extract::Extension;
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use shared::response::ActionResult;

    fn mock_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_express_config_returns_success() {
        let result = get_express_config(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_list_organization_units_returns_success() {
        let result = list_organization_units(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_sync_organization_data_returns_success() {
        let result = sync_organization_data(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_get_express_status_returns_success() {
        let result = get_express_status(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[test]
    fn test_organization_assemble_express_router_builds() {
        let pool = mock_pool();
        let _ = organization_assemble_express_router(pool);
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<Vec<String>> = ActionResult::success(vec!["a".to_string()]);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
    }
}
