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
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::PathBuf;
    use tower::ServiceExt;

    const STACK: usize = 64 * 1024 * 1024;
    const SCRATCH_DB: &str = "oa_par_gate";
    const ATTACKER: &str = "audit-user";
    /// 受害者必须是**非管理员**：迁移种子里 person-admin 绑定了 role-admin，
    /// 用它做反向身份会把「管理员合法放行」误判成越权。
    const VICTIM: &str = "gate-victim";
    /// 假门探针：路径参数 `{person}` 传管理员，验证 handler 是否误把入参当权限主体
    const ADMIN_PROBE: &str = "gate-admin";
    const TOK_ATTACKER: &str = "tok-gate-attacker";
    const TOK_VICTIM: &str = "tok-gate-victim";
    /// 单条用例超时（秒）。越权验证都是短请求；长连接端点走 EXCLUDED-LONGCONN 不参与计时。
    const CASE_TIMEOUT_SECS: u64 = 5;
    /// 整轮预算（秒）。超预算 = 整轮 FAIL（RUN-BUDGET-EXCEEDED），防止大量超时把门禁拖绿。
    const RUN_BUDGET_SECS: u64 = 300;

    const CASES: &str = include_str!("security_regression_cases.tsv");
    const SEED: &str = include_str!("security_regression_seed.sql");
    const REV_SEED: &str = include_str!("security_regression_rev_seed.sql");
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
        /// id 承载位置：path | query | body
        carrier: String,
        /// query/body 口径里承载受害者身份的字段名（如 person_id / creator）
        field: String,
        /// T1 / T2 / legacy
        tier: String,
        /// both | fwd-only
        dir: String,
        /// 管理员分支策略：deny（管理员也不该看）/ allow（管理员是合法读者）/ unknown（未裁决）/ n/a（非 idor）
        admin_policy: String,
    }

    #[derive(Clone, Debug)]
    enum Outcome {
        Pass,
        Leak(String),
        Unverified(String),
        /// 写契约破坏：接口 2xx 但未落库/落库内容不符（不是越权，但必须红）
        Integrity(String),
        /// 长连接/SSE/流式上传端点：不参与 PASS/FAIL 判定，也不进基线
        Excluded,
    }

    impl Outcome {
        fn state(&self) -> &'static str {
            match self {
                Outcome::Pass => "PASS",
                Outcome::Leak(_) => "LEAK",
                Outcome::Unverified(_) => "UNVERIFIED",
                Outcome::Integrity(_) => "INTEGRITY",
                Outcome::Excluded => "EXCLUDED-LONGCONN",
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
                // 兼容第八轮的 7 列旧行：默认 path / 无字段 / legacy / fwd-only
                let carrier = f.get(7).map(|s| s.trim()).filter(|s| !s.is_empty()).unwrap_or("path");
                let field = f.get(8).map(|s| s.trim()).unwrap_or("");
                let tier = f.get(9).map(|s| s.trim()).filter(|s| !s.is_empty()).unwrap_or("legacy");
                let dir = f.get(10).map(|s| s.trim()).filter(|s| !s.is_empty()).unwrap_or_else(|| {
                    match f[1].trim() {
                        "idor-read" | "idor-write" => "both",
                        _ => "fwd-only",
                    }
                });
                let admin_policy = f
                    .get(11)
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .unwrap_or("n/a");
                Some(Case {
                    id: f[0].to_string(),
                    kind: f[1].to_string(),
                    method: f[2].to_string(),
                    path: f[3].to_string(),
                    table: f[4].to_string(),
                    victim: f[5].to_string(),
                    leak: f[6].to_string(),
                    carrier: carrier.to_string(),
                    field: field.to_string(),
                    tier: tier.to_string(),
                    dir: dir.to_string(),
                    admin_policy: admin_policy.to_string(),
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
                let first = s
                    .lines()
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take(90)
                    .collect::<String>();
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
        // 反向守卫：同一批表再灌一份 owner=audit-user 的行（id 前缀 rev-），供「B 操作 A」使用
        errs.extend(run_sql(&pool, REV_SEED, "rev-seed").await);
        errs.extend(run_sql(&pool, FIXTURES, "fixture").await);
        for e in &errs {
            eprintln!("[SEED-ERR] {e}");
        }
        println!(
            "seed 语句失败 {} 条（对应用例将落到 UNVERIFIED）",
            errs.len()
        );

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

    fn fill_path(path: &str, id: &str, person: &str) -> String {
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
                "id" => id,
                "person" | "personId" | "person_id" => person,
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

    /// 反向守卫：rev 方向的资源 id（同表、owner=audit-user 的镜像行）
    fn resource_id(c: &Case, rev: bool) -> String {
        if rev {
            format!("rev-{}", c.victim)
        } else {
            c.victim.clone()
        }
    }

    /// query/body 口径里被冒充的所有者值（fwd=受害者，rev=攻击者）
    fn owner_value(c: &Case, rev: bool) -> String {
        let unit = c.field.contains("unit");
        if rev {
            if unit { "unit-gate-a".into() } else { ATTACKER.into() }
        } else if unit {
            "unit-gate-v".into()
        } else {
            VICTIM.into()
        }
    }

    /// 冒名新建/冒名查询的请求体：owner 字段填目标身份，其余敏感字段打 canary
    fn build_spoof_body(c: &Case, rev: bool) -> String {
        let owner = owner_value(c, rev);
        let mut m = Map::new();
        if !c.field.is_empty() {
            m.insert(c.field.clone(), Value::String(owner.clone()));
            let ck = camel(&c.field);
            if ck != c.field {
                m.insert(ck, Value::String(owner));
            }
        }
        for k in [
            "status",
            "appealStatus",
            "auditStatus",
            "name",
            "title",
            "content",
            "subject",
            "reason",
        ] {
            m.insert(k.to_string(), Value::String("GATE-HACK".into()));
        }
        m.insert("date".to_string(), Value::String("2026-01-01".into()));
        Value::Object(m).to_string()
    }

    /// 从 2xx 响应里取新建行 id（`data.id`），用于「按返回 id 回表核对归属」
    fn response_id(resp: &str) -> Option<String> {
        let v: Value = serde_json::from_str(resp).ok()?;
        v.get("data")?
            .get("id")?
            .as_str()
            .map(|s| s.to_string())
    }

    /// 表内是否出现 canary（用于冒名新建/改写的副作用判定）
    async fn count_canary(pool: &deadpool_postgres::Pool, table: &str) -> i64 {
        if table.is_empty() {
            return -1;
        }
        let client = match pool.get().await {
            Ok(c) => c,
            Err(_) => return -1,
        };
        let sql = format!(
            "SELECT COUNT(*) FROM (SELECT row_to_json(t)::text AS s FROM \"{table}\" t) q \
             WHERE q.s LIKE '%GATE-HACK%'"
        );
        client
            .query_one(&sql, &[])
            .await
            .map(|r| r.get::<_, i64>(0))
            .unwrap_or(-1)
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
        let sql =
            format!("SELECT row_to_json(t)::text FROM (SELECT * FROM \"{table}\" WHERE id = $1) t");
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

    // ── 深分页 / 写落库正控制（第十一轮） ─────────────────────────────

    const PAGE_SIZE: i64 = 2;
    const DETAIL_OWN_IDS: [&str; 6] = [
        "pg-a-01", "pg-a-02", "pg-a-03", "pg-a-04", "pg-a-05", "pg-a-06",
    ];
    const DETAIL_PREV_EXPECT: [&str; 5] =
        ["pg-a-01", "pg-a-02", "pg-a-03", "pg-a-04", "pg-a-06"];
    const DETAIL_VICTIM_IDS: [&str; 4] = ["pg-a-00", "pg-v-01", "pg-v-02", "pg-v-03"];
    const SELFHOLIDAY_IDS: [&str; 8] = [
        "pg-s-01", "pg-s-02", "pg-s-03", "pg-s-04", "pg-s-05", "pg-s-06",
        "hol-admin", "rev-hol-admin",
    ];
    const SELFHOLIDAY_PAGE_SIZE: i64 = 3;
    const MONTH_IDS: [&str; 6] = [
        "pg-m-01", "pg-m-02", "pg-m-03", "pg-m-04", "pg-m-05", "pg-m-06",
    ];
    const TOPUNIT_IDS: [&str; 6] = [
        "pg-d-01", "pg-d-02", "pg-d-03", "pg-d-04", "pg-d-05", "pg-d-06",
    ];
    const TOPUNIT_EXCLUDED_IDS: [&str; 2] = ["pg-du-01", "pg-du-02"];

    async fn page_get(
        app: &axum::Router,
        token: &str,
        url: &str,
    ) -> Result<(Vec<String>, Vec<String>), String> {
        let Some((status, resp)) = fire(app, "GET", url, Some(token), "").await else {
            return Err("请求无法构造".into());
        };
        let code = status.as_u16();
        if !(200..300).contains(&code) {
            return Err(format!("status={code}"));
        }
        let v: Value =
            serde_json::from_str(&resp).map_err(|e| format!("响应非 JSON: {e}"))?;
        let arr = v
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| "响应无 data 数组".to_string())?;
        let ids: Vec<String> = arr
            .iter()
            .filter_map(|r| r.get("id").and_then(|x| x.as_str()).map(|s| s.to_string()))
            .collect();
        let persons: Vec<String> = arr
            .iter()
            .map(|r| {
                r.get("personId")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string()
            })
            .collect();
        Ok((ids, persons))
    }

    /// 连续翻 `pages` 页，返回每页 id。next 与 prev 都以「本页最后一个 id」为下一页游标：
    /// next 的 predicate 是 `>`，prev 是 `<`，两者都取下界/上界的最后一项。
    async fn walk_pages<F>(
        app: &axum::Router,
        token: &str,
        path: F,
        first_cursor: &str,
        pages: usize,
    ) -> Result<Vec<(Vec<String>, Vec<String>)>, String>
    where
        F: Fn(&str) -> String,
    {
        let mut cursor = first_cursor.to_string();
        let mut out = Vec::new();
        for _ in 0..pages {
            let url = path(&cursor);
            let (ids, persons) = page_get(app, token, &url).await?;
            let Some(last) = ids.last().cloned() else {
                return Err(format!("游标 {cursor} 后无数据（页提前结束）"));
            };
            cursor = last;
            out.push((ids, persons));
        }
        Ok(out)
    }

    fn check_page_suite(
        label: &str,
        pages: &[(Vec<String>, Vec<String>)],
        expected: &[&str],
        forbid: &[&str],
        only_person: Option<&str>,
    ) -> Outcome {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for (ids, persons) in pages {
            for (i, id) in ids.iter().enumerate() {
                if !seen.insert(id.clone()) {
                    return Outcome::Leak(format!("{label}: 第 3 页前出现重复 id {id}"));
                }
                if forbid.contains(&id.as_str()) {
                    return Outcome::Leak(format!("{label}: 出现他人数据 {id}"));
                }
                if let Some(p) = only_person {
                    if persons.get(i).map(String::as_str) != Some(p) {
                        return Outcome::Leak(format!(
                            "{label}: 行 {id} 归属 {}，期望仅 {p}",
                            persons.get(i).cloned().unwrap_or_default()
                        ));
                    }
                }
            }
        }
        let want: BTreeSet<String> = expected.iter().map(|s| s.to_string()).collect();
        if seen != want {
            let missing: Vec<&String> = want.difference(&seen).collect();
            let extra: Vec<&String> = seen.difference(&want).collect();
            return Outcome::Unverified(format!(
                "{label}: 集合不符（漏={:?} 多={:?}）",
                missing, extra
            ));
        }
        Outcome::Pass
    }

    /// 本人写必须真落库：2xx + 返回 id + 回表命中 + 归属/内容一致。
    async fn self_write_roundtrip(
        app: &axum::Router,
        pool: &deadpool_postgres::Pool,
    ) -> Outcome {
        let marker = "GATE-SELF-WRITE-11";
        let body = format!(
            r#"{{"holidayDate":"2026-03-01","reason":"{marker}"}}"#
        );
        let Some((status, resp)) = fire(
            app,
            "POST",
            "/api/attendance/assemble/control/attendanceselfholiday",
            Some(TOK_ATTACKER),
            &body,
        )
        .await
        else {
            return Outcome::Integrity("请求无法构造".into());
        };
        if !(200..300).contains(&status.as_u16()) {
            return Outcome::Integrity(format!("本人写被拒/失败 status={}", status.as_u16()));
        }
        let Some(id) = response_id(&resp) else {
            return Outcome::Integrity(format!(
                "2xx 但未返回 data.id body={}",
                resp.chars().take(90).collect::<String>()
            ));
        };
        let client = match pool.get().await {
            Ok(c) => c,
            Err(_) => return Outcome::Integrity("回表连接失败".into()),
        };
        let row = client
            .query_opt(
                "SELECT person_id, reason FROM x_attendance_selfholiday WHERE id = $1",
                &[&id],
            )
            .await
            .ok()
            .flatten();
        let outcome = match row {
            None => Outcome::Integrity("2xx 但行未落库".into()),
            Some(r) => {
                let person = r.get::<_, Option<String>>(0).unwrap_or_default();
                let reason = r.get::<_, Option<String>>(1).unwrap_or_default();
                if person != ATTACKER {
                    Outcome::Integrity(format!("落库归属错误 person={person}"))
                } else if !reason.contains(marker) {
                    Outcome::Integrity(format!("落库内容不符 reason={reason}"))
                } else {
                    Outcome::Pass
                }
            }
        };
        let _ = client
            .execute("DELETE FROM x_attendance_selfholiday WHERE id = $1", &[&id])
            .await;
        outcome
    }

    async fn pagination_suite(app: &axum::Router) -> Vec<(String, Outcome)> {
        let detail_base =
            "/api/attendance/assemble/control/attendancedetail/filter/list";
        let mut out = Vec::new();

        // 1) detail next：3 页 2/2/2，仅本人、无受害者。
        let next_path = |c: &str| {
            format!("{detail_base}/{c}/next/{PAGE_SIZE}")
        };
        let pages = walk_pages(app, TOK_ATTACKER, next_path, "(0)", 3).await;
        let outcome = match pages {
            Ok(p) => check_page_suite(
                "page-attendance-detail-next",
                &p,
                &DETAIL_OWN_IDS,
                &DETAIL_VICTIM_IDS,
                Some(ATTACKER),
            ),
            Err(e) => Outcome::Unverified(e),
        };
        out.push(("page-attendance-detail-next".into(), outcome));

        // 2) detail prev：从哨兵行往回翻 3 页，覆盖全部 6 行；并做一次
        //    「next 到第 2 页再 prev 回第 1 页」的精确回翻断言（O33 易错点）。
        let prev_path = |c: &str| {
            format!("{detail_base}/{c}/prev/{PAGE_SIZE}")
        };
        let pages = walk_pages(app, TOK_ATTACKER, prev_path, "pg-a-00", 3).await;
        let outcome = match pages {
            Ok(p) => check_page_suite(
                "page-attendance-detail-prev",
                &p,
                &DETAIL_PREV_EXPECT,
                &DETAIL_VICTIM_IDS,
                Some(ATTACKER),
            ),
            Err(e) => Outcome::Unverified(e),
        };
        out.push(("page-attendance-detail-prev".into(), outcome));

        // 2b) next 第 1/2 页拿到 4 行；以第 2 页首行 id 为游标 prev，必须恰好回第 1 页。
        let forward = walk_pages(app, TOK_ATTACKER, next_path, "(0)", 2).await;
        let outcome = match forward {
            Ok(fp) => {
                let p1: BTreeSet<String> = fp[0].0.iter().cloned().collect();
                let p2_first = fp[1].0.first().cloned().unwrap_or_default();
                match page_get(
                    app,
                    TOK_ATTACKER,
                    &format!("{detail_base}/{p2_first}/prev/{PAGE_SIZE}"),
                )
                .await
                {
                    Ok((ids, _)) => {
                        let back: BTreeSet<String> = ids.iter().cloned().collect();
                        if back == p1 {
                            Outcome::Pass
                        } else {
                            Outcome::Leak(format!(
                                "prev 回翻不等于第 1 页（back={:?} p1={:?}）",
                                back, p1
                            ))
                        }
                    }
                    Err(e) => Outcome::Unverified(e),
                }
            }
            Err(e) => Outcome::Unverified(e),
        };
        out.push(("page-attendance-detail-next-then-prev".into(), outcome));

        // 3) selfholiday next：全组织端点，验证并列 + NULL 不重不漏。
        let sh_path = |c: &str| {
            format!(
                "/api/attendance/assemble/control/attendanceselfholiday/filter/list/{c}/next/{SELFHOLIDAY_PAGE_SIZE}"
            )
        };
        let pages = walk_pages(app, TOK_ATTACKER, sh_path, "(0)", 3).await;
        let outcome = match pages {
            Ok(p) => check_page_suite(
                "page-attendance-selfholiday-next",
                &p,
                &SELFHOLIDAY_IDS,
                &[],
                None,
            ),
            Err(e) => Outcome::Unverified(e),
        };
        out.push(("page-attendance-selfholiday-next".into(), outcome));

        // 4) statisticshow personMonth next：数字月份并列 + NULL。
        let month_path = |c: &str| {
            format!(
                "/api/attendance/assemble/control/statisticshow/filter/personMonth/list/{c}/next/{PAGE_SIZE}"
            )
        };
        let month_path_prev = |c: &str| {
            format!(
                "/api/attendance/assemble/control/statisticshow/filter/personMonth/list/{c}/prev/{PAGE_SIZE}"
            )
        };
        let pages = walk_pages(app, TOK_ATTACKER, month_path, "(0)", 3).await;
        let (next_p1, next_p2_first): (Option<Vec<String>>, Option<String>) =
            match pages.as_ref() {
                Ok(p) => (Some(p[0].0.clone()), p[1].0.first().cloned()),
                Err(_) => (None, None),
            };
        let outcome = match pages {
            Ok(p) => check_page_suite(
                "page-statisticshow-person-month-next",
                &p,
                &MONTH_IDS,
                &[],
                None,
            ),
            Err(e) => Outcome::Unverified(e),
        };
        out.push(("page-statisticshow-person-month-next".into(), outcome));

        // 4b) prev 顺序还原（O33 最易错点）：先 next 到第 2 页，再以第 2 页首行为游标
        //     往回翻一页；personMonth prev 有 `rows.reverse()`，输出必须与第 1 页
        //     完全同序。若漏掉 reverse，会得到反向序列。
        let outcome = match (next_p1, next_p2_first) {
            (Some(p1), Some(p2_first)) => {
                match page_get(
                    app,
                    TOK_ATTACKER,
                    &month_path_prev(&p2_first),
                )
                .await
                {
                    Ok((ids, _)) => {
                        if ids == p1 {
                            Outcome::Pass
                        } else {
                            Outcome::Leak(format!(
                                "page-statisticshow-person-month-prev-order: prev 顺序未还原 got={:?} want={:?}",
                                ids, p1
                            ))
                        }
                    }
                    Err(e) => Outcome::Unverified(e),
                }
            }
            _ => Outcome::Unverified("next 前两页收集失败，无法比对 prev 顺序".into()),
        };
        out.push((
            "page-statisticshow-person-month-prev-order".into(),
            outcome,
        ));

        // 5) statisticshow topUnitDay next：work_date 并列 + NULL，且 unit_id 非空行不得越界。
        let top_path = |c: &str| {
            format!(
                "/api/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{c}/next/{PAGE_SIZE}"
            )
        };
        let pages = walk_pages(app, TOK_ATTACKER, top_path, "(0)", 3).await;
        let outcome = match pages {
            Ok(p) => check_page_suite(
                "page-statisticshow-top-unit-day-next",
                &p,
                &TOPUNIT_IDS,
                &TOPUNIT_EXCLUDED_IDS,
                None,
            ),
            Err(e) => Outcome::Unverified(e),
        };
        out.push(("page-statisticshow-top-unit-day-next".into(), outcome));
        out
    }

    /// query 口径：把身份字段拼到 URL（path 已有 query 时用 &）
    fn with_carrier(c: &Case, path: String, rev: bool) -> String {
        if c.carrier != "query" || c.field.is_empty() {
            return path;
        }
        let sep = if path.contains('?') { '&' } else { '?' };
        format!("{path}{sep}{}={}", c.field, owner_value(c, rev))
    }

    /// 单向/双向执行同一条用例。`rev=true` 表示「B 操作 A」：受害者身份去操作攻击者的镜像行。
    async fn run_case(
        app: &axum::Router,
        pool: &deadpool_postgres::Pool,
        c: &Case,
        rev: bool,
    ) -> Outcome {
        if c.kind == "excluded" {
            return Outcome::Excluded;
        }
        let rid = resource_id(c, rev);
        let token = if rev { TOK_VICTIM } else { TOK_ATTACKER };
        let person = if rev { ATTACKER } else { ADMIN_PROBE };
        let path = with_carrier(c, fill_path(&c.path, &rid, person), rev);
        let markers: Vec<String> = c
            .leak
            .split('|')
            .filter(|m| !m.is_empty())
            .map(|m| {
                if rev {
                    format!("rev-{m}")
                } else {
                    m.to_string()
                }
            })
            .collect();

        let (body, before_row, before_count) = match c.kind.as_str() {
            "idor-write" => {
                let Some(b) = build_write_body(pool, &c.table, &rid).await else {
                    return Outcome::Unverified(if rev {
                        "反向镜像行不存在".into()
                    } else {
                        "受害者行不存在/无法取行".into()
                    });
                };
                (b, row_json(pool, &c.table, &rid).await, None::<i64>)
            }
            "spoof-write" => {
                let before = count_canary(pool, &c.table).await;
                (build_spoof_body(c, rev), None, Some(before))
            }
            "spoof-read" => (build_spoof_body(c, rev), None, None),
            "soft-gate" if c.method == "POST" => (
                "{\"toPerson\":\"person-admin\",\"roleId\":\"role-gate\"}".to_string(),
                None,
                Some(count_rows(pool, "x_empower").await),
            ),
            "soft-gate" => (
                "{\"roleId\":\"role-hacked\",\"enabled\":false}".to_string(),
                row_json(pool, "x_empower", &c.victim).await,
                None,
            ),
            _ => ("{}".to_string(), None, None),
        };

        let Some((status, resp)) = fire(app, &c.method, &path, Some(token), &body).await else {
            return Outcome::Unverified("请求无法构造".into());
        };
        let code = status.as_u16();

        match c.kind.as_str() {
            "spoof-write" => {
                if code == 401 || code == 403 {
                    return Outcome::Pass;
                }
                if !(200..300).contains(&code) {
                    return Outcome::Unverified(format!("status={code}"));
                }
                // 优先按响应返回的新行 id 回表核对归属：owner 字段等于被冒充身份即越权
                if let Some(newid) = response_id(&resp) {
                    if let Some(row) = row_json(pool, &c.table, &newid).await {
                        let owner = owner_value(c, rev);
                        if row.contains(&owner) {
                            return Outcome::Leak(format!(
                                "2xx 且新行 {newid} 的 owner={owner}（冒名落库）"
                            ));
                        }
                    }
                }
                let after = count_canary(pool, &c.table).await;
                if let Some(b) = before_count {
                    if after >= 0 && after > b {
                        return Outcome::Leak(format!("2xx 且新增 canary 行 {b}->{after}"));
                    }
                }
                Outcome::Unverified(format!(
                    "2xx 但表内无 canary 变化 (status={code}, body={})",
                    resp.chars().take(90).collect::<String>()
                ))
            }
            "spoof-read" => {
                if code == 401 || code == 403 {
                    return Outcome::Pass;
                }
                if !(200..300).contains(&code) {
                    return Outcome::Unverified(format!("status={code}"));
                }
                match markers.iter().find(|m| resp.contains(m.as_str())) {
                    Some(m) => Outcome::Leak(format!("2xx 响应体含 canary `{m}`")),
                    None => Outcome::Unverified(format!(
                        "2xx 但响应体无 canary（疑似种子漂移）body={}",
                        resp.chars().take(90).collect::<String>()
                    )),
                }
            }
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
                let after = row_json(pool, &c.table, &rid).await;
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
                match markers.iter().find(|m| resp.contains(m.as_str())) {
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
                let hit = markers.iter().find(|m| resp.contains(m.as_str()));
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

    /// 双向聚合：任一方向 LEAK 即 LEAK；否则 INTEGRITY；否则 UNVERIFIED；两向都拒绝才 PASS。
    fn aggregate(fwd: Outcome, rev: Option<Outcome>) -> Outcome {
        let mut any_leak = false;
        let mut any_integrity = false;
        let mut any_unv = false;
        let mut msgs = Vec::new();
        for (label, o) in [("fwd", Some(&fwd)), ("rev", rev.as_ref())] {
            let Some(o) = o else { continue };
            match o {
                Outcome::Leak(m) => {
                    any_leak = true;
                    msgs.push(format!("{label}=LEAK({m})"));
                }
                Outcome::Integrity(m) => {
                    any_integrity = true;
                    msgs.push(format!("{label}=INTEGRITY({m})"));
                }
                Outcome::Unverified(m) => {
                    any_unv = true;
                    msgs.push(format!("{label}=UNVERIFIED({m})"));
                }
                Outcome::Pass => msgs.push(format!("{label}=PASS")),
                Outcome::Excluded => msgs.push(format!("{label}=EXCLUDED")),
            }
        }
        let joined = msgs.join(" | ");
        if any_leak {
            Outcome::Leak(joined)
        } else if any_integrity {
            Outcome::Integrity(joined)
        } else if any_unv {
            Outcome::Unverified(joined)
        } else {
            Outcome::Pass
        }
    }

    fn baseline_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/security_regression_baseline.txt")
    }

    /// 基线每行 `用例 id \t 非 PASS 状态`。状态必须一起读：只比 id 会漏掉
    /// 「基线内 UNVERIFIED 恶化为 LEAK」这种最隐蔽的失效。
    fn load_baseline() -> BTreeMap<String, String> {
        std::fs::read_to_string(baseline_path())
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
            .filter_map(|l| {
                let mut f = l.split('\t');
                let id = f.next()?.trim();
                if id.is_empty() {
                    return None;
                }
                let state = f.next().map(|s| s.trim()).unwrap_or("UNKNOWN");
                Some((id.to_string(), state.to_string()))
            })
            .collect()
    }

    fn save_baseline(states: &BTreeMap<String, String>) {
        let mut out = String::from(
            "# 安全回归门禁基线：每行 `用例 id \\t 非 PASS 状态`。\n\
             # 第十一轮已完成「状态列」一次性对齐：本文件每条都带 LEAK/UNVERIFIED/INTEGRITY。\n\
             # 只比 id 的旧 runner 会漏掉「UNVERIFIED 恶化成 LEAK」；现版本对这类漂移直接判红。\n\
             # 若在首次切换后看到 STATE-DRIFT，请先确认是不是对齐差异，不要误读成业务回归。\n\
             # 修复一条就删一行（单调下降）；只增不减视为回归。\n",
        );
        for (id, state) in states {
            out.push_str(&format!("{id}\t{state}\n"));
        }
        std::fs::write(baseline_path(), out).expect("写基线");
        println!("[BASELINE] 已写入 {} 条", states.len());
    }

    /// 纯函数：比较基线与本次状态，返回（全部状态变化，UNVERIFIED→LEAK 的致命漂移）。
    fn state_drift(
        base: &BTreeMap<String, String>,
        cur: &BTreeMap<String, String>,
    ) -> (Vec<(String, String, String)>, Vec<String>) {
        let mut drift = Vec::new();
        let mut drift_to_leak = Vec::new();
        for (id, cs) in cur {
            let Some(bs) = base.get(id) else { continue };
            if bs == cs {
                continue;
            }
            drift.push((id.clone(), bs.clone(), cs.clone()));
            if bs == "UNVERIFIED" && cs == "LEAK" {
                drift_to_leak.push(id.clone());
            }
        }
        (drift, drift_to_leak)
    }

    /// 一次性对齐：只补齐/替换 UNKNOWN 的状态，不覆盖已有真实状态；
    /// 返回是否发生了对齐写入。
    fn align_unknown_states(
        base: &mut BTreeMap<String, String>,
        cur: &BTreeMap<String, String>,
    ) -> bool {
        let mut changed = false;
        for (id, cs) in cur {
            if let Some(bs) = base.get_mut(id) {
                if bs == "UNKNOWN" {
                    *bs = cs.clone();
                    changed = true;
                }
            }
        }
        changed
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn security_regression_gate() {
        let Some((app, pool)) = setup().await else {
            return;
        };
        let cases = parse_cases();
        assert!(
            cases.len() >= 180,
            "用例表疑似损坏：只读到 {} 条（预期 >=180）",
            cases.len()
        );

        let case_timeout = std::env::var("SECURITY_GATE_CASE_TIMEOUT")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(CASE_TIMEOUT_SECS);
        let reverse_on = std::env::var("SECURITY_GATE_REVERSE").as_deref() != Ok("0");
        let started = std::time::Instant::now();
        let mut budget_exceeded = false;

        // 伪用例（不占 TSV）：深分页 + 本人写落库正控制。
        // 放在主循环前：此时夹具最干净，且 self-write 会自行清理。
        let mut extras: Vec<(String, Outcome)> = Vec::new();
        match tokio::time::timeout(
            std::time::Duration::from_secs(30),
            pagination_suite(&app),
        )
        .await
        {
            Ok(mut list) => extras.append(&mut list),
            Err(_) => extras.push(("page-suite".into(), Outcome::Unverified("TIMEOUT".into()))),
        }
        let sw = tokio::time::timeout(
            std::time::Duration::from_secs(case_timeout),
            self_write_roundtrip(&app, &pool),
        )
        .await
        .unwrap_or_else(|_| Outcome::Unverified("TIMEOUT".into()));
        extras.push(("self-write-roundtrip".into(), sw));
        for (id, o) in &extras {
            println!(
                "[{}] {id} | self-write/pagination | {}",
                o.state(),
                match o {
                    Outcome::Leak(r) | Outcome::Unverified(r) | Outcome::Integrity(r) => r.as_str(),
                    Outcome::Pass => "ok",
                    Outcome::Excluded => "excluded",
                }
            );
        }

        let mut results: Vec<(Case, Outcome)> = Vec::new();
        let mut excluded = 0usize;
        let mut both = 0usize;
        let mut fwd_only = 0usize;
        for c in &cases {
            if started.elapsed().as_secs() > RUN_BUDGET_SECS {
                budget_exceeded = true;
                break;
            }
            if c.kind == "excluded" {
                println!("[EXCLUDED-LONGCONN] {} | {} {}", c.id, c.method, c.path);
                excluded += 1;
                continue;
            }
            let fwd = match tokio::time::timeout(
                std::time::Duration::from_secs(case_timeout),
                run_case(&app, &pool, c, false),
            )
            .await
            {
                Ok(o) => o,
                Err(_) => Outcome::Unverified("TIMEOUT".into()),
            };
            let rev = if reverse_on && c.dir == "both" {
                Some(
                    match tokio::time::timeout(
                        std::time::Duration::from_secs(case_timeout),
                        run_case(&app, &pool, c, true),
                    )
                    .await
                    {
                        Ok(o) => o,
                        Err(_) => Outcome::Unverified("TIMEOUT".into()),
                    },
                )
            } else {
                None
            };
            let is_both = c.dir == "both" && rev.is_some();
            if is_both {
                both += 1;
            } else {
                fwd_only += 1;
            }
            let o = aggregate(fwd, rev);
            println!(
                "[{}] {} | {} {} | tier={} | {} | {}",
                o.state(),
                c.id,
                c.method,
                c.path,
                c.tier,
                if is_both { "both" } else { "fwd-only" },
                match &o {
                    Outcome::Leak(r) | Outcome::Unverified(r) | Outcome::Integrity(r) => {
                        r.as_str()
                    }
                    Outcome::Pass => "denied",
                    Outcome::Excluded => "excluded",
                }
            );
            results.push((c.clone(), o));
        }

        let mut pass = 0;
        let mut leak = 0;
        let mut unverified = 0;
        let mut integrity = 0;
        let mut nonpass: BTreeMap<String, String> = BTreeMap::new();
        for (c, o) in &results {
            match o {
                Outcome::Pass => pass += 1,
                Outcome::Leak(_) => {
                    leak += 1;
                    nonpass.insert(c.id.clone(), "LEAK".into());
                }
                Outcome::Unverified(_) => {
                    unverified += 1;
                    nonpass.insert(c.id.clone(), "UNVERIFIED".into());
                }
                Outcome::Integrity(_) => {
                    integrity += 1;
                    nonpass.insert(c.id.clone(), "INTEGRITY".into());
                }
                Outcome::Excluded => {}
            }
        }
        for (id, o) in &extras {
            match o {
                Outcome::Pass => pass += 1,
                Outcome::Leak(_) => {
                    leak += 1;
                    nonpass.insert(id.clone(), "LEAK".into());
                }
                Outcome::Unverified(_) => {
                    unverified += 1;
                    nonpass.insert(id.clone(), "UNVERIFIED".into());
                }
                Outcome::Integrity(_) => {
                    integrity += 1;
                    nonpass.insert(id.clone(), "INTEGRITY".into());
                }
                Outcome::Excluded => {}
            }
        }
        println!(
            "\n=== 安全回归门禁 ===\ncases={} pass={} leak={} unverified={} integrity={} excluded={} both={} fwd-only={} elapsed={}s",
            results.len() + extras.len(),
            pass,
            leak,
            unverified,
            integrity,
            excluded,
            both,
            fwd_only,
            started.elapsed().as_secs()
        );

        // admin_policy 三档显式读数：unknown 就是「还没想清楚的有多少」。
        let idor_cases: Vec<&Case> = cases
            .iter()
            .filter(|c| c.kind == "idor-read" || c.kind == "idor-write")
            .collect();
        let deny = idor_cases.iter().filter(|c| c.admin_policy == "deny").count();
        let allow = idor_cases.iter().filter(|c| c.admin_policy == "allow").count();
        let unknown = idor_cases.iter().filter(|c| c.admin_policy == "unknown").count();
        println!(
            "admin-policy: idor={} deny={} allow={} unknown={}",
            idor_cases.len(),
            deny,
            allow,
            unknown
        );
        for c in &idor_cases {
            if c.admin_policy == "unknown" {
                if let Some(state) = nonpass.get(&c.id) {
                    println!("[ADMIN-POLICY-UNKNOWN] {}\tstate={}", c.id, state);
                }
            }
        }

        // 先清理再断言：无论红绿都不把 scratch 库留给下一次
        drop_scratch(SCRATCH_DB).await;

        // 整轮超预算 = FAIL：不能让大量超时把门禁拖绿
        if budget_exceeded {
            panic!(
                "RUN-BUDGET-EXCEEDED：整轮超过 {RUN_BUDGET_SECS}s 预算，未跑完的用例一律视为未验证"
            );
        }

        if std::env::var("SECURITY_GATE_UPDATE_BASELINE").as_deref() == Ok("1") {
            save_baseline(&nonpass);
            return;
        }

        let mut baseline = load_baseline();

        // 一次性对齐（只在格式迁移时用）：只补 UNKNOWN 状态，不新增基线条目。
        if std::env::var("SECURITY_GATE_ALIGN_BASELINE").as_deref() == Ok("1") {
            let changed = align_unknown_states(&mut baseline, &nonpass);
            if changed {
                save_baseline(&baseline);
                println!("[BASELINE-ALIGN] 已补齐缺失状态列；请审阅并提交一次");
            } else {
                println!("[BASELINE-ALIGN] 无需对齐：所有既有条目都带状态");
            }
            return;
        }

        // 正常跑不允许残留 UNKNOWN：否则漂移判定会静默失明。
        let unaligned: Vec<&String> = baseline
            .iter()
            .filter(|(_, s)| s.as_str() == "UNKNOWN")
            .map(|(id, _)| id)
            .collect();
        assert!(
            unaligned.is_empty(),
            "门禁红：基线有 {} 条缺少状态列（先跑一次 `SECURITY_GATE_ALIGN_BASELINE=1` 对齐）",
            unaligned.len()
        );

        let (drift, drift_to_leak) = state_drift(&baseline, &nonpass);
        let new_fail: Vec<&String> = nonpass
            .keys()
            .filter(|id| !baseline.contains_key(*id))
            .collect();
        let stale: Vec<&String> = baseline
            .keys()
            .filter(|id| !nonpass.contains_key(*id))
            .collect();

        if !drift.is_empty() || !new_fail.is_empty() || !stale.is_empty() {
            println!("\n--- 台账 ---");
            println!(
                "基线 {} 条 / 本次非 PASS {} 条",
                baseline.len(),
                nonpass.len()
            );
            for id in &new_fail {
                println!("NEW-FAIL\t{id}");
            }
            for id in &stale {
                println!("STALE-BASELINE\t{id}\t（已修复：请从基线删掉这一行）");
            }
            for (id, bs, cs) in &drift {
                println!("[STATE-DRIFT] {id}\t{bs} -> {cs}");
            }
        }

        // 致命：基线内 UNVERIFIED 恶化为 LEAK。这是「已知问题悄悄变严重」的唯一信号。
        for id in &drift_to_leak {
            println!("[DRIFT-TO-LEAK] {id}\tUNVERIFIED -> LEAK");
        }
        assert!(
            drift_to_leak.is_empty(),
            "门禁红：{} 条基线内 UNVERIFIED 已恶化为 LEAK（见上方 DRIFT-TO-LEAK）",
            drift_to_leak.len()
        );
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
            "\n门禁通过：非 PASS {} 条，与基线一致（leak={}, unverified={}, integrity={}）。",
            nonpass.len(),
            leak,
            unverified,
            integrity
        );
    }

    #[test]
    fn drift_to_leak_is_fatal() {
        let mut base = BTreeMap::new();
        base.insert("case-a".to_string(), "UNVERIFIED".to_string());
        base.insert("case-b".to_string(), "UNVERIFIED".to_string());
        base.insert("case-c".to_string(), "LEAK".to_string());
        let mut cur = BTreeMap::new();
        cur.insert("case-a".to_string(), "LEAK".to_string());
        cur.insert("case-b".to_string(), "UNVERIFIED".to_string());
        cur.insert("case-c".to_string(), "UNVERIFIED".to_string());
        let (drift, fatal) = state_drift(&base, &cur);
        assert_eq!(drift.len(), 2, "应看到两条状态变化");
        assert_eq!(fatal, vec!["case-a".to_string()], "只有 UNVERIFIED->LEAK 致命");
        assert!(
            !fatal.contains(&"case-c".to_string()),
            "LEAK->UNVERIFIED 只记 STATE-DRIFT，不升级为致命"
        );
    }
}
