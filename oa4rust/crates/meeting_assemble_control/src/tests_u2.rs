// ════════════ plan002 U2：meeting 模块端点全量闭合回归测试 ════════════
// 覆盖：blob key 规范化、归一化查重键、db 占位后端上传 fail-loud（501 非假成功）、
// 各族新路由可达性、Java 动词修正、IDOR 门禁（缺会话拒绝 / 非 owner 拒绝）、
// 归一化查重落库冲突、config upsert 往返、room photo 落库、附件引用生命周期。
#[cfg(test)]
mod u2_tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::session::Session;
    use shared::storage::DbBlobStorage;
    use tower::ServiceExt;

    const ADMIN: &str = "admin";
    const NON_ADMIN: &str = "tester@u2@P";

    fn test_session(person: &str) -> Session {
        Session {
            token: format!("u2-test-token-{person}"),
            person_unique: person.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
        }
    }

    async fn respond_inner(
        pool: deadpool_postgres::Pool,
        method: &str,
        uri: &str,
        headers: &[(&str, &str)],
        body: Body,
        auth: Option<&str>,
    ) -> (StatusCode, serde_json::Value) {
        let mut builder = Request::builder().uri(uri).method(method);
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        if let Some(person) = auth {
            builder = builder.extension(test_session(person));
        }
        let response = crate::router(pool)
            .oneshot(builder.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
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
        respond_inner(shared::testing::mock_pool(), method, uri, headers, body, None).await
    }

    async fn respond_auth(
        method: &str,
        uri: &str,
        headers: &[(&str, &str)],
        body: Body,
        person: &str,
    ) -> (StatusCode, serde_json::Value) {
        respond_inner(shared::testing::mock_pool(), method, uri, headers, body, Some(person)).await
    }

    async fn respond_db(
        method: &str,
        uri: &str,
        headers: &[(&str, &str)],
        body: Body,
        person: &str,
    ) -> (StatusCode, serde_json::Value) {
        respond_inner(shared::testing::test_pool(), method, uri, headers, body, Some(person)).await
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

    // ── 1. 纯函数单元级行为 ──────────────────────────────────────────────────

    #[test]
    fn u2_blob_key_sanitizes_and_rejects_bad_names() {
        assert_eq!(
            crate::u2_attachment_blob_key("a-1", "报告.docx").unwrap(),
            "meeting-attachment/a-1/报告.docx"
        );
        assert_eq!(
            crate::u2_attachment_blob_key("a-1", "a/b\\c.txt").unwrap(),
            "meeting-attachment/a-1/a_b_c.txt"
        );
        assert_eq!(
            crate::u2_attachment_blob_key("a-1", "x\u{0007}y.bin").unwrap(),
            "meeting-attachment/a-1/xy.bin"
        );
        assert!(crate::u2_attachment_blob_key("a-1", "").is_err());
        assert!(crate::u2_attachment_blob_key("a-1", "..").is_err());
        assert!(crate::u2_attachment_blob_key("a-1", ".hidden").is_ok()); // 前导点被剥离
    }

    #[test]
    fn u2_normalize_name_dedup_semantics() {
        // 大小写与首尾空白折叠为同一键 —— 同名不同形必须判冲突
        assert_eq!(crate::u2_normalize_name("  Main Hall "), "main hall");
        assert_eq!(crate::u2_normalize_name("MAIN   HALL"), "main hall");
        assert_eq!(crate::u2_normalize_name("main\thall"), "main hall");
        assert_ne!(crate::u2_normalize_name("hall-a"), "hall-b");
    }

    // ── 2. db 占位后端 fail-loud（红线：禁止假成功壳） ───────────────────────

    #[tokio::test]
    async fn u2_persist_blob_verified_db_placeholder_fails_loud() {
        let result = crate::u2_persist_blob_verified("meeting-attachment/x/a.txt", b"hello").await;
        match result {
            Err(shared::error::AppError::NotImplemented) => {}
            other => panic!("db placeholder upload must be NotImplemented, got {other:?}"),
        }
    }

    /// 红线：默认（db 占位）环境下，附件上传端点必须精确 501，
    /// 且绝不落元数据行（内容必丢 = 不写行）；fs 后端下真实上传成功。
    #[tokio::test]
    async fn u2_upload_endpoint_fails_loud_not_fake_success() {
        if !db_ready().await {
            eprintln!("SKIP (no PG): u2_upload_endpoint_fails_loud_not_fake_success");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        let mid = format!("u2-up-{}", uuid::Uuid::new_v4());
        client
            .execute(
                "INSERT INTO x_meeting (id, title, start_time, end_time, creator) VALUES ($1,'t',NOW(),NOW(),'x')",
                &[&mid],
            )
            .await
            .unwrap();

        let fs_env = std::env::var("STORAGE_BACKEND")
            .map(|v| v.eq_ignore_ascii_case("fs"))
            .unwrap_or(false);
        let path = format!("/jaxrs/meeting/assemble/control/attachment/meeting/{mid}/upload/false");
        let (status, json) =
            respond_db("POST", &path, MP, multipart_body("a.txt"), NON_ADMIN).await;

        if fs_env {
            assert_eq!(status, StatusCode::OK, "fs backend must persist and succeed: {json}");
            assert_eq!(json["data"]["uploaded"], true);
        } else {
            assert_eq!(
                status, StatusCode::NOT_IMPLEMENTED,
                "db placeholder must fail loud with exact 501, body={json}"
            );
            assert_eq!(json["type"], "error", "must not fake success: {json}");
            // 红线核心：501 时不允许残留"看起来已入库"的元数据行
            let cnt: i64 = client
                .query_one("SELECT COUNT(*) AS n FROM x_meeting_attachment WHERE meeting_id = $1", &[&mid])
                .await
                .unwrap()
                .get("n");
            assert_eq!(cnt, 0, "failed upload must not leave metadata rows behind");
        }

        client.execute("DELETE FROM x_meeting WHERE id = $1", &[&mid]).await.unwrap();
    }

    // ── 3. 各族新路由可达性（!=404） ─────────────────────────────────────────

    #[tokio::test]
    async fn u2_attachment_family_routes_reachable() {
        let b = "/jaxrs/meeting/assemble/control/attachment";
        let cases: Vec<(&str, String)> = vec![
            ("GET", format!("{b}/list/meeting/m-1")),
            ("GET", format!("{b}/a-1")),
            ("DELETE", format!("{b}/a-1")),
            ("GET", format!("{b}/a-1/download/true")),
            ("PUT", format!("{b}/a-1/update")),
            ("POST", format!("{b}/a-1/update/callback/cb-1")),
            ("POST", format!("{b}/create/from/processplatform")),
            ("GET", format!("{b}/list/a-1/next/5")),
            ("GET", format!("{b}/list/a-1/prev/5")),
            ("POST", format!("{b}/meeting/m-1/upload/false")),
            ("POST", format!("{b}/meeting/m-1/upload/false/callback/cb-1")),
        ];
        for (method, path) in cases {
            let (headers, body): (&[(&str, &str)], Body) = if path.contains("/upload") {
                (MP, multipart_body("f.bin"))
            } else if matches!(method, "PUT" | "POST") {
                (JSON, Body::from("{}"))
            } else {
                (&[], Body::empty())
            };
            let (status, _) = respond(method, &path, headers, body).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    /// Java 动词修正族：GET 动作端点（accept/reject/checkin 等）与 PUT 端点
    /// 必须接受 Java 原生动词，同时旧错位动词不被误认为缺口已闭。
    #[tokio::test]
    async fn u2_verb_corrected_routes_accept_java_verbs() {
        let b = "/jaxrs/meeting/assemble/control/meeting";
        let cases: Vec<(&str, String)> = vec![
            ("GET", format!("{b}/m-1/accept")),
            ("PUT", format!("{b}/m-1/add/invite")),
            ("GET", format!("{b}/m-1/checkin")),
            ("GET", format!("{b}/m-1/checkin/code")),
            ("GET", format!("{b}/m-1/confirm/allow")),
            ("GET", format!("{b}/m-1/confirm/deny")),
            ("PUT", format!("{b}/m-1/delete/invite")),
            ("GET", format!("{b}/m-1/manual/completed")),
            ("PUT", format!("{b}/m-1/modify/completedtime")),
            ("PUT", format!("{b}/m-1/modify/starttime")),
            ("GET", format!("{b}/m-1/reject")),
            ("DELETE", format!("{b}/m-1")),
            ("POST", format!("{b}/m-1")),
            ("PUT", format!("{b}/m-1")),
        ];
        for (method, path) in cases {
            let (headers, body): (&[(&str, &str)], Body) =
                if matches!(method, "PUT" | "POST") { (JSON, Body::from("{}")) } else { (&[], Body::empty()) };
            let (status, _) = respond_auth(method, &path, headers, body, NON_ADMIN).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    #[tokio::test]
    async fn u2_building_config_room_openmeeting_new_routes_reachable() {
        let b = "/jaxrs/meeting/assemble/control";
        let cases: Vec<(&str, String)> = vec![
            ("POST", format!("{b}/building")),
            ("PUT", format!("{b}/building/b-1")),
            ("DELETE", format!("{b}/building/b-1")),
            ("GET", format!("{b}/building/list/start/0/completed/0")),
            ("GET", format!("{b}/building/list/start/0/completed/0/allmeeting")),
            ("GET", format!("{b}/building/list/start/0/completed/0/room/r-1/meeting/m-1")),
            ("POST", format!("{b}/config")),
            ("GET", format!("{b}/config/system/config/manage")),
            ("POST", format!("{b}/room")),
            ("PUT", format!("{b}/room/r-1")),
            ("DELETE", format!("{b}/room/r-1")),
            ("POST", format!("{b}/room/r-1/photo")),
            ("GET", format!("{b}/openmeeting")),
            ("GET", format!("{b}/meeting/list/m-1/next/5")),
            ("GET", format!("{b}/meeting/list/m-1/prev/5")),
            ("POST", format!("{b}/meeting/list/1/size/20")),
            ("POST", format!("{b}/meeting/list/1/size/20/manage")),
            ("GET", format!("{b}/meeting/list/forward/monthcount/3/all")),
            ("POST", format!("{b}/meeting/list/invite/1/size/20")),
            ("GET", format!("{b}/meeting/list/year/2026/month/8/day/23/r-1")),
        ];
        for (method, path) in cases {
            let needs_body = matches!(method, "PUT" | "POST");
            let (status, _) = respond(method, &path, JSON, if needs_body { Body::from("{}") } else { Body::empty() }).await;
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    // ── 4. IDOR 门禁：缺会话绝不假成功 ───────────────────────────────────────

    #[tokio::test]
    async fn u2_idor_guarded_writes_reject_without_session() {
        let b = "/jaxrs/meeting/assemble/control";
        let cases: Vec<(&str, String)> = vec![
            ("DELETE", format!("{b}/meeting/m-1")),
            ("POST", format!("{b}/meeting/m-1")),
            ("PUT", format!("{b}/meeting/m-1")),
            ("DELETE", format!("{b}/attachment/a-1")),
            ("PUT", format!("{b}/attachment/a-1/update")),
            ("POST", format!("{b}/attachment/create/from/processplatform")),
        ];
        for (method, path) in cases {
            let (headers, body) = (JSON, Body::from("{}"));
            let (status, _) = respond(method, &path, headers, body).await;
            assert_ne!(status, StatusCode::OK, "unguarded write succeeded without session: {method} {path}");
            assert_ne!(status, StatusCode::NOT_FOUND, "route missing: {method} {path}");
        }
    }

    // ── 5. 真实 SQL 行为（PG 可达时执行；不可达时跳过并如实标注） ────────────

    async fn db_ready() -> bool {
        shared::testing::is_db_available().await
    }

    /// IDOR 门禁落库行为：非 owner 删除他人会议必须 403；owner 删除成功；
    /// 幂等二删为 404（NotFound 走 AppError，不是假 success）。
    #[tokio::test]
    async fn u2_meeting_delete_idor_gate() {
        if !db_ready().await {
            eprintln!("SKIP (no PG): u2_meeting_delete_idor_gate");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        let mid = format!("u2-del-{}", uuid::Uuid::new_v4());
        // 自愈清理：移除历史运行残留
        client.execute("DELETE FROM x_meeting WHERE id LIKE 'u2-del-%'", &[]).await.unwrap();
        client
            .execute(
                "INSERT INTO x_meeting (id, title, start_time, end_time, creator) VALUES ($1,'t',NOW(),NOW(),$2)",
                &[&mid, &"someone-else@u2@P"],
            )
            .await
            .unwrap();

        let app = crate::router(pool.clone());
        let path = format!("/jaxrs/meeting/assemble/control/meeting/{mid}");
        // 非 owner（且非管理员）→ 403
        let (st, _) = respond_db("DELETE", &path, &[], Body::empty(), NON_ADMIN).await;
        assert_eq!(st, StatusCode::FORBIDDEN, "non-owner delete must be 403");
        // owner → 成功删除
        let (st, json) = respond_db("DELETE", &path, &[], Body::empty(), ADMIN).await;
        assert_eq!(st, StatusCode::OK, "admin delete should pass gate: {json}");
        assert_eq!(json["data"]["deleted"], true);
        // 二删 → 404（资源不存在，fail loud）
        let (st, _) = respond_db("DELETE", &path, &[], Body::empty(), ADMIN).await;
        assert_eq!(st, StatusCode::NOT_FOUND);
    }

    /// 归一化查重 + admin 门禁落库：非 admin 创建被拒（403）；admin 创建成功；
    /// 归一化同名（大小写/空白变体）再建被拒（400 冲突）。
    #[tokio::test]
    async fn u2_room_create_dedup_and_admin_gate() {
        if !db_ready().await {
            eprintln!("SKIP (no PG): u2_room_create_dedup_and_admin_gate");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        let name = format!("U2 Room {}", &uuid::Uuid::new_v4().to_string()[..8]);
        let path = "/jaxrs/meeting/assemble/control/room";

        // 非 admin → 403（Java buildingEditAvailable ≈ manager/MeetingManager ≈ is_admin）
        let (st, _) = respond_db(
            "POST", path, JSON,
            Body::from(format!(r#"{{"name":"{name}"}}"#)),
            NON_ADMIN,
        ).await;
        assert_eq!(st, StatusCode::FORBIDDEN, "non-admin room create must be 403");

        // admin 创建成功
        let (st, json) = respond_db(
            "POST", path, JSON,
            Body::from(format!(r#"{{"name":"{name}","capacity":10}}"#)),
            ADMIN,
        ).await;
        assert_eq!(st, StatusCode::OK, "admin create must succeed: {json}");
        let rid = json["data"]["id"].as_str().unwrap().to_string();

        // 归一化变体同名 → 400 冲突（查重生效）
        let variant = name.to_uppercase();
        let (st, _) = respond_db(
            "POST", path, JSON,
            Body::from(format!(r#"{{"name":"  {variant} "}}"#)),
            ADMIN,
        ).await;
        assert_eq!(st, StatusCode::BAD_REQUEST, "normalized duplicate must be rejected");

        client.execute("DELETE FROM x_meeting_room WHERE id = $1", &[&rid]).await.unwrap();
    }

    /// config upsert 往返：save 写入 → manage 读视图可见 → 再 save 更新值。
    #[tokio::test]
    async fn u2_config_save_upsert_roundtrip() {
        if !db_ready().await {
            eprintln!("SKIP (no PG): u2_config_save_upsert_roundtrip");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        let key = format!("u2.test.{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let path = "/jaxrs/meeting/assemble/control/config";

        let (st, json) = respond_db(
            "POST", path, JSON,
            Body::from(format!(r#"{{"configKey":"{key}","configValue":"v1"}}"#)),
            ADMIN,
        ).await;
        assert_eq!(st, StatusCode::OK, "save must succeed: {json}");

        let rows = client
            .query_opt("SELECT config_value FROM x_meeting_config WHERE config_key = $1", &[&key])
            .await
            .unwrap()
            .expect("row must exist");
        assert_eq!(rows.get::<_, String>("config_value"), "v1");

        // upsert 更新而非新增第二行
        let (st, _) = respond_db(
            "POST", path, JSON,
            Body::from(format!(r#"{{"configKey":"{key}","configValue":"v2"}}"#)),
            ADMIN,
        ).await;
        assert_eq!(st, StatusCode::OK);
        let cnt: i64 = client
            .query_one("SELECT COUNT(*) AS n FROM x_meeting_config WHERE config_key = $1", &[&key])
            .await
            .unwrap()
            .get("n");
        assert_eq!(cnt, 1, "upsert must not duplicate rows");

        client.execute("DELETE FROM x_meeting_config WHERE config_key = $1", &[&key]).await.unwrap();
    }

    /// room setPhoto：multipart 上传落 x_meeting_room_photo 行，
    /// 既有 GET room/{id}/photo 视图可回读。
    #[tokio::test]
    async fn u2_room_set_photo_persists_row() {
        if !db_ready().await {
            eprintln!("SKIP (no PG): u2_room_set_photo_persists_row");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        let rid = format!("u2-photo-{}", uuid::Uuid::new_v4());
        // 自愈清理：移除历史运行残留
        client.execute("DELETE FROM x_meeting_room_photo WHERE room_id LIKE 'u2-photo-%'", &[]).await.unwrap();
        client.execute("DELETE FROM x_meeting_room WHERE id LIKE 'u2-photo-%'", &[]).await.unwrap();

        client
            .execute(
                "INSERT INTO x_meeting_room (id, name) VALUES ($1, 'u2-photo-room')",
                &[&rid],
            )
            .await
            .unwrap();

        let path = format!("/jaxrs/meeting/assemble/control/room/{rid}/photo");
        let (st, json) = respond_db("POST", &path, MP, multipart_body("pic.png"), ADMIN).await;
        assert_eq!(st, StatusCode::OK, "photo upload should succeed: {json}");

        let cnt: i64 = client
            .query_one("SELECT COUNT(*) AS n FROM x_meeting_room_photo WHERE room_id = $1", &[&rid])
            .await
            .unwrap()
            .get("n");
        assert!(cnt >= 1, "photo row must persist with room_id linkage");

        client.execute("DELETE FROM x_meeting_room_photo WHERE room_id = $1", &[&rid]).await.unwrap();
        client.execute("DELETE FROM x_meeting_room WHERE id = $1", &[&rid]).await.unwrap();
    }

    /// 附件引用生命周期：processplatform 引用创建 → 按 meeting 列表可见。
    #[tokio::test]
    async fn u2_attachment_reference_lifecycle() {
        if !db_ready().await {
            eprintln!("SKIP (no PG): u2_attachment_reference_lifecycle");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        // 自愈清理：先移除历史运行可能残留的同前缀数据，保证测试幂等
        client
            .execute("DELETE FROM x_meeting_attachment WHERE meeting_id LIKE 'u2-ref-%'", &[])
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_meeting WHERE id LIKE 'u2-ref-%'", &[])
            .await
            .unwrap();
        let mid = format!("u2-ref-{}", uuid::Uuid::new_v4());
        client
            .execute(
                "INSERT INTO x_meeting (id, title, start_time, end_time, creator) VALUES ($1,'t',NOW(),NOW(),'x')",
                &[&mid],
            )
            .await
            .unwrap();

        let b = "/jaxrs/meeting/assemble/control/attachment";
        let (st, json) = respond_db(
            "POST",
            &format!("{b}/create/from/processplatform"),
            JSON,
            Body::from(format!(r#"{{"meetingId":"{mid}","title":"ref-doc"}}"#)),
            NON_ADMIN,
        ).await;
        assert_eq!(st, StatusCode::OK, "reference create should succeed: {json}");

        let (st, json) = respond_db("GET", &format!("{b}/list/meeting/{mid}"), &[], Body::empty(), NON_ADMIN).await;
        assert_eq!(st, StatusCode::OK);
        assert!(json["data"]["data"].as_array().map(|a| !a.is_empty()).unwrap_or(false), "listed attachments must be non-empty: {json}");

        client.execute("DELETE FROM x_meeting_attachment WHERE meeting_id = $1", &[&mid]).await.unwrap();
        client.execute("DELETE FROM x_meeting WHERE id = $1", &[&mid]).await.unwrap();
    }
}
