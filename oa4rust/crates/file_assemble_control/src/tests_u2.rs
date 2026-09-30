// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// ════════════ plan002 U2：file 模块端点全量闭合回归测试 ════════════
// 覆盖：系统参数读/写真实现（读信封 + UPSERT 语义）、BlobStorage 上传 fail-loud（db 占位 → 501 非假成功）、
// blob key 规范化、输入校验先于 DB、各族路由可达性、既有路由回归保护。
#[cfg(test)]
mod u2_tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::session::Session;
    use shared::storage::{DbBlobStorage, FsBlobStorage};
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

    // upload/with/url 已从「把 URL 字符串当内容入库」的假实现改为真拉取
    // （shared::netguard SSRF 防护）。契约：缺 url 400；私网目标在发起请求前被拒 400。
    #[tokio::test]
    async fn u2_upload_with_url_requires_url() {
        let (status, _) = respond_auth(
            "POST",
            "/api/file/assemble/control/file/upload/with/url",
            JSON,
            Body::from("{}"),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn u2_upload_with_url_rejects_private_target() {
        let (status, _) = respond_auth(
            "POST",
            "/api/file/assemble/control/file/upload/with/url",
            JSON,
            Body::from(r#"{"url":"http://169.254.169.254/latest/meta-data"}"#),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
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

    // ── 1. 系统参数读/写真实现（x_system_config，094 建表）────────────────────
    // 原 fail-loud 501 契约随真实消费转向退役：桌面 Settings 页与 configApi.systemConfig
    // （/api/config/system）均指向本能力。读 = 未删行全量 legacy_success 信封；
    // 写 = POST /api/config {"configs":[…]} 按 name UPSERT，value 统一落 TEXT。

    #[tokio::test]
    async fn u2_system_config_read_returns_legacy_success_envelope() {
        if !shared::testing::is_db_available().await {
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_system_config (id, name, value, category, create_time) VALUES ($1, $2, $3, $4, NOW())",
                &[&"u2-cfg-read-test", &"u2.read.key", &"v1", &"u2test"],
            )
            .await
            .unwrap();
        for path in ["/api/config/system/config", "/api/config/system"] {
            let response = crate::router(pool.clone())
                .oneshot(
                    Request::builder()
                        .uri(path)
                        .method("GET")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                response.status(),
                StatusCode::OK,
                "read must be real: {path}"
            );
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(json["type"], "success");
            let items = json["data"].as_array().expect("data is array");
            assert!(
                items.iter().any(|it| it["name"] == "u2.read.key"),
                "seeded row must be returned via {path}"
            );
        }
        client
            .execute(
                "DELETE FROM x_system_config WHERE id = $1",
                &[&"u2-cfg-read-test"],
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn u2_system_config_save_upserts_by_name() {
        if !shared::testing::is_db_available().await {
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        // 空/坏请求 fail-loud：缺 content-type 是客户端错误，绝不静默 success
        let (status, _) = respond("POST", "/api/config", &[], Body::empty()).await;
        assert!(
            (400..500).contains(&status.as_u16()),
            "empty body must be client-error, got {status}"
        );
        let response = crate::router(pool.clone())
            .oneshot(
                Request::builder()
                    .uri("/api/config")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"configs":[{"name":"u2.save.key","value":"a","category":"u2test"},{"name":"u2.save.num","value":8}]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "save must be real");
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["saved"], 2, "two rows saved");
        // UPSERT 语义：同名二写不新增行，值被覆盖；数字 value 统一落 TEXT
        let response = crate::router(pool.clone())
            .oneshot(
                Request::builder()
                    .uri("/api/config")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"configs":[{"name":"u2.save.key","value":"b","category":"u2test"}]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let row = client
            .query_one(
                "SELECT COUNT(*)::BIGINT, MAX(value) FROM x_system_config WHERE name = $1",
                &[&"u2.save.key"],
            )
            .await
            .unwrap();
        let (n, v): (i64, Option<String>) = (row.get(0), row.get(1));
        assert_eq!(n, 1, "same-name save must upsert, not duplicate");
        assert_eq!(v.as_deref(), Some("b"), "second save wins");
        let num = client
            .query_one(
                "SELECT value FROM x_system_config WHERE name = $1",
                &[&"u2.save.num"],
            )
            .await
            .unwrap();
        let nv: String = num.get(0);
        assert_eq!(nv, "8", "numeric value lands as TEXT");
        client
            .execute(
                "DELETE FROM x_system_config WHERE name LIKE 'u2.save.%'",
                &[],
            )
            .await
            .unwrap();
    }

    // zip 打包已真实现（承 folder2_batch_download / folder2_id_download）：
    // 批量版无会话时必须 fail loud 401；文件夹版对不存在的文件夹 404。
    #[tokio::test]
    async fn u2_zip_download_batch_requires_session() {
        let (status, _) = respond("GET", "/api/folder2/batch/download", &[], Body::empty()).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn u2_zip_download_folder_missing_is_404() {
        if !shared::testing::is_db_available().await {
            // 需要 live DB 区分「handler 404 信封」与「mock_pool 500」。
            return;
        }
        let pool = shared::testing::test_pool();
        let response = crate::router(pool)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/folder2/f-1/download")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or_default();
        assert_eq!(
            json["type"], "error",
            "handler 404 must carry error envelope"
        );
    }

    // ── zip 打包真实现：happy path 必须产出可解压的有效 zip ─────────────────

    #[tokio::test]
    async fn u2_zip_folder_download_packs_files_into_valid_zip() {
        if !shared::testing::is_db_available().await {
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        let folder_id = format!("u2zipfolder-{}", uuid::Uuid::new_v4());
        let file_id = format!("u2zipfile-{}", uuid::Uuid::new_v4());
        let folder_name = "打包夹";
        let person = "tester";
        let file_name = "hello.txt";
        use base64::Engine as _;
        let b64 = base64::engine::general_purpose::STANDARD.encode(b"hello zip");
        client
            .execute(
                "INSERT INTO FILE_FOLDER (id, name, person) VALUES ($1, $2, $3)",
                &[&folder_id, &folder_name, &person],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO FILE_FILE (id, name, person, superior, content) VALUES ($1, $2, $3, $4, $5)",
                &[&file_id, &file_name, &person, &folder_id, &b64],
            )
            .await
            .unwrap();

        let response = crate::router(pool.clone())
            .oneshot(
                axum::http::Request::builder()
                    .uri(format!("/api/folder2/{folder_id}/download"))
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/zip"
        );
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes.to_vec())).unwrap();
        assert_eq!(archive.len(), 1, "folder contains exactly one packed file");
        let mut entry = archive.by_name("hello.txt").unwrap();
        let mut unpacked = Vec::new();
        std::io::Read::read_to_end(&mut entry, &mut unpacked).unwrap();
        assert_eq!(unpacked, b"hello zip", "zip payload must round-trip");

        client
            .execute("DELETE FROM FILE_FILE WHERE id = $1", &[&file_id])
            .await
            .unwrap();
        client
            .execute("DELETE FROM FILE_FOLDER WHERE id = $1", &[&folder_id])
            .await
            .unwrap();
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
        let storage = DbBlobStorage;
        let result = crate::u2_persist_verified(&storage, "attachment/x/a.txt", b"hello").await;
        match result {
            Err(crate::AppError::NotImplemented) => {}
            other => {
                panic!("db placeholder upload must be AppError::NotImplemented, got {other:?}")
            }
        }
    }

    /// FS 后端：put + 回读校验真实落盘。
    #[tokio::test]
    async fn u2_persist_verified_fs_roundtrip_succeeds() {
        let dir = std::env::temp_dir().join(format!("oa4rust_u2_file_{}", uuid::Uuid::new_v4()));
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

    /// 红线：上传端点不得「假成功」。迁移到 PgBlobStorage 后，默认（db）后端走真实
    /// DB 落盘（x_blob_storage），fs 后端走磁盘落盘后再写 FILE_FILE 元数据行——两者
    /// 在无 PG 的单测环境中都到不了成功，表现为 500（fail loud），绝不返回内容必丢的假 200。
    /// （真实 PG 环境下：db 后端上传真实持久化并回读校验通过，返回 200。）
    #[tokio::test]
    async fn u2_upload_fails_loud_not_fake_success_without_pg() {
        let expected = StatusCode::INTERNAL_SERVER_ERROR;
        let base =
            "/api/file/assemble/control/file/upload/referencetype/taskReport/reference/w-9/scale/1";
        for (method, headers, body) in [
            ("POST", &[][..], Body::from(vec![1u8, 2, 3])),
            ("PUT", MP, multipart_body("a.txt")),
        ] {
            let (status, json) = respond_auth(method, base, headers, body).await;
            assert_eq!(
                status, expected,
                "upload must fail loud ({expected}) without PG, body={json}"
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
            let (status, _) = respond_auth("POST", "/api/share", JSON, Body::from(body)).await;
            assert_eq!(status, StatusCode::BAD_REQUEST, "body={body}");
        }
    }

    #[tokio::test]
    async fn u2_folder_create_validates_name_before_db() {
        let (status, _) =
            respond_auth("POST", "/api/folder", JSON, Body::from(r#"{"name":"  "}"#)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let (status, _) = respond_auth("POST", "/api/folder2", JSON, Body::from("{}")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    // ── 4. 各族新路由可达性（!=404；无会话的门禁端点为 500） ────────────────

    #[tokio::test]
    async fn u2_attachment_family_routes_reachable() {
        let b = "/api/attachment";
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
            (
                "GET",
                format!("{b}/a-1/image/width/w2/height/h2/binary/base64"),
            ),
            ("PUT", format!("{b}/a-1/update")),
            ("POST", format!("{b}/a-1/update/callback/cb-1")),
        ];
        for (method, path) in cases {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) =
                if needs_body && path.ends_with("/update") || path.contains("/update/callback") {
                    (MP, multipart_body("a.txt"))
                } else if needs_body {
                    (JSON, Body::from("{}"))
                } else {
                    (&[], Body::empty())
                };
            let (status, _) = respond(method, &path, headers, body).await;
            assert_ne!(
                status,
                StatusCode::NOT_FOUND,
                "route missing: {method} {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2_attachment2_family_routes_reachable() {
        let b = "/api/attachment2";
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
            (
                "GET",
                format!("{b}/a-1/image/width/w2/height/h2/binary/base64"),
            ),
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
            assert_ne!(
                status,
                StatusCode::NOT_FOUND,
                "route missing: {method} {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2_file_family_prefixed_routes_reachable() {
        let b = "/api/file/assemble/control/file";
        let cases: Vec<(&str, String)> = vec![
            ("GET", format!("{b}/list/referencetype")),
            ("GET", format!("{b}/list/referencetype/cms/reference/r-1")),
            (
                "GET",
                format!("{b}/list/unused/referencetype/cmsdocument/manage"),
            ),
            ("GET", format!("{b}/list/id-1/next/20")),
            ("GET", format!("{b}/list/id-1/next/20/all")),
            ("GET", format!("{b}/list/id-1/next/20/referencetype/cms")),
            ("GET", format!("{b}/list/id-1/prev/20")),
            ("GET", format!("{b}/list/id-1/prev/20/all")),
            ("GET", format!("{b}/list/id-1/prev/20/referencetype/cms")),
            (
                "DELETE",
                format!("{b}/clean/unused/referencetype/cmsdocument/manage"),
            ),
            (
                "GET",
                format!("{b}/copy/attachment/a-1/referencetype/cms/reference/r-1/scale/1"),
            ),
            ("DELETE", format!("{b}/referencetype/cms/reference/r-1")),
            (
                "POST",
                format!("{b}/upload/referencetype/cms/reference/r-1/scale/1"),
            ),
            (
                "POST",
                format!("{b}/upload/referencetype/cms/reference/r-1/scale/1/callback/cb"),
            ),
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
            assert_ne!(
                status,
                StatusCode::NOT_FOUND,
                "route missing: {method} {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2_folder_folder2_routes_reachable() {
        for (method, path) in [
            ("POST", "/api/folder"),
            ("GET", "/api/folder/list/top"),
            ("GET", "/api/folder/list/f-1"),
            ("GET", "/api/folder/f-1"),
            ("PUT", "/api/folder/f-1"),
            ("DELETE", "/api/folder/f-1"),
            ("POST", "/api/folder2"),
            ("GET", "/api/folder2/list/top"),
            ("GET", "/api/folder2/list/f-1"),
            ("GET", "/api/folder2/f-1"),
            ("PUT", "/api/folder2/f-1"),
            ("DELETE", "/api/folder2/f-1"),
        ] {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) = if needs_body {
                (JSON, Body::from(r#"{"name":"n"}"#))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, path, headers, body).await;
            assert_ne!(
                status,
                StatusCode::NOT_FOUND,
                "route missing: {method} {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2_recycle_share_routes_reachable() {
        for (method, path) in [
            ("DELETE", "/api/recycle/empty"),
            ("GET", "/api/recycle/list"),
            ("GET", "/api/recycle/r-1"),
            ("DELETE", "/api/recycle/r-1/delete"),
            ("POST", "/api/recycle/r-1/resume"),
            ("POST", "/api/share"),
            ("GET", "/api/share/download/share/s-1/file/f-1"),
            ("GET", "/api/share/list"),
            ("GET", "/api/share/list/my"),
            ("GET", "/api/share/list/to/me"),
            ("GET", "/api/share/list/att/share/s-1/folder/fd-1"),
            ("GET", "/api/share/list/folder/share/s-1/folder/fd-1"),
            ("POST", "/api/share/share/s-1/file/f-1/folder/fd-1"),
            ("GET", "/api/share/shield/s-1"),
            ("GET", "/api/share/s-1"),
            ("DELETE", "/api/share/s-1"),
            ("GET", "/api/share/s-1/password/pw-1"),
        ] {
            let needs_body = matches!(method, "PUT" | "POST");
            let (headers, body): (&[(&str, &str)], Body) = if needs_body {
                (JSON, Body::from("{}"))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, path, headers, body).await;
            assert_ne!(
                status,
                StatusCode::NOT_FOUND,
                "route missing: {method} {path}"
            );
        }
    }

    #[tokio::test]
    async fn u2_complex_editor_anonymous_routes_reachable() {
        for (method, path) in [
            ("GET", "/api/complex/folder/c-1"),
            ("GET", "/api/complex/top"),
            ("GET", "/api/editor/list"),
            ("GET", "/api/config/is/file/manager"),
            ("GET", "/api/anonymous/file/an-1/download"),
            ("POST", "/api/anonymous/file/an-1/download"),
            ("POST", "/api/anonymous/file/an-1/download/stream"),
        ] {
            let (status, _) = respond(method, path, &[], Body::empty()).await;
            assert_ne!(
                status,
                StatusCode::NOT_FOUND,
                "route missing: {method} {path}"
            );
        }
    }

    // ── 5. 回归保护 ─────────────────────────────────────────────────────────

    /// 既有前缀 list 路由在占位名归一（{folderId}→{id}）后仍须匹配。
    #[tokio::test]
    async fn u2_legacy_prefixed_list_route_still_matches_after_param_rename() {
        let (status, _) = respond(
            "GET",
            "/api/file/assemble/control/file/list/test-id",
            &[],
            Body::empty(),
        )
        .await;
        assert_ne!(status, StatusCode::NOT_FOUND);

        // 同路径不同方法共存（GET 元数据 / DELETE 删除）
        let (del, _) = respond(
            "DELETE",
            "/api/file/assemble/control/file/test-id",
            &[],
            Body::empty(),
        )
        .await;
        assert_ne!(del, StatusCode::NOT_FOUND);
    }

    /// 同一路径的 GET 与 POST 必须都路由到下载 handler（o2server download/postDownload 对）。
    #[tokio::test]
    async fn u2_same_path_multi_method_routing() {
        let path = "/api/attachment/a-1/download";
        for method in ["GET", "POST"] {
            let (status, _) = respond(method, path, &[], Body::empty()).await;
            assert_ne!(status, StatusCode::METHOD_NOT_ALLOWED, "{method} {path}");
            assert_ne!(status, StatusCode::NOT_FOUND, "{method} {path}");
        }
    }
}
