#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use file_assemble_control::{ControlClient, ControlPool, DynControlPool, RowGet};
    use serde_json::Value;
    use shared::error::AppError;
    use shared::response::ActionResult;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower::ServiceExt;

    // ── Mock types ───────────────────────────────────────────────────────────

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

    enum MockQueryResult {
        Row(Vec<(&'static str, Value)>),
        Rows(Vec<Vec<(&'static str, Value)>>),
        Empty,
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
            Arc::new(Mutex::new(vec![MockQueryResult::Empty]))
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
                Some(MockQueryResult::Rows(r)) => Ok(r
                    .into_iter()
                    .map(|v| Box::new(MockRow { values: v }) as Box<dyn RowGet>)
                    .collect()),
                Some(MockQueryResult::Empty) => Ok(vec![]),
                Some(MockQueryResult::Error) => {
                    Err(Box::<dyn std::error::Error + Send + Sync>::from("mock error"))
                }
                _ => Ok(vec![]),
            }
        }

        async fn ctrl_query_one(
            &self,
            _q: &str,
            _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
        ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>> {
            match self.results.lock().await.pop() {
                Some(MockQueryResult::Row(v)) => Ok(Box::new(MockRow { values: v }) as Box<dyn RowGet>),
                Some(MockQueryResult::Error) => {
                    Err(Box::<dyn std::error::Error + Send + Sync>::from("mock error"))
                }
                _ => Err(Box::<dyn std::error::Error + Send + Sync>::from("no result")),
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
            Ok(1)
        }
    }

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
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn ControlClient>, AppError>> + Send + 'a>>
        {
            Box::pin(async move { Ok(self.client.clone() as Arc<dyn ControlClient>) })
        }
    }

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        Pool::builder(mgr).build().unwrap()
    }

    fn mock_control_pool(
        results: Arc<Mutex<Vec<MockQueryResult>>>,
    ) -> Arc<dyn ControlPool> {
        let client = Arc::new(MockControlClient::new(results));
        let pool = MockControlPool::new(client);
        Arc::new(DynControlPool::new(Arc::new(pool)))
    }

    // ── Route existence tests (without DB) ───────────────────────────────────

    #[tokio::test]
    async fn test_application_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_script_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/script/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_invoke_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/invoke/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_agent_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_structure_list_returns_error_without_db() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/structure/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_unknown_route_returns_404() {
        let pool = build_test_pool();
        let app = crate::program_center_core_entity_router(pool);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/unknown")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // ── ActionResult serialization ───────────────────────────────────────────

    #[test]
    fn test_action_result_success_structure() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 1, "data": []}));
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
    }

    #[test]
    fn test_action_result_error_serialization() {
        let result: ActionResult<&str> = ActionResult::error("server error");
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "server error");
        assert!(json["data"].is_null());
    }

    #[test]
    fn test_action_result_with_count_and_size() {
        let mut result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"items": []}));
        result.count = Some(10);
        result.size = Some(50);
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["count"], 10);
        assert_eq!(json["size"], 50);
    }

    #[test]
    fn test_application_list_response_format() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "count": 3,
            "data": [
                {"id": "1", "name": "测试应用", "category": "办公"}
            ]
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 3);
    }

    #[test]
    fn test_script_list_response_format() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "count": 1,
            "data": [
                {"id": "s1", "name": "脚本1", "alias": "script1", "validated": true, "creatorPerson": "admin"}
            ]
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["data"][0]["validated"], true);
    }

    // ── Mock pool tests: application_list, script_list ───────────────────────

    #[tokio::test]
    async fn test_application_list_with_mock_data() {
        let results = MockControlClient::rows(vec![
            vec![
                ("id", Value::String("app-1".to_string())),
                ("name", Value::String("测试应用".to_string())),
                ("category", Value::String("办公".to_string())),
                ("sub_category", Value::String("文档".to_string())),
                ("version", Value::String("1.0".to_string())),
                ("publisher", Value::String("OA".to_string())),
            ],
            vec![
                ("id", Value::String("app-2".to_string())),
                ("name", Value::String("流程应用".to_string())),
                ("category", Value::String("流程".to_string())),
                ("sub_category", Value::String("审批".to_string())),
                ("version", Value::String("2.0".to_string())),
                ("publisher", Value::String("IT".to_string())),
            ],
        ]);
        let pool = mock_control_pool(results);
        let app = crate::program_center_mock_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 2);
        assert_eq!(json["data"]["data"].as_array().unwrap().len(), 2);
        assert_eq!(json["data"]["data"][0]["name"], "测试应用");
    }

    #[tokio::test]
    async fn test_application_list_empty_with_mock() {
        let results = MockControlClient::empty();
        let pool = mock_control_pool(results);
        let app = crate::program_center_mock_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["data"]["count"], 0);
    }

    #[tokio::test]
    async fn test_script_list_with_mock_data() {
        let results = MockControlClient::rows(vec![
            vec![
                ("id", Value::String("s1".to_string())),
                ("name", Value::String("数据脚本".to_string())),
                ("alias", Value::String("data-script".to_string())),
                ("validated", Value::Bool(true)),
                ("creator_person", Value::String("admin".to_string())),
            ],
            vec![
                ("id", Value::String("s2".to_string())),
                ("name", Value::String("报表脚本".to_string())),
                ("alias", Value::String("report-script".to_string())),
                ("validated", Value::Bool(false)),
                ("creator_person", Value::String("user1".to_string())),
            ],
        ]);
        let pool = mock_control_pool(results);
        let app = crate::program_center_mock_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/script/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 2);
        assert_eq!(json["data"]["data"][0]["validated"], true);
        assert_eq!(json["data"]["data"][1]["validated"], false);
    }

    #[tokio::test]
    async fn test_script_list_empty_with_mock() {
        let results = MockControlClient::empty();
        let pool = mock_control_pool(results);
        let app = crate::program_center_mock_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/script/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["data"]["count"], 0);
    }

    // ── Router build tests ───────────────────────────────────────────────────

    #[test]
    fn test_program_center_core_entity_router_builds() {
        let pool = build_test_pool();
        let _ = crate::program_center_core_entity_router(pool);
    }

    #[test]
    fn test_program_center_mock_router_builds() {
        let results = MockControlClient::empty();
        let pool = mock_control_pool(results);
        let _ = crate::program_center_mock_router(pool);
    }
}