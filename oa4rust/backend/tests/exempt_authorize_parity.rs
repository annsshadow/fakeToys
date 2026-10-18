// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 豁免表与权限注册表一致性实测（四轮 §三.7 的第七格）。
//!
//! ## 这一格在证什么
//! `auth_middleware`（token.rs:142-147）命中 `AUTH_EXEMPT_PATHS` / `AUTH_EXEMPT_PREFIXES`
//! 时**直接早退且不插入 Session**；`authorize_middleware`（rbac.rs:538-544）只在注册表
//! 判定为 `Public` 时放行，否则从 extensions 取 `Session`，取不到就是 401。
//! 于是「豁免 ∧ ¬Public」这条路对**任何人**都是死的——包括带合法会话的 admin。
//! 尺子（`scripts/audit_write_gating.mjs` 第 6c 段）把这一族量化成
//! `docs/audits/artifacts/dead_exempt_routes.tsv`：修注册表建模后 19 条（8 写 + 11 只读）。
//!
//! 本文件不引用那份静态读数当结论，而是**逐条打真实全栈 HTTP**：静态推断在本轮已经
//! 错过四次口径（双锁、手工会话解析、路由配对越界、注册表三种注册写法），所以只认实测。
//!
//! ## 不变量
//! 1. 对所有列名路由：**带合法 admin 会话**必须拿不到 401（401=中间件把路拦死）。
//!    这条与「该路本来该公开还是该登录」无关，是纯粹的可达性下限。
//! 2. 对设计上必须匿名可达的两条（注册 `/api/person/regist`、登录验证码
//!    `/api/secret/captcha/verify`）：**匿名**也必须拿不到 401。
//!    注：注册路由的 handler 侧校验/限流由 `regist` 自己负责，本文件用空体触发其校验，
//!    不产生真实注册行。
//!
//! 状态码逐臂 println（含响应体）供账本引用；401 之外的读数（400/403/200-业务错）
//! 都算「路由活着」，因为这一格只量可达性，不量业务对错。
//!
//! 需要真实 PostgreSQL：
//! `cargo test --test exempt_authorize_parity -- --ignored --nocapture`
#[cfg(test)]
mod exempt_authorize_parity {
    use axum::http::Method;
    use serde_json::{json, Value};

    /// 带会话也必须可达的路由（可达性下限，与公开/登录意图无关）。
    /// 用函数而非 `const`：`json!` 不是 const 函数，探针体需要真实对象。
    fn must_reach_with_session() -> Vec<(&'static str, Method, &'static str, Value)> {
        vec![
            // 6c D1：被同层 `{id}` 通配豁免吞掉的写路由
            (
                "invoice_create",
                Method::POST,
                "/api/general/assemble/control/invoice/create",
                json!({ "name": "r48-parity-invoice-create" }),
            ),
            (
                "invoice_upload",
                Method::POST,
                "/api/general/assemble/control/invoice/upload",
                json!({ "name": "r48-parity-invoice-upload", "creator": "r48-spoofed-actor" }),
            ),
            (
                "meeting_create_alias",
                Method::POST,
                "/api/meeting/assemble/control/meeting/create",
                Value::Null,
            ),
            (
                "meeting_create_root",
                Method::POST,
                "/api/meeting/assemble/control/meeting",
                Value::Null,
            ),
            // 6c D2：豁免表里的字面量只读路由，注册表没有 Public，`/api`→Authenticated 兜死
            (
                "meeting_get_by_id",
                Method::GET,
                "/api/meeting/assemble/control/meeting/r48-parity-nonexistent",
                Value::Null,
            ),
            (
                "queryview_list",
                Method::GET,
                "/api/queryview/list",
                Value::Null,
            ),
            (
                "queryview_view_by_id",
                Method::GET,
                "/api/queryview/view/r48-parity-nonexistent",
                Value::Null,
            ),
            (
                "surface_task_list_work",
                Method::GET,
                "/api/processplatform/assemble/surface/task/list/work/r48-parity-nonexistent",
                Value::Null,
            ),
            (
                "surface_attachment_list_work",
                Method::GET,
                "/api/processplatform/assemble/surface/attachment/list/work/r48-parity-nonexistent",
                Value::Null,
            ),
            (
                "captcha_verify",
                Method::POST,
                "/api/secret/captcha/verify",
                Value::Null,
            ),
        ]
    }

    /// 设计上必须匿名可达（登录/注册前置流程）
    fn must_reach_anonymous() -> Vec<(&'static str, Method, &'static str, Value)> {
        vec![
            (
                "person_regist",
                Method::POST,
                "/api/person/regist",
                json!({}),
            ),
            (
                "captcha_verify",
                Method::POST,
                "/api/secret/captcha/verify",
                json!({}),
            ),
        ]
    }

    async fn send(
        app: &axum::Router,
        method: Method,
        uri: &str,
        body: Value,
        token: Option<&str>,
    ) -> (axum::http::StatusCode, Value) {
        // 表里用 `Value::Null` 占位的臂，实际发空对象体（避免无体请求走 body 解析分支）
        let body = if body.is_null() { json!({}) } else { body };
        shared::testing::send_request(app, method, uri, Some(body), token, None).await
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn no_route_is_dead_for_valid_session() {
        if !shared::testing::is_db_available().await {
            println!("skip: PostgreSQL 不可达，豁免/注册表一致性实测未执行");
            return;
        }
        let pool = shared::testing::test_pool();
        let session_manager = shared::session::SessionManager::with_pool(pool.clone());
        let sm = session_manager.clone();
        let rate_limiter = shared::rate_limit::RateLimiter::new();
        let app = oa4rust::create_app(pool.clone(), session_manager, rate_limiter)
            .await
            .expect("create_app 在测试环境必须构建成功");

        let client = pool.get().await.unwrap();
        let actor: String = client
            .query_opt(
                "SELECT unique_id FROM auth_person \
                 WHERE locked = false AND deleted_at IS NULL LIMIT 1",
                &[],
            )
            .await
            .unwrap()
            .map(|r| r.get("unique_id"))
            .expect("测试库必须有至少一个在册用户");
        let token = "r48-exempt-parity-token".to_string();
        sm.create_session(actor.clone(), token.clone())
            .await
            .expect("一致性实测需要能造会话");

        let mut red: Vec<String> = Vec::new();
        for (name, method, uri, body) in must_reach_with_session() {
            let (status, resp) = send(&app, method.clone(), uri, body, Some(&token)).await;
            println!(
                "会话臂 {:>22} {:<8} → HTTP {}  {}",
                name,
                method.as_str(),
                status,
                resp
            );
            if status == axum::http::StatusCode::UNAUTHORIZED {
                red.push(format!(
                    "{name} {method} {uri} 带合法会话仍 401（半开双锁）"
                ));
            }
        }
        for (name, method, uri, body) in must_reach_anonymous() {
            let (status, resp) = send(&app, method.clone(), uri, body, None).await;
            println!(
                "匿名臂 {:>22} {:<8} → HTTP {}  {}",
                name,
                method.as_str(),
                status,
                resp
            );
            if status == axum::http::StatusCode::UNAUTHORIZED {
                red.push(format!("{name} {method} {uri} 设计上匿名可达却 401"));
            }
        }

        // 放行 ≠ 放开冒名：`invoice_upload` 三把兄弟补上会话权威后，落库 creator
        // 必须是登录人，不能是请求体里伪造的值（双锁修复让这条路从死态变成登录可达，
        // 归属权威必须同时到位，否则修一条可用性就开一个冒名写面）。
        let uploaded_creator: Option<String> = client
            .query_opt(
                "SELECT creator FROM x_general_assemble_invoice WHERE name = $1",
                &[&"r48-parity-invoice-upload".to_string()],
            )
            .await
            .unwrap()
            .map(|r| r.get("creator"));
        println!(
            "creator 权威臂：invoice_upload 落库 creator={uploaded_creator:?}（会话人={actor}）"
        );
        assert_eq!(
            uploaded_creator.as_deref(),
            Some(actor.as_str()),
            "invoice_upload 把请求体的 creator 原样落库 = 冒名归属面"
        );

        // 写臂用显式探针名，清场按名字精确删。会议两臂因 `title is required` 必返 400、
        // 不落行，所以这里不写「顺手删」的兜底（旧写法 `let _ =` 会把 SQL 错误吞成静默），
        // 改成失败即响的残留断言：一旦哪天会议臂真的落行，这里必须先红一次。
        let purged = client
            .execute(
                "DELETE FROM x_general_assemble_invoice WHERE name LIKE 'r48-parity-%'",
                &[],
            )
            .await
            .unwrap();
        println!("清场：按探针名删除 invoice {purged} 行");
        let meeting_residue: i64 = client
            .query_one(
                "SELECT COUNT(*) FROM x_meeting \
                 WHERE creator = $1 AND (title IS NULL OR title = '')",
                &[&actor],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(
            meeting_residue, 0,
            "一致性实测在 x_meeting 留下了无名探针行（会议臂本应 400 不落行）"
        );
        let _ = sm.remove_session(&token).await;

        println!("一致性红名单 {} 条：\n{}", red.len(), red.join("\n"));
        assert!(
            red.is_empty(),
            "豁免表与权限注册表互不一致，以下路由处于半开双锁死态（详见上面逐臂读数与 dead_exempt_routes.tsv）"
        );
    }
}
