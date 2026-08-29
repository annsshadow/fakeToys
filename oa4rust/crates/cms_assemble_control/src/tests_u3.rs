//! plan002 U2 收尾重试批次（U3，Java canonical 对齐缺口）的测试。
//!
//! 三层（与 tests_u2 一致）：
//!   1. 路由可达性：mock_pool 无法建连，命中路由后断言 ≠404/405，
//!      证明 canonical 路由已注册且动词正确；Router 构建本身校验路径唯一性。
//!   2. 门禁单元测试：写路径 IDOR 门禁在 DB 不可用时 fail-closed
//!      （require_admin → Forbidden；所有者门禁 → Internal）。
//!   3. 真库端到端（is_db_available 守卫）：batch 生命周期、design/appdict
//!      归一化查重、comment commend 往返、correlation upsert、review 搜索。

#[cfg(test)]
mod u3_tests {
    use crate::{
        categoryinfo_ext_content_save_u3, comment_commend_u3, correlation_create_u3,
        design_appdict_create_u3, document_batch_delete_u3, document_batch_modify_u3,
        file_delete_u3, permission_save_manager_app_u3, review_v2_search_u3,
        script_post_nested_u3, u3_normalize_path_levels, viewrecord_unread_u3,
    };
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, StatusCode};
    use serde_json::json;
    use shared::error::AppError;
    use shared::session::Session;
    use shared::testing::{is_db_available, mock_pool, test_pool};
    use tower::util::ServiceExt;

    const OWNER: &str = "person-u3";
    const STRANGER: &str = "u3-stranger";

    fn session(person: &str) -> Session {
        let now = chrono::Utc::now().naive_utc();
        Session {
            token: format!("u3-token-{person}"),
            person_unique: person.to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(2),
        }
    }

    // ── 1. 路由可达性（canonical 形状） ─────────────────────────

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
    async fn u3_document_management_routes_reachable() {
        // 管理面 GET：无会话在提取器前被拒（500）或 DB 失败（500），均 ≠404/405
        for uri in [
            "/jaxrs/document/achive/d-1",
            "/jaxrs/document/batch/status",
            "/jaxrs/document/batch/b-1/status",
            "/jaxrs/document/batch/b-1/mockdeletetoget",
            "/jaxrs/document/cipher/c-1/permission/read/person/p-1",
            "/jaxrs/document/d-1/control",
            "/jaxrs/document/d-1/mockdeletetoget",
            "/jaxrs/document/d-1/permission/read",
            "/jaxrs/document/d-1/persons",
            "/jaxrs/document/d-1/view",
            "/jaxrs/document/d-1/view/count",
        ] {
            assert_ne!(status_of("GET", uri).await, StatusCode::NOT_FOUND, "GET {uri}");
            assert_ne!(
                status_of("GET", uri).await,
                StatusCode::METHOD_NOT_ALLOWED,
                "GET {uri}"
            );
        }
        // 带 Json 提取器的写端点：空 body → 415，仍 ≠404/405
        assert_ne!(
            status_of("PUT", "/jaxrs/document/filter/list/i-1/next/10").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of("POST", "/jaxrs/document/filter/list/i-1/next/10/mockputtopost").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of("POST", "/jaxrs/document/filter/list/i-1/size/10/manager").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of("PUT", "/jaxrs/document/publish/content").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of("PUT", "/jaxrs/document/cipher/publish/content").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of("POST", "/jaxrs/document/list/document/data").await,
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status_of("DELETE", "/jaxrs/document/batch/b-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn u3_file_fileinfo_canonical_routes_reachable() {
        assert_eq!(
            status_of("PUT", "/jaxrs/file/f-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status_of("DELETE", "/jaxrs/file/f-1").await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        for uri in [
            "/jaxrs/file/f-1/mockdeletetoget",
            "/jaxrs/file/f-1/content",
            "/jaxrs/file/f-1/download",
            "/jaxrs/file/f-1/appInfo/app-1",
            "/jaxrs/file/f-1/appInfo/app-1/content",
            "/jaxrs/file/f-1/appInfo/app-1/download",
            "/jaxrs/file/list/i-1/next/10",
            "/jaxrs/fileinfo/fi-1/online/info",
            "/jaxrs/fileinfo/fi-1/preview/pdf",
            "/jaxrs/fileinfo/fi-1/binary/base64/64",
            "/jaxrs/fileinfo/download/transfer/flag/x",
            "/jaxrs/fileinfo/download/document/d-1/stream",
        ] {
            assert_ne!(status_of("GET", uri).await, StatusCode::NOT_FOUND, "GET {uri}");
        }
        assert_ne!(status_of("POST", "/jaxrs/file/f-1/upload").await, StatusCode::NOT_FOUND);
        assert_ne!(
            status_of("POST", "/jaxrs/fileinfo/upload/with/url").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of("POST", "/jaxrs/fileinfo/update/c-1/content").await,
            StatusCode::NOT_FOUND
        );
        assert_ne!(
            status_of("PUT", "/jaxrs/fileinfo/edit/e-1/doc/d-1").await,
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn u3_design_permission_review_routes_reachable() {
        for uri in [
            "/jaxrs/design/appdict",
            "/jaxrs/design/appdict/da-1/mockputtopost",
            "/jaxrs/design/appdict/list/paging/1/size/10",
            "/jaxrs/review/v2/search",
            "/jaxrs/docpermission",
            "/jaxrs/comment/c-1/commend",
            "/jaxrs/comment/c-1/uncommend",
            "/jaxrs/correlation/doc/d-1",
            "/jaxrs/correlation/update/doc/d-1",
            "/jaxrs/categoryinfo/extContent",
            "/jaxrs/categoryinfo/list/objects",
            "/jaxrs/categoryinfo/c-1/execute/projection",
            "/jaxrs/script/s-1/app/app-1",
            "/jaxrs/script/s-1/appInfo/app-1",
            "/jaxrs/view/viewdata/list/v-1/next/10",
            "/jaxrs/viewrecord/unread/mockputtopost",
            "/jaxrs/output/o-1/select/mockputtopost",
            "/jaxrs/log/list/filter/1/size/10",
            "/jaxrs/image/encode/base64",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
        for (method, uri) in [
            ("PUT", "/jaxrs/design/appdict/da-1"),
            ("DELETE", "/jaxrs/design/appdict/da-1"),
            ("PUT", "/jaxrs/viewrecord/unread"),
            ("PUT", "/jaxrs/output/o-1/select"),
            ("PUT", "/jaxrs/templateform/list/category"),
            ("PUT", "/jaxrs/categoryinfo/bind/c-1/view"),
        ] {
            assert_ne!(status_of(method, uri).await, StatusCode::NOT_FOUND, "{method} {uri}");
        }
        // permission save 家族 ×6
        for uri in [
            "/jaxrs/permission/manager/appInfo/a-1",
            "/jaxrs/permission/publisher/appInfo/a-1",
            "/jaxrs/permission/viewer/appInfo/a-1",
            "/jaxrs/permission/manager/categoryInfo/c-1",
            "/jaxrs/permission/publisher/categoryInfo/c-1",
            "/jaxrs/permission/viewer/categoryInfo/c-1",
        ] {
            assert_ne!(status_of("POST", uri).await, StatusCode::NOT_FOUND, "POST {uri}");
        }
    }

    #[tokio::test]
    async fn u3_anonymous_and_designer_readonly_routes_reachable() {
        for (method, uri) in [
            ("GET", "/jaxrs/anonymous/form/f-1"),
            ("GET", "/jaxrs/anonymous/form/v2/f-1"),
            ("GET", "/jaxrs/anonymous/form/v2/f-1/mobile"),
            ("GET", "/jaxrs/anonymous/form/v2/lookup/document/d-1"),
            ("GET", "/jaxrs/anonymous/form/v2/lookup/document/d-1/mobile"),
            ("GET", "/jaxrs/anonymous/fileinfo/list/document/d-1"),
            ("GET", "/jaxrs/anonymous/fileinfo/fi-1/document/d-1"),
            ("GET", "/jaxrs/anonymous/fileinfo/download/document/d-1/stream"),
            ("GET", "/jaxrs/formversion/fv-1"),
            ("GET", "/jaxrs/formversion/list/form/f-1"),
            ("GET", "/jaxrs/scriptversion/sv-1"),
            ("GET", "/jaxrs/scriptversion/list/script/s-1"),
            ("GET", "/jaxrs/templateform/tf-1"),
            ("GET", "/jaxrs/templateform/tf-1/mockdeletetoget"),
            ("GET", "/jaxrs/view/v-1/mockdeletetoget"),
            ("GET", "/jaxrs/viewcategory/vc-1/mockdeletetoget"),
            ("GET", "/jaxrs/viewfieldconfig/vfc-1/mockdeletetoget"),
            ("GET", "/jaxrs/script/s-1/app/app-1/imported"),
            ("GET", "/jaxrs/script/list/i-1/next/10"),
            ("GET", "/jaxrs/viewrecord/person/p-1"),
            ("GET", "/jaxrs/viewrecord/document/d-1/has/view"),
            ("GET", "/jaxrs/appinfo/a-1/control"),
            ("GET", "/jaxrs/appinfo/a-1/mockdeletetoget"),
            ("GET", "/jaxrs/appinfo/get/user/publish/app-1"),
            ("GET", "/jaxrs/appinfo/alias/alpha"),
            ("GET", "/jaxrs/categoryinfo/c-1/control"),
            ("GET", "/jaxrs/categoryinfo/alias/alpha"),
            ("GET", "/jaxrs/form/f-1/appinfo/app-1"),
            ("GET", "/jaxrs/form/f-1/mockdeletetoget"),
            ("GET", "/jaxrs/form/v2/f-1/mobile"),
        ] {
            assert_ne!(status_of(method, uri).await, StatusCode::NOT_FOUND, "{method} {uri}");
        }
        for (method, uri) in [
            ("PUT", "/jaxrs/comment/list/i-1/next/10"),
            ("PUT", "/jaxrs/comment/list/i-1/prev/10"),
            ("PUT", "/jaxrs/anonymous/document/filter/list/i-1/next/10"),
            ("PUT", "/jaxrs/anonymous/document/filter/list/p-1/size/10"),
            ("PUT", "/jaxrs/appinfo/filter/list/i-1/next/10"),
            ("PUT", "/jaxrs/categoryinfo/filter/list/p-1/size/10"),
            ("PUT", "/jaxrs/document/cipher/filter/list/p-1/size/10"),
            ("PUT", "/jaxrs/document/draft/list/i-1/next/10"),
            ("POST", "/jaxrs/log/filter/list/i-1/next/10"),
        ] {
            assert_ne!(status_of(method, uri).await, StatusCode::NOT_FOUND, "{method} {uri}");
        }
        assert_ne!(
            status_of("POST", "/jaxrs/appinfo/a-1/icon/size/64").await,
            StatusCode::NOT_FOUND
        );
    }

    // ── 2. 门禁单元测试（mock pool，fail-closed） ────────────────

    #[tokio::test]
    async fn u3_batch_modify_requires_admin() {
        let r = document_batch_modify_u3(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Json(json!({"docIds": ["d-1"], "data": {"k": "v"}})),
        )
        .await;
        match r {
            Err(AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_batch_delete_requires_admin() {
        let r = document_batch_delete_u3(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Path("b-1".to_string()),
        )
        .await;
        match r {
            Err(AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_permission_save_fails_closed_without_db() {
        // scope 管理员门禁：DB 不可用 → is_admin fail-closed=false，
        // 且 owner 查询失败 → Internal（同样拒绝放行）
        let r = permission_save_manager_app_u3(
            Extension(mock_pool()),
            Extension(session(OWNER)),
            axum::extract::Path("a-1".to_string()),
            axum::extract::Json(json!({"personIds": ["p-1"]})),
        )
        .await;
        match r {
            Err(AppError::Forbidden) | Err(AppError::Internal) => {}
            other => panic!("expected gate denial, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_owner_gated_write_fails_closed_without_db() {
        let r = file_delete_u3(
            Extension(mock_pool()),
            Extension(session(OWNER)),
            axum::extract::Path("f-1".to_string()),
        )
        .await;
        match r {
            Err(AppError::Internal) => {}
            other => panic!("expected Internal(fail closed), got {:?}", other.map(|_| "ok")),
        }
        let r = comment_commend_u3(
            Extension(mock_pool()),
            Extension(session(OWNER)),
            axum::extract::Path("c-1".to_string()),
        )
        .await;
        assert!(matches!(r, Err(AppError::Internal)));
    }

    #[tokio::test]
    async fn u3_validation_rejects_empty_payloads_before_db() {
        // 参数校验先于 DB 访问：空 payload → BadRequest 而非 Internal
        let r = correlation_create_u3(
            Extension(mock_pool()),
            axum::extract::Path("d-1".to_string()),
            axum::extract::Json(json!({})),
        )
        .await;
        match r {
            Err(AppError::BadRequest(_)) => {}
            other => panic!("expected BadRequest, got {:?}", other.map(|_| "ok")),
        }
        let r = review_v2_search_u3(Extension(mock_pool()), axum::extract::Json(json!({"keyword": ""})))
            .await;
        // 空 keyword 合法（全量搜索），DB 不可用 → Internal
        assert!(matches!(r, Err(AppError::Internal)));
        let r = viewrecord_unread_u3(Extension(mock_pool()), Extension(session(OWNER))).await;
        assert!(matches!(r, Err(AppError::Internal)));
        let r = categoryinfo_ext_content_save_u3(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Json(json!({"extContent": "x"})),
        )
        .await;
        match r {
            Err(AppError::BadRequest(_)) => {}
            other => panic!("expected BadRequest, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u3_script_nested_requires_scope_manager() {
        let r = script_post_nested_u3(
            Extension(mock_pool()),
            Extension(session(STRANGER)),
            axum::extract::Path(("s-1".to_string(), "app-1".to_string())),
            axum::extract::Json(json!({"importedScripts": [{"name": "n"}]})),
        )
        .await;
        // DB 不可用：owner 查询失败 → Internal fail-closed
        match r {
            Err(AppError::Internal) | Err(AppError::Forbidden) => {}
            other => panic!("expected gate denial, got {:?}", other.map(|_| "ok")),
        }
    }

    #[test]
    fn u3_normalize_path_levels_dedupes_shapes() {
        assert_eq!(u3_normalize_path_levels(&["b".into(), "a".into()]), "a/b");
        assert_eq!(u3_normalize_path_levels(&[" a ".into(), "".into(), "a".into()]), "a/a");
        assert_eq!(u3_normalize_path_levels(&[]), "");
    }

    // ── 3. 真库端到端（守卫与既有 tests.rs 一致） ─────────────────

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

    #[tokio::test]
    async fn u3_batch_lifecycle_real_db() {
        if !is_db_available().await {
            return;
        }
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        let _ = client
            .execute(
                "DELETE FROM x_cms_data_document WHERE batch_name LIKE 'u3-batch-%' OR id LIKE 'u3-batch-%'",
                &[],
            )
            .await;

        for n in 0..2 {
            client
                .execute(
                    "INSERT INTO x_cms_data_document (id, title, creator, batch_name) \
                     VALUES ($1, 'u3 batch doc', 'person-u3', 'u3-batch-x')",
                    &[&format!("u3-batch-doc-{n}")],
                )
                .await
                .unwrap();
        }

        let (status, body) = call("GET", "/jaxrs/document/batch/u3-batch-x/status", None, None).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["data"]["count"], serde_json::json!(2));

        // 非管理员删除批次 → 403（IDOR 门禁）
        let (status, _) = call(
            "DELETE",
            "/jaxrs/document/batch/u3-batch-x",
            None,
            Some(session(STRANGER)),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        // mockdeletetoget 只预览不删除
        let (status, body) =
            call("GET", "/jaxrs/document/batch/u3-batch-x/mockdeletetoget", None, None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"]["wouldDelete"], serde_json::json!(2));

        // 管理员删除批次 → 数据软删
        if !shared::middleware::is_admin(&pool, "__admin__").await {
            return; // 测试库无管理员账号时跳过删除段（上方门禁段已覆盖语义）
        }
        let admin = Session {
            token: "u3-admin".to_string(),
            person_unique: "__admin__".to_string(),
            ..session(STRANGER)
        };
        let (status, body) = call("DELETE", "/jaxrs/document/batch/u3-batch-x", None, Some(admin)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"]["deleted"], serde_json::json!(2));

        let _ = client
            .execute("DELETE FROM x_cms_data_document WHERE batch_name = 'u3-batch-x'", &[])
            .await;
    }

    #[tokio::test]
    async fn u3_design_appdict_normalized_dedupe_real_db() {
        if !is_db_available().await {
            return;
        }
        let pool = test_pool();
        if !shared::middleware::is_admin(&pool, "__admin__").await {
            return; // 无管理员种子时跳过真库段
        }
        let admin = Session {
            token: "u3-admin".to_string(),
            person_unique: "__admin__".to_string(),
            ..session(STRANGER)
        };
        let _ = call("DELETE", "/jaxrs/design/appdict/u3-dedup-target", None, Some(admin.clone())).await;

        let payload = json!({
            "appInfoFlag": "app-u3",
            "appDictFlag": "dict-u3",
            "pathLevels": ["b", "a"],
            "dataValue": {"v": 1}
        });
        let (status, body) =
            call("POST", "/jaxrs/design/appdict", Some(payload), Some(admin.clone())).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let created_id = body["data"]["id"].as_str().unwrap_or_default().to_string();

        // 相同归一化 pathLevels（顺序不同）→ 查重拒绝
        let dup_payload = json!({
            "appInfoFlag": "app-u3",
            "appDictFlag": "dict-u3",
            "pathLevels": ["a", "b"],
            "dataValue": {"v": 2}
        });
        let (status, body) = call("POST", "/jaxrs/design/appdict", Some(dup_payload), Some(admin)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["type"], serde_json::json!("error"), "{body}");

        let client = pool.get().await.unwrap();
        let _ = client
            .execute(
                "DELETE FROM x_cms_surface_appdict WHERE id = $1 OR (app_info_flag='app-u3' AND app_dict_flag='dict-u3')",
                &[&created_id],
            )
            .await;
    }

    #[tokio::test]
    async fn u3_comment_commend_roundtrip_real_db() {
        if !is_db_available().await {
            return;
        }
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        let _ = client.execute("DELETE FROM x_cms_comment WHERE id = 'u3-cmt'", &[]).await;
        let _ = client.execute("DELETE FROM x_cms_commend WHERE doc_id = 'u3-doc'", &[]).await;
        client
            .execute(
                "INSERT INTO x_cms_comment (id, doc_id, person_id, content) VALUES ('u3-cmt', 'u3-doc', 'other', 'hi')",
                &[],
            )
            .await
            .unwrap();

        let (status, body) =
            call("GET", "/jaxrs/comment/u3-cmt/commend", None, Some(session(OWNER))).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["data"]["commended"], serde_json::json!(true));

        let (status, body) =
            call("GET", "/jaxrs/comment/u3-cmt/uncommend", None, Some(session(OWNER))).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["data"]["uncommended"], serde_json::json!(true));

        let _ = client.execute("DELETE FROM x_cms_comment WHERE id = 'u3-cmt'", &[]).await;
        let _ = client.execute("DELETE FROM x_cms_commend WHERE doc_id = 'u3-doc'", &[]).await;
    }

    #[tokio::test]
    async fn u3_review_search_returns_published_only_real_db() {
        if !is_db_available().await {
            return;
        }
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        let _ = client
            .execute("DELETE FROM x_cms_data_document WHERE id LIKE 'u3-review-%'", &[])
            .await;
        client
            .execute(
                "INSERT INTO x_cms_data_document (id, title, content, creator, status) \
                 VALUES ('u3-review-pub', 'u3 needle title', 'c', 'p', 'published')",
                &[],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO x_cms_data_document (id, title, content, creator, status) \
                 VALUES ('u3-review-draft', 'u3 needle draft', 'c', 'p', 'draft')",
                &[],
            )
            .await
            .unwrap();

        let (status, body) =
            call("POST", "/jaxrs/review/v2/search", Some(json!({"keyword": "needle"})), None).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        // java_success format: {"count":N,"data":[...]}
        let ids: Vec<&str> = body["data"]
            .as_array()
            .map(|a| a.iter().filter_map(|d| d["id"].as_str()).collect())
            .unwrap_or_default();
        assert!(ids.contains(&"u3-review-pub"), "{body}");
        assert!(!ids.contains(&"u3-review-draft"), "{body}");

        let _ = client
            .execute("DELETE FROM x_cms_data_document WHERE id LIKE 'u3-review-%'", &[])
            .await;
    }

    #[tokio::test]
    async fn u3_correlation_update_upsert_real_db() {
        if !is_db_available().await {
            return;
        }
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        let _ = client
            .execute("DELETE FROM x_cms_correlation WHERE doc_id = 'u3-cor-doc'", &[])
            .await;

        let (status, body) = call(
            "POST",
            "/jaxrs/correlation/update/doc/u3-cor-doc",
            Some(json!({"relatedDocId": "u3-cor-target", "correlationType": "reference"})),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");

        let cnt = client
            .query_one(
                "SELECT COUNT(*)::bigint AS cnt FROM x_cms_correlation \
                 WHERE doc_id = 'u3-cor-doc' AND related_doc_id = 'u3-cor-target'",
                &[],
            )
            .await
            .unwrap();
        assert_eq!(cnt.get::<_, i64>("cnt"), 1);

        let _ = client
            .execute("DELETE FROM x_cms_correlation WHERE doc_id = 'u3-cor-doc'", &[])
            .await;
    }
}
