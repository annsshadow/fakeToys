// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 R4-7）：本 crate 的
//! `GET /api/processplatform/assemble/surface/application/list/complex/manage/{person}`
//! 与其字面量别名 `…/manage/person`。
//!
//! 这一格是「修好写门禁尺子才露出来」的真匿名写：路径落在
//! `shared::middleware::constants::AUTH_EXEMPT_PREFIXES` 的
//! `/api/processplatform/assemble/surface/application/list` 里（auth_middleware 直接放行），
//! 又注册在 **GET** 上（csrf_middleware 与 requires_admin 都只认 POST/PUT/PATCH/DELETE），
//! 所以中间件层两道门一道都够不着；而 handler 原本不取 Session，函数体第一条 SQL 就是
//! `UPDATE PP_E_APPLICATION SET "xupdateTime" = NOW() WHERE xid = $1`，随后按同一 ID 回读
//! createTime/updateTime 返回。即：**未认证**请求可按 ID 改任意应用的时间戳（审计字段伪造、
//! 可批量枚举），并拿到该应用的存在性与时间信息。
//!
//! 现在按应用管理面口径收口（桌面侧只有运维控制台弹窗手输人员会调它，不是终端用户流程）：
//! 无会话 401（先于任何 SQL，可离线证），非 admin 403（`shared::middleware::is_admin`
//! 无库 fail-closed）。
#[cfg(test)]
mod security_gate_tests {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use tower::ServiceExt;

    const MANAGE_URI: &str =
        "/api/processplatform/assemble/surface/application/list/complex/manage/r47-target";
    const MANAGE_ALIAS_URI: &str =
        "/api/processplatform/assemble/surface/application/list/complex/manage/person";

    fn app() -> axum::Router {
        crate::router(shared::testing::test_pool())
    }

    async fn get(uri: &str) -> StatusCode {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        resp.status()
    }

    async fn get_as(uri: &str, person: &str) -> StatusCode {
        let session = shared::testing::test_session(person);
        let resp = app()
            .layer(axum::Extension(session))
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        resp.status()
    }

    /// 未认证必须 401，而且是在拿到连接、发任何 SQL 之前——门在 `pool.get()` 之上。
    #[tokio::test]
    async fn r47_manage_touch_rejects_without_session() {
        assert_eq!(
            get(MANAGE_URI).await,
            StatusCode::UNAUTHORIZED,
            "参数位路由：匿名 GET 写必须 401"
        );
        assert_eq!(
            get(MANAGE_ALIAS_URI).await,
            StatusCode::UNAUTHORIZED,
            "字面量别名路由与参数位路由同 handler，必须同口径"
        );
    }

    /// 有效会话但非 admin 必须 403。用一个绝不会被授予角色的唯一身份，
    /// 因此该断言与测试库里是否恰好有 admin 数据无关。
    #[tokio::test]
    async fn r47_manage_touch_forbidden_for_non_admin() {
        assert_eq!(
            get_as(MANAGE_URI, "r47-plain-person-no-role").await,
            StatusCode::FORBIDDEN,
            "非 admin 按 ID touch 他人应用时间戳必须 403"
        );
        assert_eq!(
            get_as(MANAGE_ALIAS_URI, "r47-plain-person-no-role").await,
            StatusCode::FORBIDDEN,
            "别名路由同口径"
        );
    }

    /// 四轮 R4-2 追加：`service_work_id_touch` / `service_work_id_touch_mockputtopost`
    /// 四把路由（PUT/POST × 正式/别名）原先不取会话，任一有效会话按 work ID 就能
    /// `UPDATE PP_C_WORK."xupdateTime"` 并回读时间字段。无会话必须 401，且发生在
    /// `pool.get()` 之前——所以本支不碰库，离线可证。
    #[tokio::test]
    async fn surface_touch_writes_reject_without_session() {
        for (method, uri) in [
            (
                Method::POST,
                "/api/processplatform/assemble/surface/service/work/touch/r47-work",
            ),
            (
                Method::POST,
                "/api/processplatform/assemble/surface/service/work/touch/mockputtopost/r47-work",
            ),
            (
                Method::PUT,
                "/api/processplatform/assemble/surface/service/work/r47-work/touch",
            ),
            (
                Method::POST,
                "/api/processplatform/assemble/surface/service/work/r47-work/touch/mockputtopost",
            ),
        ] {
            let resp = app()
                .oneshot(
                    Request::builder()
                        .uri(uri)
                        .method(method.clone())
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                resp.status(),
                StatusCode::UNAUTHORIZED,
                "无会话的 touch 写必须 401（门在取连接之前）: {method} {uri}"
            );
        }
    }

    // ── 四轮第十批 T1：applicationdict 深度路径的别名写族 ──────────────────────
    //
    // 16 条注册点：`…/data/mockputtopost` 8 档全注册在 POST，
    // `…/data/mockdeletetoget` 只有 4 参那档注册在 DELETE、其余 7 档注册在 **GET**——
    // GET 既不过 csrf_middleware 也不过 requires_admin（两者只认 POST/PUT/PATCH/DELETE），
    // 中间件层两道门都够不着，handler 自己是唯一一道门。而这些 handler 原先**既不取会话、
    // 也不校任何门**，体内直接 `UPDATE PP_E_APPLICATIONDICT` / `DELETE FROM PP_E_APPLICATIONDICT`
    // （后者同一条删两次）。系统级元数据 → 按本 crate R4-7 口径收 admin。

    /// handler 体内 SQL 的 `$1` 取的是**首段** `applicationDictFlag`，不是末尾的 `data` 段，
    /// 所以 DB 臂要落库必须把唯一标签写进 flag（见 `dict_uri_at`）。
    fn dict_uri_at(flag: &str, arity: usize, suffix: &str) -> String {
        let mut u = format!(
            "/api/processplatform/assemble/surface/applicationdict/{flag}/application/r48-app"
        );
        for i in 0..arity {
            u.push_str(&format!("/r48-p{i}"));
        }
        u.push_str("/data/");
        u.push_str(suffix);
        u
    }

    fn dict_uri(arity: usize, suffix: &str) -> String {
        dict_uri_at("r48-dict", arity, suffix)
    }

    /// 与 `routes.rs` 注册面逐格对应：8 把 POST 别名 + 7 把 GET 删除别名 + 1 把 DELETE 删除别名。
    fn dict_alias_routes() -> Vec<(Method, String)> {
        let mut v: Vec<(Method, String)> = (1..=8)
            .map(|k| (Method::POST, dict_uri(k, "mockputtopost")))
            .collect();
        for k in 1..=8 {
            let m = if k == 4 { Method::DELETE } else { Method::GET };
            v.push((m, dict_uri(k, "mockdeletetoget")));
        }
        assert_eq!(
            v.len(),
            16,
            "本族注册点恰好 16 条，多一条/少一条都要先红在这里"
        );
        v
    }

    async fn call_dict_route(method: Method, uri: &str, person: Option<&str>) -> StatusCode {
        let mut app = app();
        if let Some(p) = person {
            app = app.layer(axum::Extension(shared::testing::test_session(p)));
        }
        app.oneshot(
            Request::builder()
                .uri(uri)
                .method(method.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    /// 无会话必须 401，且发生在 `pool.get()` 之前——本支不碰库，离线可证。
    /// 16 条注册点逐条点名（少一条就是门只盖住了部分入口）。
    #[tokio::test]
    async fn surface_applicationdict_alias_writes_reject_anonymous() {
        let mut bad: Vec<String> = Vec::new();
        for (method, uri) in dict_alias_routes() {
            let status = call_dict_route(method.clone(), &uri, None).await;
            if status != StatusCode::UNAUTHORIZED {
                bad.push(format!("{method} {uri} -> {status}"));
            }
        }
        assert!(
            bad.is_empty(),
            "16 把别名写必须全部无会话 401，异常格：{bad:?}"
        );
    }

    /// 有效会话但非 admin 必须 403。用一个绝不会被授予角色的唯一身份，
    /// 因此该断言与测试库里是否恰好有 admin 数据无关；`is_admin` 无库也 fail-closed。
    #[tokio::test]
    async fn surface_applicationdict_alias_writes_forbidden_for_non_admin() {
        let mut bad: Vec<String> = Vec::new();
        for (method, uri) in dict_alias_routes() {
            let status =
                call_dict_route(method.clone(), &uri, Some("r48-plain-person-no-role")).await;
            if status != StatusCode::FORBIDDEN {
                bad.push(format!("{method} {uri} -> {status}"));
            }
        }
        assert!(
            bad.is_empty(),
            "16 把别名写必须全部非 admin 403，异常格：{bad:?}"
        );
    }

    /// 按 `is_admin` 的真实口径幂等造一个 admin，返回其会话人 unique_id。
    async fn seed_admin(pool: &deadpool_postgres::Pool, tag: &str) -> String {
        let c = pool.get().await.unwrap();
        let person = format!("u-r48s-admin-{tag}");
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ($1, 'admin', false) ON CONFLICT (id) DO NOTHING",
            &[&format!("r48s-role-{tag}")],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ($1, $1, 'r48s admin', 'x') ON CONFLICT (id) DO NOTHING",
            &[&person],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            &[&person, &format!("r48s-role-{tag}"), &format!("r48s-unit-{tag}")],
        )
        .await
        .unwrap();
        person
    }

    async fn dict_row_count(pool: &deadpool_postgres::Pool, xid: &str) -> i64 {
        let c = pool.get().await.unwrap();
        c.query_one(
            "SELECT COUNT(*) AS c FROM \"pp_e_applicationdict\" WHERE \"xid\" = $1",
            &[&xid.to_string()],
        )
        .await
        .unwrap()
        .get::<_, i64>("c")
    }

    /// 本臂用到的三列都按迁移 032 的口径存在：`id` 是 `TEXT PRIMARY KEY`（NOT NULL），
    /// `xid`/`xupdateTime` 是 TEXT。表已由迁移建好时 CREATE 是无操作，
    /// 只有裸库才走这条兜底，两种情况下 INSERT 都能成功。
    async fn seed_dict_row(pool: &deadpool_postgres::Pool, xid: &str, update_time: &str) {
        let c = pool.get().await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS \"pp_e_applicationdict\" \
             (\"id\" TEXT PRIMARY KEY, \"xid\" TEXT, \"xcreateTime\" TEXT, \"xupdateTime\" TEXT)",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO \"pp_e_applicationdict\" (\"id\", \"xid\", \"xupdateTime\") VALUES ($1, $2, $3)",
            &[&format!("r48s-row-{xid}"), &xid.to_string(), &update_time.to_string()],
        )
        .await
        .unwrap();
    }

    async fn dict_update_time(pool: &deadpool_postgres::Pool, xid: &str) -> Option<String> {
        let c = pool.get().await.unwrap();
        c.query_opt(
            "SELECT \"xupdateTime\" FROM \"pp_e_applicationdict\" WHERE \"xid\" = $1",
            &[&xid.to_string()],
        )
        .await
        .unwrap()
        .and_then(|row| row.get::<_, Option<String>>("xupdateTime"))
    }

    /// DB 臂（行为差）：闯入者按 DELETE 档解除必须 403 **且行还在**；
    /// admin 才真的删掉。只看状态码不够——本族 handler 未命中行时也回 200。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_applicationdict_delete_alias_only_admin_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!(
                "skip: surface_applicationdict_delete_alias_only_admin_lands (no DATABASE_URL)"
            );
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let xid = format!("r48s-dict-{tag}");
        seed_dict_row(&pool, &xid, "r48s-before").await;
        // routes.rs 里 4 参那档的删除别名注册在 DELETE 方法上
        let uri = dict_uri_at(&xid, 4, "mockdeletetoget");
        let intruder = call_dict_route(Method::DELETE, &uri, Some("r48s-intruder-no-role")).await;
        assert_eq!(
            intruder,
            StatusCode::FORBIDDEN,
            "闯入者解除系统字典必须 403"
        );
        assert_eq!(
            dict_row_count(&pool, &xid).await,
            1,
            "拒绝路径不得留下「行已被删」的证据"
        );

        let admin = seed_admin(&pool, &tag).await;
        let ok = call_dict_route(Method::DELETE, &uri, Some(&admin)).await;
        assert_eq!(
            ok,
            StatusCode::OK,
            "admin 经 DELETE 档解除应放行（若 401/403 说明门夹死了放行侧）"
        );
        assert_eq!(
            dict_row_count(&pool, &xid).await,
            0,
            "放行臂必须真的删掉那一行"
        );
    }

    /// DB 臂（放行侧的行为差 + 一处**类型猜测被实测否证**的读数）：
    /// admin 打 `mockputtopost` 必须放行**且 UPDATE 真的落库**——只看「不是 401/403」不够，
    /// 本族 handler 未命中行时也回 200（`applicationdict not found`），那条路上 SQL 根本没执行。
    ///
    /// 我在写本臂前按「迁移 032 把 `xupdateTime` 建成 TEXT，`SET … = NOW()` 是 timestamp→text
    /// 赋值、必报 42846 所以整族会 500」预测过缺陷，**实测否证**：PG 允许经 I/O 转换的赋值转换，
    /// 落库成 `2026-10-14 16:32:23.456789+00` 这种文本时间戳。故全仓那 79 处同型写法不是
    /// 「报错」缺陷，而是「TEXT 列被写进与会话/其它列不同构的时间文本」的语义分歧，
    /// 归 §五 单独定标；此处只把读数钉在臂的输出里。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_applicationdict_put_alias_admin_write_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!(
                "skip: surface_applicationdict_put_alias_admin_write_lands (no DATABASE_URL)"
            );
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let xid = format!("r48s-dict-{tag}");
        let admin = seed_admin(&pool, &tag).await;
        seed_dict_row(&pool, &xid, "r48s-before").await;

        let uri = dict_uri_at(&xid, 1, "mockputtopost");
        let status = call_dict_route(Method::POST, &uri, Some(&admin)).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "admin 放行臂应命中已存在的字典行（{uri}）"
        );
        let stored = dict_update_time(&pool, &xid).await;
        println!("mockputtopost 放行后 xupdateTime 落库读数：{stored:?}");
        assert_ne!(
            stored.as_deref(),
            Some("r48s-before"),
            "SQL 未执行也会回 200（not found 分支），落库才是放行侧的真凭据"
        );

        // 其余 7 档同样必须放行：门若把某一档夹死，本臂负责变红
        let mut bad: Vec<String> = Vec::new();
        for k in 2..=8 {
            let uri = dict_uri_at(&xid, k, "mockputtopost");
            let status = call_dict_route(Method::POST, &uri, Some(&admin)).await;
            if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
                bad.push(format!("{uri} -> {status}"));
            }
        }
        assert!(bad.is_empty(), "admin 打 2..=8 档别名不得被门拒绝：{bad:?}");

        pool.get()
            .await
            .unwrap()
            .execute(
                "DELETE FROM \"pp_e_applicationdict\" WHERE \"xid\" = $1",
                &[&xid.to_string()],
            )
            .await
            .unwrap();
    }

    // ══ 四轮 R4-10 第三批：`data/work/{id}` 的 mock* 别名族（18 把 handler / 45 条注册点）══
    //
    // 这一族原先**既不取会话、也不校归属**，体内直接
    // `UPDATE PP_C_WORK SET "xupdateTime" = NOW() WHERE xid = $1` /
    // `DELETE FROM PP_C_WORK WHERE xid = $1`（删除族同一条 SQL 执行两次）。
    // 于是未认证请求按 work ID 就能物理删除流程实例或伪造其更新时间。
    // 45 条注册点里有 10 条挂在 **GET** 上——csrf_middleware 与 requires_admin
    // 都只认 POST/PUT/PATCH/DELETE，中间件两道门对这 10 条同时够不着。
    // 门与本 crate 已定的口径同锁（`service_work_id_touch` 用的那两把）：
    // 无会话 401（先于 `pool.get()`，离线可证）；有会话先读 `"xcreatorPerson"` 再校归属，
    // 归属非空 → 本人或 admin，归属为空/行不存在 → 只允许 admin。
    //
    /// 把注册模板里的 `{id}` 换成实际 xid、`{pathN}` 换成固定哨兵段。
    /// handler 体内 SQL 的 `$1` 绑的是 `Path((id, _path0, …))` 的**首段** `id`，
    /// 所以 DB 臂要命中行必须把 xid 写在 `{id}` 的位置上。
    fn work_uri_at(xid: &str, tpl: &str) -> String {
        let mut u = tpl.replace("{id}", xid);
        for i in 0..8usize {
            u = u.replace(&format!("{{path{i}}}"), &format!("r49-p{i}"));
        }
        assert!(!u.contains('{') && !u.contains('}'), "模板残留未替换：{u}");
        u
    }

    /// 与 `routes.rs` 注册面逐格对应（下表由 routes.rs 机器枚举生成，45 条）。
    fn work_alias_routes() -> Vec<(Method, &'static str)> {
        let v: Vec<(Method, &'static str)> = vec![
            // ← data_work_id_mockdeletetoget
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/mockdeletetoget"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/mockdeletetoget/{id}"),
            // ← data_work_id_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/mockputtopost"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/mockputtopost/{id}"),
            // ← data_work_id_path0_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/mockdeletetoget"),
            // ← data_work_id_path0_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/mockputtopost"),
            // ← data_work_id_path0_path1_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/mockdeletetoget"),
            // ← data_work_id_path0_path1_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/mockputtopost"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/mockputtopost"),
            // ← data_work_id_path0_path1_path2_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/mockdeletetoget"),
            // ← data_work_id_path0_path1_path2_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/mockputtopost"),
            // ← data_work_id_path0_path1_path2_path3_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/mockdeletetoget"),
            // ← data_work_id_path0_path1_path2_path3_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/mockputtopost"),
            // ← data_work_id_path0_path1_path2_path3_path4_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/mockdeletetoget"),
            // ← data_work_id_path0_path1_path2_path3_path4_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/mockputtopost"),
            // ← data_work_id_path0_path1_path2_path3_path4_path5_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockdeletetoget"),
            // ← data_work_id_path0_path1_path2_path3_path4_path5_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockputtopost"),
            // ← data_work_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockdeletetoget"),
            // ← data_work_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockputtopost"),
            // ← data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget
            (Method::DELETE, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}"),
            (Method::GET, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockdeletetoget"),
            // ← data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}"),
            (Method::PUT, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}"),
            (Method::POST, "/api/processplatform/assemble/surface/data/work/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockputtopost"),
        ];
        assert_eq!(
            v.len(),
            45,
            "本族注册点恰好 45 条（18 把 handler），多一条/少一条都要先红在这里"
        );
        v
    }

    async fn call_work_as(
        xid: &str,
        method: Method,
        tpl: &str,
        person: Option<&str>,
    ) -> StatusCode {
        let uri = work_uri_at(xid, tpl);
        let mut app = app();
        if let Some(p) = person {
            app = app.layer(axum::Extension(shared::testing::test_session(p)));
        }
        app.oneshot(
            Request::builder()
                .uri(&uri)
                .method(method.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    /// 无会话必须 401，且发生在 `pool.get()` 之前——本支不碰库，离线可证。
    /// 45 条注册点逐条点名（少一条就是门只盖住了部分入口）。
    #[tokio::test]
    async fn surface_data_work_alias_writes_reject_anonymous() {
        let mut bad: Vec<String> = Vec::new();
        for (method, tpl) in work_alias_routes() {
            let status = call_work_as("r49-anon-target", method.clone(), tpl, None).await;
            if status != StatusCode::UNAUTHORIZED {
                bad.push(format!("{method} {tpl} -> {status}"));
            }
        }
        assert!(
            bad.is_empty(),
            "45 把 data/work 别名写必须全部无会话 401，异常格：{bad:?}"
        );
    }

    /// 造一行 `pp_c_work`：`id` 是 TEXT PRIMARY KEY（NOT NULL），`xid`/`xcreatorPerson`/
    /// `xupdateTime` 均为 TEXT（迁移 032 口径）。表已存在时 CREATE 是无操作，裸库才走兜底；
    /// `creator` 传空串即造「空归属」行。
    async fn seed_work_row(pool: &deadpool_postgres::Pool, xid: &str, creator: &str) {
        let c = pool.get().await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS \"pp_c_work\" (\"id\" TEXT PRIMARY KEY, \"xid\" TEXT, \
             \"xcreatorPerson\" TEXT, \"xcreateTime\" TEXT, \"xupdateTime\" TEXT)",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO \"pp_c_work\" (\"id\", \"xid\", \"xcreatorPerson\", \"xupdateTime\") \
             VALUES ($1, $1, $2, 'r49-before') ON CONFLICT (\"id\") DO UPDATE SET \
             \"xcreatorPerson\" = EXCLUDED.\"xcreatorPerson\", \"xupdateTime\" = 'r49-before'",
            &[&xid.to_string(), &creator.to_string()],
        )
        .await
        .unwrap();
    }

    /// 返回该 work 行的 `xupdateTime`；行不存在 → None。
    async fn work_update_time(pool: &deadpool_postgres::Pool, xid: &str) -> Option<String> {
        let c = pool.get().await.unwrap();
        c.query_opt(
            "SELECT \"xupdateTime\" AS u FROM pp_c_work WHERE xid = $1",
            &[&xid.to_string()],
        )
        .await
        .unwrap()
        .and_then(|row| row.get::<_, Option<String>>("u"))
    }

    async fn delete_work_row(pool: &deadpool_postgres::Pool, xid: &str) {
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM pp_c_work WHERE xid = $1", &[&xid.to_string()])
            .await
            .unwrap();
    }

    const DEL_TPL: &str = "/api/processplatform/assemble/surface/data/work/{id}/mockdeletetoget";
    const DEL_POST_TPL: &str =
        "/api/processplatform/assemble/surface/data/work/mockdeletetoget/{id}";
    const TOUCH_GET_TPL: &str =
        "/api/processplatform/assemble/surface/data/work/mockputtopost/{id}";

    /// 删除族（真洞本体：匿名可按 ID 物理删除流程实例）的四格：
    /// ① 他人会话 403 且行必须还在；② 本人 200 且行确实被删；
    /// ③ 空归属行不得被「人人可删」洗白——普通会话 403、admin 200；
    /// ④ admin 逐一打 45 档不得出现 401/403（门夹死放行侧就是本批的回归）。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_data_work_delete_alias_owner_gate_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: surface_data_work_delete_alias_owner_gate_lands (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let creator = format!("u-r49s-creator-{tag}");
        let intruder = format!("u-r49s-intruder-{tag}");

        let xid = format!("r49s-work-{tag}-own");
        seed_work_row(&pool, &xid, &creator).await;
        let s = call_work_as(&xid, Method::GET, DEL_TPL, Some(&intruder)).await;
        assert_eq!(s, StatusCode::FORBIDDEN, "他人会话删他人 work 必须 403");
        assert!(
            work_update_time(&pool, &xid).await.is_some(),
            "403 路径不得留下「行已被删」的证据"
        );

        // handler 未命中行时也回 200（error 分支），所以行数才是放行侧的真凭据
        let s = call_work_as(&xid, Method::GET, DEL_TPL, Some(&creator)).await;
        assert_eq!(s, StatusCode::OK, "本人删除应放行");
        assert!(
            work_update_time(&pool, &xid).await.is_none(),
            "放行臂必须真的删掉那一行"
        );

        let orphan = format!("r49s-work-{tag}-orphan");
        seed_work_row(&pool, &orphan, "").await;
        let s = call_work_as(&orphan, Method::POST, DEL_POST_TPL, Some(&intruder)).await;
        assert_eq!(
            s,
            StatusCode::FORBIDDEN,
            "空归属 work 不得对普通会话开放删除"
        );
        let admin = seed_admin(&pool, &tag).await;
        let s = call_work_as(&orphan, Method::GET, DEL_TPL, Some(&admin)).await;
        assert_eq!(s, StatusCode::OK, "admin 删空归属 work 应放行");
        assert!(work_update_time(&pool, &orphan).await.is_none());

        let arity = format!("r49s-work-{tag}-arity");
        seed_work_row(&pool, &arity, &creator).await;
        let mut bad: Vec<String> = Vec::new();
        for (method, tpl) in work_alias_routes() {
            let status = call_work_as(&arity, method.clone(), tpl, Some(&admin)).await;
            if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
                bad.push(format!("{method} {tpl} -> {status}"));
            }
        }
        assert!(bad.is_empty(), "admin 打 45 档别名不得被门拒绝：{bad:?}");
        delete_work_row(&pool, &arity).await;
    }

    /// touch 族（`UPDATE PP_C_WORK."xupdateTime"`）的归属门 + 落库行为差：
    /// 他人 403 且时间戳不变，本人 200 且时间戳确实被改写。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_data_work_touch_alias_owner_write_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: surface_data_work_touch_alias_owner_write_lands (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let creator = format!("u-r49s-creator-{tag}");
        let intruder = format!("u-r49s-intruder-{tag}");
        let xid = format!("r49s-work-{tag}-touch");
        seed_work_row(&pool, &xid, &creator).await;

        let s = call_work_as(&xid, Method::GET, TOUCH_GET_TPL, Some(&intruder)).await;
        assert_eq!(
            s,
            StatusCode::FORBIDDEN,
            "他人会话 touch 他人 work 必须 403"
        );
        assert_eq!(
            work_update_time(&pool, &xid).await.as_deref(),
            Some("r49-before"),
            "403 路径不得改写时间戳"
        );

        let s = call_work_as(&xid, Method::GET, TOUCH_GET_TPL, Some(&creator)).await;
        assert_eq!(s, StatusCode::OK, "本人 touch 应放行");
        let stored = work_update_time(&pool, &xid).await;
        println!("work mockputtopost 放行后 xupdateTime 落库读数：{stored:?}");
        assert_ne!(
            stored.as_deref(),
            Some("r49-before"),
            "SQL 未执行也会回 200（not found 分支），落库才是放行侧的真凭据"
        );
        delete_work_row(&pool, &xid).await;
    }

    // ═══ 第十一批：data/workcompleted 与 data/job 的 mockputtopost 别名写族 ═══
    // 18 把 handler / 36 条注册点。形状与上一批 data/work 同锁：handler 只按「xid = $1」
    // touch 一行「"xupdateTime"」，取 Path 的**首段**当 xid，且原先一条门都不上——
    // 任一登录会话（甚至匿名，因为有 10 条挂在 GET 上）都能改写他人流程实例的更新时间。
    // 本批口径与 u4_require_ppc_actor 一致：无会话 401（先于 pool.get()，离线可证）；
    // 有会话先读「"xcreatorPerson"」再校归属，归属为空或行不存在 → 只允许 admin。
    // 门夹在 SQL 之前还会改变既有契约测试的期望码（500 → 401），那一处返工在 tests_u2.rs。

    /// 注册模板里的「{id}」「{job}」换成实际 xid、「{pathN}」换成固定哨兵段。
    /// 注意 job 族还有一档 GET 别名写成字面量「/data/job/job/mockputtopost」（o2server 的
    /// {job} 位被写死成 "job"），它没有占位符可替换——SQL 绑到的 xid 就是字符串 "job"，
    /// 所以那一档只用于「门有没有盖住这条注册点」，不用于 DB 落库取证。
    fn ppc_uri_at(xid: &str, tpl: &str) -> String {
        let mut u = tpl.replace("{id}", xid).replace("{job}", xid);
        for i in 0..8usize {
            u = u.replace(&format!("{{path{i}}}"), &format!("r50-p{i}"));
        }
        assert!(!u.contains('{') && !u.contains('}'), "模板残留未替换：{u}");
        u
    }

    /// 与 routes.rs 注册面逐格对应（下表由 routes.rs 机器枚举生成，36 条 / 18 把 handler）。
    fn ppc_alias_routes() -> Vec<(Method, &'static str)> {
        let v: Vec<(Method, &'static str)> = vec![
            // ← data_job_job_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/mockputtopost"),
            // ← data_job_job_path0_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/mockputtopost"),
            // ← data_job_job_path0_path1_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/path1/mockputtopost"),
            // ← data_job_job_path0_path1_path2_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/path1/path2/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_path5_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_path5_path6_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost"),
            // ← data_workcompleted_id_mockputtopost
            (Method::GET, "/api/processplatform/assemble/surface/data/workcompleted/mockputtopost/{id}"),
            // ← data_job_job_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/mockputtopost"),
            // ← data_job_job_path0_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/mockputtopost"),
            // ← data_job_job_path0_path1_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/{path1}/mockputtopost"),
            // ← data_job_job_path0_path1_path2_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/{path1}/{path2}/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/{path1}/{path2}/{path3}/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/{path1}/{path2}/{path3}/{path4}/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_path5_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_path5_path6_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockputtopost"),
            // ← data_job_job_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/job/{job}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockputtopost"),
            // ← data_workcompleted_id_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/mockputtopost"),
            // ← data_workcompleted_id_path0_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/mockputtopost"),
            // ← data_workcompleted_id_path0_path1_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/mockputtopost"),
            // ← data_workcompleted_id_path0_path1_path2_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/mockputtopost"),
            // ← data_workcompleted_id_path0_path1_path2_path3_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/mockputtopost"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/mockputtopost"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_path5_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/mockputtopost"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/mockputtopost"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost
            (Method::POST, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}/mockputtopost"),
            // ← data_workcompleted_id_path0_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}"),
            // ← data_workcompleted_id_path0_path1_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}"),
            // ← data_workcompleted_id_path0_path1_path2_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}"),
            // ← data_workcompleted_id_path0_path1_path2_path3_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_path5_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}"),
            // ← data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost
            (Method::PUT, "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}/{path1}/{path2}/{path3}/{path4}/{path5}/{path6}/{path7}"),
        ];
        assert_eq!(
            v.len(),
            36,
            "本族注册点恰好 36 条（18 把 handler），多一条/少一条都要先红在这里"
        );
        v
    }

    async fn call_ppc_as(xid: &str, method: Method, tpl: &str, person: Option<&str>) -> StatusCode {
        let uri = ppc_uri_at(xid, tpl);
        let mut app = app();
        if let Some(p) = person {
            app = app.layer(axum::Extension(shared::testing::test_session(p)));
        }
        app.oneshot(
            Request::builder()
                .uri(&uri)
                .method(method.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    /// 无会话必须 401，且发生在 pool.get() 之前——本支不碰库，离线可证。
    /// 36 条注册点分两族逐格点名：
    /// ① 路径里带 `{}` 的 27 条是**活**别名，必须撞在会话门上（401）；
    /// ② `GET /data/job/job[/pathN…]/mockputtopost` 这 9 条是**死档**——注册时把 `{job}`
    ///    槽写成了字面量 `job`、又没给 `{id}` 段，路径零参数而 handler 取 `Path<String>`，
    ///    axum 在进入函数体之前就抛 500「Wrong number of path arguments」。
    ///    判据用「模板里有几个 `{}` 段」而不是硬编码行数，是因为这条规则可机器复核：
    ///    一旦有人把死档补上参数段，它会立刻从 ② 落到 ① 并要求 401，而不是悄悄留在 500。
    ///    本支不把 ② 放宽成「401 或 500 都算过」——那样「别名被补活却忘了加门」也读成绿。
    #[tokio::test]
    async fn surface_ppc_alias_writes_reject_anonymous() {
        let mut bad: Vec<String> = Vec::new();
        let mut dead = 0usize;
        for (method, tpl) in ppc_alias_routes() {
            let live = tpl.contains('{');
            let want = if live {
                StatusCode::UNAUTHORIZED
            } else {
                dead += 1;
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let status = call_ppc_as("r50-anon-target", method.clone(), tpl, None).await;
            if status != want {
                bad.push(format!("{method} {tpl} -> 期望 {want}，实际 {status}"));
            }
        }
        assert!(bad.is_empty(), "workcompleted/job 别名写门禁不符：{bad:?}");
        assert_eq!(dead, 9, "死档别名恰好 9 条（GET job 字面量族）");
    }

    /// 造一行 pp_c_workcompleted / pp_c_job。迁移 032 里这两张表的 id 是 TEXT PRIMARY KEY，
    /// xid / xcreatorPerson / xcreateTime / xupdateTime 均为 TEXT；handler 放行后还会
    /// SELECT xcreateTime，所以兜底建表必须带上它。表已存在时 CREATE 是无操作，裸库才走兜底；
    /// creator 传空串即造「空归属」行。
    async fn seed_ppc_row(pool: &deadpool_postgres::Pool, table: &str, xid: &str, creator: &str) {
        let c = pool.get().await.unwrap();
        c.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS \"{table}\" (\"id\" TEXT PRIMARY KEY, \"xid\" TEXT, \
                 \"xcreatorPerson\" TEXT, \"xcreateTime\" TEXT, \"xupdateTime\" TEXT)"
            )
            .as_str(),
            &[],
        )
        .await
        .unwrap();
        c.execute(
            format!(
                "INSERT INTO \"{table}\" (\"id\", \"xid\", \"xcreatorPerson\", \"xupdateTime\") \
                 VALUES ($1, $1, $2, 'r50-before') ON CONFLICT (\"id\") DO UPDATE SET \
                 \"xcreatorPerson\" = EXCLUDED.\"xcreatorPerson\", \"xupdateTime\" = 'r50-before'"
            )
            .as_str(),
            &[&xid.to_string(), &creator.to_string()],
        )
        .await
        .unwrap();
    }

    async fn ppc_update_time(
        pool: &deadpool_postgres::Pool,
        table: &str,
        xid: &str,
    ) -> Option<String> {
        let c = pool.get().await.unwrap();
        c.query_opt(
            format!("SELECT \"xupdateTime\" AS u FROM \"{table}\" WHERE xid = $1").as_str(),
            &[&xid.to_string()],
        )
        .await
        .unwrap()
        .and_then(|row| row.get::<_, Option<String>>("u"))
    }

    async fn delete_ppc_row(pool: &deadpool_postgres::Pool, table: &str, xid: &str) {
        let c = pool.get().await.unwrap();
        c.execute(
            format!("DELETE FROM \"{table}\" WHERE xid = $1").as_str(),
            &[&xid.to_string()],
        )
        .await
        .unwrap();
    }

    const WC_TPL: &str =
        "/api/processplatform/assemble/surface/data/workcompleted/{id}/mockputtopost";
    const WC_PUT_TPL: &str =
        "/api/processplatform/assemble/surface/data/workcompleted/{id}/{path0}";
    const JOB_TPL: &str = "/api/processplatform/assemble/surface/data/job/{job}/mockputtopost";

    /// touch 族的归属门 + 落库行为差（workcompleted 一臂）：
    /// ① 闯入者 403 且时间戳仍为哨兵；② 本人 200 且时间戳确实被改写（handler 未命中行时
    /// 也回 200，所以落库才是放行侧凭据）；③ 空归属行普通会话 403、admin 放行。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_data_workcompleted_touch_alias_owner_gate_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!(
                "skip: surface_data_workcompleted_touch_alias_owner_gate_lands (no DATABASE_URL)"
            );
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let creator = format!("u-r50s-creator-{tag}");
        let intruder = format!("u-r50s-intruder-{tag}");
        let xid = format!("r50s-wc-{tag}-touch");
        seed_ppc_row(&pool, "pp_c_workcompleted", &xid, &creator).await;

        let s = call_ppc_as(&xid, Method::POST, WC_TPL, Some(&intruder)).await;
        assert_eq!(
            s,
            StatusCode::FORBIDDEN,
            "他人会话 touch 他人 workcompleted 必须 403"
        );
        assert_eq!(
            ppc_update_time(&pool, "pp_c_workcompleted", &xid)
                .await
                .as_deref(),
            Some("r50-before"),
            "403 路径不得改写时间戳"
        );

        let s = call_ppc_as(&xid, Method::POST, WC_TPL, Some(&creator)).await;
        assert_eq!(s, StatusCode::OK, "本人 touch 应放行");
        let stored = ppc_update_time(&pool, "pp_c_workcompleted", &xid).await;
        println!("workcompleted mockputtopost 放行后 xupdateTime 落库读数：{stored:?}");
        assert_ne!(
            stored.as_deref(),
            Some("r50-before"),
            "SQL 未执行也会回 200（not found 分支），落库才是放行侧的真凭据"
        );

        let orphan = format!("r50s-wc-{tag}-orphan");
        seed_ppc_row(&pool, "pp_c_workcompleted", &orphan, "").await;
        let s = call_ppc_as(&orphan, Method::PUT, WC_PUT_TPL, Some(&intruder)).await;
        assert_eq!(
            s,
            StatusCode::FORBIDDEN,
            "空归属 workcompleted 不得对普通会话开放改写"
        );
        let admin = seed_admin(&pool, &tag).await;
        let s = call_ppc_as(&orphan, Method::POST, WC_TPL, Some(&admin)).await;
        assert_eq!(s, StatusCode::OK, "admin 改写空归属 workcompleted 应放行");

        delete_ppc_row(&pool, "pp_c_workcompleted", &xid).await;
        delete_ppc_row(&pool, "pp_c_workcompleted", &orphan).await;
    }

    /// job 一臂同锁（表 pp_c_job，xcreatorPerson 是流程实例发起人）。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_data_job_touch_alias_owner_gate_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: surface_data_job_touch_alias_owner_gate_lands (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let creator = format!("u-r50s-creator-{tag}");
        let intruder = format!("u-r50s-intruder-{tag}");
        let xid = format!("r50s-job-{tag}-touch");
        seed_ppc_row(&pool, "pp_c_job", &xid, &creator).await;

        let s = call_ppc_as(&xid, Method::POST, JOB_TPL, Some(&intruder)).await;
        assert_eq!(s, StatusCode::FORBIDDEN, "他人会话 touch 他人 job 必须 403");
        assert_eq!(
            ppc_update_time(&pool, "pp_c_job", &xid).await.as_deref(),
            Some("r50-before"),
            "403 路径不得改写时间戳"
        );

        let s = call_ppc_as(&xid, Method::POST, JOB_TPL, Some(&creator)).await;
        assert_eq!(s, StatusCode::OK, "本人 touch 应放行");
        let stored = ppc_update_time(&pool, "pp_c_job", &xid).await;
        println!("job mockputtopost 放行后 xupdateTime 落库读数：{stored:?}");
        assert_ne!(
            stored.as_deref(),
            Some("r50-before"),
            "SQL 未执行也会回 200（not found 分支），落库才是放行侧的真凭据"
        );
        delete_ppc_row(&pool, "pp_c_job", &xid).await;
    }

    /// 放行侧不得被门夹死：admin 逐一打 36 档，任何一格出现 401/403 都是本批的回归。
    /// 9 条 GET job 字面量档是「Path 参数段缺失」的死档，带会话也进不到函数体（500），
    /// 本支刻意只盯 401/403——死档不是放行侧的凭据，放行侧的真凭据由上面两支落库差给出。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_ppc_alias_admin_passes_all_registrations() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: surface_ppc_alias_admin_passes_all_registrations (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let admin = seed_admin(&pool, &tag).await;
        let mut bad: Vec<String> = Vec::new();
        for (method, tpl) in ppc_alias_routes() {
            let status = call_ppc_as("r50s-any", method.clone(), tpl, Some(&admin)).await;
            if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
                bad.push(format!("{method} {tpl} -> {status}"));
            }
        }
        assert!(
            bad.is_empty(),
            "admin 打 36 档 workcompleted/job 别名不得被门拒绝：{bad:?}"
        );
    }

    // ── R4 第十五批：mock 动词别名写族 22 把 / 44 条注册点 ─────────────────────────
    // 这一族把**物理删除**（`mockdeletetoget`）和 **touch 改写时间戳**（`mockputtopost`）挂在
    // GET 上（GET 既不过 csrf 也不过 requires_admin），改前体内不取会话、不校归属：
    // 任一有效会话按 xid 就能删他人的待办 / 已办 / 待阅 / 已阅 / 草稿 / 流水号 / 签核 / 圈阅 /
    // 流程实例。门按「这一行属于谁」定三档，判据来自库里的真实列（本轮用 information_schema 逐表验过）：
    //   xperson        —— 行上属主列是 xperson：pp_c_draft / pp_c_read / pp_c_readcompleted /
    //                      pp_c_task / pp_c_taskcompleted / pp_c_serialnumber / pp_c_doc_sign /
    //                      pp_c_review。本 crate 的待办待阅列表本来就按 `WHERE xperson = <本人>`
    //                      过滤（lib.rs:592 / :631 为证），所以「谁的和谁删」同锁。
    //   xcreatorPerson —— 行上属主列是 xcreatorPerson：pp_c_work（发起人）/ pp_c_workcompleted（办结人），
    //                      与第十、十一批的 data/work 族同一口径。
    //   ""（admin 档） —— pp_c_record 两列都没有（探针点名：xperson=false creatorPerson=false），
    //                      按「无属主的系统级数据 = admin」收，与 §三.4 / §三.8 同档；
    //                      本批不给它编一个库里不存在的属主列，也不把「空归属」洗成「人人可删」。
    // 每条目 = (handler, 表, 属主列, 落笔形状, 注册点1, 注册点2)。
    type MockAliasArm = (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        (&'static str, &'static str),
        (&'static str, &'static str),
    );
    const MOCK_ALIAS_ARMS: &[MockAliasArm] = &[
        ( "draft_id_mockdeletetoget", "pp_c_draft", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/draft/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/draft/{id}/mockdeletetoget") ),
        ( "read_id_manage_mockdeletetoget", "pp_c_read", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/read/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/read/{id}/manage/mockdeletetoget") ),
        ( "readcompleted_id_manage_mockdeletetoget", "pp_c_readcompleted", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/readcompleted/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/readcompleted/{id}/manage/mockdeletetoget") ),
        ( "record_id_manage_mockdeletetoget", "pp_c_record", "", "del", ("POST", "/api/processplatform/assemble/surface/record/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/record/{id}/manage/mockdeletetoget") ),
        ( "review_id_application_applicationFlag_manage_mockdeletetoget", "pp_c_review", "xcreatorPerson", "del", ("POST", "/api/processplatform/assemble/surface/review/application/manage/mockdeletetoget/{id}/{applicationFlag}"), ("GET", "/api/processplatform/assemble/surface/review/{id}/application/{applicationFlag}/manage/mockdeletetoget") ),
        ( "serialnumber_id_mockdeletetoget", "pp_c_serialnumber", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/serialnumber/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/serialnumber/{id}/mockdeletetoget") ),
        ( "sign_task_taskId_mockdeletetoget", "pp_c_doc_sign", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/sign/task/mockdeletetoget/{taskId}"), ("GET", "/api/processplatform/assemble/surface/sign/task/{taskId}/mockdeletetoget") ),
        ( "sign_id_mockdeletetoget", "pp_c_doc_sign", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/sign/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/sign/{id}/mockdeletetoget") ),
        ( "task_id_manage_mockdeletetoget", "pp_c_task", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/task/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/task/{id}/manage/mockdeletetoget") ),
        ( "taskcompleted_id_manage_mockdeletetoget", "pp_c_taskcompleted", "xperson", "del", ("POST", "/api/processplatform/assemble/surface/taskcompleted/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/taskcompleted/{id}/manage/mockdeletetoget") ),
        ( "work_id_mockdeletetoget", "pp_c_work", "xcreatorPerson", "del", ("POST", "/api/processplatform/assemble/surface/work/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/work/{id}/mockdeletetoget") ),
        ( "work_id_relative_manage_mockdeletetoget", "pp_c_work", "xcreatorPerson", "del", ("POST", "/api/processplatform/assemble/surface/work/relative/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/work/{id}/relative/manage/mockdeletetoget") ),
        ( "work_id_single_manage_mockdeletetoget", "pp_c_work", "xcreatorPerson", "del", ("POST", "/api/processplatform/assemble/surface/work/single/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/work/{id}/single/manage/mockdeletetoget") ),
        ( "workcompleted_id_delete_manage_mockdeletetoget", "pp_c_workcompleted", "xcreatorPerson", "del", ("POST", "/api/processplatform/assemble/surface/workcompleted/delete/manage/mockdeletetoget/{id}"), ("GET", "/api/processplatform/assemble/surface/workcompleted/{id}/delete/manage/mockdeletetoget") ),
        ( "read_id_mockputtopost", "pp_c_read", "xperson", "touch", ("GET", "/api/processplatform/assemble/surface/read/mockputtopost/{id}"), ("POST", "/api/processplatform/assemble/surface/read/{id}/mockputtopost") ),
        ( "record_id_manage_mockputtopost", "pp_c_record", "", "touch", ("GET", "/api/processplatform/assemble/surface/record/manage/mockputtopost/{id}"), ("POST", "/api/processplatform/assemble/surface/record/{id}/manage/mockputtopost") ),
        ( "serialnumber_id_mockputtopost", "pp_c_serialnumber", "xperson", "touch", ("GET", "/api/processplatform/assemble/surface/serialnumber/mockputtopost/{id}"), ("POST", "/api/processplatform/assemble/surface/serialnumber/{id}/mockputtopost") ),
        ( "task_id_mockputtopost", "pp_c_task", "xperson", "touch", ("GET", "/api/processplatform/assemble/surface/task/mockputtopost/{id}"), ("POST", "/api/processplatform/assemble/surface/task/{id}/mockputtopost") ),
        ( "work_v2_id_reroute_mockputtopost", "pp_c_work", "xcreatorPerson", "touch", ("GET", "/api/processplatform/assemble/surface/work/v2/reroute/mockputtopost/{id}"), ("POST", "/api/processplatform/assemble/surface/work/v2/{id}/reroute/mockputtopost") ),
        ( "work_v2_id_retract_mockputtopost", "pp_c_work", "xcreatorPerson", "touch", ("GET", "/api/processplatform/assemble/surface/work/v2/retract/mockputtopost/{id}"), ("POST", "/api/processplatform/assemble/surface/work/v2/{id}/retract/mockputtopost") ),
        ( "work_v2_id_rollback_mockputtopost", "pp_c_work", "xcreatorPerson", "touch", ("GET", "/api/processplatform/assemble/surface/work/v2/rollback/mockputtopost/{id}"), ("POST", "/api/processplatform/assemble/surface/work/v2/{id}/rollback/mockputtopost") ),
        ( "workcompleted_flag_rollback_mockputtopost", "pp_c_workcompleted", "xcreatorPerson", "touch", ("GET", "/api/processplatform/assemble/surface/workcompleted/rollback/mockputtopost/{flag}"), ("POST", "/api/processplatform/assemble/surface/workcompleted/{flag}/rollback/mockputtopost") ),
    ];

    fn mock_uri_at(xid: &str, tpl: &str) -> String {
        let u = tpl
            .replace("{id}", xid)
            .replace("{taskId}", xid)
            .replace("{flag}", xid)
            .replace("{applicationFlag}", "r51-app");
        assert!(!u.contains('{') && !u.contains('}'), "模板残留未替换：{u}");
        u
    }

    fn verb_of(v: &str) -> Method {
        match v {
            "GET" => Method::GET,
            "POST" => Method::POST,
            other => panic!("名单里出现未点名的动词：{other}"),
        }
    }

    async fn call_mock(xid: &str, verb: &str, tpl: &str, person: Option<&str>) -> StatusCode {
        let uri = mock_uri_at(xid, tpl);
        let mut app = app();
        if let Some(p) = person {
            app = app.layer(axum::Extension(shared::testing::test_session(p)));
        }
        app.oneshot(
            Request::builder()
                .uri(&uri)
                .method(verb_of(verb))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    /// 名单里那条 GET 注册（双盲区那一侧）。缺 GET 说明名单与 routes.rs 脱钩，当场红。
    fn alias_get_reg(a: &MockAliasArm) -> (&'static str, &'static str) {
        for r in [a.4, a.5] {
            if r.0 == "GET" {
                return r;
            }
        }
        panic!("别名档缺 GET 注册点：{}", a.0);
    }

    /// 造一行并写入属主。`owner_col` 为空串（admin 档）**什么都不写**——探针实测
    /// `pp_c_record` 的列是 create_time / creator / creator_person / deleted_at / id / order_number /
    /// sequence / update_person / update_time / xcreateTime / xid / xjob / xtitle / xupdateTime / xwork，
    /// `xperson` 与 `xcreatorPerson` 都不存在。所以「行上没有可认的属主列」是这张表的实测事实：
    /// 臂②里「被 seed 的那个人也 403」证的正是「把 record 改成归属档」不可行——
    /// 改回去要么全拒（假收口），要么 42703 变 500，两种都会在臂②当场红。
    /// 兜底建表只在裸库生效（真库里这些 o2server 表已存在，CREATE IF NOT EXISTS 是无操作）。
    async fn seed_mock_row(
        pool: &deadpool_postgres::Pool,
        table: &str,
        xid: &str,
        owner_col: &str,
        owner: &str,
    ) {
        let c = pool.get().await.unwrap();
        c.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS \"{table}\" (\"id\" TEXT PRIMARY KEY, \"xid\" TEXT, \
                 \"xcreatorPerson\" TEXT, xperson TEXT, \"xcreateTime\" TEXT, \"xupdateTime\" TEXT)"
            )
            .as_str(),
            &[],
        )
        .await
        .unwrap();
        if owner_col.is_empty() {
            c.execute(
                format!(
                    "INSERT INTO \"{table}\" (\"id\", \"xid\", \"xupdateTime\") VALUES ($1, $1, 'r51-before') \
                     ON CONFLICT (\"id\") DO UPDATE SET \"xupdateTime\" = 'r51-before'"
                )
                .as_str(),
                &[&xid.to_string()],
            )
            .await
            .unwrap();
        } else {
            c.execute(
                format!(
                    "INSERT INTO \"{table}\" (\"id\", \"xid\", \"{owner_col}\", \"xupdateTime\") VALUES ($1, $1, $2, 'r51-before') \
                     ON CONFLICT (\"id\") DO UPDATE SET \"{owner_col}\" = EXCLUDED.\"{owner_col}\", \"xupdateTime\" = 'r51-before'"
                )
                .as_str(),
                &[&xid.to_string(), &owner.to_string()],
            )
            .await
            .unwrap();
        }
    }

    async fn mock_row_exists(pool: &deadpool_postgres::Pool, table: &str, xid: &str) -> bool {
        let c = pool.get().await.unwrap();
        c.query(
            format!("SELECT 1 FROM \"{table}\" WHERE xid = $1").as_str(),
            &[&xid.to_string()],
        )
        .await
        .unwrap()
        .len()
            > 0
    }

    async fn delete_mock_row(pool: &deadpool_postgres::Pool, table: &str, xid: &str) {
        let c = pool.get().await.unwrap();
        c.execute(
            format!("DELETE FROM \"{table}\" WHERE xid = $1").as_str(),
            &[&xid.to_string()],
        )
        .await
        .unwrap();
    }

    /// 臂①（离线）：44 条注册点无会话一律 401。22 把逐条点名 + 注册点计数钉死 44，
    /// 少一格（有人删了别名注册）或多一格（又长出一把别名）都会先红在计数上。
    #[tokio::test]
    async fn surface_mock_alias_writes_reject_anonymous() {
        assert_eq!(
            MOCK_ALIAS_ARMS.len(),
            22,
            "本批 22 把别名 handler 必须逐条点名"
        );
        let mut bad: Vec<String> = Vec::new();
        let mut regs = 0usize;
        for a in MOCK_ALIAS_ARMS {
            for r in [a.4, a.5] {
                regs += 1;
                let s = call_mock("r51-anon", r.0, r.1, None).await;
                if s != StatusCode::UNAUTHORIZED {
                    bad.push(format!("{} {}/{} -> 期望 401，实际 {s}", a.0, r.0, r.1));
                }
            }
        }
        assert_eq!(regs, 44, "22 把 × 2 条注册点 = 44 格，一格不许少");
        assert!(
            bad.is_empty(),
            "mock 动词别名写缺 401（门缺失或门晚于取会话）：{bad:?}"
        );
    }

    /// 臂②（真库）：逐把打「拒绝侧 + 放行侧」两半，落笔形状分别用**行是否还在**（del 族）
    /// 和 `xupdateTime` 是否脱离哨兵（touch 族）作证——handler 未命中行也回 200，
    /// 状态码不足以证明写发生了，所以放行侧只认落库差。
    /// 三档各自的证伪格：
    ///   归属档（col 非空）：他人 403 且未落笔 → 本人 200 且落笔 → admin 200 且落笔（门不许把 admin 夹死）；
    ///   admin 档（record，col 为空）：他人 403 → **归属列里那个人也 403 且未落笔**（这一格证伪
    ///   「把 record 改成归属档就算收口」，因为行上根本没有属主列）→ admin 200 且落笔。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_mock_alias_owner_gate_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: surface_mock_alias_owner_gate_lands (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let owner = format!("u-r51-owner-{tag}");
        let stranger = format!("u-r51-stranger-{tag}");
        let admin = seed_admin(&pool, &tag).await;
        let mut bad: Vec<String> = Vec::new();

        /// 放行侧凭据：del 族要求行已消失，touch 族要求时间戳脱离哨兵。
        async fn landed(
            pool: &deadpool_postgres::Pool,
            table: &str,
            xid: &str,
            shape: &str,
        ) -> bool {
            match shape {
                "del" => !mock_row_exists(pool, table, xid).await,
                _ => ppc_update_time(pool, table, xid)
                    .await
                    .map(|u| u != "r51-before")
                    .unwrap_or(false),
            }
        }

        for a in MOCK_ALIAS_ARMS {
            let (table, col, shape, name) = (a.1, a.2, a.3, a.0);
            let (_, get_tpl) = alias_get_reg(a);
            let xid = format!("r51-{name}-{tag}");
            seed_mock_row(&pool, table, &xid, col, &owner).await;

            // ① 他人会话：403 且未落笔
            let s = call_mock(&xid, "GET", get_tpl, Some(&stranger)).await;
            if s != StatusCode::FORBIDDEN {
                bad.push(format!("{name}: 他人会话期望 403，实际 {s}"));
            } else if !mock_row_exists(&pool, table, &xid).await {
                bad.push(format!("{name}: 403 之后行已消失，门晚于 SQL"));
            } else if shape == "touch"
                && ppc_update_time(&pool, table, &xid).await.as_deref() != Some("r51-before")
            {
                bad.push(format!("{name}: 403 之后 xupdateTime 被改写，门晚于 SQL"));
            }

            if col.is_empty() {
                // ② admin 档：归属列里那个人也照样拒，且未落笔
                let s = call_mock(&xid, "GET", get_tpl, Some(&owner)).await;
                if s != StatusCode::FORBIDDEN {
                    bad.push(format!(
                        "{name}: record 行上没有属主列，非 admin 会话期望 403，实际 {s}"
                    ));
                } else if shape == "touch"
                    && ppc_update_time(&pool, table, &xid).await.as_deref() != Some("r51-before")
                {
                    bad.push(format!(
                        "{name}: 非 admin 走了 touch 落笔，admin 档被写成归属档"
                    ));
                } else if shape == "del" && !mock_row_exists(&pool, table, &xid).await {
                    bad.push(format!(
                        "{name}: 非 admin 真删了 record 行，admin 档被写成归属档"
                    ));
                }
            } else {
                // ② 归属档：本人放行且真落笔
                let s = call_mock(&xid, "GET", get_tpl, Some(&owner)).await;
                if s != StatusCode::OK {
                    bad.push(format!("{name}: 本人期望 200，实际 {s}"));
                }
                if !landed(&pool, table, &xid, shape).await {
                    bad.push(format!("{name}: 本人放行后写未落库（{shape} 族）"));
                }
            }

            // ③ admin 放行侧：另起一行，免得与 ② 的落笔互相污染读数
            let axid = format!("r51adm-{name}-{tag}");
            seed_mock_row(&pool, table, &axid, col, &owner).await;
            let s = call_mock(&axid, "GET", get_tpl, Some(&admin)).await;
            if s == StatusCode::UNAUTHORIZED || s == StatusCode::FORBIDDEN {
                bad.push(format!("{name}: admin 被门夹死，实际 {s}"));
            }
            if !landed(&pool, table, &axid, shape).await {
                bad.push(format!("{name}: admin 放行后写未落库（{shape} 族）"));
            }

            delete_mock_row(&pool, table, &xid).await;
            delete_mock_row(&pool, table, &axid).await;
        }
        assert!(bad.is_empty(), "mock 动词别名归属门不符：{bad:?}");
    }

    /// 臂④（真库）：他人会话打 44 条注册点（含 POST 侧）一律 403，钉「门不挑动词」。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn surface_mock_alias_gate_covers_both_verbs() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: surface_mock_alias_gate_covers_both_verbs (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let owner = format!("u-r51o-{tag}");
        let stranger = format!("u-r51s-{tag}");
        let mut bad: Vec<String> = Vec::new();
        let mut regs = 0usize;
        for a in MOCK_ALIAS_ARMS {
            let xid = format!("r51bv-{name}-{tag}", name = a.0);
            seed_mock_row(&pool, a.1, &xid, a.2, &owner).await;
            for r in [a.4, a.5] {
                regs += 1;
                let s = call_mock(&xid, r.0, r.1, Some(&stranger)).await;
                if s != StatusCode::FORBIDDEN {
                    bad.push(format!("{} {} -> 期望 403，实际 {s}", a.0, r.0));
                }
            }
            delete_mock_row(&pool, a.1, &xid).await;
        }
        assert_eq!(regs, 44, "两动词同门必须打满 44 条注册点");
        assert!(
            bad.is_empty(),
            "mock 别名门漏了 POST 侧（或反过来）：{bad:?}"
        );
    }
}

/// 第十九批：GET 面上「按 ID touch 他人流程条目行」的 16 把归属门。
///
/// 这些测试编码业务意图：
///  1. `{read|readcompleted|task|taskcompleted|work|workcompleted}/list/…/process` 的名字是
///     「列表计数」，函数体第一句却是 `UPDATE PP_C_* SET "xupdateTime" = NOW() WHERE xid = $1`
///     ——按 ID 改**他人**条目的审计时间戳。注册在 GET 上 ⇒ csrf 与 requires_admin 两道中间件门
///     都够不着，原先任一登录用户可逐 ID 伪造他人的待办 / 待阅 / 单据时间。
///  2. 门按「行上到底有没有可认的属主列」定档：READ / READCOMPLETED / TASK / TASKCOMPLETED /
///     DRAFT / HANDOVER / TASK_PROCESS_MODE 取 `xperson`（本 crate 的列表本来就按 xperson 过滤），
///     WORK / WORKCOMPLETED 行上没有 xperson，取 `"xcreatorPerson"`。口径与 `u4_require_ppc_*` 同锁：
///     无会话先 401（不触库、离线可证）；归属非空 → 本人或 admin；归属为空或行不存在 → 只允许 admin。
///  3. `draft/{id}/start` 本批**只加门、不修**它的既有坏状态：真库探针（事务内跑完回滚）证
///     `pp_c_draft` **没有 xstatus 列**（42703），且 `SET xstatus = $1 … WHERE xid = $2` 只绑了
///     一个参数（`there is no parameter $2`）⇒ 谁调都 500。归属门插在 SQL 之前，所以本人身份
///     也救不了它——这条由下面的 owner 臂**逐把点名**，不许被读成门的缺口。
#[cfg(test)]
mod r419_gate_tests {
    use axum::extract::{Extension, Path};
    use shared::error::AppError;
    use shared::testing::{is_db_available, mock_pool, test_pool, test_session};

    const OWNER: &str = "u-r419-owner";
    const OTHER: &str = "u-r419-other";
    const ADMIN: &str = "u-r419-admin";

    /// (表, 归属列, 行 id)——id 与 `call_all` 里的路径参数一一对应，改一处必须改另一处。
    const ROWS: &[(&str, &str, &str)] = &[
        ("PP_C_READ", "xperson", "r419-read"),
        ("PP_C_READCOMPLETED", "xperson", "r419-readcompleted"),
        ("PP_C_TASK", "xperson", "r419-task"),
        ("PP_C_TASKCOMPLETED", "xperson", "r419-taskcompleted"),
        ("PP_C_WORK", "\"xcreatorPerson\"", "r419-work"),
        (
            "PP_C_WORKCOMPLETED",
            "\"xcreatorPerson\"",
            "r419-workcompleted",
        ),
        ("PP_C_DRAFT", "xperson", "r419-draft"),
        ("PP_C_HANDOVER", "xperson", "r419-handover"),
        ("PP_C_TASK_PROCESS_MODE", "xperson", "r419-mode"),
    ];

    /// 16 把 handler 逐个具名调用。`person = None` 即「无会话」；表长本身就是本批覆盖面，
    /// 由 `assert_eq!(cases.len(), 16)` 钉住——少一把（或门漏一把）都会红。
    async fn call_all(
        pool: &deadpool_postgres::Pool,
        person: Option<&str>,
    ) -> Vec<(&'static str, Result<(), AppError>)> {
        let p = || Extension(pool.clone());
        let s = || person.map(|x| Extension(test_session(x)));
        let one = |v: &str| Path(v.to_string());
        let two = |v: &str| Path(("0".to_string(), v.to_string()));
        vec![
            (
                "handover_id_cancel",
                crate::handover_id_cancel(p(), s(), one("r419-handover"))
                    .await
                    .map(|_| ()),
            ),
            (
                "handover_id_process",
                crate::handover_id_process(p(), s(), one("r419-handover"))
                    .await
                    .map(|_| ()),
            ),
            (
                "draft_id_start",
                crate::draft_id_start(p(), s(), one("r419-draft"))
                    .await
                    .map(|_| ()),
            ),
            (
                "mode_id_delete",
                crate::mode_id_delete(p(), s(), one("r419-mode"))
                    .await
                    .map(|_| ()),
            ),
            (
                "read_list_count_application_applicationFlag_process",
                crate::read_list_count_application_applicationFlag_process(
                    p(),
                    s(),
                    one("r419-read"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "read_list_count_application_applicationFlag_process_p2",
                crate::read_list_count_application_applicationFlag_process_p2(
                    p(),
                    s(),
                    two("r419-read"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "readcompleted_list_count_application_applicationFlag_process",
                crate::readcompleted_list_count_application_applicationFlag_process(
                    p(),
                    s(),
                    one("r419-readcompleted"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "readcompleted_list_count_application_applicationFlag_process_p2",
                crate::readcompleted_list_count_application_applicationFlag_process_p2(
                    p(),
                    s(),
                    two("r419-readcompleted"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "task_list_count_application_applicationFlag_process",
                crate::task_list_count_application_applicationFlag_process(
                    p(),
                    s(),
                    one("r419-task"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "task_list_count_application_applicationFlag_process_p2",
                crate::task_list_count_application_applicationFlag_process_p2(
                    p(),
                    s(),
                    two("r419-task"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "taskcompleted_list_count_application_applicationFlag_process",
                crate::taskcompleted_list_count_application_applicationFlag_process(
                    p(),
                    s(),
                    one("r419-taskcompleted"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "taskcompleted_list_count_application_applicationFlag_process_p2",
                crate::taskcompleted_list_count_application_applicationFlag_process_p2(
                    p(),
                    s(),
                    two("r419-taskcompleted"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_list_count_application_applicationFlag_process",
                crate::work_list_count_application_applicationFlag_process(
                    p(),
                    s(),
                    one("r419-work"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_list_count_application_applicationFlag_process_p2",
                crate::work_list_count_application_applicationFlag_process_p2(
                    p(),
                    s(),
                    two("r419-work"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "workcompleted_list_count_application_applicationFlag_process",
                crate::workcompleted_list_count_application_applicationFlag_process(
                    p(),
                    s(),
                    one("r419-workcompleted"),
                )
                .await
                .map(|_| ()),
            ),
            (
                "workcompleted_list_count_application_applicationFlag_process_p2",
                crate::workcompleted_list_count_application_applicationFlag_process_p2(
                    p(),
                    s(),
                    two("r419-workcompleted"),
                )
                .await
                .map(|_| ()),
            ),
        ]
    }

    /// 表名 / 列名只取本文件 `ROWS` 里的字面量，ID 与人员走参数绑定。
    async fn seed_rows(person: &str) {
        let c = test_pool().get().await.unwrap();
        for (t, col, id) in ROWS {
            c.execute(
                format!(
                    "INSERT INTO {t} (id, xid, {col}) VALUES ($1, $1, $2) \
                     ON CONFLICT (id) DO UPDATE SET xid = EXCLUDED.xid, {col} = EXCLUDED.{col}"
                )
                .as_str(),
                &[&id.to_string(), &person.to_string()],
            )
            .await
            .unwrap();
        }
    }

    /// 空归属（xperson = ''）——用来校「归属为空只允许 admin」那半口径。
    async fn clear_actor() {
        let c = test_pool().get().await.unwrap();
        c.execute(
            "UPDATE PP_C_READ SET xperson = '' WHERE id = $1",
            &[&"r419-read".to_string()],
        )
        .await
        .unwrap();
    }

    async fn unseed() {
        let c = test_pool().get().await.unwrap();
        for (t, _, id) in ROWS {
            c.execute(
                format!("DELETE FROM {t} WHERE id = $1").as_str(),
                &[&id.to_string()],
            )
            .await
            .unwrap();
        }
    }

    /// 按 `is_admin` 的真实口径（auth_role.name='admin' ∧ 未删未禁）幂等造 admin。
    async fn seed_admin_role() {
        let c = test_pool().get().await.unwrap();
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ('r419-role-admin', 'admin', false) ON CONFLICT (id) DO NOTHING",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ($1, $1, 'r419 admin', 'x') ON CONFLICT (id) DO NOTHING",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1, 'r419-role-admin', 'r419-unit') ON CONFLICT DO NOTHING",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
    }

    /// 无会话 → 401，且**先于任何读库**：`mock_pool()` 连不上库，读数里出现 `Internal`
    /// 就说明门插在了 `pool.get()` 之后。
    #[tokio::test]
    async fn r419_get_writes_reject_without_session() {
        let pool = mock_pool();
        let cases = call_all(&pool, None).await;
        let bad: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            bad.is_empty(),
            "以下 GET touch 面无会话未回 401（出现 Internal 就是门插在读库之后）：{bad:?}"
        );
        assert_eq!(cases.len(), 16, "本批 16 把必须逐把点名，少一把即漏门");
    }

    /// 他人会话 → 16 把全 403；本人会话 → 恰好 15 把放行、只有 `draft_id_start` 因既有
    /// schema drift 落 `Internal`；空归属行 → 他人 403、admin 放行。三段合起来才证明
    /// 门不是「恒 403」的摆设，也不是「谁都能 touch」的漏门。
    #[tokio::test]
    async fn r419_get_writes_owner_gate_lands() {
        if !is_db_available().await {
            eprintln!("skipping r419_get_writes_owner_gate_lands: DATABASE_URL not reachable");
            return;
        }
        seed_admin_role().await;
        seed_rows(OWNER).await;
        let pool = test_pool();
        let bad: Vec<String> = call_all(&pool, Some(OTHER))
            .await
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            bad.is_empty(),
            "他人会话按 ID touch 他人条目行必须 403，不得落笔：{bad:?}"
        );

        let owner = call_all(&pool, Some(OWNER)).await;
        let ok: Vec<&str> = owner
            .iter()
            .filter(|(_, r)| r.is_ok())
            .map(|(k, _)| *k)
            .collect();
        let broke: Vec<&str> = owner
            .iter()
            .filter(|(_, r)| matches!(r, Err(AppError::Internal)))
            .map(|(k, _)| *k)
            .collect();
        assert_eq!(
            broke,
            vec!["draft_id_start"],
            "除 draft/{{id}}/start（pp_c_draft 无 xstatus + 少绑一个参数，本批只加门不修）之外，\
             本人调用不该触库失败：{broke:?}｜放行 {ok:?}"
        );
        assert_eq!(ok.len(), 15, "本人 15 把必须逐把放行，实际放行 {ok:?}");

        clear_actor().await;
        let r = crate::read_list_count_application_applicationFlag_process(
            Extension(pool.clone()),
            Some(Extension(test_session(OTHER))),
            Path("r419-read".to_string()),
        )
        .await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "空归属行只允许 admin，不得把数据缺陷洗成权限放行：{:?}",
            r.as_ref().err()
        );
        let a = crate::read_list_count_application_applicationFlag_process(
            Extension(pool.clone()),
            Some(Extension(test_session(ADMIN))),
            Path("r419-read".to_string()),
        )
        .await;
        assert!(a.is_ok(), "admin 应能在空归属行上放行：{:?}", a.err());
        unseed().await;
    }
}

/// 第 38 批回归臂：表面生命周期（create/save/publish/delete，`x_process_surface`）
/// 与 `mode_save`（`PP_C_TASK_PROCESS_MODE`）写门。
/// 离线：无会话 401（先于任何 `pool.get()`）；表面四把非 admin（mock pool 下
/// `is_admin` fail-closed）403。`mode_save` 的非属主 403 需真库（其门先按 `xperson`
/// 查归属，mock pool 会在 `pool.get` 处落 `Internal`），故此处只断言其 401 档。
#[cfg(test)]
mod surface_gate_tests {
    use axum::extract::{Extension, Path};
    use shared::error::AppError;
    use shared::testing::{mock_pool, test_session};

    const OTHER: &str = "u-surface-other";
    const OWNER: &str = "u-surface-owner";

    /// `mode_save` 的归属档真库臂：`u4_require_ppc_person` 先按 `PP_C_TASK_PROCESS_MODE.xperson`
    /// 查归属，mock pool 证不到 403 那一档，故这里造一行按真实库跑「非属主 403 不落笔 / 属主放行」。
    #[tokio::test]
    async fn mode_save_owner_gate_lands() {
        use shared::testing::{is_db_available, test_pool};
        if !is_db_available().await {
            eprintln!("skipping mode_save_owner_gate_lands: DATABASE_URL not reachable");
            return;
        }
        let pool = test_pool();
        let c = pool.get().await.unwrap();
        c.execute(
            "INSERT INTO PP_C_TASK_PROCESS_MODE (id, xid, xperson) VALUES ('b38-mode','b38-mode',$1) \
             ON CONFLICT (id) DO UPDATE SET xperson = $1",
            &[&OWNER.to_string()],
        )
        .await
        .unwrap();

        // 非属主 → 403，不落笔
        let r = crate::mode_save(
            Extension(pool.clone()),
            Some(Extension(test_session(OTHER))),
            axum::Json(serde_json::json!({ "id": "b38-mode" })),
        )
        .await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "mode_save 非属主必须 403，实得 {:?}",
            r.as_ref().err()
        );

        // 属主 → 放行
        let r = crate::mode_save(
            Extension(pool.clone()),
            Some(Extension(test_session(OWNER))),
            axum::Json(serde_json::json!({ "id": "b38-mode" })),
        )
        .await;
        assert!(r.is_ok(), "mode_save 属主应放行：{:?}", r.as_ref().err());

        // 收口零残留
        c.execute(
            "DELETE FROM PP_C_TASK_PROCESS_MODE WHERE id = 'b38-mode'",
            &[],
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn surface_and_mode_writes_require_gate() {
        let dead = mock_pool();
        let body = || {
            axum::Json(crate::CreateSurfaceRequest {
                name: Some("x".to_string()),
                template: None,
                category: None,
                content: None,
            })
        };

        // 无会话：五把全 401
        assert!(
            matches!(
                crate::create_surface(Extension(dead.clone()), None, body()).await,
                Err(AppError::Unauthorized)
            ),
            "create_surface 无会话必须 401"
        );
        assert!(
            matches!(
                crate::save_surface(
                    Extension(dead.clone()),
                    None,
                    Path("s1".to_string()),
                    axum::Json(serde_json::json!({}))
                )
                .await,
                Err(AppError::Unauthorized)
            ),
            "save_surface 无会话必须 401"
        );
        assert!(
            matches!(
                crate::publish_surface(Extension(dead.clone()), None, Path("s1".to_string())).await,
                Err(AppError::Unauthorized)
            ),
            "publish_surface 无会话必须 401"
        );
        assert!(
            matches!(
                crate::delete_surface(Extension(dead.clone()), None, Path("s1".to_string())).await,
                Err(AppError::Unauthorized)
            ),
            "delete_surface 无会话必须 401"
        );
        assert!(
            matches!(
                crate::mode_save(
                    Extension(dead.clone()),
                    None,
                    axum::Json(serde_json::json!({ "id": "m1" }))
                )
                .await,
                Err(AppError::Unauthorized)
            ),
            "mode_save 无会话必须 401"
        );

        // 非 admin（mock pool 下 is_admin fail-closed）：表面四把全 403
        let s = || Some(Extension(test_session(OTHER)));
        assert!(
            matches!(
                crate::create_surface(Extension(dead.clone()), s(), body()).await,
                Err(AppError::Forbidden)
            ),
            "create_surface 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::save_surface(
                    Extension(dead.clone()),
                    s(),
                    Path("s1".to_string()),
                    axum::Json(serde_json::json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "save_surface 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::publish_surface(Extension(dead.clone()), s(), Path("s1".to_string())).await,
                Err(AppError::Forbidden)
            ),
            "publish_surface 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::delete_surface(Extension(dead.clone()), s(), Path("s1".to_string())).await,
                Err(AppError::Forbidden)
            ),
            "delete_surface 非 admin 必须 403"
        );
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 第五十八批（2026-10 四轮）：processplatform_assemble_surface 的 12 把 mockputtopost 别名写。
//
// 这批是「GET 名 + PUT 实」的别名：`*_manage` 是 SELECT，`*_manage_mockputtopost` 才是
// UPDATE/DELETE 真写（例如 task_id_processing_manage 只读、其 mockputtopost 才 UPDATE
// PP_C_TASK）。原 12 把均不取会话、无门禁。现按归属列收门：
// - 待阅/待办/已办：`u4_require_ppc_person`（`xperson`，本人或 admin）；
// - work：`u4_require_work_actor`（`xcreatorPerson`）；
// - 引擎 job 物理删：`u4_require_ppc_person(PP_C_JOB)`。
// 无会话一律 401（先于任何 SQL，离线可证）。
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod b58_gate_tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    /// 无会话臂：dead pool + None 会话，12 把必须全 401（门在 pool.get() 之前）。
    #[tokio::test]
    async fn b58_mockputtopost_writes_reject_without_session() {
        let pool = dead_pool();
        let p = || Path("x1".to_string());
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "read_id_opinion_manage_mockputtopost",
                crate::read_id_opinion_manage_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "read_id_processing_manage_mockputtopost",
                crate::read_id_processing_manage_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "read_id_reset_manage_mockputtopost",
                crate::read_id_reset_manage_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_opinion_manage_mockputtopost",
                crate::task_id_opinion_manage_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_processing_manage_mockputtopost",
                crate::task_id_processing_manage_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "task_id_reset_manage_mockputtopost",
                crate::task_id_reset_manage_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "task_v2_id_reset",
                crate::task_v2_id_reset(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "task_v2_id_reset_mockputtopost",
                crate::task_v2_id_reset_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "taskcompleted_id_opinion_manage_mockputtopost",
                crate::taskcompleted_id_opinion_manage_mockputtopost(
                    Extension(pool.clone()),
                    None,
                    p(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "work_id_processing_mockputtopost",
                crate::work_id_processing_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "work_v2_id_add_split_mockputtopost",
                crate::work_v2_id_add_split_mockputtopost(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
            (
                "correlation_job_job_delete",
                crate::correlation_job_job_delete(Extension(pool.clone()), None, p())
                    .await
                    .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 12, "本批 12 把必须逐把点名");
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }
}
