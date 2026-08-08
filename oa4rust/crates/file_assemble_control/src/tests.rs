#[cfg(test)]
mod tests {
    use crate::{
        ControlClient, ControlPool, DynControlPool, RowGet,
        file_assemble_control_router, get_control_config, list_control_categories,
        list_storage_pools, update_control_config,
    };
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::Pool;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use serde_json::Value;
    use shared::response::ActionResult;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower::ServiceExt;

    // ---- Mock types ----

    enum MockQueryResult {
        Row(Vec<(&'static str, Value)>),
        Rows(Vec<Vec<(&'static str, Value)>>),
        EmptyRows,
        Count(u64),
        Error,
    }

    struct MockControlClient {
        results: Arc<Mutex<Vec<MockQueryResult>>>,
    }

    impl MockControlClient {
        fn new(results: Arc<Mutex<Vec<MockQueryResult>>>) -> Self {
            Self { results }
        }

        fn single_row(values: Vec<(&'static str, Value)>) -> Arc<Mutex<Vec<MockQueryResult>>> {
            Arc::new(Mutex::new(vec![MockQueryResult::Row(values)]))
        }

        fn rows(values: Vec<Vec<(&'static str, Value)>>) -> Arc<Mutex<Vec<MockQueryResult>>> {
            Arc::new(Mutex::new(vec![MockQueryResult::Rows(values)]))
        }

        fn empty() -> Arc<Mutex<Vec<MockQueryResult>>> {
            Arc::new(Mutex::new(vec![MockQueryResult::EmptyRows]))
        }

        fn count(n: u64) -> Arc<Mutex<Vec<MockQueryResult>>> {
            Arc::new(Mutex::new(vec![MockQueryResult::Count(n)]))
        }
    }

    /// MockRow stores column values as (name, Value) pairs.
    /// RowGet methods extract typed values from the Value.
    #[derive(Clone)]
    struct MockRow {
        values: Vec<(&'static str, Value)>,
    }

    impl RowGet for MockRow {
        fn get_i32(&self, col: &str) -> i32 {
            self.values
                .iter()
                .find(|(k, _)| *k == col)
                .and_then(|(_, v)| v.as_i64())
                .unwrap_or(0) as i32
        }

        fn get_i64(&self, col: &str) -> i64 {
            self.values
                .iter()
                .find(|(k, _)| *k == col)
                .and_then(|(_, v)| v.as_i64())
                .unwrap_or(0)
        }

        fn get_str(&self, col: &str) -> &str {
            self.values
                .iter()
                .find(|(k, _)| *k == col)
                .and_then(|(_, v)| v.as_str())
                .unwrap_or("")
        }

        fn get_bool(&self, col: &str) -> bool {
            self.values
                .iter()
                .find(|(k, _)| *k == col)
                .and_then(|(_, v)| v.as_bool())
                .unwrap_or(false)
        }
    }

    #[async_trait::async_trait]
    impl ControlClient for MockControlClient {
        async fn ctrl_query(
            &self,
            _q: &str,
            _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
        ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
            match self.results.lock().await.pop() {
                Some(MockQueryResult::Rows(rows)) => Ok(rows.into_iter().map(|v| Box::new(MockRow { values: v }) as Box<dyn RowGet>).collect()),
                Some(MockQueryResult::EmptyRows) => Ok(vec![]),
                Some(MockQueryResult::Error) => Err(Box::<dyn std::error::Error + Send + Sync>::from("mock query error")),
                _ => Ok(vec![]),
            }
        }

        async fn ctrl_query_one(
            &self,
            _q: &str,
            _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
        ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>> {
            match self.results.lock().await.pop() {
                Some(MockQueryResult::Row(values)) => {
                    Ok(Box::new(MockRow { values }) as Box<dyn RowGet>)
                }
                Some(MockQueryResult::Error) => Err(Box::<dyn std::error::Error + Send + Sync>::from("mock query error")),
                _ => Err(Box::<dyn std::error::Error + Send + Sync>::from("mock: no result")),
            }
        }

        async fn ctrl_query_opt(
            &self,
            _q: &str,
            _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
        ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(None)
        }

        async fn ctrl_execute(
            &self,
            _q: &str,
            _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
        ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
            match self.results.lock().await.pop() {
                Some(MockQueryResult::Count(c)) => Ok(c),
                _ => Ok(1),
            }
        }
    }

    // ---- MockControlPool ----

    struct MockControlPool {
        client: Arc<MockControlClient>,
    }

    impl MockControlPool {
        fn new(client: Arc<MockControlClient>) -> Self {
            Self { client }
        }
    }

    #[async_trait::async_trait]
    impl ControlPool for MockControlPool {
        fn acquire<'a>(
            &'a self,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn ControlClient>, crate::AppError>> + Send + 'a>>
        {
            Box::pin(async move { Ok(self.client.clone() as Arc<dyn ControlClient>) })
        }
    }

    // ---- Test helpers ----

    fn mock_pool() -> Pool {
        let mgr = deadpool_postgres::Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn mock_control_pool(
        results: Arc<Mutex<Vec<MockQueryResult>>>,
    ) -> Arc<dyn ControlPool> {
        let client = Arc::new(MockControlClient::new(results));
        let pool = MockControlPool::new(client);
        Arc::new(DynControlPool::new(Arc::new(pool)))
    }

    // ---- Tests ----

    #[tokio::test]
    async fn test_get_control_config_returns_success() {
        let results = MockControlClient::single_row(vec![
            ("enabled", Value::Bool(true)),
            ("default_storage", Value::String("local".to_string())),
            ("max_upload_size", Value::Number(serde_json::Number::from(104857600_i64))),
        ]);
        let pool = mock_control_pool(results);

        let result = get_control_config(Extension(pool)).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
        let data = action.data.unwrap();
        assert!(data.get("enabled").is_some());
        assert!(data.get("defaultStorage").is_some());
        assert!(data.get("maxUploadSize").is_some());
    }

    #[tokio::test]
    async fn test_list_storage_pools_returns_success() {
        let results = MockControlClient::empty();
        let pool = mock_control_pool(results);

        let result = list_storage_pools(Extension(pool)).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }

    #[tokio::test]
    async fn test_list_control_categories_returns_success() {
        let results = MockControlClient::empty();
        let pool = mock_control_pool(results);

        let result = list_control_categories(Extension(pool)).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
        let data = action.data.unwrap();
        assert!(data.get("count").is_some());
        assert!(data.get("data").is_some());
    }

    #[tokio::test]
    async fn test_update_control_config_returns_success() {
        let results = MockControlClient::count(1);
        let pool = mock_control_pool(results);

        let body = axum::extract::Json(serde_json::json!({"enabled": false}));
        let result = update_control_config(Extension(pool), body).await.unwrap();
        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
        let data = action.data.unwrap();
        assert_eq!(data.get("updated"), Some(&Value::Bool(true)));
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

    // ── route existence: routes defined in file_assemble_control_router ──────

    #[tokio::test]
    async fn test_file_list_requires_db() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/list/folder-1")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_file_requires_db() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/file-1")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_upload_file_requires_db() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let body = serde_json::to_string(&serde_json::json!({"name": "test.txt", "path": "/tmp", "folderId": "f1", "size": 100})).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/upload")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_create_file_requires_db() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let body = serde_json::to_string(&serde_json::json!({"name": "new.txt", "path": "/tmp", "folderId": "f1"})).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/create")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_delete_file_requires_db() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/delete/file-1")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_unknown_route_returns_404() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/unknown/path")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // ── Request validation: create/update/delete with invalid input ─────────

    #[tokio::test]
    async fn test_create_file_empty_name() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let body = serde_json::to_string(&serde_json::json!({"name": "", "path": "/tmp"})).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/create")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_upload_file_missing_path() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let body = serde_json::to_string(&serde_json::json!({"name": "test.txt", "folderId": "f1"})).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/upload")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_delete_file_empty_id_path() {
        let pool = mock_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/delete/")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
