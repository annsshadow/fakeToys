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
    use serde_json::Value;
    use shared::response::ActionResult;
    use shared::testing::test_pool;
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

    #[ignore = "requires a running PostgreSQL server"]
    #[test]
    fn test_file_assemble_control_router_builds() {
        let pool = test_pool();
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

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_file_list_route_exists() {
        let pool = test_pool();
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
        // Route exists - should not return 404
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_file_route_exists() {
        let pool = test_pool();
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
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_upload_file_route_exists() {
        let pool = test_pool();
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
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_create_file_route_exists() {
        let pool = test_pool();
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
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_delete_file_route_exists() {
        let pool = test_pool();
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
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_unknown_route_returns_404() {
        let pool = test_pool();
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

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_create_file_empty_name_route_exists() {
        let pool = test_pool();
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
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_upload_file_missing_path_route_exists() {
        let pool = test_pool();
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
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_delete_file_empty_id_path_returns_404() {
        let pool = test_pool();
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

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_office_preview_route_registered() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/attachment2/some-id/office/preview/type/docx")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    fn build_docx(document_xml: &str) -> Vec<u8> {
        use std::io::Write;
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut buf);
            let opts = zip::write::SimpleFileOptions::default();
            zip.start_file("word/document.xml", opts).unwrap();
            zip.write_all(document_xml.as_bytes()).unwrap();
            zip.finish().unwrap();
        }
        buf.into_inner()
    }

    #[test]
    fn test_docx_to_html_extracts_paragraphs() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>Hello world</w:t></w:r></w:p>
    <w:p><w:r><w:t>Second &amp; third</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
        let bytes = build_docx(xml);
        let html = crate::docx_to_html(&bytes).expect("docx should parse");
        assert!(html.contains("<p>Hello world</p>"), "got: {}", html);
        assert!(html.contains("<p>Second &amp; third</p>"), "got: {}", html);
    }

    #[test]
    fn test_docx_to_html_returns_none_for_missing_document_xml() {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut buf);
            zip.start_file(
                "word/styles.xml",
                zip::write::SimpleFileOptions::default(),
            )
            .unwrap();
            zip.finish().unwrap();
        }
        let bytes = buf.into_inner();
        assert!(crate::docx_to_html(&bytes).is_none());
    }

    #[test]
    fn test_docx_to_html_returns_none_for_table_content() {
        let xml = r#"<w:document><w:body><w:tbl><w:tr><w:tc><w:p><w:r><w:t>x</w:t></w:r></w:p></w:tc></w:tr></w:tbl></w:body></w:document>"#;
        let bytes = build_docx(xml);
        assert!(crate::docx_to_html(&bytes).is_none());
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_jaxrs_anonymous_file_id_download_stream() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/file/test-id/download/stream")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_jaxrs_attachment_download_attid_stream() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attachment/download/test-id/stream")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_jaxrs_file_assemble_control_attachment2_() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/attachment2/test-id/office/preview/type/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_jaxrs_file_assemble_control_file_list_fo() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_jaxrs_file_assemble_control_file_id() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_jaxrs_file_id_download_stream() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/test-id/download/stream")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_jaxrs_file_assemble_control_file_create() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_jaxrs_file_assemble_control_file_delete_() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_jaxrs_file_assemble_control_file_upload() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/upload")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_jaxrs_file_core_entity_file_create() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_jaxrs_file_core_entity_file_delete_id() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_jaxrs_file_core_entity_file_update_id() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/update/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }


}
