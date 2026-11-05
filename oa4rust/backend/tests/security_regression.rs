// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O26：安全回归门禁（行为门禁 + 棘轮基线）。
//!
//! 把行为审计（report-6/report-7）里已实证的越权、fail-open、假门、软门
//! 固化成一条可复跑的门禁。与 `idor_matrix.rs` 的「全路由穷举」不同，本门禁
//! 是**逐条断言**：每条用例写明端点、身份、预期，失败列表可直接读到这些字段。
//!
//! # 运行
//!
//! ```text
//! cargo test --test security_regression -- --nocapture
//! ```
//!
//! 或 `python scripts/security_regression_gate.py`（同一命令的薄封装）。
//!
//! 需要真实 PostgreSQL（默认 `postgres://o2server:password@localhost:5432/oa4rust_test`，
//! 可用 `DATABASE_URL` 覆盖）。门禁自己建 scratch 库 `oa_par_gate`、跑全量迁移、
//! 灌最小数据，跑完 DROP —— 不读、不写共享测试库。
//!
//! # 基线棘轮
//!
//! `tests/security_regression_baseline.txt` 记录「当前仍非 PASS 的用例 id」。
//!
//! - 出现基线之外的非 PASS（新越权 / 新未验证）→ 红。
//! - 基线里已有、但这次 PASS 了（修复了却没减基线）→ 红，要求删行。
//!   这条是「单调下降」的保证：修复线每修一条必须把基线减一。
//!
//! 更新基线（仅在确认新增项无回归、或修复后减一时）：
//!
//! ```text
//! SECURITY_GATE_UPDATE_BASELINE=1 cargo test --test security_regression -- --nocapture
//! ```
//!
//! # 判定口径
//!
//! - PASS：401/403（明确拒绝）；软门另要求「拒绝且零写入」。
//! - LEAK：2xx 且响应体含受害者 canary，或写用例 2xx 且数据被改。
//! - UNVERIFIED：404/5xx/种子缺失/200 无 canary —— **不算 PASS**。
//!   假阴性会把「加了过滤器但返回空」误判为已修，故一律计入基线，宁可红也不假绿。

#[cfg(test)]
mod security_regression {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use serde_json::{Map, Value};
    use std::collections::BTreeSet;
    use std::path::PathBuf;
    use tower::ServiceExt;

    const STACK: usize = 64 * 1024 * 1024;
    const SCRATCH_DB: &str = "oa_par_gate";
    const ATTACKER: &str = "audit-user";
    const VICTIM: &str = "person-admin";
    /// 假门探针：路径参数 `{person}` 传管理员，验证 handler 是否误把入参当权限主体
    const ADMIN_PROBE: &str = "gate-admin";
    const TOK_ATTACKER: &str = "tok-gate-attacker";
    const TOK_VICTIM: &str = "tok-gate-victim";

    const CASES: &str = include_str!("security_regression_cases.tsv");
    const SEED: &str = include_str!("security_regression_seed.sql");
    const FIXTURES: &str = include_str!("security_regression_fixtures.sql");

    #[derive(Clone, Debug)]
    struct Case {
        id: String,
        kind: String,
        method: String,
        path: String,
        table: String,
        victim: String,
        leak: String,
    }

    #[derive(Clone, Debug)]
    enum Outcome {
        Pass,
        Leak(String),
        Unverified(String),
    }

    impl Outcome {
        fn state(&self) -> &'static str {
            match self {
                Outcome::Pass => "PASS",
                Outcome::Leak(_) => "LEAK",
                Outcome::Unverified(_) => "UNVERIFIED",
            }
        }
    }

    fn parse_cases() -> Vec<Case> {
        CASES
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
            .filter_map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                if f.len() < 7 {
                    return None;
                }
                Some(Case {
                    id: f[0].to_string(),
                    kind: f[1].to_string(),
                    method: f[2].to_string(),
                    path: f[3].to_string(),
                    table: f[4].to_string(),
                    victim: f[5].to_string(),
                    leak: f[6].to_string(),
                })
            })
            .collect()
    }

    fn db_url(dbname: &str) -> String {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
        let mut parsed = url::Url::parse(&url).expect("DATABASE_URL 解析失败");
        parsed.set_path(dbname);
        parsed.to_string()
    }

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

    /// 逐条执行 SQL；单条失败不中断（该表用例会落到 UNVERIFIED），但会打印。
    async fn run_sql(pool: &deadpool_postgres::Pool, sql: &str, label: &str) -> Vec<String> {
        let client = pool.get().await.expect("seed 连接");
        let mut errs = Vec::new();
        for stmt in sql.split(";\n") {
            let s = stmt.trim();
            if s.is_empty() || s.starts_with("--") {
                continue;
            }
            if let Err(e) = client.batch_execute(s).await {
                let first = s.lines().next().unwrap_or("").chars().take(90).collect::<String>();
                errs.push(format!("{label}: {first} => {e}"));
            }
        }
        errs
    }

    fn create_app_big_stack(
        pool: deadpool_postgres::Pool,
        dbname: &str,
        session_manager: shared::session::SessionManager,
        rate_limiter: shared::rate_limit::RateLimiter,
    ) -> axum::Router {
        let sea_url = db_url(dbname);
        std::thread::Builder::new()
            .name("gate-app".to_string())
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

    async fn setup() -> Option<(axum::Router, deadpool_postgres::Pool)> {
        if !shared::testing::is_db_available().await {
            eprintln!("SKIP: no database available");
            return None;
        }
        std::env::set_var("OA4RUST_GENERAL_RATE_LIMIT", "1000000");
        std::env::set_var("OA4RUST_AUTH_RATE_LIMIT", "1000000");

        fresh_scratch(SCRATCH_DB).await;
        let pool = pool_to(SCRATCH_DB);
        let client = pool.get().await.expect("pool client");

        for (pid, unit) in [
            (ATTACKER, "unit-gate-a"),
            (VICTIM, "unit-gate-v"),
            (ADMIN_PROBE, "unit-root"),
        ] {
            client
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, unit) \
                     VALUES ($1, $1, $1, 'gate-noop', false, $2) ON CONFLICT (id) DO NOTHING",
                    &[&pid, &unit],
                )
                .await
                .expect("插入人员夹具");
        }
        // 假门探针：cipher 端点把「路径上的 person」当作权限主体，攻击者可传管理员身份
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 VALUES ($1, 'role-admin', 'unit-root') ON CONFLICT DO NOTHING",
                &[&ADMIN_PROBE],
            )
            .await
            .expect("绑定 admin 角色");

        // 先灌 82 张表的受害者行（owner=person-admin），再灌本门禁专用夹具
        let mut errs = run_sql(&pool, SEED, "seed").await;
        errs.extend(run_sql(&pool, FIXTURES, "fixture").await);
        for e in &errs {
            eprintln!("[SEED-ERR] {e}");
        }
        println!("seed 语句失败 {} 条（对应用例将落到 UNVERIFIED）", errs.len());

        let session_manager = shared::session::SessionManager::with_pool(pool.clone());
        for (uid, tok) in [(ATTACKER, TOK_ATTACKER), (VICTIM, TOK_VICTIM)] {
            session_manager
                .create_session(uid.to_string(), tok.to_string())
                .await
                .expect("建会话");
        }

        let rate_limiter = shared::rate_limit::RateLimiter::new();
        let app = create_app_big_stack(pool.clone(), SCRATCH_DB, session_manager, rate_limiter);
        Some((app, pool))
    }

    fn fill_path(path: &str, victim: &str) -> String {
        let mut out = String::with_capacity(path.len());
        let mut rest = path;
        while let Some(open) = rest.find('{') {
            out.push_str(&rest[..open]);
            let Some(close) = rest[open..].find('}') else {
                rest = &rest[open..];
                break;
            };
            let name = &rest[open + 1..open + close];
            let val = match name {
                "id" => victim,
                "person" | "personId" | "person_id" => ADMIN_PROBE,
                "date" => "2026-01-01",
                "count" | "size" | "page" => "1",
                _ => "1",
            };
            out.push_str(val);
            rest = &rest[open + close + 1..];
        }
        out.push_str(rest);
        out
    }

    fn camel(key: &str) -> String {
        let mut out = String::new();
        for (i, part) in key.split('_').enumerate() {
            if i == 0 {
                out.push_str(part);
            } else {
                let mut c = part.chars();
                if let Some(f) = c.next() {
                    out.push(f.to_ascii_uppercase());
                    out.push_str(c.as_str());
                }
            }
        }
        out
    }

    fn mutate(v: &Value) -> Value {
        match v {
            Value::String(_) => Value::String("GATE-HACK".into()),
            Value::Bool(b) => Value::Bool(!*b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Number((i + 1).into())
                } else {
                    Value::Number(serde_json::Number::from_f64(1.0).unwrap())
                }
            }
            other => other.clone(),
        }
    }

    async fn row_json(pool: &deadpool_postgres::Pool, table: &str, id: &str) -> Option<String> {
        if id.is_empty() {
            return None;
        }
        let client = pool.get().await.ok()?;
        let sql = format!(
            "SELECT row_to_json(t)::text FROM (SELECT * FROM \"{table}\" WHERE id = $1) t"
        );
        client
            .query_opt(&sql, &[&id])
            .await
            .ok()
            .flatten()
            .map(|r| r.get::<_, String>(0))
    }

    async fn build_write_body(
        pool: &deadpool_postgres::Pool,
        table: &str,
        id: &str,
    ) -> Option<String> {
        let json = row_json(pool, table, id).await?;
        let parsed: Value = serde_json::from_str(&json).ok()?;
        let Value::Object(src) = parsed else {
            return None;
        };
        let mut out = Map::new();
        for (k, v) in src.iter() {
            if k == "id" {
                continue;
            }
            let mv = mutate(v);
            out.insert(k.clone(), mv.clone());
            let ck = camel(k);
            if ck != *k {
                out.insert(ck, mv);
            }
        }
        Some(Value::Object(out).to_string())
    }

    async fn fire(
        app: &axum::Router,
        method: &str,
        url: &str,
        token: Option<&str>,
        body: &str,
    ) -> Option<(StatusCode, String)> {
        let m = Method::from_bytes(method.as_bytes()).ok()?;
        let mut b = Request::builder()
            .method(m)
            .uri(url)
            .header("content-type", "application/json");
        if let Some(t) = token {
            b = b.header("authorization", format!("Bearer {t}"));
        }
        let req = b.body(Body::from(body.to_string())).ok()?;
        let resp = app.clone().oneshot(req).await.ok()?;
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20)
            .await
            .unwrap_or_default();
        Some((status, String::from_utf8_lossy(&bytes).into_owned()))
    }

    async fn count_rows(pool: &deadpool_postgres::Pool, table: &str) -> i64 {
        let client = match pool.get().await {
            Ok(c) => c,
            Err(_) => return -1,
        };
        let sql = format!("SELECT COUNT(*) FROM \"{table}\"");
        client
            .query_one(&sql, &[])
            .await
            .map(|r| r.get::<_, i64>(0))
            .unwrap_or(-1)
    }

    async fn run_case(
        app: &axum::Router,
        pool: &deadpool_postgres::Pool,
        c: &Case,
    ) -> Outcome {
        let path = fill_path(&c.path, &c.victim);
        let (body, before_row, before_count) = match c.kind.as_str() {
            "idor-write" => {
                let Some(b) = build_write_body(pool, &c.table, &c.victim).await else {
                    return Outcome::Unverified("受害者行不存在/无法取行".into());
                };
                (
                    b,
                    row_json(pool, &c.table, &c.victim).await,
                    None::<i64>,
                )
            }
            "soft-gate" if c.method == "POST" => {
                ("{\"toPerson\":\"person-admin\",\"roleId\":\"role-gate\"}".to_string(), None, Some(count_rows(pool, "x_empower").await))
            }
            "soft-gate" => (
                "{\"roleId\":\"role-hacked\",\"enabled\":false}".to_string(),
                row_json(pool, "x_empower", &c.victim).await,
                None,
            ),
            _ => ("{}".to_string(), None, None),
        };

        let Some((status, resp)) = fire(app, &c.method, &path, Some(TOK_ATTACKER), &body).await
        else {
            return Outcome::Unverified("请求无法构造".into());
        };
        let code = status.as_u16();
        let markers: Vec<&str> = c
            .leak
            .split('|')
            .filter(|m| !m.is_empty())
            .collect();

        match c.kind.as_str() {
            "soft-gate" => {
                if code == 401 || code == 403 {
                    return Outcome::Pass;
                }
                if c.method == "POST" {
                    let after = count_rows(pool, "x_empower").await;
                    if let Some(b) = before_count {
                        if after > b {
                            return Outcome::Leak(format!("2xx 且新增 x_empower 行 {b}->{after}"));
                        }
                    }
                } else {
                    let after = row_json(pool, "x_empower", &c.victim).await;
                    if let (Some(b), Some(a)) = (&before_row, &after) {
                        if b != a {
                            return Outcome::Leak("2xx 且 x_empower 行被修改".into());
                        }
                    }
                }
                let lowered = resp.to_lowercase();
                if lowered.contains("forbidden") || resp.contains("权限") {
                    Outcome::Pass
                } else {
                    Outcome::Unverified(format!("2xx 无拒绝语义且零写入 (status={code})"))
                }
            }
            "idor-write" => {
                if code == 401 || code == 403 {
                    return Outcome::Pass;
                }
                if !(200..300).contains(&code) {
                    return Outcome::Unverified(format!("status={code}"));
                }
                let after = row_json(pool, &c.table, &c.victim).await;
                match (&before_row, &after) {
                    (Some(b), a) if Some(b) != a.as_ref() => {
                        Outcome::Leak("2xx 且受害者行被改/被删".into())
                    }
                    _ => Outcome::Unverified(format!(
                        "2xx 但行未变 (status={code}, body={})",
                        resp.chars().take(90).collect::<String>()
                    )),
                }
            }
            "fail-open" => {
                if code == 401 || code == 403 {
                    return Outcome::Pass;
                }
                if !(200..300).contains(&code) {
                    return Outcome::Unverified(format!("status={code}"));
                }
                let after = row_json(pool, &c.table, &c.victim).await;
                let changed = match (&before_row, &after) {
                    (Some(_), a) => a.is_none(),
                    _ => false,
                };
                // fail-open 的痛点是 creator 为空时直接放行；行被删/状态被改即实证
                if changed || resp.contains("success") {
                    Outcome::Leak("2xx 且 fail-open 分支被执行".into())
                } else {
                    Outcome::Unverified(format!(
                        "2xx 但未见副作用 (status={code}, body={})",
                        resp.chars().take(90).collect::<String>()
                    ))
                }
            }
            "fake-gate" => {
                if code == 401 || code == 403 {
                    return Outcome::Pass;
                }
                if !(200..300).contains(&code) {
                    return Outcome::Unverified(format!("status={code}"));
                }
                // 假门端点恒定返回布尔；2xx 且无泄漏标记 = 布尔为 false（正确拒绝）
                match markers.iter().find(|m| resp.contains(**m)) {
                    Some(m) => Outcome::Leak(format!("2xx 响应体含 canary `{m}`")),
                    None => Outcome::Pass,
                }
            }
            _ => {
                if code == 401 || code == 403 {
                    return Outcome::Pass;
                }
                if !(200..300).contains(&code) {
                    return Outcome::Unverified(format!("status={code}"));
                }
                let hit = markers.iter().find(|m| resp.contains(**m));
                match hit {
                    Some(m) => Outcome::Leak(format!("2xx 响应体含 canary `{m}`")),
                    None => Outcome::Unverified(format!(
                        "2xx 但响应体无 canary（疑似种子漂移）body={}",
                        resp.chars().take(90).collect::<String>()
                    )),
                }
            }
        }
    }

    fn baseline_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/security_regression_baseline.txt")
    }

    fn load_baseline() -> BTreeSet<String> {
        std::fs::read_to_string(baseline_path())
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
            .filter_map(|l| l.split('\t').next().map(|s| s.trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn save_baseline(ids: &BTreeSet<(String, String)>) {
        let mut out = String::from(
            "# 安全回归门禁基线：每行 `用例 id \\t 非 PASS 状态`。\n\
             # 修复一条就删一行（单调下降）；只增不减视为回归。\n",
        );
        for (id, state) in ids {
            out.push_str(&format!("{id}\t{state}\n"));
        }
        std::fs::write(baseline_path(), out).expect("写基线");
        println!("[BASELINE] 已写入 {} 条", ids.len());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn security_regression_gate() {
        let Some((app, pool)) = setup().await else {
            return;
        };
        let cases = parse_cases();
        assert!(
            cases.len() >= 140,
            "用例表疑似损坏：只读到 {} 条（预期 >=140）",
            cases.len()
        );

        let mut results: Vec<(Case, Outcome)> = Vec::new();
        for c in &cases {
            let o = run_case(&app, &pool, c).await;
            println!(
                "[{}] {} | {} {} | ident={} | expect=deny | {}",
                o.state(),
                c.id,
                c.method,
                c.path,
                ATTACKER,
                match &o {
                    Outcome::Leak(r) | Outcome::Unverified(r) => r.as_str(),
                    Outcome::Pass => "denied",
                }
            );
            results.push((c.clone(), o));
        }

        let mut pass = 0;
        let mut leak = 0;
        let mut unverified = 0;
        let mut nonpass: BTreeSet<(String, String)> = BTreeSet::new();
        for (c, o) in &results {
            match o {
                Outcome::Pass => pass += 1,
                Outcome::Leak(_) => {
                    leak += 1;
                    nonpass.insert((c.id.clone(), "LEAK".into()));
                }
                Outcome::Unverified(_) => {
                    unverified += 1;
                    nonpass.insert((c.id.clone(), "UNVERIFIED".into()));
                }
            }
        }
        println!(
            "\n=== 安全回归门禁 ===\ncases={} pass={} leak={} unverified={}",
            cases.len(),
            pass,
            leak,
            unverified
        );

        // 先清理再断言：无论红绿都不把 scratch 库留给下一次
        drop_scratch(SCRATCH_DB).await;

        if std::env::var("SECURITY_GATE_UPDATE_BASELINE").as_deref() == Ok("1") {
            save_baseline(&nonpass);
            return;
        }

        let baseline = load_baseline();
        let base: BTreeSet<String> = baseline.iter().cloned().collect();
        let cur: BTreeSet<String> = nonpass.iter().map(|(id, _)| id.clone()).collect();
        let new_fail: Vec<&String> = cur.difference(&base).collect();
        let stale: Vec<&String> = base.difference(&cur).collect();

        if !new_fail.is_empty() || !stale.is_empty() {
            println!("\n--- 台账 ---");
            println!("基线 {} 条 / 本次非 PASS {} 条", base.len(), cur.len());
            for id in &new_fail {
                println!("NEW-FAIL\t{id}");
            }
            for id in &stale {
                println!("STALE-BASELINE\t{id}\t（已修复：请从基线删掉这一行）");
            }
        }

        assert!(
            new_fail.is_empty(),
            "门禁红：{} 条用例不在基线内（新增越权或新增未验证）。见上方 NEW-FAIL 列表。",
            new_fail.len()
        );
        assert!(
            stale.is_empty(),
            "门禁红：{} 条基线用例已转为 PASS 但基线未减一（请删对应行）。见上方 STALE-BASELINE。",
            stale.len()
        );
        println!(
            "\n门禁通过：非 PASS {} 条，与基线一致（leak={}, unverified={}）。",
            cur.len(),
            leak,
            unverified
        );
    }
}
