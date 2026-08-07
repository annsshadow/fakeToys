#[cfg(test)]
mod tests {
    use crate::{area_list, is_workday, security_clearance_enable};
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use serde_json::Value;

    fn mock_pool() -> deadpool_postgres::Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_security_clearance_enable_returns_success() {
        let result = security_clearance_enable().await.unwrap();
        let action: ActionResult<Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
        let data = action.data.unwrap();
        assert_eq!(data.get("enable").and_then(|v| v.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn test_is_workday_returns_success() {
        let result = is_workday(Path("20240101".to_string())).await.unwrap();
        let action: ActionResult<Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
        let data = action.data.unwrap();
        assert_eq!(data.get("date").and_then(|v| v.as_str()), Some("20240101"));
        assert_eq!(data.get("value").and_then(|v| v.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn test_is_workday_empty_date_returns_error() {
        let result = is_workday(Path("".to_string())).await.unwrap();
        let action: ActionResult<Value> = result.0;
        assert_eq!(action.r#type, Some("error".to_string()));
        assert!(action.message.is_some());
    }

    #[tokio::test]
    async fn test_area_list_without_db_returns_internal_error() {
        let result = area_list(Extension(mock_pool())).await;
        assert!(result.is_err());
    }
}
