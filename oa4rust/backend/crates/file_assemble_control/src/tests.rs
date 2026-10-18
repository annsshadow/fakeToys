// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use crate::{
        file_assemble_control_router, get_control_config, list_control_categories,
        list_storage_pools, update_control_config, ControlClient, ControlPool, DynControlPool,
        RowGet,
    };
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use serde_json::Value;
    use shared::response::ActionResult;
    use shared::testing::test_pool;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower::ServiceExt;

    // ---- Mock types ----

    // Mock 全形状枚举：Rows/Error 变体供未来用例扩展，未构造处显式豁免
    #[allow(dead_code)]
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

        #[allow(dead_code)]
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
                Some(MockQueryResult::Rows(rows)) => Ok(rows
                    .into_iter()
                    .map(|v| Box::new(MockRow { values: v }) as Box<dyn RowGet>)
                    .collect()),
                Some(MockQueryResult::EmptyRows) => Ok(vec![]),
                Some(MockQueryResult::Error) => Err(
                    Box::<dyn std::error::Error + Send + Sync>::from("mock query error"),
                ),
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
                Some(MockQueryResult::Error) => Err(
                    Box::<dyn std::error::Error + Send + Sync>::from("mock query error"),
                ),
                _ => Err(Box::<dyn std::error::Error + Send + Sync>::from(
                    "mock: no result",
                )),
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
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<Arc<dyn ControlClient>, crate::AppError>>
                    + Send
                    + 'a,
            >,
        > {
            Box::pin(async move { Ok(self.client.clone() as Arc<dyn ControlClient>) })
        }
    }

    // ---- Test helpers ----

    fn mock_control_pool(results: Arc<Mutex<Vec<MockQueryResult>>>) -> Arc<dyn ControlPool> {
        let client = Arc::new(MockControlClient::new(results));
        let pool = MockControlPool::new(client);
        Arc::new(DynControlPool::new(Arc::new(pool)))
    }

    // ---- Tests ----

    #[tokio::test]
    async fn test_get_control_config_returns_success() {
        let results = MockControlClient::single_row(vec![
            // round4-b77：enabled/max_upload_size 实为 TEXT 列，handler 按字符串解析，mock 同步。
            ("enabled", Value::String("true".to_string())),
            ("default_storage", Value::String("local".to_string())),
            (
                "max_upload_size",
                Value::String("104857600".to_string()),
            ),
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
        assert!(action.data.unwrap().is_array());
        assert!(action.count.is_some());
    }

    #[tokio::test]
    async fn test_update_control_config_returns_success() {
        // round4-b77：update_control_config 已收 admin 门（全局配置写属管理面）。
        // 离线环境下无法产出 admin（is_admin 需真库），原"mock 写通道成功"臂
        // 迁移至 security_gate_tests_b77 的真库 #[ignore] 臂；
        // 本臂改验门禁契约：mock 写通道 + 死池（is_admin fail-closed）+ 非 admin → 403。
        let results = MockControlClient::count(1);
        let pool = mock_control_pool(results);

        // 死池：is_admin 查不到即 fail-closed → 非 admin 会话必须 403（离线可证）。
        let dead = {
            let mgr = deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            );
            deadpool_postgres::Pool::builder(mgr).max_size(1).build().unwrap()
        };

        let body = axum::extract::Json(serde_json::json!({"enabled": false}));
        let result = update_control_config(
            Extension(pool),
            Extension(dead),
            Some(Extension(shared::testing::test_session("b77-nonadmin"))),
            body,
        )
        .await;
        assert!(
            matches!(result, Err(shared::error::AppError::Forbidden)),
            "非 admin 会话调用 update_control_config 必须 403（is_admin fail-closed）"
        );
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

    // 优化二轮 47：attachment2 列表分页 offset 原 page*size（page=1 跳过首页）
    // + size 无上限 → 修为 (page-1)*size + size 夹到 1..=200。
    #[test]
    fn attachment2_limit_offset_is_one_based_and_clamped() {
        // page=1 必须 offset=0（修复前为 size）
        assert_eq!(crate::attachment2_limit_offset("1", "20"), (20, 0));
        assert_eq!(crate::attachment2_limit_offset("2", "20"), (20, 20));
        assert_eq!(crate::attachment2_limit_offset("3", "50"), (50, 100));
        // page<=0 / 非法 → 兜底第一页
        assert_eq!(crate::attachment2_limit_offset("0", "20"), (20, 0));
        assert_eq!(crate::attachment2_limit_offset("x", "20"), (20, 0));
        // size 非法回退 20、上限夹到 200、下限 1
        assert_eq!(crate::attachment2_limit_offset("1", "bad"), (20, 0));
        assert_eq!(crate::attachment2_limit_offset("1", "9999"), (200, 0));
        assert_eq!(crate::attachment2_limit_offset("2", "0"), (1, 1));
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
                    .uri("/api/file/assemble/control/file/list/folder-1")
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
                    .uri("/api/file/assemble/control/file/file-1")
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
        let body = serde_json::to_string(
            &serde_json::json!({"name": "test.txt", "path": "/tmp", "folderId": "f1", "size": 100}),
        )
        .unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/upload")
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
        let body = serde_json::to_string(
            &serde_json::json!({"name": "new.txt", "path": "/tmp", "folderId": "f1"}),
        )
        .unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/create")
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
                    .uri("/api/file/assemble/control/file/delete/file-1")
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
                    .uri("/api/file/assemble/control/unknown/path")
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
                    .uri("/api/file/assemble/control/file/create")
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
        let body =
            serde_json::to_string(&serde_json::json!({"name": "test.txt", "folderId": "f1"}))
                .unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/upload")
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
                    .uri("/api/file/assemble/control/file/delete/")
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
                    .uri("/api/file/assemble/control/attachment2/some-id/office/preview/type/docx")
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
            zip.start_file("word/styles.xml", zip::write::SimpleFileOptions::default())
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

    /// 四轮安全审计 N1 回归：段落数上限必须生效且恰好输出 `DOCX_MAX_PARAGRAPHS` 个，
    /// 超出的段落不得渲染（无上限时百 MB 级输入会产出行数无界的 HTML）。
    #[test]
    fn test_docx_to_html_caps_paragraph_count() {
        let mut xml = String::from("<w:document><w:body>");
        for i in 0..(crate::DOCX_MAX_PARAGRAPHS + 50) {
            xml.push_str(&format!("<w:p><w:r><w:t>P{i}</w:t></w:r></w:p>"));
        }
        xml.push_str("</w:body></w:document>");
        let html = crate::docx_to_html(&build_docx(&xml)).expect("docx should parse");
        assert_eq!(
            html.matches("<p>").count(),
            crate::DOCX_MAX_PARAGRAPHS,
            "段落数必须恰好截断在上限"
        );
        assert!(html.contains("<p>P0</p>"));
        assert!(
            !html.contains(&format!("<p>P{}</p>", crate::DOCX_MAX_PARAGRAPHS)),
            "上限之后的段落不得输出"
        );
    }

    /// 四轮安全审计 N1 回归：zip 炸弹（高压缩比 document.xml）不得让解压结果
    /// 超出 `DOCX_MAX_XML_BYTES`。断言方式是「输出规模有界」——HTML 段落数仍受
    /// 段落上限约束、体积不随解压字节线性膨胀，且全程不 panic。
    #[test]
    fn test_docx_to_html_caps_decompressed_size() {
        // ~8 MiB 纯 ASCII XML（远超 4 MiB 读取上限），压缩后体量极小
        let filler = "A".repeat(2000);
        let mut xml = String::with_capacity(9 * 1024 * 1024);
        xml.push_str("<w:document><w:body>");
        for _ in 0..5000 {
            xml.push_str("<w:p><w:r><w:t>");
            xml.push_str(&filler);
            xml.push_str("</w:t></w:r></w:p>");
        }
        xml.push_str("</w:body></w:document>");
        assert!(xml.len() > crate::DOCX_MAX_XML_BYTES as usize * 2);
        let bytes = build_docx(&xml);
        let html = crate::docx_to_html(&bytes).expect("bounded read should still parse");
        assert_eq!(
            html.matches("<p>").count(),
            crate::DOCX_MAX_PARAGRAPHS,
            "zip 炸弹下段落数仍受上限约束"
        );
        assert!(
            html.len() <= (crate::DOCX_MAX_PARAGRAPHS + 1) * (filler.len() + 8),
            "HTML 输出规模必须有界，got {}",
            html.len()
        );
    }

    /// 四轮安全审计 N4-01 回归：`read_zip_entry`（XLSX sharedStrings / PPTX 每张 slide
    /// 都经它）必须有解压字节上限。`XLSX_MAX_ROWS` / `PPTX_MAX_SLIDES` 只限制解压后的
    /// 输出规模，不限制驻留内存，故上限只能落在本函数。
    #[test]
    fn test_read_zip_entry_caps_decompressed_size() {
        use std::io::Write;
        let payload = "B".repeat(9 * 1024 * 1024);
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut buf);
            zip.start_file(
                "xl/sharedStrings.xml",
                zip::write::SimpleFileOptions::default(),
            )
            .unwrap();
            zip.write_all(payload.as_bytes()).unwrap();
            zip.finish().unwrap();
        }
        let bytes = buf.into_inner();
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes.as_slice())).unwrap();
        let s = crate::read_zip_entry(&mut archive, "xl/sharedStrings.xml")
            .expect("entry should read");
        assert_eq!(s.len(), crate::ZIP_ENTRY_MAX_BYTES as usize, "必须截断在上限");
        assert!(s.len() < payload.len(), "解压结果不得等于源条目大小");
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_anonymous_file_id_download_stream() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/anonymous/file/test-id/download/stream")
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
    async fn test_get_attachment_download_attid_stream() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attachment/download/test-id/stream")
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
    async fn test_get_file_assemble_control_attachment2_() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/attachment2/test-id/office/preview/type/test-id")
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
    async fn test_get_file_assemble_control_file_list_fo() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/list/test-id")
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
    async fn test_get_file_assemble_control_file_id() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/test-id")
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
    async fn test_get_file_id_download_stream() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/test-id/download/stream")
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
    async fn test_post_file_assemble_control_file_create() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/create")
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
    async fn test_post_file_assemble_control_file_delete_() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/delete/test-id")
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
    async fn test_post_file_assemble_control_file_upload() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/upload")
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
    async fn test_post_file_core_entity_file_create() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/core/entity/file/create")
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
    async fn test_post_file_core_entity_file_delete_id() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/core/entity/file/delete/test-id")
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
    async fn test_post_file_core_entity_file_update_id() {
        let pool = test_pool();
        let app = crate::file_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/core/entity/file/update/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    // 优化二轮 52：complex_top「最新/置顶」展示原漏 `deleted_at IS NULL`，软删的
    // 文件/文件夹会出现在 top 展示里。修复后须排除软删行。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_complex_top_excludes_soft_deleted_rows() {
        let pool = test_pool();
        let c = pool.get().await.unwrap();
        for stmt in [
            "DELETE FROM FILE_FILE WHERE id IN ('u2-cx-live','u2-cx-dead')",
            "DELETE FROM FILE_FOLDER WHERE id IN ('u2-cx-flive','u2-cx-fdead')",
        ] {
            c.execute(stmt, &[]).await.unwrap();
        }
        c.execute(
            "INSERT INTO FILE_FILE (id, name, person, reference_type, extension, length, deleted_at) \
             VALUES ('u2-cx-live','00zx-live','u2p','file','png',10,NULL), \
                    ('u2-cx-dead','00zx-dead','u2p','file','png',10,NOW())",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO FILE_FOLDER (id, name, person, superior, deleted_at) \
             VALUES ('u2-cx-flive','00zf-live','u2p',NULL,NULL), \
                    ('u2-cx-fdead','00zf-dead','u2p',NULL,NOW())",
            &[],
        )
        .await
        .unwrap();

        let result = crate::complex_top(Extension(pool.clone())).await;
        assert!(result.is_ok());
        let j = serde_json::to_value(&result.unwrap().0).unwrap();
        let files: Vec<&str> = j["data"]["attachmentList"]
            .as_array()
            .map(|a| a.iter().filter_map(|e| e["id"].as_str()).collect())
            .unwrap_or_default();
        let folders: Vec<&str> = j["data"]["folderList"]
            .as_array()
            .map(|a| a.iter().filter_map(|e| e["id"].as_str()).collect())
            .unwrap_or_default();

        assert!(files.contains(&"u2-cx-live"), "未删文件应出现在 top 展示");
        assert!(
            !files.contains(&"u2-cx-dead"),
            "软删文件不得出现在 top 展示"
        );
        assert!(
            folders.contains(&"u2-cx-flive"),
            "未删文件夹应出现在 top 展示"
        );
        assert!(
            !folders.contains(&"u2-cx-fdead"),
            "软删文件夹不得出现在 top 展示"
        );

        c.execute(
            "DELETE FROM FILE_FILE WHERE id IN ('u2-cx-live','u2-cx-dead')",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM FILE_FOLDER WHERE id IN ('u2-cx-flive','u2-cx-fdead')",
            &[],
        )
        .await
        .unwrap();
    }
}

#[cfg(test)]
mod office_preview_tests {
    fn build_zip(entries: &[(&str, &str)]) -> Vec<u8> {
        let buf = std::io::Cursor::new(Vec::new());
        let mut w = zip::ZipWriter::new(buf);
        let opts = zip::write::SimpleFileOptions::default();
        for (name, content) in entries {
            w.start_file(*name, opts).unwrap();
            std::io::Write::write_all(&mut w, content.as_bytes()).unwrap();
        }
        w.finish().unwrap().into_inner()
    }

    #[test]
    fn test_xlsx_to_html_renders_shared_and_literal_cells() {
        let shared = "<?xml version=\"1.0\"?><sst><si><t>名称</t></si><si><t>数量</t></si></sst>";
        let sheet = "<?xml version=\"1.0\"?><worksheet><row><c r=\"A1\" t=\"s\"><v>0</v></c><c r=\"B1\" t=\"s\"><v>1</v></c></row><row><c r=\"A2\"><v>42</v></c><c r=\"B2\"><v>3.14</v></c></row></worksheet>";
        let bytes = build_zip(&[
            ("xl/sharedStrings.xml", shared),
            ("xl/worksheets/sheet1.xml", sheet),
        ]);
        let html = crate::xlsx_to_html(&bytes).expect("should render");
        assert!(html.starts_with("<table>"));
        assert!(html.contains("<td>名称</td>"));
        assert!(html.contains("<td>42</td>"));
        assert!(html.matches("<tr>").count() == 2);
    }

    #[test]
    fn test_xlsx_to_html_empty_sheet_returns_none() {
        let sheet = "<?xml version=\"1.0\"?><worksheet></worksheet>";
        let bytes = build_zip(&[
            ("xl/sharedStrings.xml", "<sst></sst>"),
            ("xl/worksheets/sheet1.xml", sheet),
        ]);
        assert!(crate::xlsx_to_html(&bytes).is_none());
    }

    #[test]
    fn test_pptx_to_html_renders_slides_in_order() {
        let s2 = "<?xml version=\"1.0\"?><p:sp><a:t>第二页</a:t><a:t>要点</a:t></p:sp>";
        let s1 = "<?xml version=\"1.0\"?><p:sp><a:t>封面标题</a:t></p:sp>";
        let bytes = build_zip(&[("ppt/slides/slide2.xml", s2), ("ppt/slides/slide1.xml", s1)]);
        let html = crate::pptx_to_html(&bytes).expect("should render");
        let h2_pos = html.find("<h2>封面标题</h2>").expect("slide1 title");
        let s2_pos = html.find("<h2>第二页</h2>").expect("slide2 title");
        assert!(h2_pos < s2_pos, "slides must render in numeric order");
        assert!(html.contains("<p>要点</p>"));
    }

    #[test]
    fn test_parsers_return_none_for_garbage_bytes() {
        assert!(crate::xlsx_to_html(b"not a zip").is_none());
        assert!(crate::pptx_to_html(b"not a zip").is_none());
    }

    // 优化二轮 45：附件名搜索 ILIKE 通配符转义。搜索词里的 `_` 必须按字面匹配，
    // 不得当通配符命中任意单字（否则搜 "_" 近乎列全部附件）。
    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn attachment_name_search_treats_underscore_as_literal() {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS FILE_FILE (id VARCHAR(255) PRIMARY KEY, name VARCHAR(500) NOT NULL, person VARCHAR(255) NOT NULL, reference_id VARCHAR(255) DEFAULT '', reference_type VARCHAR(100) DEFAULT 'file', extension VARCHAR(50), length BIGINT DEFAULT 0, mime_type VARCHAR(200), create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, deleted_at TIMESTAMP)",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM FILE_FILE WHERE id IN ('u2-ff-under','u2-ff-plain')",
            &[],
        )
        .await
        .unwrap();
        // name 'a_c' 含字面下划线；'abc' 不含
        c.execute(
            "INSERT INTO FILE_FILE (id, name, person) VALUES ('u2-ff-under','a_c','u2p'), ('u2-ff-plain','abc','u2p')",
            &[],
        )
        .await
        .unwrap();

        let resp = crate::attachment2_list_filter_name(
            axum::Extension(pool.clone()),
            axum::extract::Path("_".to_string()),
        )
        .await
        .unwrap();
        let j = serde_json::to_value(&resp.0).unwrap();
        let ids: Vec<String> = j["data"]
            .as_array()
            .expect("data array")
            .iter()
            .map(|r| r["id"].as_str().unwrap_or_default().to_string())
            .collect();
        assert!(
            ids.contains(&"u2-ff-under".to_string()),
            "应命中含字面下划线的附件名"
        );
        assert!(
            !ids.contains(&"u2-ff-plain".to_string()),
            "下划线须按字面匹配，不得当通配符命中 'abc'"
        );

        c.execute(
            "DELETE FROM FILE_FILE WHERE id IN ('u2-ff-under','u2-ff-plain')",
            &[],
        )
        .await
        .unwrap();
    }

    // 回归（四轮审计 GET 写面队列 · file_assemble_control 2 把）：
    // file_clean_unused_.../manage（批量删 FILE_FILE，收 admin 门）与
    // file_copy_attachment_...（复制附件新建行，收归属门）。
    #[tokio::test]
    async fn test_file_get_writes_owner_admin_gate() {
        use axum::extract::Extension;
        // 无会话离线臂：两把全 401（先于读库）。
        let dead = {
            let mgr = deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            );
            deadpool_postgres::Pool::builder(mgr).max_size(1).build().unwrap()
        };
        assert!(
            matches!(
                crate::file_clean_unused_referencetype_cmsdocument_manage(
                    Extension(dead.clone()),
                    None
                )
                .await,
                Err(shared::error::AppError::Unauthorized)
            ),
            "clean_unused 无会话必须 401"
        );
        assert!(
            matches!(
                crate::file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale(
                    Extension(dead.clone()),
                    None,
                    axum::extract::Path(("f".to_string(), "doc".to_string(), "r".to_string(), "1".to_string()))
                )
                .await,
                Err(shared::error::AppError::Unauthorized)
            ),
            "copy_attachment 无会话必须 401"
        );

        if !shared::testing::is_db_available().await {
            eprintln!(
                "skipping test_file_get_writes_owner_admin_gate(real-db arm): DATABASE_URL not reachable"
            );
            return;
        }
        let pool = shared::testing::test_pool();
        const OWNER: &str = "file-gate-owner";
        const OTHER: &str = "file-gate-other";
        const ADMIN: &str = "file-gate-admin";
        const ROLE: &str = "file-gate-role-admin";
        let sess = |p: &str| Some(Extension(shared::testing::test_session(p)));
        let copy_path = |refid: &str| {
            axum::extract::Path((
                "file-gate-src".to_string(),
                "doc".to_string(),
                refid.to_string(),
                "1".to_string(),
            ))
        };

        // 造 admin + 归属文件行（clean 行 reference_type=cmsdocument_manage，src 行 doc）。
        {
            let c = pool.get().await.unwrap();
            c.execute(
                "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) \
                 ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
                &[&ROLE.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ($1,$1,'file gate admin','x') ON CONFLICT (id) DO NOTHING",
                &[&ADMIN.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 VALUES ($1,$2,'file-gate-unit') ON CONFLICT DO NOTHING",
                &[&ADMIN.to_string(), &ROLE.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO file_file (id, name, person, reference_id, reference_type, create_time) \
                 VALUES ('file-gate-clean','f',$1,'ref-clean','cmsdocument_manage',NOW()) \
                 ON CONFLICT (id) DO NOTHING",
                &[&OWNER.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO file_file (id, name, person, reference_id, reference_type, create_time) \
                 VALUES ('file-gate-src','f',$1,'ref-src','doc',NOW()) \
                 ON CONFLICT (id) DO NOTHING",
                &[&OWNER.to_string()],
            )
            .await
            .unwrap();
        }

        // 非 admin → clean_unused 403 且不落笔；非属主 → copy 403 且不建行。
        assert!(
            matches!(
                crate::file_clean_unused_referencetype_cmsdocument_manage(
                    Extension(pool.clone()),
                    sess(OTHER)
                )
                .await,
                Err(shared::error::AppError::Forbidden)
            ),
            "clean_unused 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale(
                    Extension(pool.clone()),
                    sess(OTHER),
                    copy_path("ref-other")
                )
                .await,
                Err(shared::error::AppError::Forbidden)
            ),
            "copy 他人附件必须 403"
        );
        {
            let c = pool.get().await.unwrap();
            let n: i64 = c
                .query_one("SELECT COUNT(*) FROM file_file WHERE id='file-gate-clean'", &[])
                .await
                .unwrap()
                .get(0);
            assert_eq!(n, 1, "非 admin 403 侧不得删 clean 行");
            let m: i64 = c
                .query_one("SELECT COUNT(*) FROM file_file WHERE reference_id='ref-other'", &[])
                .await
                .unwrap()
                .get(0);
            assert_eq!(m, 0, "非属主 403 侧不得建复制行");
        }

        // 属主放行：copy 附件成功，新建一行（reference_id=ref-owner）。
        assert!(
            crate::file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale(
                Extension(pool.clone()),
                sess(OWNER),
                copy_path("ref-owner")
            )
            .await
            .is_ok(),
            "属主 copy 应放行"
        );

        // admin 放行：clean_unused 删掉 cmsdocument_manage 行；admin 也能 copy（admin 旁路）。
        assert!(
            crate::file_clean_unused_referencetype_cmsdocument_manage(
                Extension(pool.clone()),
                sess(ADMIN)
            )
            .await
            .is_ok(),
            "admin clean_unused 应放行"
        );
        assert!(
            crate::file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale(
                Extension(pool.clone()),
                sess(ADMIN),
                copy_path("ref-admin")
            )
            .await
            .is_ok(),
            "admin copy 应放行（admin 旁路）"
        );
        {
            let c = pool.get().await.unwrap();
            let n: i64 = c
                .query_one("SELECT COUNT(*) FROM file_file WHERE id='file-gate-clean'", &[])
                .await
                .unwrap()
                .get(0);
            assert_eq!(n, 0, "admin clean_unused 应删掉 cmsdocument_manage 行");
            let o: i64 = c
                .query_one("SELECT COUNT(*) FROM file_file WHERE reference_id='ref-owner'", &[])
                .await
                .unwrap()
                .get(0);
            let a: i64 = c
                .query_one("SELECT COUNT(*) FROM file_file WHERE reference_id='ref-admin'", &[])
                .await
                .unwrap()
                .get(0);
            assert_eq!(o, 1, "属主 copy 应新建 1 行");
            assert_eq!(a, 1, "admin copy 应新建 1 行");
        }

        // 收口：清造数与复制行，零残留。
        let c = pool.get().await.unwrap();
        c.execute(
            "DELETE FROM file_file WHERE id LIKE 'file-gate-%' OR reference_id IN ('ref-owner','ref-admin')",
            &[],
        )
        .await
        .unwrap();
        c.execute("DELETE FROM auth_person_role WHERE person_id=$1", &[&ADMIN.to_string()])
            .await
            .unwrap();
        c.execute("DELETE FROM auth_person WHERE id=$1", &[&ADMIN.to_string()])
            .await
            .unwrap();
        c.execute("DELETE FROM auth_role WHERE id=$1", &[&ROLE.to_string()])
            .await
            .unwrap();
    }
}
