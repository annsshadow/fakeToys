// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 匿名写禁令守卫（四轮 §三.7 实测面）。
//!
//! 尺子（`scripts/audit_write_gating.mjs`）本轮修掉三处口径缺陷后，「匿名可达」这件事
//! 第一次有了**双锁**定义：请求要能不带会话打到 handler，必须同时满足
//! ① `auth_middleware` 按 `AUTH_EXEMPT_PATHS` / `AUTH_EXEMPT_PREFIXES` 早退（不插 Session），
//! ② `authorize_middleware` 在权限注册表上命中 `PermissionLevel::Public` 而早退
//!    （否则 `request.extensions()::<Session>` 取不到就是 401，rbac.rs:542-544）。
//!
//! 静态推断在这里连吃三次假阳性（`invoice_upload` 只是第一把锁被 `{id}` 通配段吞掉、
//! `custom_edit` 走 `resolve_current_person_unique` 手工解析、`register` 是路由配对越界配错路径），
//! 所以本文件把结论改成**实测**：每张目标表取 before/after 行数，匿名打一次，要求行数一动不动。
//! 状态码逐臂 println 出真实读数供账本引用。
//!
//! ## 本轮实测纠正的两条静态推断
//! - `POST /api/general/assemble/control/invoice/{upload,create}`、
//!   `POST /api/meeting/assemble/control/meeting/create`：**第一把锁**确实被
//!   `AUTH_EXEMPT_PATHS` 里同层 `{id}` 通配段吞开（尺子 6a 的碰撞面），但注册表没有这些
//!   前缀 → **第二把锁兜住**，实测三把全 401、零写入。
//! - `POST/PUT /api/person/icon[/upload]`：**两把锁全开**（`/api/person/icon` 既是豁免前缀
//!   constants.rs:452，又注册成 Public 前缀 rbac.rs:212——公开**读**头像的前缀把同前缀的
//!   **写**路由一起放行了），唯一防线是 handler 体内的会话门禁。所以这一族必须用
//!   **真 multipart** 打：`Multipart::from_request`（axum 0.8.9 :74-81）只解析 boundary、
//!   不读体，因此 401 是门禁给的、不是 content-type 撞错给的——这才是「门禁有效」的证明。
//!   顺带证伪了「匿名可在门禁前消耗请求体」这条假设。
//!
//! ## 覆盖格
//! - invoice `upload`/`create`（含 creator 冒名体）、meeting `create`——通配碰撞 + 第二把锁。
//! - `PUT /api/person/custom/{name}`——豁免 pattern + Public 前缀全开，靠体内门禁。
//! - `POST /api/person/icon/upload`（personal，写 `auth_person.icon` base64）、
//!   `PUT /api/person/icon`（personal_extend，**先落盘 `data/avatar/{uuid}.{ext}`** 再更库）
//!   ——同时钉库面和文件系统面。
//! - `POST /api/reset/password/anonymous`、`POST /api/secret/set`——设计上匿名可达
//!   （前者必须过验证码，后者只在未初始化窗口豁免），这一格证明「匿名可达 ≠ 匿名可写」。
//! - `GET /api/secret/set/cancel`——初始化密钥的**销毁侧**，必须与 `set` 同锁（只在未
//!   初始化窗口匿名）。它曾因 `SECRET_INIT_PATHS` 少列一条而永久匿名可达，本臂红过。
//!
//! ## 仪器自证（末尾）
//! 真插一行让行数出现 1 的差、真建一个 `data/avatar` 文件让目录计数变化，
//! 证明这两把尺子不是恒真——否则上面所有断言毫无意义。
//!
//! 需要真实 PostgreSQL：
//! `cargo test --test anonymous_write_guard -- --ignored --nocapture`
#[cfg(test)]
mod anonymous_write_guard {
    use axum::body::Body;
    use axum::http::{header, Method, Request, StatusCode};
    use deadpool_postgres::Pool;
    use serde_json::json;
    use tower::ServiceExt;

    /// multipart 探针的文件字段（与 `body` 互斥）
    struct MultipartFile {
        field: &'static str,
        filename: &'static str,
        content_type: &'static str,
        /// 真实 PNG magic bytes 开头，确保能通过 handler 侧校验路径而不被 content-type 早拒
        content: &'static [u8],
    }

    const PNG_MAGIC: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
    ];

    struct Probe {
        method: Method,
        uri: &'static str,
        body: Option<serde_json::Value>,
        file: Option<MultipartFile>,
        table: &'static str,
        filter_col: &'static str,
        filter_val: &'static str,
        /// 是否同时统计 `data/avatar` 目录条目数（头像上传族的文件系统写面）
        avatar_dir: bool,
        /// 设计上允许匿名可达（此时只要求「零写入」，不要求 401/403）
        design_anonymous: bool,
    }

    async fn count(pool: &Pool, table: &str, col: &str, val: &str) -> i64 {
        let client = pool.get().await.unwrap();
        // 表名/列名来自本文件常量，非请求输入
        let sql = format!("SELECT COUNT(*) FROM {table} WHERE {col} = $1");
        let row = client
            .query_one(&sql, &[&val.to_string()])
            .await
            .unwrap_or_else(|e| panic!("计数尺读 {table} 失败（表是否存在？）：{e}"));
        row.get::<_, i64>(0)
    }

    async fn purge(pool: &Pool, table: &str, col: &str, val: &str) {
        let client = pool.get().await.unwrap();
        let sql = format!("DELETE FROM {table} WHERE {col} = $1");
        let _ = client.execute(&sql, &[&val.to_string()]).await;
    }

    /// `data/avatar` 目录条目数；目录不存在视为 0（写侧首次上传才创建）
    async fn avatar_entries() -> usize {
        match tokio::fs::read_dir("data/avatar").await {
            Ok(mut rd) => {
                let mut n = 0usize;
                while rd.next_entry().await.unwrap().is_some() {
                    n += 1;
                }
                n
            }
            Err(_) => 0,
        }
    }

    /// 匿名 multipart 请求：真实 boundary + 文件字段，走完整 handler 取体链
    async fn send_anon_multipart(
        app: &axum::Router,
        method: Method,
        uri: &str,
        f: &MultipartFile,
    ) -> (StatusCode, String) {
        const B: &str = "r48anonboundary";
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("--{B}\r\n").as_bytes());
        buf.extend_from_slice(
            format!(r#"Content-Disposition: form-data; name="{}"; filename="{}""#, f.field, f.filename)
                .as_bytes(),
        );
        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(format!("Content-Type: {}\r\n\r\n", f.content_type).as_bytes());
        buf.extend_from_slice(f.content);
        buf.extend_from_slice(format!("\r\n--{B}--\r\n").as_bytes());

        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={B}"),
            )
            .body(Body::from(buf))
            .unwrap();
        // 不带 Authorization / Cookie / token：纯匿名
        let resp = app.clone().oneshot(req).await.unwrap();
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), 4096)
            .await
            .unwrap_or_default();
        (status, String::from_utf8_lossy(&bytes).to_string())
    }

    async fn send_anon_json(
        app: &axum::Router,
        method: Method,
        uri: &str,
        body: &serde_json::Value,
    ) -> (StatusCode, serde_json::Value) {
        shared::testing::send_request(app, method, uri, Some(body.clone()), None, None).await
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn anonymous_requests_cannot_write() {
        if !shared::testing::is_db_available().await {
            println!("skip: PostgreSQL 不可达，匿名写禁令实测未执行");
            return;
        }
        let pool = shared::testing::test_pool();
        let session_manager = shared::session::SessionManager::with_pool(pool.clone());
        let sm_for_control = session_manager.clone();
        let rate_limiter = shared::rate_limit::RateLimiter::new();
        let app = oa4rust::create_app(pool.clone(), session_manager, rate_limiter)
            .await
            .expect("create_app 在测试环境必须构建成功");

        let probes = [
            Probe {
                method: Method::POST,
                uri: "/api/general/assemble/control/invoice/upload",
                body: Some(json!({ "name": "r48-anon-upload", "creator": "r48-spoofed-actor" })),
                file: None,
                table: "x_general_assemble_invoice",
                filter_col: "name",
                filter_val: "r48-anon-upload",
                avatar_dir: false,
                design_anonymous: false,
            },
            Probe {
                method: Method::POST,
                uri: "/api/general/assemble/control/invoice/create",
                body: Some(json!({ "name": "r48-anon-create", "creator": "r48-spoofed-actor" })),
                file: None,
                table: "x_general_assemble_invoice",
                filter_col: "name",
                filter_val: "r48-anon-create",
                avatar_dir: false,
                design_anonymous: false,
            },
            Probe {
                method: Method::POST,
                uri: "/api/meeting/assemble/control/meeting/create",
                body: Some(json!({ "title": "r48-anon-meeting" })),
                file: None,
                table: "x_meeting",
                filter_col: "title",
                filter_val: "r48-anon-meeting",
                avatar_dir: false,
                design_anonymous: false,
            },
            Probe {
                method: Method::PUT,
                uri: "/api/person/custom/r48anon",
                body: Some(json!("r48-anon-value")),
                file: None,
                table: "x_custom",
                filter_col: "name",
                filter_val: "r48anon",
                avatar_dir: false,
                design_anonymous: false,
            },
            // personal：两把锁全开，体内 resolve_current_person_unique 是唯一防线
            Probe {
                method: Method::POST,
                uri: "/api/person/icon/upload",
                body: None,
                file: Some(MultipartFile {
                    field: "file",
                    filename: "r48-anon.png",
                    content_type: "image/png",
                    content: PNG_MAGIC,
                }),
                table: "auth_person",
                filter_col: "unique_id",
                filter_val: "r48-anon-nonexistent-person",
                avatar_dir: false,
                design_anonymous: false,
            },
            // personal_extend：先写 data/avatar 文件再 UPDATE，文件系统面也要钉
            Probe {
                method: Method::PUT,
                uri: "/api/person/icon",
                body: None,
                file: Some(MultipartFile {
                    field: "file",
                    filename: "r48-anon.png",
                    content_type: "image/png",
                    content: PNG_MAGIC,
                }),
                table: "auth_person",
                filter_col: "unique_id",
                filter_val: "r48-anon-nonexistent-person",
                avatar_dir: true,
                design_anonymous: false,
            },
            // 设计上匿名可达：写库前必须过验证码，所以仍然应当零写入
            Probe {
                method: Method::POST,
                uri: "/api/reset/password/anonymous",
                body: Some(json!({
                    "credential": "r48-anon-nonexistent-person",
                    "code": "000000",
                    "password": "r48anonpw123"
                })),
                file: None,
                table: "auth_person",
                filter_col: "unique_id",
                filter_val: "r48-anon-nonexistent-person",
                avatar_dir: false,
                design_anonymous: true,
            },
            // 设计上条件匿名：仅未初始化窗口豁免；已初始化必须被 has_person 兜住
            Probe {
                method: Method::POST,
                uri: "/api/secret/set",
                body: Some(json!({ "secret": "r48-anon-secret" })),
                file: None,
                table: "auth_person",
                filter_col: "unique_id",
                filter_val: "r48-anon-nonexistent-person",
                avatar_dir: false,
                design_anonymous: true,
            },
        ];

        // secret_config 现场（`/api/secret/set` 的真实写目标）：取密文原文做 before/after 逐字比较
        let client = pool.get().await.unwrap();
        let secret_before: Option<String> = client
            .query_opt(
                "SELECT secret_encrypted FROM secret_config WHERE id = $1",
                &[&"init-secret".to_string()],
            )
            .await
            .unwrap()
            .map(|r| r.get("secret_encrypted"));
        let has_person: i64 = client
            .query_one("SELECT COUNT(*) FROM auth_person WHERE locked = false AND deleted_at IS NULL", &[])
            .await
            .unwrap()
            .get(0);
        println!(
            "secret_config 现场：已存密钥行={} 系统在册可用用户={}",
            secret_before.is_some(),
            has_person
        );

        let mut statuses: Vec<(&'static str, StatusCode)> = Vec::new();

        for p in &probes {
            // 先读现场：若上一轮真的被匿名写进去过，这里就该红，不能靠清场把证据抹掉
            let before = count(&pool, p.table, p.filter_col, p.filter_val).await;
            assert_eq!(
                before, 0,
                "{} 的探针行 {}={} 在本轮请求之前已存在（历史匿名写入残留）",
                p.uri, p.table, p.filter_val
            );
            let dir_before = if p.avatar_dir { avatar_entries().await } else { 0 };

            let (status, desc) = if let Some(f) = &p.file {
                let (s, text) = send_anon_multipart(&app, p.method.clone(), p.uri, f).await;
                (s, text)
            } else {
                let body = p.body.as_ref().expect("JSON 探针必须有 body");
                let (s, v) = send_anon_json(&app, p.method.clone(), p.uri, body).await;
                (s, v.to_string())
            };

            let after = count(&pool, p.table, p.filter_col, p.filter_val).await;
            let dir_after = if p.avatar_dir { avatar_entries().await } else { 0 };
            println!(
                "匿名实测 {:>6} {:<52} → HTTP {} 行数 {}→{} 头像目录 {}{}  {}",
                p.method.as_str(),
                p.uri,
                status,
                before,
                after,
                dir_before,
                if p.avatar_dir {
                    format!("→{dir_after}")
                } else {
                    "（未涉及）".to_string()
                },
                desc
            );
            statuses.push((p.uri, status));
            assert_eq!(
                before, after,
                "匿名请求改动了 {}：第二把锁（注册表 Public 早退）或 handler 门禁失效",
                p.uri
            );
            if p.avatar_dir {
                assert_eq!(
                    dir_before, dir_after,
                    "匿名请求往 data/avatar 落了文件：头像上传的体内会话门禁失效（匿名任意文件写）"
                );
            }
            if !p.design_anonymous {
                assert!(
                    status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN,
                    "{} 应被门禁挡成 401/403，实测 {}（500=Extension extractor 泄漏，非 2xx 但同样是门禁缺陷）",
                    p.uri,
                    status
                );
            }
            // 万一真被写入，收尾把探针行清掉，避免污染下一轮（判据已经在上面红过）
            purge(&pool, p.table, p.filter_col, p.filter_val).await;
        }

        // /api/secret/set 的写目标必须逐字不变
        let secret_after: Option<String> = pool
            .get()
            .await
            .unwrap()
            .query_opt(
                "SELECT secret_encrypted FROM secret_config WHERE id = $1",
                &[&"init-secret".to_string()],
            )
            .await
            .unwrap()
            .map(|r| r.get("secret_encrypted"));
        println!(
            "匿名 /api/secret/set 后 secret_config 行：{} → {}",
            secret_before.as_deref().unwrap_or("<无行>"),
            secret_after.as_deref().unwrap_or("<无行>")
        );
        assert_eq!(
            secret_before, secret_after,
            "匿名 /api/secret/set 改动了初始化密钥（has_person 兜底或 SECRET_INIT_PATHS 条件豁免失效）"
        );
        // 条件豁免的诚实口径：`secret_before == secret_after` 在「库里根本没有密钥行」时
        // 会退化（None == None 恒成立，读不到“写不了”这件事）。因此这一格的主判据是状态码：
        // 系统已有在册用户 ⇒ `system_uninitialized()` 必须为假 ⇒ 豁免不生效 ⇒ 必须 401/403。
        let secret_status = statuses
            .iter()
            .find(|(uri, _)| *uri == "/api/secret/set")
            .map(|(_, s)| *s)
            .expect("探针名单里必须有 /api/secret/set");
        if has_person > 0 {
            assert!(
                secret_status == StatusCode::UNAUTHORIZED || secret_status == StatusCode::FORBIDDEN,
                "系统已有 {has_person} 个在册可用用户，/api/secret/set 的匿名豁免必须失效，实测 {secret_status}"
            );
        } else {
            println!("警告：测试库无在册用户，/api/secret/set 处于设计上的未初始化匿名窗口，本臂只验零写入");
        }

        // ── 销毁侧同锁检验：GET /api/secret/set/cancel ──────────────────────────
        // 它是 `set` 的销毁兄弟，但 `SECRET_INIT_PATHS` 只列了 `/api/secret/check` 与
        // `/api/secret/set`，而 `path_matches` 要求**段数相等**（`/api/secret/set` 不吞
        // `/api/secret/set/cancel`）⇒ 这条路径不在条件豁免名单内，注册表又判它 Public，
        // 于是系统初始化完成后**仍然匿名可达**，销毁初始化密钥只剩 handler 体内一道
        // `has_person`。全库用户被锁定/软删（维护态）时该兜底失效，匿名一个 GET 即可
        // `DELETE FROM secret_config WHERE id='init-secret'`。
        // 尺子要两维都能红：测试库通常没有密钥行（上面读到 `<无行> → <无行>`，逐字比较会
        // 退化成 None == None 的恒真），所以这里插一只哨兵行让「被删」读得到；打完无条件
        // 复原——哨兵是本测试造的，删掉它不算破坏现场。
        const SECRET_ROW: &str = "init-secret";
        const SENTINEL: &str = "r48-cancel-sentinel";
        let client2 = pool.get().await.unwrap();
        // 先清自己上一轮的红退出残留（`SENTINEL` 只可能由本测试写入，删它不碰真实现场），
        // 否则一次 panic 退出就会把哨兵行永久留在 secret_config 里、下一轮误读成「既有现场」。
        let stale = client2
            .execute(
                "DELETE FROM secret_config WHERE secret_encrypted = $1",
                &[&SENTINEL.to_string()],
            )
            .await
            .unwrap();
        if stale > 0 {
            println!("注意：清掉上一轮匿名销毁臂的哨兵残留 {stale} 行");
        }
        let existed_before: bool = client2
            .query_opt("SELECT 1 FROM secret_config WHERE id = $1", &[&SECRET_ROW.to_string()])
            .await
            .unwrap()
            .is_some();
        if !existed_before {
            client2
                .execute(
                    "INSERT INTO secret_config (id, secret_encrypted, created_at, updated_at) \
                     VALUES ($1, $2, NOW(), NOW())",
                    &[&SECRET_ROW.to_string(), &SENTINEL.to_string()],
                )
                .await
                .unwrap();
        }
        let (cancel_status, cancel_body) =
            send_anon_json(&app, Method::GET, "/api/secret/set/cancel", &json!({})).await;
        let row_after_cancel: bool = client2
            .query_opt("SELECT 1 FROM secret_config WHERE id = $1", &[&SECRET_ROW.to_string()])
            .await
            .unwrap()
            .is_some();
        println!(
            "匿名实测 {:<6} {:<38} → HTTP {}  密钥行 {} → {}（哨兵{}） {}",
            "GET",
            "/api/secret/set/cancel",
            cancel_status,
            existed_before,
            row_after_cancel,
            if existed_before { "为既有现场" } else { "由本测试插入" },
            cancel_body
        );
        if has_person > 0 {
            assert!(
                cancel_status == StatusCode::UNAUTHORIZED
                    || cancel_status == StatusCode::FORBIDDEN,
                "系统已有 {has_person} 个在册可用用户，/api/secret/set/cancel 必须与 set 同锁（仅初始化窗口匿名），实测 {cancel_status}"
            );
            assert!(
                row_after_cancel,
                "匿名 GET /api/secret/set/cancel 真的删掉了初始化密钥行（销毁侧未与 set 同锁）"
            );
        } else {
            println!("警告：测试库无在册用户，/api/secret/set/cancel 处于设计上的未初始化匿名窗口，本臂只验哨兵行不被删");
        }
        if !existed_before {
            client2
                .execute(
                    "DELETE FROM secret_config WHERE id = $1 AND secret_encrypted = $2",
                    &[&SECRET_ROW.to_string(), &SENTINEL.to_string()],
                )
                .await
                .unwrap();
        }

        // ── 仪器自证 ──────────────────────────────────────────────────────────
        // 1) 行数尺：真插一行 → count 必须读到 1，清掉后必须回 0
        client
            .execute(
                "INSERT INTO x_general_assemble_invoice (id, name, flag, status, creator, create_time) \
                 VALUES ($1, $2, $3, 'draft', $4, NOW())",
                &[
                    &"r48-instrument-control".to_string(),
                    &"r48-instrument".to_string(),
                    &"r48-flag".to_string(),
                    &"r48-self".to_string(),
                ],
            )
            .await
            .unwrap();
        let hit = count(&pool, "x_general_assemble_invoice", "name", "r48-instrument").await;
        let _ = client
            .execute(
                "DELETE FROM x_general_assemble_invoice WHERE id = $1",
                &[&"r48-instrument-control".to_string()],
            )
            .await;
        let miss = count(&pool, "x_general_assemble_invoice", "name", "r48-instrument").await;
        println!("行数尺自证：真插一行后 count {hit}→{miss}");
        assert_eq!(hit, 1, "计数尺读不到真实写入，说明本文件所有断言恒真");
        assert_eq!(miss, 0, "清场失败，残留会污染下一轮");

        // 2) 头像目录尺：真建一个文件 → 计数必须 +1，删掉后必须复原
        tokio::fs::create_dir_all("data/avatar").await.unwrap();
        let base = avatar_entries().await;
        let ctl = std::path::Path::new("data/avatar").join("r48-instrument.png");
        tokio::fs::write(&ctl, PNG_MAGIC).await.unwrap();
        let up = avatar_entries().await;
        let _ = tokio::fs::remove_file(&ctl).await;
        let back = avatar_entries().await;
        println!("头像目录尺自证：{base} → {up} → {back}");
        assert_eq!(up, base + 1, "头像目录尺读不到真实落盘文件，匿名文件写探针恒真");
        assert_eq!(back, base, "头像目录尺清场失败，残留会污染下一轮");

        // 3) 放行正控：同一条路由带**真会话**必须插得进去，且 creator 必须取会话权威而不是请求体。
        //    没有这一条，前面所有「匿名 401 + 零行」都可能是「这条路由本来就写不进测试库」的假绿。
        let actor: String = client
            .query_opt(
                "SELECT unique_id FROM auth_person \
                 WHERE locked = false AND deleted_at IS NULL LIMIT 1",
                &[],
            )
            .await
            .unwrap()
            .map(|r| r.get("unique_id"))
            .expect("测试库必须有至少一个在册用户，否则放行正控无法执行");
        let token = "r48-anon-guard-control-token".to_string();
        sm_for_control
            .create_session(actor.clone(), token.clone())
            .await
            .expect("放行正控需要能造会话");

        let ctl_name = "r48-positive-control";
        let (ctl_status, ctl_body) = shared::testing::send_request(
            &app,
            Method::POST,
            "/api/general/assemble/control/invoice/create",
            Some(json!({ "name": ctl_name, "creator": "r48-spoofed-actor" })),
            Some(&token),
            None,
        )
        .await;
        let stored_creator: Option<String> = client
            .query_opt(
                "SELECT creator FROM x_general_assemble_invoice WHERE name = $1",
                &[&ctl_name.to_string()],
            )
            .await
            .unwrap()
            .map(|r| r.get("creator"));
        let _ = client
            .execute(
                "DELETE FROM x_general_assemble_invoice WHERE name = $1",
                &[&ctl_name.to_string()],
            )
            .await;
        let _ = sm_for_control.remove_session(&token).await;
        println!(
            "放行正控：会话 {} 打 invoice/create → HTTP {} 落库 creator={:?} 响应 {}",
            actor, ctl_status, stored_creator, ctl_body
        );
        assert_eq!(
            stored_creator.as_deref(),
            Some(actor.as_str()),
            "带真会话必须写得进去且 creator 记的是会话权威；实测落库 creator={stored_creator:?}（None=匿名臂的零写入可能是假绿，r48-spoofed-actor=请求体冒名值被采信）",
        );
    }
}
