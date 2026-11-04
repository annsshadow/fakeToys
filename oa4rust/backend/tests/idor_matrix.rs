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
#[path = "integration_tests/error_capture.rs"]
mod error_capture;

#[cfg(test)]
mod idor_matrix {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::Path;
    use tower::ServiceExt;

    use crate::error_capture;

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
    /// O23：扫描专用 admin 身份。handler 级 `require_admin` 大量存在，
    /// 非 admin 的 TOK_B 曾让写路由大面积 401，等于从没执行过。
    const UID_ADMIN: &str = "idor-o9-admin";
    const TOK_ADMIN: &str = "tok-o9-admin";

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
    /// 种子化全路由扫描专用 scratch 库（与 IDOR 矩阵隔离，两测试可并行）
    const SEEDED_DB: &str = "oa_par_seeded";

    /// 已知存量缺陷豁免（非 IDOR 面）：
    /// 这两条 legacy 上传路由的 handler 元数（3/4 参 Path）与路径槽位（1 参）
    /// 恒定不符（"Wrong number of path arguments" 500），已被 arity_probe 的
    /// 棘轮基线收录。语义修复需先核 o2server 契约（reference/scale 双段字面量
    /// 无来源），按约定「核不到不开轮」——矩阵显式豁免并计数，防静默扩大。
    /// 后两条同理：注册路径字面量缺 `{param}` 但 handler 提取 Path（arity 家族，
    /// 与「表里有行」无关），同属 arity_probe 面，种子化扫描不重复计。
    const KNOWN_ARTIFACT_ROUTES: &[&str] = &[
        "/api/file/upload/referencetype/reference/reference/scale/scale/{referenceType}",
        "/api/file/upload/referencetype/reference/reference/scale/scale/callback/callback/{referenceType}",
        "/api/file/share/share/shareId/file/fileId/folder/folderId",
        "/api/file/share/shield/id",
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
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
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
                    let Ok(text) = std::fs::read_to_string(&p) else {
                        continue;
                    };
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
            let Some(endq) = text[start..].find('"') else {
                break;
            };
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

    /// `DATABASE_URL` 解析后覆盖 dbname 的完整 URL（给 SeaORM 用）。
    fn db_url(dbname: &str) -> String {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
        let mut parsed = url::Url::parse(&url).expect("DATABASE_URL 解析失败");
        parsed.set_path(dbname);
        parsed.to_string()
    }

    /// 重建 scratch 库（存在则强删），全量迁移。
    async fn fresh_scratch(dbname: &str) {
        let admin = pool_to("postgres");
        let c = admin.get().await.expect("admin 连接");
        c.execute(
            &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
            &[],
        )
        .await
        .expect("drop scratch");
        c.execute(&format!("CREATE DATABASE {dbname}"), &[])
            .await
            .expect("create scratch");
        let pool = pool_to(dbname);
        shared::migrate::run_migrations(&pool)
            .await
            .expect("scratch 库迁移");
    }

    async fn drop_scratch(dbname: &str) {
        if let Ok(c) = pool_to("postgres").get().await {
            let _ = c
                .execute(
                    &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
                    &[],
                )
                .await;
        }
    }

    async fn setup() -> Option<(
        axum::Router,
        deadpool_postgres::Pool,
        shared::session::SessionManager,
    )> {
        setup_with_db(SCRATCH_DB).await
    }

    async fn setup_with_db(
        dbname: &str,
    ) -> Option<(
        axum::Router,
        deadpool_postgres::Pool,
        shared::session::SessionManager,
    )> {
        if !shared::testing::is_db_available().await {
            eprintln!("SKIP: no database available");
            return None;
        }
        // 穷举量级请求会打爆默认 300/分钟 限流——探针环境把限流阈值抬高
        // （只影响本测试进程；限流本身的用例在 middleware 测试中已覆盖）。
        std::env::set_var("OA4RUST_GENERAL_RATE_LIMIT", "1000000");
        std::env::set_var("OA4RUST_AUTH_RATE_LIMIT", "1000000");

        // 探针写流量全部落在 scratch 库（见文件头「探针卫生」）
        fresh_scratch(dbname).await;
        let pool = pool_to(dbname);
        let client = pool.get().await.expect("pool client");

        // 身份夹具：A（UA 单元，非所有者）、B（UB 单元，受害者）、C（UC 单元，跨组织）、
        // D（禁用）、E（会话将过期）
        for (pid, uid, unit, locked) in [
            (UID_A, UID_A, "unit-o9-UA", false),
            (UID_B, UID_B, "unit-o9-UB", false),
            (UID_C, UID_C, "unit-o9-UC", false),
            (UID_D, UID_D, "unit-o9-UD", true),
            (UID_E, UID_E, "unit-o9-UE", false),
            (UID_ADMIN, UID_ADMIN, "unit-root", false),
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

        // O23：给扫描身份补 admin 角色最小集（auth_role.name='admin' 关联）。
        // 只影响扫描；IDOR 矩阵的 A/B/C/D/E 语义不变。
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 VALUES ($1, 'role-admin', 'unit-root') ON CONFLICT DO NOTHING",
                &[&UID_ADMIN],
            )
            .await
            .expect("绑定 admin 角色");

        // 会话：A/B/C/D/admin 有效，E 稍后过期化
        let session_manager = shared::session::SessionManager::with_pool(pool.clone());
        for (uid, tok) in [
            (UID_A, TOK_A),
            (UID_B, TOK_B),
            (UID_C, TOK_C),
            (UID_D, TOK_D),
            (UID_E, TOK_E),
            (UID_ADMIN, TOK_ADMIN),
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
        let app = create_app_big_stack(pool.clone(), dbname, session_manager.clone(), rate_limiter);
        Some((app, pool, session_manager))
    }

    /// O23：写路由里有 logout/删会话/改人，会让扫描 token 在写阶段被前一条路由销毁。
    /// 每条写路由执行前恢复「人员 + admin 角色 + 会话」最小集合，使写路由真的被执行；
    /// 这样 401 只可能来自 handler 权限本身（或被收紧），不会来自身份已失效。
    async fn ensure_scan_admin(
        pool: &deadpool_postgres::Pool,
        sessions: &shared::session::SessionManager,
    ) {
        let client = pool.get().await.expect("恢复扫描 admin：取连接");
        client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, unit) \
                 VALUES ($1, $2, $3, 'idor-o9-noop', false, 'unit-root') \
                 ON CONFLICT (id) DO NOTHING",
                &[&UID_ADMIN, &UID_ADMIN, &UID_ADMIN],
            )
            .await
            .expect("恢复扫描 admin 人员");
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 VALUES ($1, 'role-admin', 'unit-root') ON CONFLICT DO NOTHING",
                &[&UID_ADMIN],
            )
            .await
            .expect("恢复扫描 admin 角色");
        drop(client);
        sessions
            .create_session(UID_ADMIN.to_string(), TOK_ADMIN.to_string())
            .await
            .expect("恢复扫描 admin 会话");
    }

    /// O24：应用必须完全落在 scratch 库。
    ///
    /// `oa4rust::create_app` 会用进程级 `DATABASE_URL` 建 SeaORM 连接，那条连接指向
    /// **开发库**；探针因此一直在「PG 走 scratch、ORM 走开发库」的混合状态下跑，
    /// 结果随本机开发库里的历史垃圾行漂移（实证：cms 500 在本机时有时无）。
    /// 改为 `create_app_with_sea_orm` + 显式 scratch URL（与
    /// `tests/integration_tests/helpers.rs` 同一做法），本地与 CI 才等价。
    fn create_app_big_stack(
        pool: deadpool_postgres::Pool,
        dbname: &str,
        session_manager: shared::session::SessionManager,
        rate_limiter: shared::rate_limit::RateLimiter,
    ) -> axum::Router {
        let sea_url = db_url(dbname);
        std::thread::Builder::new()
            .name("idor-app".to_string())
            .stack_size(STACK)
            .spawn(move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("runtime")
                    .block_on(async move {
                        let sea_orm = shared::db::create_sea_orm_pool_with_url(sea_url)
                            .await
                            .expect("scratch SeaORM 池");
                        oa4rust::create_app_with_sea_orm(
                            pool,
                            sea_orm,
                            session_manager,
                            rate_limiter,
                        )
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
        let Some((app, pool, _sessions)) = setup().await else {
            return;
        };

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let all = scan_routes(root);
        let candidates: Vec<&Route> = all.iter().filter(|r| is_candidate(r)).collect();

        // 规模下限：注册表/扫描器坏掉时直接失败，防止守卫静默变绿。
        assert!(
            candidates.len() >= 800,
            "候选路由数 {} 低于下限 800（注册表扫描异常？）",
            candidates.len()
        );
        eprintln!(
            "IDOR 矩阵候选路由: {}（全量注册 {}）",
            candidates.len(),
            all.len()
        );

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
            let public = matches!(registry.get_permission(&url), Some(PermissionLevel::Public));
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
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM file_file WHERE id = $1)",
                &[&FILE_ID],
            )
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
        drop_scratch(SCRATCH_DB).await;

        // handler 级 500 棘轮（O10 升级为集合基线）：基线文件逐条入库，
        // 「当前 ⊆ 基线」才绿——新增 500 路由红；修复后从基线删条目收缩。
        // 基线 = 2026-11-03 run15 实测 161 条（全部 owner 同 500，无越权掩盖）。
        // O10 收官：161 条全修，基线收缩为空集（零容忍：任何新增 handler 级 500 即红）。
        // 解析口径：跳过空行与 `#` 注释行。
        let baseline_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/idor_handler500_baseline.txt");
        let baseline: std::collections::BTreeSet<String> = std::fs::read_to_string(&baseline_path)
            .expect("handler500 基线文件必须存在（删文件=废棘轮）")
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
        let current: std::collections::BTreeSet<String> = handler500.keys().cloned().collect();
        let new_entries: Vec<&String> = current.difference(&baseline).collect();
        let fixed: Vec<&String> = baseline.difference(&current).collect();
        if !fixed.is_empty() {
            eprintln!(
                "== handler500 基线可收缩 {} 条（已修复，请从基线文件删除）==",
                fixed.len()
            );
            for f in &fixed {
                eprintln!("  [已修复] {f}");
            }
        }
        assert!(
            new_entries.is_empty(),
            "handler 级 500 棘轮回归：新增 {} 条不在基线的 500 路由（修复或记录入基线）：\n{}",
            new_entries.len(),
            new_entries
                .iter()
                .map(|k| format!(
                    "{k} {}",
                    handler500.get(*k).map(|v| v.as_str()).unwrap_or("")
                ))
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

    // ── 种子化全路由扫描（行为级守卫）──────────────────────────────────
    //
    // 静态扫描器（scripts/scan_row_get_column_mismatch.py）只能做「SQL 列类型 vs
    // Rust 取值类型」的静态推断，裸 `row.get("x")` 无法识别 → 必然有残余漏报。
    // 本守卫是行为级：真的往每张表写 ≥1 行非空/非默认值，再真的发全量路由请求，
    // 只要 handler 真的读到了行，反序列化错配就会变成 500 被抓到。
    // 门禁口径：handler500 == 0（不做棘轮基线，零容忍）。

    struct SeedCol {
        name: String,
        coltype: String,
        notnull: bool,
        has_default: bool,
        is_pk: bool,
    }

    fn q(s: &str) -> String {
        format!("'{}'", s.replace('\'', "''"))
    }

    /// 列名启发式：这些 varchar/text 列若被代码按 bool 读，就是已知缺陷形态
    /// （bam `xenabled` 等）。种成布尔字面量才能触发转换失败。
    fn boolish_name(name: &str) -> bool {
        let n = name.to_lowercase();
        const KEYS: [&str; 12] = [
            "enabled", "disabled", "active", "flag", "allowed", "valid", "locked", "deleted",
            "public", "visible", "need", "has_",
        ];
        KEYS.iter().any(|k| n.contains(k)) || n.starts_with("is_") || n.ends_with("_is")
    }

    /// 列名启发式：文本列存时间串、代码按时间读（process_id_complex 那类）
    fn timeish_name(name: &str) -> bool {
        let n = name.to_lowercase();
        n.ends_with("_at")
            || n.ends_with("time")
            || n.ends_with("date")
            || n.starts_with("create_")
            || n.starts_with("update_")
            || n.starts_with("start_")
            || n.starts_with("end_")
            || n.starts_with("begin_")
            || n.starts_with("finish_")
            || n.starts_with("fire_")
            || n.starts_with("expire")
    }

    fn pk_literal(t: &str) -> String {
        if t == "uuid" {
            "'00000000-0000-0000-0000-000000000001'".to_string()
        } else if t.contains("int") {
            "1".to_string()
        } else {
            "'1'".to_string()
        }
    }

    fn array_literal(t: &str) -> Option<String> {
        let base = t.trim_end_matches("[]").trim();
        let elem = if base == "boolean" {
            "true"
        } else if base == "uuid" {
            "00000000-0000-0000-0000-000000000001"
        } else if base.contains("int")
            || matches!(
                base,
                "numeric" | "decimal" | "real" | "double precision" | "money"
            )
        {
            "1"
        } else if base.contains("timestamp") {
            "2026-01-01 10:00:00"
        } else if base == "date" {
            "2026-01-01"
        } else if base == "jsonb" || base == "json" {
            return Some(format!("ARRAY['{{\"seed\":1}}'::{base}]"));
        } else {
            "seed"
        };
        Some(format!("'{{{elem}}}'::{t}"))
    }

    /// CHECK 约束把文本列限死在枚举值上（migrations 实证）：
    /// 通用 'seed' 会撞 23514，按约束给合法值才能种进去。
    fn constrained_text(table: &str, col: &str) -> Option<&'static str> {
        match (table, col) {
            ("init_server_command", "command") => Some("'execute'"),
            ("x_query_neural_job", "action") => Some("'learn'"),
            _ => None,
        }
    }

    /// 单列种子字面量；None = 该类型无法安全生成（列被跳过，失败由重试兜底）
    fn seed_literal(
        col: &SeedCol,
        enums: &BTreeMap<String, String>,
        table: &str,
    ) -> Option<String> {
        let t = col.coltype.to_lowercase();
        let n = col.name.to_lowercase();
        if col.is_pk {
            return Some(pk_literal(&t));
        }
        if t.ends_with("[]") {
            return array_literal(&t);
        }
        if t.starts_with("timestamp") || t.contains("timestamp") {
            return Some("'2026-01-01 10:00:00'".into());
        }
        if t == "date" {
            return Some("'2026-01-01'".into());
        }
        if t == "time" || t == "time without time zone" || t == "time with time zone" {
            return Some("'10:00:00'".into());
        }
        if t == "boolean" {
            return Some("true".into());
        }
        if t == "uuid" {
            return Some("'00000000-0000-0000-0000-000000000001'".into());
        }
        if t == "json" || t == "jsonb" {
            return Some("'{\"seed\":1}'".into());
        }
        if t == "bytea" {
            return Some("decode('01','hex')".into());
        }
        if t.contains("int")
            || matches!(
                t.as_str(),
                "numeric" | "decimal" | "real" | "double precision" | "money"
            )
        {
            return Some("1".into());
        }
        if t == "inet" {
            return Some("'127.0.0.1'".into());
        }
        if t == "cidr" {
            return Some("'127.0.0.0/8'".into());
        }
        if t == "macaddr" {
            return Some("'00:00:00:00:00:00'".into());
        }
        if t == "interval" {
            return Some("'1 hour'".into());
        }
        if t.starts_with("bit") || t.starts_with("varbit") {
            return Some("B'1'".into());
        }
        if t == "xml" {
            return Some("'<seed/>'".into());
        }
        if t.contains("char") || t == "text" || t == "citext" || t == "name" {
            if let Some(v) = constrained_text(table, &n) {
                return Some(v.to_string());
            }
            // 缺陷形态优先：文本列存布尔/时间字面量，代码按 bool/时间读
            if boolish_name(&n) {
                return Some("'true'".into());
            }
            if timeish_name(&n) {
                return Some("'2026-01-01 10:00:00'".into());
            }
            if n.ends_with("id") || n == "id" {
                return Some("'1'".into());
            }
            return Some("'seed'".into());
        }
        // 枚举类型（format_type 返回类型名）：取第一个 label
        if let Some(label) = enums.get(&col.coltype) {
            return Some(q(label));
        }
        None
    }

    async fn try_insert_seed(
        client: &deadpool_postgres::Client,
        table: &str,
        cols: &[SeedCol],
        enums: &BTreeMap<String, String>,
        only_required: bool,
    ) -> Result<u64, deadpool_postgres::tokio_postgres::Error> {
        let mut names = Vec::new();
        let mut vals = Vec::new();
        for c in cols {
            if only_required && !(c.is_pk || (c.notnull && !c.has_default)) {
                continue;
            }
            if let Some(v) = seed_literal(c, enums, table) {
                names.push(format!("\"{}\"", c.name));
                vals.push(v);
            }
        }
        if names.is_empty() {
            return client
                .execute(
                    &format!("INSERT INTO public.\"{table}\" DEFAULT VALUES"),
                    &[],
                )
                .await;
        }
        let sql = format!(
            "INSERT INTO public.\"{table}\" ({}) VALUES ({}) ON CONFLICT DO NOTHING",
            names.join(", "),
            vals.join(", ")
        );
        client.execute(&sql, &[]).await
    }

    /// tokio_postgres 的 Display 只有 "db error"；取真实 SQLSTATE/message
    fn pg_err(e: &deadpool_postgres::tokio_postgres::Error) -> String {
        match e.as_db_error() {
            Some(d) => format!("{} (SQLSTATE {})", d.message(), d.code().code()),
            None => e.to_string(),
        }
    }

    /// 每张 public 表种 ≥1 行。返回（成功表集合，失败表+原因）。
    /// 失败必须显式返回并打印——静默少种表 = 守卫静默变绿。
    async fn seed_all_tables(pool: &deadpool_postgres::Pool) -> (Vec<String>, Vec<String>) {
        let client = pool.get().await.expect("seeded pool client");
        if client
            .execute("SET session_replication_role = replica", &[])
            .await
            .is_err()
        {
            eprintln!("WARN: 无法关闭 FK/触发器（session_replication_role 需超级用户），依赖 FK 的种子会失败并列出");
        }

        let mut enums: BTreeMap<String, String> = BTreeMap::new();
        if let Ok(rows) = client
            .query(
                "SELECT t.typname, e.enumlabel FROM pg_type t \
                 JOIN pg_enum e ON e.enumtypid = t.oid \
                 ORDER BY t.typname, e.enumsortorder",
                &[],
            )
            .await
        {
            for r in rows {
                let t: String = r.get(0);
                let l: String = r.get(1);
                enums.entry(t).or_insert(l);
            }
        }

        let tables: Vec<String> = client
            .query(
                "SELECT c.relname::text FROM pg_class c \
                 JOIN pg_namespace n ON n.oid = c.relnamespace \
                 WHERE n.nspname = 'public' AND c.relkind IN ('r','p') \
                 ORDER BY c.relname",
                &[],
            )
            .await
            .expect("枚举表失败")
            .iter()
            .map(|r| r.get(0))
            .collect();

        let mut seeded = Vec::new();
        let mut failures = Vec::new();
        for table in &tables {
            let rows = match client
                .query(
                    "SELECT a.attname::text, format_type(a.atttypid, a.atttypmod), \
                            a.attnotnull, (d.adbin IS NOT NULL), \
                            EXISTS(SELECT 1 FROM pg_constraint pc \
                                   WHERE pc.conrelid = a.attrelid AND pc.contype = 'p' \
                                     AND a.attnum = ANY(pc.conkey)) \
                     FROM pg_attribute a \
                     LEFT JOIN pg_attrdef d ON d.adrelid = a.attrelid AND d.adnum = a.attnum \
                     WHERE a.attrelid = to_regclass($1) AND a.attnum > 0 \
                       AND NOT a.attisdropped AND a.attgenerated = '' AND a.attidentity = '' \
                     ORDER BY a.attnum",
                    &[&format!("public.\"{table}\"")],
                )
                .await
            {
                Ok(rows) => rows,
                Err(e) => {
                    failures.push(format!("{table}: 取列失败: {}", pg_err(&e)));
                    continue;
                }
            };
            let cols: Vec<SeedCol> = rows
                .iter()
                .map(|r| SeedCol {
                    name: r.get(0),
                    coltype: r.get(1),
                    notnull: r.get(2),
                    has_default: r.get(3),
                    is_pk: r.get(4),
                })
                .collect();
            if cols.is_empty() {
                continue;
            }
            let mut last_err = None;
            let mut ok = false;
            for only_required in [false, true] {
                match try_insert_seed(&client, table, &cols, &enums, only_required).await {
                    Ok(_) => {
                        ok = true;
                        break;
                    }
                    Err(e) => last_err = Some(e),
                }
            }
            if ok {
                seeded.push(table.clone());
            } else {
                failures.push(format!(
                    "{table}: {}",
                    last_err
                        .as_ref()
                        .map(pg_err)
                        .unwrap_or_else(|| "未知".into())
                ));
            }
        }

        // O24：`organization/core/express` 读 `config_key='sync_enabled'`。只种通用
        // 「seed」行时两条路由永远走 503 分支，等于新加的 200 分支从没被执行过。
        // 这里补一条合法行，让种子化扫描真的进入 200 分支。
        if let Err(e) = client
            .execute(
                "INSERT INTO x_org_config (id, config_key, config_value) \
                 VALUES ('seed-sync-enabled', 'sync_enabled', 'true') \
                 ON CONFLICT (id) DO UPDATE SET config_key = EXCLUDED.config_key, \
                 config_value = EXCLUDED.config_value",
                &[],
            )
            .await
        {
            failures.push(format!("x_org_config(sync_enabled): {}", pg_err(&e)));
        }

        // 恢复会话设置：该连接回池后仍会被 app 查询复用，不能留下 replica 语义
        let _ = client
            .execute("SET session_replication_role = DEFAULT", &[])
            .await;
        (seeded, failures)
    }

    /// 当前基线条目数上限（必须与基线文件行数相等）。O24-3 批次 4 起为 30。
    /// 每修复一条：删对应基线路 → 下调本常量，同 PR 可见。
    /// O23 第二步写路由首次真正执行后，存量 500 从 1 暴露到 40；O24 修掉探针
    /// 库隔离缺陷后 ORM 路由首次读种子库，再 +2（meeting room 的 equipment 解码）。
    /// O24-3 批次 1 修掉 portal surface publish（写库后才 ColumnDecode panic）：42 → 41。
    /// O24-3 批次 2 修掉 JSONB 参数绑定族（meeting/processplatform/queryview×2）：41 → 37。
    /// O24-3 批次 3 修掉查询设计器 permission 族（query/stat/table/view）：37 → 33。
    /// O24-3 批次 4 修掉 INSERT 缺必填 id/规范列族（console/document/message）：33 → 30。
    /// O24-3 批次 5 修掉 draft/start 参数失配（POST + GET 共用 handler）：30 → 28。
    const BASELINE_COUNT: usize = 28;

    /// 棘轮基线：`<METHOD> <route>\t<reason>`，reason 见文件头。
    const BASELINE_RAW: &str = include_str!("idor_seeded_handler500_baseline.txt");

    // O23：扫描门禁的下限。枚举数量与执行数量都要卡，且读/写分别计数——
    // 只数总数会让「写路由从未执行」静默变绿。
    // 数值按 2026-11-05 实测（读 1603 / 写 1980）留 ~5% 余量；目录枚举顺序
    // 变化不再影响执行顺序（执行前按 path+method 稳定排序）。
    const MIN_SCANNED_READ_ROUTES: usize = 1500;
    const MIN_SCANNED_WRITE_ROUTES: usize = 1850;
    const MIN_EXECUTED_READ_ROUTES: usize = 1450;
    const MIN_EXECUTED_WRITE_ROUTES: usize = 1800;
    const MIN_EXECUTED_ROUTES: usize = 3300;
    /// 扫描身份是 admin：401 比例异常高说明夹具/权限层级没打通。
    const MAX_401_RATIO: f64 = 0.20;
    /// 写路由里**按设计**不会用 Bearer 会话执行的两条，逐条留理由：
    /// - refresh 只接受 Cookie 凭据（auth/src/lib.rs `refresh`，Bearer → 401）；
    /// - meeting config/system/config/manage 刻意注册在匿名可达 router
    ///   （模块前缀契约），session 不注入，handler 内 401 拦截。
    ///
    /// 新增任何允许项都必须写理由；未登记的 401 一律红（破坏实验锚点）。
    const WRITE_401_ALLOWED: &[&str] = &[
        "POST /api/authentication/refresh",
        "POST /api/meeting/assemble/control/config/system/config/manage",
    ];

    /// 从本次请求捕获的 ERROR 串里判定类别：优先取失败列名 / SQLSTATE /
    /// 类型错配家族，最后才归入无日志类。O23 起 500 一定有日志，类别用于
    /// 把同类根因归并，便于按族修复后一次删多行。
    fn classify_error(joined: &str) -> String {
        if let Some(idx) = joined.find("error retrieving column ") {
            let rest = &joined[idx + "error retrieving column ".len()..];
            let col: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !col.is_empty() {
                return format!("column:{col}");
            }
        }
        // sqlx 的 ColumnDecode 形态：`index: "\"equipment\""`（不是 "error retrieving column"）
        if let Some(idx) = joined.find("ColumnDecode { index: \"\\\"") {
            let rest = &joined[idx + "ColumnDecode { index: \"\\\"".len()..];
            let col: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !col.is_empty() {
                return format!("column:{col}");
            }
        }
        if let Some(idx) = joined.find("code: SqlState(") {
            let rest = &joined[idx + "code: SqlState(".len()..];
            let code: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            if !code.is_empty() {
                return format!("category:{code}");
            }
        }
        if joined.contains("WrongType") {
            return "category:wrong-type".to_string();
        }
        if joined.contains("kind: Parameters(") {
            return "category:param-count".to_string();
        }
        if joined.contains("source=-") {
            return "category:no-source".to_string();
        }
        "category:no-error-log".to_string()
    }

    /// 基线的一行：`key\t<reason>` 决定棘轮相等性，`owner/due/cond` 是到期门禁的
    /// 元数据（缺项、非法日期、超期都会让门禁红）。
    struct BaselineRow {
        key: String,
        reason: String,
        owner: String,
        due: String,
        cond: String,
    }

    impl BaselineRow {
        /// 棘轮比较用的身份串（不含元数据，改责任人/日期不会影响相等性）。
        fn identity(&self) -> String {
            format!("{}\t{}", self.key, self.reason)
        }
    }

    fn parse_baseline(raw: &str) -> Vec<BaselineRow> {
        raw.lines()
            .map(|l| l.trim_end())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| {
                let fields: Vec<&str> = l.split('\t').collect();
                BaselineRow {
                    key: fields.first().copied().unwrap_or("").to_string(),
                    reason: fields.get(1).copied().unwrap_or("").to_string(),
                    owner: fields
                        .get(2)
                        .and_then(|f| f.strip_prefix("owner:"))
                        .unwrap_or("")
                        .to_string(),
                    due: fields
                        .get(3)
                        .and_then(|f| f.strip_prefix("due:"))
                        .unwrap_or("")
                        .to_string(),
                    cond: fields
                        .get(4)
                        .and_then(|f| f.strip_prefix("cond:"))
                        .unwrap_or("")
                        .to_string(),
                }
            })
            .collect()
    }

    /// 业务时区（UTC+8）的「当天」。不用 CI 的 UTC 日界：否则北京时间 00:00-08:00
    /// 会晚一天才判超期。
    fn business_today() -> chrono::NaiveDate {
        let offset = chrono::FixedOffset::east_opt(8 * 3600).expect("UTC+8");
        chrono::Utc::now().with_timezone(&offset).date_naive()
    }

    /// 行为级守卫：种子化全路由扫描 + 棘轮基线。
    ///
    /// 规则：当前 500 集合必须与基线集合**完全相等**。
    /// - 基线外的新 500 → 红（新写的类型错配当天被拦）；
    /// - 基线里复现不出来的条目 → 红（已修但没删行 / 没下调上限）。
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn seeded_route_scan_handler500_ratchet() {
        error_capture::init();
        let Some((app, pool, sessions)) = setup_with_db(SEEDED_DB).await else {
            return;
        };
        let (seeded, failures) = seed_all_tables(&pool).await;
        eprintln!(
            "== 种子化：成功 {} 表，失败 {} 表 ==",
            seeded.len(),
            failures.len()
        );
        for f in &failures {
            eprintln!("  [种子失败] {f}");
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut all = scan_routes(root);
        // O23：扫描结果不得依赖目录枚举顺序。先按 path+method 稳定排序再执行；
        // 同 path 下按 method 排，保证每次运行的执行序列一致、失败可复现。
        all.sort_by(|a, b| a.path.cmp(&b.path).then_with(|| a.method.cmp(b.method)));

        let candidates: Vec<&Route> = all
            .iter()
            .filter(|r| {
                !KNOWN_ARTIFACT_ROUTES.contains(&r.path.as_str())
                    // mock 路由（Handler 元数错配家族，O9 is_candidate 同口径排除）
                    && !r.path.contains("mock")
            })
            .collect();
        let read_total = candidates.iter().filter(|r| r.method == "GET").count();
        let write_total = candidates.iter().filter(|r| r.method != "GET").count();
        assert!(
            read_total >= MIN_SCANNED_READ_ROUTES,
            "枚举到的读路由 {read_total} 条，低于下限 {MIN_SCANNED_READ_ROUTES}（扫描器坏了？）"
        );
        assert!(
            write_total >= MIN_SCANNED_WRITE_ROUTES,
            "枚举到的写路由 {write_total} 条，低于下限 {MIN_SCANNED_WRITE_ROUTES}（写路由没收全，门禁不可信）"
        );

        // 先读后写：写路由（DELETE/UPDATE）会销毁种子行，必须放在读之后。
        // candidates 已排序，两段都保序。
        let mut ordered: Vec<&Route> = candidates
            .iter()
            .copied()
            .filter(|r| r.method == "GET")
            .collect();
        ordered.extend(candidates.iter().copied().filter(|r| r.method != "GET"));

        // setup 阶段的 ERROR（迁移/夹具）不计入任何路由
        let _ = error_capture::drain();
        let mut handler500: BTreeMap<String, String> = BTreeMap::new();
        let mut executed = 0usize;
        let mut executed_read = 0usize;
        let mut executed_write = 0usize;
        let mut read_401 = 0usize;
        let mut read_403 = 0usize;
        let mut write_401 = 0usize;
        let mut write_403 = 0usize;
        let mut write_401_routes: Vec<String> = Vec::new();
        for r in ordered {
            let is_write = r.method != "GET";
            if is_write {
                ensure_scan_admin(&pool, &sessions).await;
            }
            let url = fill_params(&r.path);
            let _ = error_capture::drain();
            // 「执行到」= 请求真的发出去并拿到响应；枚举到但 401/panic 的不算。
            let Some((status, body)) = fire(&app, r.method, &url, Some(TOK_ADMIN)).await else {
                continue;
            };
            executed += 1;
            if is_write {
                executed_write += 1;
                match status.as_u16() {
                    401 => {
                        write_401 += 1;
                        write_401_routes.push(format!("{} {}", r.method, r.path));
                    }
                    403 => write_403 += 1,
                    _ => {}
                }
            } else {
                executed_read += 1;
                match status.as_u16() {
                    401 => read_401 += 1,
                    403 => read_403 += 1,
                    _ => {}
                }
            }
            let errs = error_capture::drain();
            if status.as_u16() == 500 {
                handler500.insert(
                    format!("{} {}", r.method, r.path),
                    format!(
                        "body={} | errs={}",
                        body.chars().take(200).collect::<String>(),
                        if errs.is_empty() {
                            "<handler 未输出 ERROR 日志>".to_string()
                        } else {
                            errs.join(" || ")
                        }
                    ),
                );
            }
        }

        eprintln!("== 种子化扫描 handler500：{} 条 ==", handler500.len());
        for (k, v) in &handler500 {
            eprintln!("  {k} → {v}");
        }

        let unauthorized_total = read_401 + write_401;
        let unauthorized_ratio = if executed == 0 {
            1.0
        } else {
            unauthorized_total as f64 / executed as f64
        };
        eprintln!(
            "== 扫描覆盖：枚举 读 {read_total} / 写 {write_total}；执行 读 {executed_read} / 写 {executed_write}（共 {executed}）；\
             401 读 {read_401} / 写 {write_401}；403 读 {read_403} / 写 {write_403}；401 比例 {:.1}% ==",
            unauthorized_ratio * 100.0
        );
        assert!(
            executed_read >= MIN_EXECUTED_READ_ROUTES,
            "实际执行的读路由 {executed_read} 条，低于下限 {MIN_EXECUTED_READ_ROUTES}（枚举到但没执行）"
        );
        assert!(
            executed_write >= MIN_EXECUTED_WRITE_ROUTES,
            "实际执行的写路由 {executed_write} 条，低于下限 {MIN_EXECUTED_WRITE_ROUTES}（写路由没真正被执行，门禁不可信）"
        );
        assert!(
            executed >= MIN_EXECUTED_ROUTES,
            "实际执行的路由 {executed} 条，低于下限 {MIN_EXECUTED_ROUTES}"
        );
        // 破坏实验锚点：任意一条写路由被收紧成 401，这里必须红（不允许静默少扫）。
        let unexpected_write_401: Vec<&str> = write_401_routes
            .iter()
            .map(|r| r.as_str())
            .filter(|r| !WRITE_401_ALLOWED.contains(r))
            .collect();
        assert!(
            unexpected_write_401.is_empty(),
            "写路由出现未登记的 401 共 {} 条（admin 扫描身份未生效，或权限被收紧）：\n{}",
            unexpected_write_401.len(),
            unexpected_write_401.join("\n")
        );
        assert!(
            unauthorized_ratio <= MAX_401_RATIO,
            "401 比例 {:.1}% 超过上限 {:.1}%（枚举 {} 条 / 执行 {} 条）",
            unauthorized_ratio * 100.0,
            MAX_401_RATIO * 100.0,
            candidates.len(),
            executed
        );

        drop_scratch(SEEDED_DB).await;

        assert!(
            seeded.len() >= 300,
            "种子成功表数 {} 低于下限 300（种子器坏了，守卫会静默变绿）",
            seeded.len()
        );
        assert!(
            failures.is_empty(),
            "种子化失败表 {} 张（这些表的读路径不被覆盖）：\n{}",
            failures.len(),
            failures.join("\n")
        );

        let current: BTreeSet<String> = handler500
            .iter()
            .map(|(k, v)| format!("{k}\t{}", classify_error(v)))
            .collect();
        let baseline_rows = parse_baseline(BASELINE_RAW);
        let baseline: BTreeSet<String> = baseline_rows.iter().map(BaselineRow::identity).collect();

        // 下限断言：解析器若静默失效（行被吞、分隔符变更），这里先红，
        // 避免「基线 0 行 / 当前 0 条」的假绿。
        assert!(
            baseline_rows.len() >= current.len(),
            "解析出的基线行数 {} < 当前实际 500 条数 {}（解析器静默失效？）",
            baseline_rows.len(),
            current.len()
        );

        // 元校验：条目格式、元数据齐全、due 合法、去重、条目数 == BASELINE_COUNT
        let mut bad_format = Vec::new();
        for row in &baseline_rows {
            if !row.key.contains(' ') {
                bad_format.push(format!("缺少 METHOD/路径: {}", row.key));
            }
            if !(row.reason.starts_with("column:") || row.reason.starts_with("category:")) {
                bad_format.push(format!("reason 不合规: {} ({})", row.key, row.reason));
            }
            if row.owner.is_empty() {
                bad_format.push(format!("缺少 owner（责任人）: {}", row.key));
            }
            if row.cond.is_empty() {
                bad_format.push(format!("缺少 cond（转绿条件）: {}", row.key));
            }
            if chrono::NaiveDate::parse_from_str(&row.due, "%Y-%m-%d").is_err() {
                bad_format.push(format!(
                    "due 非法（必须是 YYYY-MM-DD）: {} → {:?}",
                    row.key, row.due
                ));
            }
        }
        assert!(
            bad_format.is_empty(),
            "基线条目格式/元数据错误：\n{}",
            bad_format.join("\n")
        );

        // 到期门禁：当天（UTC+8）超过 due 且该行仍在基线 → FAIL。
        // 报出超期行、超期天数、原定转绿条件，避免「写了期限没人执行」。
        let today = business_today();
        let mut overdue = Vec::new();
        for row in &baseline_rows {
            if let Ok(due) = chrono::NaiveDate::parse_from_str(&row.due, "%Y-%m-%d") {
                if today > due {
                    overdue.push(format!(
                        "{} | due {} | 超期 {} 天 | 责任人 {} | 原定转绿条件：{}",
                        row.key,
                        row.due,
                        (today - due).num_days(),
                        row.owner,
                        row.cond
                    ));
                }
            }
        }
        assert!(
            overdue.is_empty(),
            "基线中有 {} 行已超过期限仍在基线（今天 {}，UTC+8）：\n{}",
            overdue.len(),
            today,
            overdue.join("\n")
        );
        assert_eq!(
            baseline.len(),
            baseline_rows.len(),
            "基线里有重复条目（会造成『相等』假象）"
        );
        assert_eq!(
            baseline_rows.len(),
            BASELINE_COUNT,
            "基线行数 {} ≠ BASELINE_COUNT {}；修复时必须删行并下调该常量",
            baseline_rows.len(),
            BASELINE_COUNT
        );

        let new_500: Vec<&String> = current.difference(&baseline).collect();
        let stale: Vec<&String> = baseline.difference(&current).collect();

        assert!(
            new_500.is_empty(),
            "新增 / 未登记 handler500：{} 条（先修，禁止往基线里加行）：\n{}",
            new_500.len(),
            new_500
                .iter()
                .map(|e| format!("  {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert!(
            stale.is_empty(),
            "基线里 {} 条在本轮复现不出来（已修复但没删行 / 没下调 BASELINE_COUNT）：\n{}",
            stale.len(),
            stale
                .iter()
                .map(|e| format!("  {e}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        eprintln!(
            "== 棘轮基线：{} 条（当前 {} 条），集合相等 ==",
            baseline.len(),
            current.len()
        );
    }

    /// O24：改前/改后契约快照。
    ///
    /// 只在显式设置 `O24_DUMP_DIR` 时执行；未设置时零成本返回，不影响常规测试与 CI。
    /// 输出 `routes.tsv`：`METHOD\tPATH\tSTATUS\t<归一化后的响应体>`。
    /// 响应体里的 `date` 是请求时刻，跨版本比较时归一化为 `<NOW>`。
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn o24_contract_snapshot() {
        let Ok(dir) = std::env::var("O24_DUMP_DIR") else {
            return;
        };
        error_capture::init();
        let Some((app, pool, sessions)) = setup_with_db(SEEDED_DB).await else {
            return;
        };
        let (seeded, failures) = seed_all_tables(&pool).await;

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut all = scan_routes(root);
        all.sort_by(|a, b| a.path.cmp(&b.path).then_with(|| a.method.cmp(b.method)));
        let candidates: Vec<&Route> = all
            .iter()
            .filter(|r| {
                !KNOWN_ARTIFACT_ROUTES.contains(&r.path.as_str()) && !r.path.contains("mock")
            })
            .collect();
        let mut ordered: Vec<&Route> = candidates
            .iter()
            .copied()
            .filter(|r| r.method == "GET")
            .collect();
        ordered.extend(candidates.iter().copied().filter(|r| r.method != "GET"));

        let _ = error_capture::drain();
        let mut rows = String::new();
        let mut error_rows = String::new();
        let mut executed = 0usize;
        for r in ordered {
            if r.method != "GET" {
                ensure_scan_admin(&pool, &sessions).await;
            }
            let url = fill_params(&r.path);
            let _ = error_capture::drain();
            let Some((status, body)) = fire(&app, r.method, &url, Some(TOK_ADMIN)).await else {
                continue;
            };
            executed += 1;
            rows.push_str(&format!(
                "{}\t{}\t{}\t{}\n",
                r.method,
                r.path,
                status.as_u16(),
                o24_normalize(&body)
            ));
            if status.as_u16() >= 400 {
                let errs = error_capture::drain();
                error_rows.push_str(&format!(
                    "{}\t{}\t{}\t{}\n",
                    r.method,
                    r.path,
                    status.as_u16(),
                    errs.join(" || ").replace(['\t', '\n'], " ")
                ));
            }
        }

        let dir = Path::new(&dir);
        std::fs::create_dir_all(dir).expect("创建快照目录");
        std::fs::write(dir.join("routes.tsv"), &rows).expect("写 routes.tsv");
        std::fs::write(dir.join("errors.tsv"), &error_rows).expect("写 errors.tsv");
        std::fs::write(
            dir.join("meta.txt"),
            format!(
                "executed={executed}\nseeded={}\nseed_failures={}\n",
                seeded.len(),
                failures.len()
            ),
        )
        .expect("写 meta.txt");
        eprintln!("O24 快照已写入 {}：{executed} 条路由", dir.display());
        drop_scratch(SEEDED_DB).await;
    }

    /// 响应体归一化：`date`（请求时刻）→ `<NOW>`；制表符/换行压成空格保证一行一路由。
    fn o24_normalize(body: &str) -> String {
        let mut s = String::with_capacity(body.len());
        let mut rest = body;
        while let Some(pos) = rest.find("\"date\":\"") {
            s.push_str(&rest[..pos]);
            let after = &rest[pos + "\"date\":\"".len()..];
            let Some(end) = after.find('"') else {
                break;
            };
            s.push_str("\"date\":\"<NOW>\"");
            rest = &after[end + 1..];
        }
        s.push_str(rest);
        s.replace(['\t', '\n'], " ")
    }
}
