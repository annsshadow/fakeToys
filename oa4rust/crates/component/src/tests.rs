#[cfg(test)]
mod tests {
    use crate::count;
    use crate::get_component;
    use crate::list_all;
    use axum::extract::{Extension, Path};
    use shared::response::ActionResult;

    fn mock_pool() -> deadpool_postgres::Pool {
        let mgr = deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        deadpool_postgres::Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_list_all_returns_success() {
        let result = list_all(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
        let data = action.data.unwrap();
        assert!(data.get("count").is_some());
        assert!(data.get("data").is_some());
    }

    #[tokio::test]
    async fn test_get_component_existing() {
        let result = get_component(Extension(mock_pool()), Path("comp-001".to_string()))
            .await
            .unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        let data = action.data.unwrap();
        assert_eq!(data.get("id").and_then(|v| v.as_str()), Some("comp-001"));
        assert_eq!(data.get("name").and_then(|v| v.as_str()), Some("desktop"));
    }

    #[tokio::test]
    async fn test_get_component_not_found() {
        let result = get_component(Extension(mock_pool()), Path("nonexistent".to_string()))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_count_returns_success() {
        let result = count(Extension(mock_pool())).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        let data = action.data.unwrap();
        assert_eq!(data.get("count").and_then(|v| v.as_i64()), Some(3));
    }
}
