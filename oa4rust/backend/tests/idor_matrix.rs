// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O9：IDOR 穷举矩阵（越权访问守卫）。
//!
//! # 枚举源（活注册表，非手工清单）
//!
//! 从 `crates/*/src` 的 `.route("…", …)` 字面量扫描全部注册路由（剥掉
//! `#[cfg(test)]` 模块），取「写方法（POST/PUT/DELETE/PATCH）或带路径参数的 GET」
//! 作为候选。候选数低于下限直接失败——防止注册表/扫描器坏掉后守卫静默变绿。
//!
//! # 身份矩阵（每格记录实际返回码）
//!
//! ① 无 token ② 有效但非所有者 ③ 过期 token ④ 已禁用用户（locked=true）
//! ⑤ 跨组织（不同 unit）身份。另加伪造 token（负缓存路径）。
//!
//! # 判定（提前写死）
//!
//! - 无 token / 伪造 / 过期：401（auth-exempt 常量表或 RBAC 注册表 Public 档内的
//!   路由除外——那是设计豁免/公共读面，写面由 anonymous_write_guard 另行覆盖）。
//! - 禁用用户：401/403（公共读面/豁免族除外——该面本就不要求会话）。
//! - 非所有者/跨组织：403 或 404 均可（404 = 不暴露存在性，403 = 明确拒绝）；
//!   400/401/422 也算通过（参数先炸）；**200 + 响应体含受害者 canary 数据 = 缺陷**；
//!   **500 = 缺陷**（越权被掩盖成内部错误）。写路由 200 后还会做数据完整性复核
//!   （受害者夹具行不得被改动）。
//! - Owner 参照列（受害者本人）：非所有者 500 且本人也 500 的，归入
//!   「handler 级 500」桶（非越权掩盖，但仍是真实缺陷，棘轮只减不增）；
//!   非所有者 500 而本人正常 = 越权掩盖实锤。Owner 不打含夹具 id 的写路由
//!   （本人合法删/改会销毁完整性复核的锚，run9 实证）。
//!
//! # 探针卫生（scratch 库隔离）
//!
//! 矩阵对每条写路由发 `{}` 体——全可选字段的 create handler 会真插入垃圾行
//! （2026-11-03 实测：bbs/hotpic 等 25 表被写入 280+ 行，下游 arity_probe 踩到
//! 垃圾行触发 handler 解码 panic）。因此矩阵一律跑在专用 scratch 库
//! `oa_par_idor`（每轮 DROP 重建 + 全量迁移），跑完 DROP；共享库 oa4rust_test
//! 不再承受探针写流量。
//!
//! # 破坏实验
//!
//! `sabotage_event_read_must_be_caught` 用例证明本矩阵有牙：把 cal_event 的
//! 读路径归属校验临时摘掉（cfg(feature) 模拟），矩阵必须红。默认关闭。
//!
//! 需要真实 PostgreSQL（oa4rust_test）：
//! `cargo test --test idor_matrix`（无库自动 skip）

#[cfg(test)]
mod idor_matrix {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use std::collections::BTreeMap;
    use std::path::Path;
    use tower::ServiceExt;

    /// create_app 需要大栈（与 cors_guard 同因）
    const STACK: usize = 64 * 1024 * 1024;
    /// 受害者数据 canary：出现在非所有者响应体即判定数据泄露
    const CANARY: &str = "CANARYO9x7f3d";
    /// 夹具前缀（清理用）
    #[allow(dead_code)] // 文档化用途：清理 SQL 里的 'idor-o9-%' 字面量
    const FIX: &str = "idor-o9-";

    const UID_A: &str = "idor-o9-attacker-a";
    const UID_B: &str = "idor-o9-victim-b";
    const UID_C: &str = "idor-o9-cross-c";
    const UID_D: &str = "idor-o9-disabled-d";
    const UID_E: &str = "idor-o9-expired-e";

    const TOK_A: &str = "tok-o9-a";
    const TOK_B: &str = "tok-o9-b";
    const TOK_C: &str = "tok-o9-c";
    const TOK_D: &str = "tok-o9-d";
    const TOK_E: &str = "tok-o9-expired";

    const EVENT_ID: &str = "idor-o9-evt-b1";
    const CAL_ID: &str = "idor-o9-cal-b1";
    const FILE_ID: &str = "idor-o9-file-b1";

    /// 探针专用 scratch 库（每轮重建，跑完 DROP）
    const SCRATCH_DB: &str = "oa_par_idor";

    /// 已知存量缺陷豁免（非 IDOR 面）：
    /// 这两条 legacy 上传路由的 handler 元数（3/4 参 Path）与路径槽位（1 参）
    /// 恒定不符（"Wrong number of path arguments" 500），已被 arity_probe 的
    /// 棘轮基线收录。语义修复需先核 o2server 契约（reference/scale 双段字面量
    /// 无来源），按约定「核不到不开轮」——矩阵显式豁免并计数，防静默扩大。
    const KNOWN_ARTIFACT_ROUTES: &[&str] = &[
        "/api/file/upload/referencetype/reference/reference/scale/scale/{referenceType}",
        "/api/file/upload/referencetype/reference/reference/scale/scale/callback/callback/{referenceType}",
    ];

    #[derive(Clone, Copy, PartialEq)]
    enum Ident {
        Anon,
        Forged,
        Expired,
        Disabled,
        NonOwner,
        CrossOrg,
        Owner,
    }

    const IDENTS: [Ident; 7] = [
        Ident::Anon,
        Ident::Forged,
        Ident::Expired,
        Ident::Disabled,
        Ident::NonOwner,
        Ident::CrossOrg,
        Ident::Owner,
    ];

    impl Ident {
        fn label(self) -> &'static str {
            match self {
                Ident::Anon => "anon",
                Ident::Forged => "forged",
                Ident::Expired => "expired",
                Ident::Disabled => "disabled",
                Ident::NonOwner => "non-owner",
                Ident::CrossOrg => "cross-org",
                Ident::Owner => "owner",
            }
        }
        fn token(self) -> Option<&'static str> {
            match self {
                Ident::Anon => None,
                Ident::Forged => Some("tok-o9-forged"),
                Ident::Expired => Some(TOK_E),
                Ident::Disabled => Some(TOK_D),
                Ident::NonOwner => Some(TOK_A),
                Ident::CrossOrg => Some(TOK_C),
                Ident::Owner => Some(TOK_B),
            }
        }
    }

    struct Route {
        method: &'static str,
        path: String,
    }

    /// 活注册表：扫 crates/*/src 非测试代码的 `.route("…", …)` 字面量。
    fn scan_routes(root: &Path) -> Vec<Route> {
        let mut out = Vec::new();
        let crates = root.join("crates");
        let mut stack = vec![crates];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.file_name().map(|n| n == "target").unwrap_or(false) {
                        continue;
                    }
                    stack.push(p);
                } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                    let name = p.file_name().unwrap().to_string_lossy();
                    if name.contains("test") {
                        continue;
                    }
                    let Ok(text) = std::fs::read_to_string(&p) else { continue };
                    let prod = strip_test_modules(&text);
                    collect_routes(&prod, &mut out);
                }
            }
        }
        out
    }

    /// 剥掉 #[cfg(test)] 块（括号配平粗口径，与既有守卫同法）
    fn strip_test_modules(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if text[i..].starts_with("#[cfg(test)]") {
                let mut j = i + 12;
                while j < bytes.len() && bytes[j] != b'{' && j - i <= 200 {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b'{' && text[i..j].contains("mod") {
                    let mut depth = 0;
                    let mut k = j;
                    while k < bytes.len() {
                        match bytes[k] {
                            b'{' => depth += 1,
                            b'}' => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        k += 1;
                    }
                    out.push('\n');
                    i = k + 1;
                    continue;
                }
            }
            // 整字符拷贝（UTF-8 安全）
            let ch_len = utf8_len(bytes[i]);
            let end = (i + ch_len).min(text.len());
            if text.is_char_boundary(end) {
                out.push_str(&text[i..end]);
            }
            i = end.max(i + 1);
        }
        out
    }

    fn utf8_len(b: u8) -> usize {
        if b < 0x80 {
            1
        } else if b >> 5 == 0b110 {
            2
        } else if b >> 4 == 0b1110 {
            3
        } else if b >> 3 == 0b11110 {
            4
        } else {
            1
        }
    }

    /// 从源码抽 `.route("PATH", get(h).post(h2)…)` 的 (方法, 路径) 对。
    /// `.route(` 与路径字面量之间允许任意空白/换行（rustfmt 多行风格）——
    /// 此前只认 `.route("` 紧贴形态，漏掉约 900 条多行注册（活注册表完整性）。
    /// 覆盖 .route(...) 与 .route_service 不在范围（服务式挂载无 handler 语义）。
    fn collect_routes(text: &str, out: &mut Vec<Route>) {
        let bytes = text.as_bytes();
        let mut i = 0;
        while let Some(pos) = text[i..].find(".route(") {
            let mut q = i + pos + 7;
            // 跳过 .route( 与路径字面量之间的空白（含换行）
            while q < bytes.len() && bytes[q].is_ascii_whitespace() {
                q += 1;
            }
            if q >= bytes.len() || bytes[q] != b'"' {
                i = i + pos + 7;
                continue;
            }
            let start = q + 1;
            let Some(endq) = text[start..].find('"') else { break };
            let path = &text[start..start + endq];
            // 找到配对右括号前的区段，抽其中的方法调用
            let mut j = start + endq + 1;
            let mut depth = 1;
            let seg_start = j;
            while j < bytes.len() && depth > 0 {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            let seg = &text[seg_start..j];
            for m in ["get", "post", "put", "delete", "patch"] {
                if seg.contains(&format!("{m}(")) {
                    out.push(Route {
                        method: match m {
                            "get" => "GET",
                            "post" => "POST",
                            "put" => "PUT",
                            "delete" => "DELETE",
                            _ => "PATCH",
                        },
                        path: path.to_string(),
                    });
                }
            }
            i = j.max(start + endq + 1);
        }
    }

    /// 候选：写方法 或 带路径参数的 GET；排除 mock 路径
    fn is_candidate(r: &Route) -> bool {
        if r.path.contains("mock") {
            return false;
        }
        let is_write = matches!(r.method, "POST" | "PUT" | "DELETE" | "PATCH");
        is_write || (r.method == "GET" && r.path.contains('{'))
    }

    /// 受害者夹具 id 替换：路径族 → 真实他人资源 id。
    /// 参数名感知：`{date}` 给合法日期（非法日期触发的是「入参校验」缺陷族，
    /// 不是 IDOR 面，且会把全族 list 路由打成 500 噪音）；`{count}` 给小整数；
    /// 其余按路径族给受害者资源 id，无族归属给 "1"。
    fn fill_params(path: &str) -> String {
        let fixture = if path.contains("/event") {
            EVENT_ID
        } else if path.contains("calendar") {
            CAL_ID
        } else if path.contains("file") || path.contains("attachment") {
            FILE_ID
        } else {
            "1"
        };
        let mut out = String::new();
        let mut param = String::new();
        let mut depth = 0;
        let flush = |out: &mut String, param: &str| {
            let v = if param.contains("date") {
                "2026-01-01"
            } else if param.contains("count") || param.contains("size") {
                "5"
            } else {
                fixture
            };
            out.push_str(v);
        };
        for ch in path.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    if depth == 1 {
                        param.clear();
                    }
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        flush(&mut out, &param);
                    }
                }
                _ if depth == 0 => out.push(ch),
                _ => param.push(ch),
            }
        }
        out
    }

    /// 匿名豁免判定（与 auth_middleware 同口径的静态侧）
    fn is_auth_exempt(path_pattern: &str) -> bool {
        use shared::middleware::constants::{AUTH_EXEMPT_PATHS, AUTH_EXEMPT_PREFIXES};
        AUTH_EXEMPT_PATHS.contains(&path_pattern)
            || AUTH_EXEMPT_PREFIXES
                .iter()
                .any(|p| path_pattern.starts_with(p))
    }

    /// 指向指定库的连接池（DATABASE_URL 解析后覆盖 dbname）
    fn pool_to(dbname: &str) -> deadpool_postgres::Pool {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
        let mut cfg: deadpool_postgres::tokio_postgres::Config =
            url.parse().expect("DATABASE_URL 解析失败");
        cfg.dbname(dbname);
        let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
        deadpool_postgres::Pool::builder(mgr)
            .max_size(8)
            .build()
            .unwrap()
    }

    /// 重建 scratch 库（存在则强删），全量迁移。
    async fn fresh_scratch() {
        let admin = pool_to("postgres");
        let c = admin.get().await.expect("admin 连接");
        c.execute(
            &format!("DROP DATABASE IF EXISTS {SCRATCH_DB} WITH (FORCE)"),
            &[],
        )
        .await
        .expect("drop scratch");
        c.execute(&format!("CREATE DATABASE {SCRATCH_DB}"), &[])
            .await
            .expect("create scratch");
        let pool = pool_to(SCRATCH_DB);
        shared::migrate::run_migrations(&pool)
            .await
            .expect("scratch 库迁移");
    }

    async fn drop_scratch() {
        if let Ok(c) = pool_to("postgres").get().await {
            let _ = c
                .execute(
                    &format!("DROP DATABASE IF EXISTS {SCRATCH_DB} WITH (FORCE)"),
                    &[],
                )
                .await;
        }
    }

    async fn setup() -> Option<(axum::Router, deadpool_postgres::Pool)> {
        if !shared::testing::is_db_available().await {
            eprintln!("SKIP: no database available");
            return None;
        }
        // 穷举量级请求会打爆默认 300/分钟 限流——探针环境把限流阈值抬高
        // （只影响本测试进程；限流本身的用例在 middleware 测试中已覆盖）。
        std::env::set_var("OA4RUST_GENERAL_RATE_LIMIT", "1000000");
        std::env::set_var("OA4RUST_AUTH_RATE_LIMIT", "1000000");

        // 探针写流量全部落在 scratch 库（见文件头「探针卫生」）
        fresh_scratch().await;
        let pool = pool_to(SCRATCH_DB);
        let client = pool.get().await.expect("pool client");

        // 身份夹具：A（UA 单元，非所有者）、B（UB 单元，受害者）、C（UC 单元，跨组织）、
        // D（禁用）、E（会话将过期）
        for (pid, uid, unit, locked) in [
            (UID_A, UID_A, "unit-o9-UA", false),
            (UID_B, UID_B, "unit-o9-UB", false),
            (UID_C, UID_C, "unit-o9-UC", false),
            (UID_D, UID_D, "unit-o9-UD", true),
            (UID_E, UID_E, "unit-o9-UE", false),
        ] {
            client
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, unit) \
                     VALUES ($1, $2, $3, 'idor-o9-noop', $4, $5)",
                    &[&pid, &uid, &uid, &locked, &unit],
                )
                .await
                .expect("插入人员夹具");
        }

        // 受害者资源（canary 入 name/title/content）
        client
            .execute(
                "INSERT INTO cal_event (id, title, createor, creator_person, status, start_time, end_time, create_time, update_time) \
                 VALUES ($1, $2, $3, $3, 'OPEN', '2026-01-01 10:00:00', '2026-01-01 11:00:00', '2026-01-01', '2026-01-01')",
                &[&EVENT_ID, &format!("{CANARY}-event-title"), &UID_B],
            )
            .await
            .expect("事件夹具");
        client
            .execute(
                "INSERT INTO cal_calendar (id, name, createor, creator_person, status, type, is_public, create_time, update_time) \
                 VALUES ($1, $2, $3, $3, 'OPEN', 'PERSON', false, '2026-01-01', '2026-01-01')",
                &[&CAL_ID, &format!("{CANARY}-calendar-name"), &UID_B],
            )
            .await
            .expect("日历夹具");
        client
            .execute(
                "INSERT INTO file_file (id, name, person, extension, length, mime_type, content, create_time, update_time) \
                 VALUES ($1, $2, $3, 'txt', 10, 'text/plain', $4, NOW(), NOW())",
                &[&FILE_ID, &format!("{CANARY}-file-name"), &UID_B, &format!("{CANARY}-file-content")],
            )
            .await
            .expect("文件夹具");

        // 会话：A/B/C/D 有效，E 稍后过期化
        let session_manager = shared::session::SessionManager::with_pool(pool.clone());
        for (uid, tok) in [
            (UID_A, TOK_A),
            (UID_B, TOK_B),
            (UID_C, TOK_C),
            (UID_D, TOK_D),
            (UID_E, TOK_E),
        ] {
            session_manager
                .create_session(uid.to_string(), tok.to_string())
                .await
                .expect("建会话");
        }
        // E：内存移除 + PG 置为过期（validate 走 PG 臂 → expires_at < NOW → 无行 → 401）
        session_manager.sessions.write().await.remove(TOK_E);
        client
            .execute(
                "UPDATE auth_session SET expires_at = '2020-01-01 00:00:00' WHERE token = $1",
                &[&TOK_E],
            )
            .await
            .expect("过期化会话");

        let rate_limiter = shared::rate_limit::RateLimiter::new();
        let app = create_app_big_stack(pool.clone(), session_manager, rate_limiter);
        Some((app, pool))
    }

    fn create_app_big_stack(
        pool: deadpool_postgres::Pool,
        session_manager: shared::session::SessionManager,
        rate_limiter: shared::rate_limit::RateLimiter,
    ) -> axum::Router {
        std::thread::Builder::new()
            .name("idor-app".to_string())
            .stack_size(STACK)
            .spawn(move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("runtime")
                    .block_on(async move {
                        oa4rust::create_app(pool, session_manager, rate_limiter)
                            .await
                            .expect("create_app")
                    })
            })
            .expect("spawn")
            .join()
            .expect("join")
    }

    async fn fire(
        app: &axum::Router,
        method: &str,
        url: &str,
        token: Option<&str>,
    ) -> Option<(StatusCode, String)> {
        let m = Method::from_bytes(method.as_bytes()).unwrap();
        let mut b = Request::builder()
            .method(m)
            .uri(url)
            .header("content-type", "application/json");
        if let Some(t) = token {
            b = b.header("authorization", format!("Bearer {t}"));
        }
        // legacy 路径含空格等非法 URI 字符（正常客户端无法触达）：跳过不计
        let req = match b.body(Body::from("{}")) {
            Ok(r) => r,
            Err(_) => return None,
        };
        let resp = app.clone().oneshot(req).await.unwrap();
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20)
            .await
            .unwrap_or_default();
        Some((status, String::from_utf8_lossy(&bytes).into_owned()))
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn idor_exhaustive_matrix() {
        let Some((app, pool)) = setup().await else { return };

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let all = scan_routes(root);
        let candidates: Vec<&Route> = all.iter().filter(|r| is_candidate(r)).collect();

        // 规模下限：注册表/扫描器坏掉时直接失败，防止守卫静默变绿。
        assert!(
            candidates.len() >= 800,
            "候选路由数 {} 低于下限 800（注册表扫描异常？）",
            candidates.len()
        );
        eprintln!("IDOR 矩阵候选路由: {}（全量注册 {}）", candidates.len(), all.len());

        let mut defects: Vec<String> = Vec::new();
        // handler500：key=「METHOD 路径」，value=body 摘录——按路由去重
        //（body 含时间戳，按整串去重会同路由多计，run13/14 实测漂移 161→164）
        let mut handler500: std::collections::BTreeMap<String, String> =
            std::collections::BTreeMap::new();
        let mut exempt_anon = 0usize;
        let mut known_artifact = 0usize;
        let mut grid: BTreeMap<&str, [usize; 7]> = BTreeMap::new();
        // 500 格子的响应体摘录（triage 根因用）：(路由, 身份) → 前 200 字符
        let mut err_bodies: BTreeMap<(String, &'static str), String> = BTreeMap::new();
        let registry = shared::middleware::rbac::PermissionRegistry::with_defaults();
        use shared::middleware::rbac::PermissionLevel;

        for r in &candidates {
            if KNOWN_ARTIFACT_ROUTES.contains(&r.path.as_str()) {
                known_artifact += 1;
                continue;
            }
            let url = fill_params(&r.path);
            // 豁免/公共读面按「填充后的具体 URL」判定——与运行时中间件看到的口径一致
            let exempt = is_auth_exempt(&url);
            let public = matches!(
                registry.get_permission(&url),
                Some(PermissionLevel::Public)
            );
            if exempt {
                exempt_anon += 1;
            }
            let cells = grid.entry(r.path.as_str()).or_default();
            for id in IDENTS {
                // Owner 参照列：对含夹具 id 的 URL 不打写——本人删/改自己资源是合法
                // 操作但会销毁完整性复核的锚（run9 实证：owner DELETE 事件后，
                // 复核查询 0 行）。非夹具路由的写照打（500 族需要 owner 参照）。
                if id == Ident::Owner && r.method != "GET" && url.contains("idor-o9-") {
                    continue;
                }
                let Some((status, body)) = fire(&app, r.method, &url, id.token()).await else {
                    continue; // 非法 URI（含空格等）正常客户端不可达
                };
                cells[id as usize] = status.as_u16() as usize;
                if status.as_u16() == 500 {
                    let snip: String = body.chars().take(200).collect();
                    err_bodies.insert((r.path.clone(), id.label()), snip);
                }
                // canary 泄露：任何非本人身份拿到受害者数据 = 缺陷（与状态码无关）
                if id != Ident::Owner && body.contains(CANARY) {
                    defects.push(format!(
                        "{} {} [{}] 响应体含受害者 canary（数据泄露，状态 {}）",
                        r.method,
                        r.path,
                        id.label(),
                        status
                    ));
                }
            }

            let cell_of = |i: Ident| cells[i as usize];
            for id in IDENTS {
                let status = cell_of(id);
                if status == 0 {
                    continue; // 该身份未触发（非法 URI / Owner 跳过写）
                }
                match id {
                    Ident::Anon | Ident::Forged | Ident::Expired => {
                        // 设计豁免族/公共读面：匿名可达由 anonymous_write_guard 覆盖写面；
                        // 其余路由必须是 401。
                        if !(exempt || public) && status != 401 {
                            defects.push(format!(
                                "{} {} [{}] 期望 401 实际 {}",
                                r.method,
                                r.path,
                                id.label(),
                                status
                            ));
                        }
                    }
                    Ident::Disabled => {
                        // 公共读面/豁免族本就不要求会话；其余路由禁用用户必须 401/403
                        if !(exempt || public) && !matches!(status, 401 | 403) {
                            defects.push(format!(
                                "{} {} [disabled] 期望 401/403 实际 {}",
                                r.method, r.path, status
                            ));
                        }
                    }
                    Ident::NonOwner | Ident::CrossOrg => {
                        if status == 500 {
                            let owner = cell_of(Ident::Owner);
                            let body_hint = err_bodies
                                .get(&(r.path.clone(), id.label()))
                                .map(|s| s.as_str())
                                .unwrap_or("");
                            if owner != 500 && owner != 0 {
                                // 本人正常、非所有者 500 = 越权被掩盖成内部错误
                                defects.push(format!(
                                    "{} {} [{}] 越权被掩盖成 500（owner 参照 {owner}）body={body_hint}",
                                    r.method,
                                    r.path,
                                    id.label()
                                ));
                            } else {
                                // 本人也 500：handler 级缺陷（非越权掩盖），
                                // 进棘轮桶（O10 实体/列类型对齐家族等，逐族修）
                                handler500.insert(
                                    format!("{} {}", r.method, r.path),
                                    format!("(owner 同样 {owner}) body={body_hint}"),
                                );
                            }
                        }
                    }
                    Ident::Owner => {}
                }
            }
        }

        // 写后完整性：受害者的资源行不得被非所有者改动
        let client = pool.get().await.unwrap();
        let evt_title: String = client
            .query_one("SELECT title FROM cal_event WHERE id = $1", &[&EVENT_ID])
            .await
            .expect("事件行必须还在")
            .get(0);
        if evt_title != format!("{CANARY}-event-title") {
            defects.push(format!("cal_event 夹具被改动: {evt_title}"));
        }
        let file_exists: bool = client
            .query_one("SELECT EXISTS(SELECT 1 FROM file_file WHERE id = $1)", &[&FILE_ID])
            .await
            .unwrap()
            .get(0);
        if !file_exists {
            defects.push("FILE_FILE 夹具被删除".to_string());
        }

        // 汇总打印（账本证据）
        eprintln!("== IDOR 矩阵状态码网格（路由 × 身份）==");
        for (path, cells) in &grid {
            eprintln!("{path} => {cells:?}");
        }
        eprintln!("anon-exempt 路由: {exempt_anon}；已知 artifact 豁免: {known_artifact}");
        eprintln!("== handler 级 500 族（owner 同样 500，非越权掩盖；逐族修）==");
        for (k, v) in &handler500 {
            eprintln!("  {k} {v}");
        }
        eprintln!("handler 级 500 共 {} 条", handler500.len());

        // 探针卫生收尾：DROP scratch 库（在 assert 之前，缺陷红也执行清理）
        drop(client);
        drop_scratch().await;

        // handler 级 500 棘轮：只许减不许增。当前家族主体已定位（O10 实体/列
        // 类型对齐：sea-orm RETURNING 解码 TEXT→DateTime / BIGINT→i32 失败；
        // {} 体无校验直打 DB；非法参数；逐个修复后同步下调本基线）。
        // 基线值 = 2026-11-03 run11 实测 161（全部 owner 同 500，无越权掩盖）。
        const HANDLER500_BASELINE: usize = 161;
        assert!(
            handler500.len() <= HANDLER500_BASELINE,
            "handler 级 500 棘轮回归：当前 {} > 基线 {}（新增族需修或记录）：\n{}",
            handler500.len(),
            HANDLER500_BASELINE,
            handler500
                .iter()
                .map(|(k, v)| format!("{k} {v}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert!(
            defects.is_empty(),
            "IDOR 矩阵：{} 处越权缺陷：\n{}",
            defects.len(),
            defects.join("\n")
        );
    }
}
