// ════════════ plan002 U2：file 模块端点全量闭合回归测试 ════════════
// 覆盖：无引擎端点精确 501、BlobStorage 上传 fail-loud（db 占位 → 501 非假成功）、
// blob key 规范化、输入校验先于 DB、各族路由可达性、既有路由回归保护。
#[cfg(test)]
mod u2_tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::session::Session;
    use shared::storage::{BlobStorage, DbBlobStorage, FsBlobStorage};
    use tower::ServiceExt;

    fn test_session() -> Session {
        Session {
            token: "u2-test-token".to_string(),
            person_unique: "tester@u2@P".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
        }
    }

    async fn respond_inner(
        method: &str,
        uri: &str,
        headers: &[(&str, &str)],
        body: Body,
        auth: bool,
    ) -> (StatusCode, serde_json::Value) {
        let mut builder = Request::builder().uri(uri).method(method);
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        if auth {
            builder = builder.extension(test_session());
        }
        let response = crate::router(shared::testing::mock_pool())
            .oneshot(builder.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json = if bytes.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
        };
        (status, json)
    }

    async fn respond(
        method: &str,
        uri: &str,
        headers: &[(&str, &str)],
        body: Body,
    ) -> (StatusCode, serde_json::Value) {
        respond_inner(method, uri, headers, body, false).await
    }

    async fn respond_auth(
        method: &str,
        uri: &str,
        headers: &[(&str, &str)],
        body: Body,
    ) -> (StatusCode, serde_json::Value) {
        respond_inner(method, uri, headers, body, true).await
    }

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        respond(method, uri, &[], Body::empty()).await.0
    }

    fn multipart_body(filename: &str) -> Body {
        Body::from(format!(
            "--xboundary\r\n\
             Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n\
             Content-Type: text/plain\r\n\r\nhello\r\n--xboundary--\r\n"
        ))
    }

    const MP: &[(&str, &str)] = &[("content-type", "multipart/form-data; boundary=xboundary")];
    const JSON: &[(&str, &str)] = &[("content-type", "application/json")];

    // ── 1. 无引擎能力端点：精确 501（fail loud，非静默 success） ─────────────

    #[tokio::test]
    async fn u2_engine_less_endpoints_return_exact_501() {
        for (method, path) in [
            ("GET", "/jaxrs/folder2/batch/download"),
            ("GET", "/jaxrs/folder2/f-1/download"),
            ("POST", "/jaxrs/config"),
            ("GET", "/jaxrs/config/system/config"),
        ] {
            let (status, _) = respond(method, path, &[], Body::empty()).await;
            assert_eq!(
                status,
                StatusCode::NOT_IMPLEMENTED,
                "engine-less endpoint must answer exact 501: {method} {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2_engine_less_501_body_is_action_result_error() {
        let (status, json) = respond("GET", "/jaxrs/config/system/config", &[], Body::empty()).await;
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(json["type"], "error");
        assert!(json.get("message").is_some(), "ActionResult.message required");
        assert!(json["data"].is_null());
    }

    // ── 2. BlobStorage 接入点单元级行为 ──────────────────────────────────────

    #[test]
    fn u2_blob_key_sanitizes_and_rejects_bad_names() {
        assert_eq!(
            crate::u2_blob_key("a-1", "报告.docx").unwrap(),
            "attachment/a-1/报告.docx"
        );
        // 路径分隔符与控制字符被清洗
        assert_eq!(
            crate::u2_blob_key("a-1", "a/b\\c.txt").unwrap(),
            "attachment/a-1/a_b_c.txt"
        );
        assert_eq!(
            crate::u2_blob_key("a-1", "x\u{0007}y.bin").unwrap(),
            "attachment/a-1/xy.bin"
        );
        // 空 / 纯点 / 穿越形态必须拒绝（400），绝不生成逃逸 key
        assert!(crate::u2_blob_key("a-1", "").is_err());
        assert!(crate::u2_blob_key("a-1", "..").is_err());
        assert!(crate::u2_blob_key("a-1", ".hidden").is_ok()); // 前导点被剥离
    }

    #[test]
    fn u2_ext_of_parses_extension() {
        assert_eq!(crate::u2_ext_of("a.b.docx"), "docx");
        assert_eq!(crate::u2_ext_of("noext"), "noext");
    }

    /// 红线：STORAGE_BACKEND=db 占位后端下，put 是 no-op —— 上传必须显式失败，
    /// 绝不能返回"看起来成功但内容丢失"的假成功。
    #[tokio::test]
    async fn u2_persist_verified_db_placeholder_fails_loud() {
        let storage = DbBlobStorage::default();
        let result = crate::u2_persist_verified(&storage, "attachment/x/a.txt", b"hello").await;
        match result {
            Err(crate::AppError::NotImplemented) => {}
            other => panic!("db placeholder upload must be AppError::NotImplemented, got {other:?}"),
        }
    }

    /// FS 后端：put + 回读校验真实落盘。
    #[tokio::test]
    async fn u2_persist_verified_fs_roundtrip_succeeds() {
        let dir = std::env::temp_dir()
            .join(format!("oa4rust_u2_file_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let storage = FsBlobStorage::new(&dir);
        crate::u2_persist_verified(&storage, "attachment/x/a.txt", b"hello")
            .await
            .expect("fs backend must persist and verify");
        assert_eq!(
            std::fs::read(dir.join("attachment").join("x").join("a.txt")).unwrap(),
            b"hello"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// 红线：默认（db 占位）环境下，走 BlobStorage 的新上传端点返回精确 501，
    /// 不写元数据行、不假装成功。（若环境显式配置 STORAGE_BACKEND=fs，则继续走到
    /// DB 写入阶段，在无 PG 的单测环境中表现为 500。）
    #[tokio::test]
    async fn u2_upload_db_placeholder_fails_loud_not_fake_success() {
        let fs_env = std::env::var("STORAGE_BACKEND")
            .map(|v| v.eq_ignore_ascii_case("fs"))
            .unwrap_or(false);
        let expected = if fs_env {
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            StatusCode::NOT_IMPLEMENTED
        };
        let base = "/jaxrs/file/assemble/control/file/upload/referencetype/taskReport/reference/w-9/scale/1";
        for (method, headers, body) in [
            ("POST", &[][..], Body::from(vec![1u8, 2, 3])),
            ("PUT", MP, multipart_body("a.txt")),
        ] {
            let (status, json) = respond_auth(method, base, headers, body).await;
            assert_eq!(
                status, expected,
                "upload must fail loud ({expected}), body={json}"
            );
            assert_eq!(json["type"], "error", "must not fake success: {json}");
        }
    }

    // ── 3. 输入校验先于 DB（确定性 400，不依赖 PG） ─────────────────────────

    #[tokio::test]
    async fn u2_share_create_validation_precedes_db() {
        // 缺 fileId / shareType / password 型分享缺密码 → 400，而非 DB 500
        for body in [
            "{}",
            r#"{"fileId":"f-1"}"#,
            r#"{"shareType":"password"}"#,
            r#"{"fileId":"f-1","shareType":"password"}"#,
        ] {
            let (status, _) = respond_auth("POST", "/jaxrs/share", JSON, Body::from(body)).await;
            assert_eq!(status, StatusCode::BAD_REQUEST, "body={body}");
        }
    }

    #[tokio::test]
    async fn u2_folder_create_validates_name_before_db() {
        let (status, _) = respond_auth("POST", "/jaxrs/folder", JSON, Body::from(r#"{"name":"  "}"#)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let (status, _) = respond_auth("POST", "/jaxrs/folder2", JSON, Body::from("{}")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    // ── 4. 各族新路由可达性（!=404；无会话的门禁端点为 500） ────────────────

    #[tokio::test]
    async fn u2_attachment_family_routes_reachable() {
        let b = "/jaxrs/attachment";
        let cases: Vec<(&str, String)> = vec![
            ("GET", format!("{b}/list/top")),
            ("GET", format!("{b}/list/editor/o-1")),
            ("GET", format!("{b}/list/folder/f-1")),
            ("GET", format!("{b}/list/share/o-1")),
            ("GET", format!("{b}/a-1")),
            ("PUT", format!("{b}/a-1")),
            ("DELETE", format!("{b}/a-1")),
            ("GET", format!("{b}/a-1/binary/base64")),
            ("GET", format!("{b}/a-1/download")),
            ("POST", format!("{b}/a-1/download")),
            ("GET", format!("{b}/a-1/download/stream")),
            ("POST", format!("{b}/a-1/download/stream")),
            ("GET", format!("{b}/a-1/image/scale/s2/binary/base64")),
            ("GET", format!("{b}/a-1/image/width/w2/height/h2/binary/base64")),
            ("PUT", format!("{b}/a-1/update")),
            ("POST", format!("{b}/a-1/update/callback/cb-1")),
        ];
        for (method, path) in cases {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) = if needs_body && path.ends_with("/update") || path.contains("/update/callback") {
                (MP, multipart_body("a.txt"))
            } else if needs_body {
                (JSON, Body::from("{}"))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, &path, headers, body).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    #[tokio::test]
    async fn u2_attachment2_family_routes_reachable() {
        let b = "/jaxrs/attachment2";
        let cases: Vec<(&str, String)> = vec![
            ("GET", format!("{b}/exist/file/md5-1")),
            ("GET", format!("{b}/list/top")),
            ("GET", format!("{b}/list/editor/o-1")),
            ("GET", format!("{b}/list/filter/name-x")),
            ("GET", format!("{b}/list/folder/f-1")),
            ("GET", format!("{b}/list/share/o-1")),
            ("POST", format!("{b}/list/type/1/size/20")),
            ("GET", format!("{b}/user/capacity")),
            ("GET", format!("{b}/a-1")),
            ("PUT", format!("{b}/a-1")),
            ("DELETE", format!("{b}/a-1")),
            ("GET", format!("{b}/a-1/binary/base64")),
            ("GET", format!("{b}/a-1/download")),
            ("POST", format!("{b}/a-1/download")),
            ("GET", format!("{b}/a-1/download/image/width/w2/height/h2")),
            ("GET", format!("{b}/a-1/download/stream")),
            ("POST", format!("{b}/a-1/download/stream")),
            ("GET", format!("{b}/a-1/image/scale/s2/binary/base64")),
            ("GET", format!("{b}/a-1/image/width/w2/height/h2/binary/base64")),
            ("GET", format!("{b}/a-1/office/preview/type/docx")),
        ];
        for (method, path) in cases {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) = if needs_body {
                (JSON, Body::from("{}"))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, &path, headers, body).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    #[tokio::test]
    async fn u2_file_family_prefixed_routes_reachable() {
        let b = "/jaxrs/file/assemble/control/file";
        let cases: Vec<(&str, String)> = vec![
            ("GET", format!("{b}/list/referencetype")),
            ("GET", format!("{b}/list/referencetype/cms/reference/r-1")),
            ("GET", format!("{b}/list/unused/referencetype/cmsdocument/manage")),
            ("GET", format!("{b}/list/id-1/next/20")),
            ("GET", format!("{b}/list/id-1/next/20/all")),
            ("GET", format!("{b}/list/id-1/next/20/referencetype/cms")),
            ("GET", format!("{b}/list/id-1/prev/20")),
            ("GET", format!("{b}/list/id-1/prev/20/all")),
            ("GET", format!("{b}/list/id-1/prev/20/referencetype/cms")),
            ("DELETE", format!("{b}/clean/unused/referencetype/cmsdocument/manage")),
            ("GET", format!("{b}/copy/attachment/a-1/referencetype/cms/reference/r-1/scale/1")),
            ("DELETE", format!("{b}/referencetype/cms/reference/r-1")),
            ("POST", format!("{b}/upload/referencetype/cms/reference/r-1/scale/1")),
            ("POST", format!("{b}/upload/referencetype/cms/reference/r-1/scale/1/callback/cb")),
            ("POST", format!("{b}/upload/with/url")),
            ("GET", format!("{b}/id-1/binary/base64")),
            ("GET", format!("{b}/id-1/download")),
            ("POST", format!("{b}/id-1/download")),
            ("POST", format!("{b}/id-1/download/stream")),
            ("DELETE", format!("{b}/id-1")),
        ];
        for (method, path) in cases {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) = if needs_body {
                (JSON, Body::from("{}"))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, &path, headers, body).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    #[tokio::test]
    async fn u2_folder_folder2_routes_reachable() {
        for (method, path) in [
            ("POST", "/jaxrs/folder"),
            ("GET", "/jaxrs/folder/list/top"),
            ("GET", "/jaxrs/folder/list/f-1"),
            ("GET", "/jaxrs/folder/f-1"),
            ("PUT", "/jaxrs/folder/f-1"),
            ("DELETE", "/jaxrs/folder/f-1"),
            ("POST", "/jaxrs/folder2"),
            ("GET", "/jaxrs/folder2/list/top"),
            ("GET", "/jaxrs/folder2/list/f-1"),
            ("GET", "/jaxrs/folder2/f-1"),
            ("PUT", "/jaxrs/folder2/f-1"),
            ("DELETE", "/jaxrs/folder2/f-1"),
        ] {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) = if needs_body {
                (JSON, Body::from(r#"{"name":"n"}"#))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, path, headers, body).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    #[tokio::test]
    async fn u2_recycle_share_routes_reachable() {
        for (method, path) in [
            ("DELETE", "/jaxrs/recycle/empty"),
            ("GET", "/jaxrs/recycle/list"),
            ("GET", "/jaxrs/recycle/r-1"),
            ("DELETE", "/jaxrs/recycle/r-1/delete"),
            ("POST", "/jaxrs/recycle/r-1/resume"),
            ("POST", "/jaxrs/share"),
            ("GET", "/jaxrs/share/download/share/s-1/file/f-1"),
            ("GET", "/jaxrs/share/list"),
            ("GET", "/jaxrs/share/list/my"),
            ("GET", "/jaxrs/share/list/to/me"),
            ("GET", "/jaxrs/share/list/att/share/s-1/folder/fd-1"),
            ("GET", "/jaxrs/share/list/folder/share/s-1/folder/fd-1"),
            ("POST", "/jaxrs/share/share/s-1/file/f-1/folder/fd-1"),
            ("GET", "/jaxrs/share/shield/s-1"),
            ("GET", "/jaxrs/share/s-1"),
            ("DELETE", "/jaxrs/share/s-1"),
            ("GET", "/jaxrs/share/s-1/password/pw-1"),
        ] {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) = if needs_body {
                (JSON, Body::from("{}"))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, path, headers, body).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    #[tokio::test]
    async fn u2_complex_editor_anonymous_routes_reachable() {
        for (method, path) in [
            ("GET", "/jaxrs/complex/folder/c-1"),
            ("GET", "/jaxrs/complex/top"),
            ("GET", "/jaxrs/editor/list"),
            ("GET", "/jaxrs/config/is/file/manager"),
            ("GET", "/jaxrs/anonymous/file/an-1/download"),
            ("POST", "/jaxrs/anonymous/file/an-1/download"),
            ("POST", "/jaxrs/anonymous/file/an-1/download/stream"),
        ] {
            let (status, _) = respond(method, path, &[], Body::empty()).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    // ── 5. 回归保护 ─────────────────────────────────────────────────────────

    /// 既有前缀 list 路由在占位名归一（{folderId}→{id}）后仍须匹配。
    #[tokio::test]
    async fn u2_legacy_prefixed_list_route_still_matches_after_param_rename() {
        let (status, _) = respond(
            "GET",
            "/jaxrs/file/assemble/control/file/list/test-id",
            &[],
            Body::empty(),
        )
        .await;
        assert_ne!(status, StatusCode::NOT_FOUND);

        // 同路径不同方法共存（GET 元数据 / DELETE 删除）
        let (del, _) = respond(
            "DELETE",
            "/jaxrs/file/assemble/control/file/test-id",
            &[],
            Body::empty(),
        )
        .await;
        assert_ne!(del, StatusCode::NOT_FOUND);
    }

    /// 同一路径的 GET 与 POST 必须都路由到下载 handler（Java download/postDownload 对）。
    #[tokio::test]
    async fn u2_same_path_multi_method_routing() {
        let path = "/jaxrs/attachment/a-1/download";
        for method in ["GET", "POST"] {
            let (status, _) = respond(method, path, &[], Body::empty()).await;
            assert_ne!(status, StatusCode::METHOD_NOT_ALLOWED, "{method} {path}");
            assert_ne!(status, StatusCode::NOT_FOUND, "{method} {path}");
        }
    }
}
