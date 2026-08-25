//! plan002 U2 新增端点（cms_assemble_control）的测试。
//!
//! 三层：
//!   1. 路由可达性：mock_pool 无法建连，请求命中路由后返回 500/415，
//!      断言 ≠404/405 即证明路由已注册且动词正确；Router 构建本身会
//!      校验路径唯一性——重复注册将直接 panic。
//!   2. 门禁单元测试：直接调用 handler，require_admin 类端点在 DB 不可用
//!      时 is_admin fail-closed 返回 false → Forbidden；所有者门禁类端点
//!      在取所有者前即失败 → Internal（同样拒绝放行）。
//!   3. 真库端到端：is_db_available 守卫（与既有 tests.rs 一致），验证
//!      create→publish→top→delete 生命周期、IDOR 403、以及派生资源经父
//!      资源所有者校验。

#[cfg(test)]
mod u2_tests {
    use crate::{
        appinfo_u2_create, comment_u2_create, document_u2_category_change, document_u2_delete,
        permission_u2_app_info, permission_u2_category_info, script_u2_list_manager,
        u2_body_i64, u2_body_str, u2_body_strs,
    };
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, StatusCode};
    use serde_json::json;
    use shared::error::AppError;
    use shared::session::Session;
    use shared::testing::{is_db_available, mock_pool, test_pool};
    use tower::util::ServiceExt;

    const OWNER: &str = "person-u2";
    const STRANGER: &str = "u2-stranger";

    fn session(person: &str) -> Session {
        let now = chrono::Utc::now().naive_utc();
        Session {
            token: format!("u2-token-{person}"),
            person_unique: person.to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(2),
        }
    }

    // ── 1. 路由可达性 ───────────────────────────────────────────

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        status_with(method, uri, None).await
    }

    async fn status_with(method: &str, uri: &str, body: Option<serde_json::Value>) -> StatusCode {
        let app = crate::router(mock_pool());
        let mut builder = Request::builder().method(method).uri(uri);
        if body.is_some() {
            builder = builder.header("content-type", "application/json");
        }
        let req = match body {
            Some(v) => builder.body(Body::from(v.to_string())).unwrap(),
            None => builder.body(Body::empty()).unwrap(),
        };
        app.oneshot(req).await.unwrap().status()
    }

    #[tokio::test]
    async fn u2_document_read_routes_reachable() {
        assert_eq!(status_of("GET", "/jaxrs/document/d-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("GET", "/jaxrs/document/d-1/document/data").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("GET", "/jaxrs/document/document/fields").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(status_of("GET", "/jaxrs/document/d-1/top").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/document/d-1/unTop").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_document_write_routes_reachable() {
        // 带 Json 提取器的写端点：空 body → 415，仍 ≠404/405
        assert_ne!(status_of("POST", "/jaxrs/document").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/document/d-1/update").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("PUT", "/jaxrs/document/category/change").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/document/category/change/mockputtopost").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/document/list/document").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("PUT", "/jaxrs/document/filter/count").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/document/filter/count/mockputtopost").await, StatusCode::NOT_FOUND);
        assert_ne!(
            status_of("POST", "/jaxrs/document/publish/d-1/mockputtopost").await,
            StatusCode::NOT_FOUND
        );
        // DELETE / publish / cancel：会话缺失在 handler 前被拒（500），证明已注册
        assert_eq!(status_of("DELETE", "/jaxrs/document/d-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("PUT", "/jaxrs/document/publish/d-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("PUT", "/jaxrs/document/publish/d-1/cancel").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(status_of("GET", "/jaxrs/document/d-1/commend").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/document/d-1/uncommend").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_comment_routes_reachable() {
        assert_ne!(status_of("POST", "/jaxrs/comment").await, StatusCode::NOT_FOUND);
        assert_eq!(status_of("DELETE", "/jaxrs/comment/c-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            status_of("PUT", "/jaxrs/comment/list/1/size/10").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_ne!(
            status_of("POST", "/jaxrs/comment/list/1/size/10/mockputtopost").await,
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn u2_correlation_file_fileinfo_routes_reachable() {
        assert_ne!(
            status_of("POST", "/jaxrs/correlation/doc/doc-9/delete").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(status_of("POST", "/jaxrs/file").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/file/f-1/mockputtopost").await, StatusCode::NOT_FOUND);
        assert_eq!(status_of("DELETE", "/jaxrs/fileinfo/fi-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_ne!(status_of("POST", "/jaxrs/fileinfo/list/filter").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/fileinfo/copy/to/doc/doc-9").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("POST", "/jaxrs/fileinfo/replace/to/doc/doc-9").await, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_design_resource_routes_reachable() {
        for uri in [
            "/jaxrs/form",
            "/jaxrs/script",
            "/jaxrs/templateform",
            "/jaxrs/view",
            "/jaxrs/viewcategory",
            "/jaxrs/viewfieldconfig",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
        for (method, uri) in [
            ("PUT", "/jaxrs/form/f-1"),
            ("DELETE", "/jaxrs/form/f-1"),
            ("PUT", "/jaxrs/script/s-1"),
            ("DELETE", "/jaxrs/script/s-1"),
            ("DELETE", "/jaxrs/templateform/tf-1"),
            ("PUT", "/jaxrs/view/v-1"),
            ("DELETE", "/jaxrs/view/v-1"),
            ("DELETE", "/jaxrs/viewcategory/vc-1"),
            ("PUT", "/jaxrs/viewfieldconfig/vfc-1"),
            ("DELETE", "/jaxrs/viewfieldconfig/vfc-1"),
        ] {
            assert_ne!(status_of(method, uri).await, StatusCode::NOT_FOUND, "{method} {uri}");
            assert_ne!(
                status_of(method, uri).await,
                StatusCode::METHOD_NOT_ALLOWED,
                "{method} {uri}"
            );
        }
        assert_eq!(
            status_of("POST", "/jaxrs/script/list/manager").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u2_appinfo_categoryinfo_permission_appconfig_designer_reachable() {
        for uri in [
            "/jaxrs/appinfo",
            "/jaxrs/categoryinfo",
            "/jaxrs/appinfo/a-1/permission",
            "/jaxrs/categoryinfo/c-1/permission",
            "/jaxrs/appconfig/app-1",
            "/jaxrs/designer/search",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
        assert_eq!(
            status_of("DELETE", "/jaxrs/appinfo/a-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("DELETE", "/jaxrs/categoryinfo/c-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(status_of("GET", "/jaxrs/appconfig/a-1").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ── 2. 门禁单元测试（mock pool，fail-closed） ──────────────

    #[tokio::test]
    async fn u2_owner_gated_delete_fails_closed_without_db() {
        let r = document_u2_delete(
            Extension(mock_pool()),
            Extension(session(OWNER)),
            axum::extract::Path("d-1".to_string()),
        )
        .await;
        match r {
            Err(AppError::Internal) => {}
            other => panic!("expected Internal(fail closed), got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_document_category_change_requires_admin() {
        let r = document_u2_category_change(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Json(json!({"categoryId": "c-1", "docIds": ["d-1"]})),
        )
        .await;
        match r {
            Err(AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_script_list_manager_requires_admin() {
        let r = script_u2_list_manager(Extension(mock_pool()), Extension(session(STRANGER))).await;
        match r {
            Err(AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_appinfo_create_requires_admin() {
        let r = appinfo_u2_create(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Json(json!({"alias": "x"})),
        )
        .await;
        match r {
            Err(AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_permission_endpoints_require_admin() {
        let r = permission_u2_app_info(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Path("a-1".to_string()),
            axum::extract::Json(json!({"roleType": "viewer", "personIds": ["p-1"]})),
        )
        .await;
        assert!(matches!(r, Err(AppError::Forbidden)));
        let r = permission_u2_category_info(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Path("c-1".to_string()),
            axum::extract::Json(json!({"roleType": "manager", "personIds": ["p-1"]})),
        )
        .await;
        assert!(matches!(r, Err(AppError::Forbidden)));
    }

    // ── 纯单元测试 ─────────────────────────────────────────────

    #[test]
    fn u2_body_helpers_extract_values() {
        let body = json!({
            "title": "t",
            "ids": ["a", "b"],
            "size": 7,
            "empty": ""
        });
        assert_eq!(u2_body_str(&body, "title"), Some("t".to_string()));
        assert_eq!(u2_body_str(&body, "missing"), None);
        assert_eq!(u2_body_str(&body, "empty"), Some(String::new()));
        assert_eq!(u2_body_strs(&body, "ids"), vec!["a".to_string(), "b".to_string()]);
        assert!(u2_body_strs(&body, "missing").is_empty());
        assert_eq!(u2_body_i64(&body, "size"), Some(7));
        assert_eq!(u2_body_i64(&body, "title"), None);
    }

    // ── 3. 真库端到端 ──────────────────────────────────────────

    async fn call(
        method: &str,
        uri: &str,
        body: Option<serde_json::Value>,
        sess: Option<Session>,
    ) -> (StatusCode, serde_json::Value) {
        let app = crate::router(test_pool());
        let mut builder = Request::builder().method(method).uri(uri);
        if let Some(s) = &sess {
            builder = builder.extension(s.clone());
        }
        let req = match body {
            Some(v) => builder
                .header("content-type", "application/json")
                .body(Body::from(v.to_string()))
                .unwrap(),
            None => builder.body(Body::empty()).unwrap(),
        };
        let resp = app.oneshot(req).await.unwrap();
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), 1_048_576).await.unwrap();
        let json = if bytes.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
        };
        (status, json)
    }

    async fn cleanup(client: &deadpool_postgres::Client, sqls: &[&str]) {
        for sql in sqls {
            let _ = client.execute(*sql, &[]).await;
        }
    }

    #[tokio::test]
    async fn u2_document_lifecycle_real_db() {
        use shared::testing::test_pool as tp;
        if !is_db_available().await {
            eprintln!("skipping u2_document_lifecycle_real_db: db not reachable");
            return;
        }
        let doc_id = "u2test-doc-lifecycle";
        {
            let client = tp().get().await.unwrap();
            cleanup(
                &client,
                &[
                    "DELETE FROM x_cms_commend WHERE doc_id = 'u2test-doc-lifecycle'",
                    &format!("DELETE FROM x_cms_data_document WHERE id = '{doc_id}'"),
                ],
            )
            .await;
            client
                .execute(
                    "INSERT INTO x_cms_data_document (id, title, creator) VALUES ($1, 'U2 Lifecycle', $2)",
                    &[&doc_id, &OWNER],
                )
                .await
                .unwrap();
        }

        // 读：无会话可读
        let (status, json) = call("GET", &format!("/jaxrs/document/{doc_id}"), None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["title"], "U2 Lifecycle");

        // 发布（所有者）：状态真实落库
        let (status, json) = call(
            "PUT",
            &format!("/jaxrs/document/publish/{doc_id}"),
            None,
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "publish: {json}");
        assert_eq!(json["data"]["published"], true);

        let (_, json) = call("GET", &format!("/jaxrs/document/{doc_id}"), None, None).await;
        assert_eq!(json["data"]["status"], "published");

        // 过滤计数命中刚发布的文档（此时仍是 published）
        let (status, json) = call("PUT", "/jaxrs/document/filter/count", Some(json!({"status": "published"})), None).await;
        assert_eq!(status, StatusCode::OK);
        assert!(json["data"]["total"].as_i64().unwrap() >= 1);

        // mockputtopost 别名与 PUT 共用同一发布语义：cancel 恢复草稿
        let (status, _) = call(
            "PUT",
            &format!("/jaxrs/document/publish/{doc_id}/cancel"),
            None,
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let (_, json) = call("GET", &format!("/jaxrs/document/{doc_id}"), None, None).await;
        assert_eq!(json["data"]["status"], "draft");

        // top 标记真实落库
        let (status, json) = call("GET", &format!("/jaxrs/document/{doc_id}/top"), None, Some(session(OWNER))).await;
        assert_eq!(status, StatusCode::OK, "top: {json}");
        assert_eq!(json["data"]["isTop"], true);

        // 删除后再读 → 明确 not found 语义
        let (status, json) = call("DELETE", &format!("/jaxrs/document/{doc_id}"), None, Some(session(OWNER))).await;
        assert_eq!(status, StatusCode::OK, "delete: {json}");
        let (status, json) = call("GET", &format!("/jaxrs/document/{doc_id}"), None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "document not found");
    }

    #[tokio::test]
    async fn u2_document_idor_forbidden_real_db() {
        if !is_db_available().await {
            eprintln!("skipping u2_document_idor_forbidden_real_db: db not reachable");
            return;
        }
        let doc_id = "u2test-doc-idor";
        {
            let client = test_pool().get().await.unwrap();
            client
                .execute(
                    "DELETE FROM x_cms_data_document WHERE id = $1",
                    &[&doc_id],
                )
                .await
                .unwrap();
            client
                .execute(
                    "INSERT INTO x_cms_data_document (id, title, creator) VALUES ($1, 'U2 IDOR', $2)",
                    &[&doc_id, &OWNER],
                )
                .await
                .unwrap();
        }

        // 非所有者删除 → 403，且文档未被删除
        let (status, _) = call("DELETE", &format!("/jaxrs/document/{doc_id}"), None, Some(session(STRANGER))).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, json) = call("GET", &format!("/jaxrs/document/{doc_id}"), None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["type"], "success");

        // 所有者删除成功
        let (status, json) = call("DELETE", &format!("/jaxrs/document/{doc_id}"), None, Some(session(OWNER))).await;
        assert_eq!(status, StatusCode::OK, "owner delete: {json}");
        assert_eq!(json["data"]["deleted"], true);
    }

    #[tokio::test]
    async fn u2_comment_create_paging_delete_real_db() {
        if !is_db_available().await {
            eprintln!("skipping u2_comment_create_paging_delete_real_db: db not reachable");
            return;
        }
        let doc_id = "u2test-doc-comment";
        {
            let client = test_pool().get().await.unwrap();
            cleanup(
                &client,
                &[
                    "DELETE FROM x_cms_comment WHERE doc_id = 'u2test-doc-comment'",
                    "DELETE FROM x_cms_data_document WHERE id = 'u2test-doc-comment'",
                ],
            )
            .await;
            client
                .execute(
                    "INSERT INTO x_cms_data_document (id, title, creator) VALUES ($1, 'U2 Comment', $2)",
                    &[&doc_id, &OWNER],
                )
                .await
                .unwrap();
        }

        let (status, json) = call(
            "POST",
            "/jaxrs/comment",
            Some(json!({"docId": doc_id, "content": "hello u2"})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "comment create: {json}");
        assert_eq!(json["type"], "success");
        let comment_id = json["data"]["id"].as_str().unwrap().to_string();

        let (status, json) = call("PUT", "/jaxrs/comment/list/1/size/50", None, None).await;
        assert_eq!(status, StatusCode::OK, "paging: {json}");
        assert!(json["data"]["count"].as_i64().unwrap() >= 1);
        assert_eq!(json["data"]["page"], 1);

        let (status, json) = call(
            "DELETE",
            &format!("/jaxrs/comment/{comment_id}"),
            None,
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "comment delete: {json}");
        assert_eq!(json["data"]["deleted"], true);
    }

    #[tokio::test]
    async fn u2_designer_search_real_db() {
        if !is_db_available().await {
            eprintln!("skipping u2_designer_search_real_db: db not reachable");
            return;
        }
        {
            let client = test_pool().get().await.unwrap();
            client
                .execute("DELETE FROM x_cms_form WHERE id = 'u2test-form-searchprobe'", &[])
                .await
                .unwrap();
            client
                .execute(
                    "INSERT INTO x_cms_form (id, app_id, name, creator) VALUES ('u2test-form-searchprobe', 'probe-app', 'U2SearchProbeForm', 'system')",
                    &[],
                )
                .await
                .unwrap();
        }
        let (status, json) = call(
            "POST",
            "/jaxrs/designer/search",
            Some(json!({"keyword": "U2SearchProbe"})),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "search: {json}");
        assert!(json["data"]["count"].as_i64().unwrap() >= 1);
        assert_eq!(json["data"]["data"][0]["kind"], "form");

        let _ = test_pool()
            .get()
            .await
            .unwrap()
            .execute("DELETE FROM x_cms_form WHERE id = 'u2test-form-searchprobe'", &[])
            .await;
    }

    #[tokio::test]
    async fn u2_appconfig_roundtrip_real_db() {
        if !is_db_available().await {
            eprintln!("skipping u2_appconfig_roundtrip_real_db: db not reachable");
            return;
        }
        let app_id = "u2test-app-config";
        {
            let client = test_pool().get().await.unwrap();
            client
                .execute(
                    "DELETE FROM x_cms_appinfo WHERE id = $1",
                    &[&app_id],
                )
                .await
                .unwrap();
            client
                .execute(
                    "INSERT INTO x_cms_appinfo (id, alias, app_type, manager) VALUES ($1, 'U2Cfg', 'cms', $2)",
                    &[&app_id, &OWNER],
                )
                .await
                .unwrap();
        }

        // 非管理者写配置 → 403
        let (status, _) = call(
            "POST",
            &format!("/jaxrs/appconfig/{app_id}"),
            Some(json!({"maxDocs": 5})),
            Some(session(STRANGER)),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        // 管理者写配置并回读
        let (status, json) = call(
            "POST",
            &format!("/jaxrs/appconfig/{app_id}"),
            Some(json!({"maxDocs": 5})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "appconfig save: {json}");
        assert_eq!(json["data"]["saved"], true);

        let (status, json) = call("GET", &format!("/jaxrs/appconfig/{app_id}"), None, None).await;
        assert_eq!(status, StatusCode::OK, "appconfig get: {json}");
        assert_eq!(json["data"]["maxDocs"], 5);

        let _ = test_pool()
            .get()
            .await
            .unwrap()
            .execute("DELETE FROM x_cms_appinfo WHERE id = $1", &[&app_id])
            .await;
    }

    #[tokio::test]
    async fn u2_viewfieldconfig_parent_gate_real_db() {
        if !is_db_available().await {
            eprintln!("skipping u2_viewfieldconfig_parent_gate_real_db: db not reachable");
            return;
        }
        let view_id = "u2test-view-gate";
        {
            let client = test_pool().get().await.unwrap();
            cleanup(
                &client,
                &[
                    "DELETE FROM x_cms_viewfieldconfig WHERE view_id = 'u2test-view-gate'",
                    "DELETE FROM x_cms_view WHERE id = 'u2test-view-gate'",
                ],
            )
            .await;
            client
                .execute(
                    "INSERT INTO x_cms_view (id, name, creator) VALUES ($1, 'gate view', $2)",
                    &[&view_id, &OWNER],
                )
                .await
                .unwrap();
        }

        // 视图非所有者不能挂字段配置 → 403
        let (status, _) = call(
            "POST",
            "/jaxrs/viewfieldconfig",
            Some(json!({"viewId": view_id, "fieldName": "f1"})),
            Some(session(STRANGER)),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        // 所有者可以创建，随后按 id 更新与删除
        let (status, json) = call(
            "POST",
            "/jaxrs/viewfieldconfig",
            Some(json!({"viewId": view_id, "fieldName": "f1"})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "vfc create: {json}");
        let vfc_id = json["data"]["id"].as_str().unwrap().to_string();

        let (status, json) = call(
            "PUT",
            &format!("/jaxrs/viewfieldconfig/{vfc_id}"),
            Some(json!({"sortOrder": 3})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "vfc update: {json}");

        let (status, _) = call(
            "DELETE",
            &format!("/jaxrs/viewfieldconfig/{vfc_id}"),
            None,
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn u2_fileinfo_copy_replace_real_db() {
        if !is_db_available().await {
            eprintln!("skipping u2_fileinfo_copy_replace_real_db: db not reachable");
            return;
        }
        let d1 = "u2test-doc-copy-src";
        let d2 = "u2test-doc-copy-dst";
        let fi = "u2test-fileinfo-copy";
        {
            let client = test_pool().get().await.unwrap();
            cleanup(
                &client,
                &[
                    &format!("DELETE FROM x_cms_fileinfo WHERE doc_id IN ('{d1}', '{d2}') OR id = '{fi}'"),
                    &format!("DELETE FROM x_cms_data_document WHERE id IN ('{d1}', '{d2}')"),
                ],
            )
            .await;
            for id in [d1, d2] {
                client
                    .execute(
                        "INSERT INTO x_cms_data_document (id, title, creator) VALUES ($1, 'U2 Copy', $2)",
                        &[&id, &OWNER],
                    )
                    .await
                    .unwrap();
            }
            client
                .execute(
                    "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, upload_person) VALUES ($1, $2, 'a.pdf', $3)",
                    &[&fi, &d1, &OWNER],
                )
                .await
                .unwrap();
        }

        // 复制附件到目标文档
        let (status, json) = call(
            "POST",
            &format!("/jaxrs/fileinfo/copy/to/doc/{d2}"),
            Some(json!({"attachmentIds": [fi]})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "copy: {json}");
        assert_eq!(json["data"]["copied"], 1);

        // 移动原附件到目标文档
        let (status, json) = call(
            "POST",
            &format!("/jaxrs/fileinfo/replace/to/doc/{d2}"),
            Some(json!({"attachmentIds": [fi]})),
            Some(session(OWNER)),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "replace: {json}");
        assert_eq!(json["data"]["moved"], 1);

        // 数据库中原附件确实换了宿主文档
        let row = test_pool()
            .get()
            .await
            .unwrap()
            .query_one("SELECT doc_id FROM x_cms_fileinfo WHERE id = $1", &[&fi])
            .await
            .unwrap();
        let moved_doc: String = row.get("doc_id");
        assert_eq!(moved_doc, d2);

        cleanup(
            &test_pool().get().await.unwrap(),
            &[
                &format!("DELETE FROM x_cms_fileinfo WHERE doc_id IN ('{d1}', '{d2}')"),
                &format!("DELETE FROM x_cms_data_document WHERE id IN ('{d1}', '{d2}')"),
            ],
        )
        .await;
    }
}
