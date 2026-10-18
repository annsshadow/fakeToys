// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第二十一批）：attendance 的 GET 写面 9 把。
//!
//! 这 9 把原本函数体第一条语句就是写 SQL，**一把都不读会话**：路由注册在 GET 上
//! （csrf 与 requires_admin 两道中间件都只认 POST/PUT/PATCH/DELETE），且前缀不在豁免名单里，
//! 所以尺子归成 K3「已认证无会话无门禁」——任何登录用户都能按 id / 日期区间 / 周期 / 设置 code
//! 改他人或全员的考勤数据。收口档位按**本 crate 自己的既有分工**定（见 lib.rs 里 plan002 U2 的
//! 族注释：管理员动作 require_admin，配置人自己的行走 creator_person 归属档）：
//!
//! - admin 档 8 把：appeal archive（归档会把申诉移出待审批队列，与同表 audit/check/workflow_sync 同级）、
//!   detail analyse/id、analyse/{start}/{end}（同族 analyse/redo 就是 require_admin）、
//!   detail archive、checkDetailWithPersonByCycle、setting/enable/type（全员生效的开关）、
//!   dingding / qywx sync start（`x_attendance_sync_record` **连 creator 列都没有**，无从按归属量）。
//! - 归属档 1 把：v2/group/{id}/refresh/participate——考勤组是配置人自己的行，同组 delete 就走
//!   `creator_person`，而 `shared::middleware::require_owner` 内含 admin 放行，管理员照样能刷。
//!
//! 两臂：无会话臂用**连不上库**的 pool 跑，9 把必须全 401——出现 `Internal` 就说明门插在了
//! `pool.get()` 之后；真库臂跑「他人 403 且不落笔 → 本人只放行归属那把 → admin 全档落笔」，
//! 落笔与不落笔都逐格回读，免得门被写成「一律拒」还测的是空气。
//!
//! 历史豁免已撤：`checkDetailWithPersonByCycle` 曾因 `cycle_year` / `cycle_month` 在真库是
//! integer 而 handler 把路径字符串直接绑进 `WHERE cycle_year = $1`（42883 operator does not
//! exist: integer = text）。本批原先只加门不修语义，故 admin 格一度只允许 `Internal`。现已把
//! handler 路径改成 `Path<(i32, i32)>`，类型口径修好：admin 放行后 UPDATE 真落笔，
//! `x_attendance_detail.checked` 由 false 变 true。本文件 admin 段要求它对 9 把全部放行，
//! 并在落笔回读里钉住 `checked=true`，防止口径回退再次引入 42883。

#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, Json, Path};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Client, Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;

    const APPEAL_ID: &str = "b21-appeal-own";
    const DETAIL_ID: &str = "b21-detail-own";
    const GROUP_ID: &str = "b21-group-own";
    const GROUP_EMPTY_ID: &str = "b21-group-empty";
    const GROUP_MISSING_ID: &str = "b21-group-does-not-exist";
    const SETTING_CODE: &str = "b21-setting-code";
    /// 只为把 analyse/{start}/{end} 与 checkDetailWithPersonByCycle 两把的射程圈在本臂自己的行内：
    /// date 是 TEXT，按字典序 `b21-day-0001 <= b21-day-0500 <= b21-day-9999` 成立，
    /// 而真实明细的 date 形如 `2026-10-01`，永远不会落进这个区间（否则就是测试互相踩，不是门禁失效）。
    const DATE_FROM: &str = "b21-day-0001";
    const DATE_TO: &str = "b21-day-9999";
    const DETAIL_DATE: &str = "b21-day-0500";
    /// 同理，周期取真库不可能出现的 9999-99，别把别人的明细圈进这把批量 UPDATE 的射程。
    /// handler 路径按真库 integer 口径用 i32 绑定（42883 修复），故常量同步为 i32。
    const CYCLE_YEAR: i32 = 9999;
    const CYCLE_MONTH: i32 = 99;
    const SYNC_FROM: &str = "b21-sync-from";
    const SYNC_TO: &str = "b21-sync-to";
    const SENTINEL: &str = "b21-before";

    const ADMIN_PERSON: &str = "b21-admin-person";
    const OWNER_PERSON: &str = "b21-owner-person";
    const OTHER_PERSON: &str = "b21-other-person";

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn p(v: &str) -> Path<String> {
        Path(v.to_string())
    }

    fn p2(a: &str, b: &str) -> Path<(String, String)> {
        Path((a.to_string(), b.to_string()))
    }

    fn p2i(a: i32, b: i32) -> Path<(i32, i32)> {
        Path((a, b))
    }

    /// 9 把逐个具名调用；表长由 `assert_eq!(cases.len(), 9)` 钉住，少点名一把即红。
    /// 除 `setting/enable/type`（要把 code 关成 false，好让 admin 格有可观测的落笔）外一律不带体：
    /// 门在读体之前，体的内容不影响门禁读数。
    async fn call_all(pool: &Pool, person: Option<&str>) -> Vec<(&'static str, Result<(), AppError>)> {
        let sess = || person.map(|w| Extension(shared::testing::test_session(w)));
        let ext = || Extension(pool.clone());
        vec![
            (
                "appeal_archive_id",
                crate::attendanceappealInfo_archive_id(ext(), sess(), p(APPEAL_ID))
                    .await
                    .map(|_| ()),
            ),
            (
                "detail_analyse_id_id",
                crate::attendancedetail_analyse_id_id(ext(), sess(), p(DETAIL_ID))
                    .await
                    .map(|_| ()),
            ),
            (
                "detail_analyse_range",
                crate::attendancedetail_analyse_startDate_endDate(
                    ext(),
                    sess(),
                    p2(DATE_FROM, DATE_TO),
                )
                .await
                .map(|_| ()),
            ),
            (
                "detail_archive_id",
                crate::attendancedetail_archive_id(ext(), sess(), p(DETAIL_ID))
                    .await
                    .map(|_| ()),
            ),
            (
                "detail_check_by_cycle",
                crate::attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth(
                    ext(),
                    sess(),
                    p2i(CYCLE_YEAR, CYCLE_MONTH),
                )
                .await
                .map(|_| ()),
            ),
            (
                "setting_enable_type",
                crate::attendancesetting_enable_type(
                    ext(),
                    sess(),
                    Some(Json(json!({ "code": SETTING_CODE, "enabled": false }))),
                )
                .await
                .map(|_| ()),
            ),
            (
                "group_refresh_participate",
                crate::v2_group_refresh_participate(ext(), sess(), p(GROUP_ID))
                    .await
                    .map(|_| ()),
            ),
            (
                "dingding_sync_start",
                crate::dingding_sync_start(ext(), sess(), p2(SYNC_FROM, SYNC_TO))
                    .await
                    .map(|_| ()),
            ),
            (
                "qywx_sync_start",
                crate::qywx_sync_start(ext(), sess(), p2(SYNC_FROM, SYNC_TO))
                    .await
                    .map(|_| ()),
            ),
        ]
    }

    /// 只在 `call_all` 之外单列的两格：归属为空 / 行不存在（都打在归属档那把上）。
    async fn call_edge(
        pool: &Pool,
        person: Option<&str>,
    ) -> Vec<(&'static str, Result<(), AppError>)> {
        let sess = || person.map(|w| Extension(shared::testing::test_session(w)));
        let ext = || Extension(pool.clone());
        vec![
            (
                "group_empty_owner",
                crate::v2_group_refresh_participate(ext(), sess(), p(GROUP_EMPTY_ID))
                    .await
                    .map(|_| ()),
            ),
            (
                "group_missing_row",
                crate::v2_group_refresh_participate(ext(), sess(), p(GROUP_MISSING_ID))
                    .await
                    .map(|_| ()),
            ),
        ]
    }

    /// 一格一读数：列一律 `::text` 取，避免 bool/文本两套解码口径；行不存在取 `__absent__`。
    async fn cell(c: &Client, tbl: &str, col: &str, key: &str) -> String {
        let sql = format!("SELECT {col}::text AS v FROM {tbl} WHERE id = $1");
        c.query_opt(sql.as_str(), &[&key.to_string()])
            .await
            .unwrap()
            .map(|r| r.get::<_, Option<String>>("v").unwrap_or_default())
            .unwrap_or_else(|| "__absent__".to_string())
    }

    /// 应仍等于造数时的原值（标志位 false / 开关 true / 哨兵 / 同步表零行）。
    /// `group_moved` 是「本人放行的归属档那把是否已刷过 GROUP_ID」：从 owner 段起它仍等于
    /// 哨兵反而是门吞了放行，所以这一格的原值期望必须跟着段走，其余格不受影响。
    async fn untouched(group_moved: bool) -> Vec<String> {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        let want = [
            ("x_attendance_appeal_info", "archived", APPEAL_ID, "false"),
            ("x_attendance_detail", "analysed", DETAIL_ID, "false"),
            ("x_attendance_detail", "checked", DETAIL_ID, "false"),
            ("x_attendance_detail", "archived", DETAIL_ID, "false"),
            ("x_attendance_setting", "enabled", SETTING_CODE, "true"),
            ("x_attendance_v2_group", "update_time", GROUP_EMPTY_ID, SENTINEL),
        ];
        let mut out = Vec::new();
        for (tbl, col, key, exp) in want {
            let got = cell(&c, tbl, col, key).await;
            if got != exp {
                out.push(format!("{tbl}.{col}/{key} => {got:?} 期望 {exp:?}"));
            }
        }
        let group = cell(&c, "x_attendance_v2_group", "update_time", GROUP_ID).await;
        if group_moved && group == SENTINEL {
            out.push(format!(
                "x_attendance_v2_group.update_time/{GROUP_ID} 仍是哨兵：本人放行的归属档那把没落笔"
            ));
        }
        if !group_moved && group != SENTINEL {
            out.push(format!(
                "x_attendance_v2_group.update_time/{GROUP_ID} => {group:?}：拒人的段不得刷到本人那行"
            ));
        }
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) FROM x_attendance_sync_record WHERE start_date = $1",
                &[&SYNC_FROM.to_string()],
            )
            .await
            .unwrap()
            .get(0);
        if n != 0 {
            out.push(format!("x_attendance_sync_record 落笔 {n} 行"));
        }
        out
    }

    async fn seed_admin_role(c: &Client) {
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ('b21-role-admin', 'admin', false) \
             ON CONFLICT (id) DO UPDATE SET name = 'admin', disable = false",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'b21 attendance admin', 'x') ON CONFLICT (id) DO NOTHING",
            &[&ADMIN_PERSON.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'b21-role-admin', 'b21-unit') ON CONFLICT DO NOTHING",
            &[&ADMIN_PERSON.to_string()],
        )
        .await
        .unwrap();
    }

    /// 造数前先 `unseed()` 按 id 删净，再普通 INSERT——保持「先删后插」的可重复序列，
    /// 本臂连跑两遍读数一致靠的是这条序列，而不是上一遍留下的数据。
    /// （曾因 migration 032 未给这三张表建主键、`ON CONFLICT (id)` 会撞 42P10 而禁用 ON
    /// CONFLICT；migration 109 已补上主键，此处保留先删后插语义不变。）
    async fn seed() {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        unseed().await;
        c.execute(
            "INSERT INTO x_attendance_appeal_info (id, person_id, creator_person, archived, update_time) \
             VALUES ($1, $2, $2, false, $3)",
            &[
                &APPEAL_ID.to_string(),
                &OWNER_PERSON.to_string(),
                &SENTINEL.to_string(),
            ],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO x_attendance_detail (id, person_id, creator_person, date, cycle_year, \
                                             cycle_month, analysed, checked, archived, update_time) \
             VALUES ($1, $2, $2, $3, 9999, 99, false, false, false, $4)",
            &[
                &DETAIL_ID.to_string(),
                &OWNER_PERSON.to_string(),
                &DETAIL_DATE.to_string(),
                &SENTINEL.to_string(),
            ],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO x_attendance_setting (id, code, creator_person, enabled, update_time) \
             VALUES ($1, $2, $3, true, $4)",
            &[
                &SETTING_CODE.to_string(),
                &SETTING_CODE.to_string(),
                &OWNER_PERSON.to_string(),
                &SENTINEL.to_string(),
            ],
        )
        .await
        .unwrap();
        for (gid, owner) in [(GROUP_ID, OWNER_PERSON), (GROUP_EMPTY_ID, "")] {
            c.execute(
                "INSERT INTO x_attendance_v2_group (id, group_name, creator_person, update_time) \
                 VALUES ($1, $2, $3, $4) \
                 ON CONFLICT (id) DO UPDATE SET group_name = $2, creator_person = $3, update_time = $4",
                &[
                    &gid.to_string(),
                    &format!("b21 group {gid}"),
                    &owner.to_string(),
                    &SENTINEL.to_string(),
                ],
            )
            .await
            .unwrap();
        }
        seed_admin_role(&c).await;
    }

    async fn unseed() {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        for (tbl, key) in [
            ("x_attendance_appeal_info", APPEAL_ID),
            ("x_attendance_detail", DETAIL_ID),
            ("x_attendance_setting", SETTING_CODE),
        ] {
            let sql = format!("DELETE FROM {tbl} WHERE id = $1");
            c.execute(sql.as_str(), &[&key.to_string()]).await.unwrap();
        }
        c.execute("DELETE FROM x_attendance_v2_group WHERE id LIKE 'b21-%'", &[])
            .await
            .unwrap();
        c.execute(
            "DELETE FROM x_attendance_sync_record WHERE start_date = $1",
            &[&SYNC_FROM.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM auth_person_role WHERE person_id = $1",
            &[&ADMIN_PERSON.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM auth_person WHERE id = $1",
            &[&ADMIN_PERSON.to_string()],
        )
        .await
        .unwrap();
        c.execute("DELETE FROM auth_role WHERE id = 'b21-role-admin'", &[])
            .await
            .unwrap();
    }

    /// 无会话 → 9 把全 401，且**先于任何读库**：这个 pool 连不上库，读数里出现
    /// `Internal` 就说明门插在了 `pool.get()` 之后。
    #[tokio::test]
    async fn b21_get_writes_reject_without_session() {
        let pool = dead_pool();
        let cases = call_all(&pool, None).await;
        let bad: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            bad.is_empty(),
            "以下 GET 写面无会话未回 401（出现 Internal 就是门插在读库之后）：{bad:?}"
        );
        assert_eq!(cases.len(), 9, "本批 9 把必须逐把点名，少一把即漏门");
    }

    /// 真库三段 + 两格归属边界，外加收口后的零残留计数。
    #[tokio::test]
    async fn b21_get_writes_gate_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!("skipping b21_get_writes_gate_lands: DATABASE_URL not reachable");
            return;
        }
        seed().await;

        let other = call_all(&shared::testing::test_pool(), Some(OTHER_PERSON)).await;
        let leaked: Vec<String> = other
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "他人会话改他人考勤明细 / 申诉 / 考勤组 / 全局开关必须 403：{leaked:?}"
        );
        assert_eq!(other.len(), 9, "本批 9 把必须逐把点名，少一把即漏门");
        let written = untouched(false).await;
        assert!(
            written.is_empty(),
            "他人 403 侧不得落笔（标志位应仍 false、开关仍 true、哨兵仍 {SENTINEL}、同步表仍零行）：{written:?}"
        );

        let owner = call_all(&shared::testing::test_pool(), Some(OWNER_PERSON)).await;
        let wrong: Vec<String> = owner
            .iter()
            .filter(|(k, r)| {
                if *k == "group_refresh_participate" {
                    r.is_err()
                } else {
                    !matches!(r, Err(AppError::Forbidden))
                }
            })
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            wrong.is_empty(),
            "本人只允许放行归属那把 group_refresh_participate，其余 8 把 admin 档必须仍 403：{wrong:?}"
        );
        // 归属那把本人放行，但也只 touch 自己那行的 update_time——哨兵必须已经换掉。
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        let touched = cell(&c, "x_attendance_v2_group", "update_time", GROUP_ID).await;
        assert_ne!(
            touched, SENTINEL,
            "本人放行的归属档那把必须真的落笔（仍等于哨兵说明门把放行也吞了）"
        );

        let edge_other = call_edge(&shared::testing::test_pool(), Some(OTHER_PERSON)).await;
        let bad: Vec<String> = edge_other
            .iter()
            .filter(|(k, r)| match *k {
                // 归属为空：数据缺陷不许洗成权限放行，旁人只能 403
                "group_empty_owner" => !matches!(r, Err(AppError::Forbidden)),
                // 行不存在：按 crate 既有口径（delete_owned_record 同形）回 Ok(error) 且不落笔
                _ => r.is_err(),
            })
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            bad.is_empty(),
            "归属为空那行只允许 admin（他人必须 403），行不存在回 Ok(error)：{bad:?}"
        );
        let written = untouched(true).await;
        assert!(
            written.is_empty(),
            "本人 403 与归属边界两格都不得动其它行（GROUP_ID 只能被 owner 段那一把刷动）：{written:?}"
        );

        let admin = call_all(&shared::testing::test_pool(), Some(ADMIN_PERSON)).await;
        // 42883 已修：detail_check_by_cycle 路径绑 i32，admin 放行后同样回 Ok 并真落笔。
        let broke: Vec<String> = admin
            .iter()
            .filter(|(_, r)| r.is_err())
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            broke.is_empty(),
            "admin 段 9 把必须全部放行（含 detail_check_by_cycle，其 42883 已由 i32 路径绑定修复）：{broke:?}"
        );
        assert_eq!(admin.len(), 9, "admin 段同样逐把点名");

        // 正向对照：门不是一律拒，admin 放行时逐格必须真落笔。
        let mut missing = Vec::new();
        for (tbl, col, key, exp) in [
            ("x_attendance_appeal_info", "archived", APPEAL_ID, "true"),
            ("x_attendance_detail", "analysed", DETAIL_ID, "true"),
            ("x_attendance_detail", "archived", DETAIL_ID, "true"),
            // 造数时 enabled=true，admin 那格请求的是关掉它
            ("x_attendance_setting", "enabled", SETTING_CODE, "false"),
            // detail_check_by_cycle 的 42883 已修：admin 放行后 UPDATE 真把 checked 置真
            // （造数行 cycle_year=9999, cycle_month=99 恰在射程内）。
            ("x_attendance_detail", "checked", DETAIL_ID, "true"),
        ] {
            let got = cell(&c, tbl, col, key).await;
            if got != exp {
                missing.push(format!("{tbl}.{col}/{key} => {got:?} 期望 {exp:?}"));
            }
        }
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) FROM x_attendance_sync_record WHERE start_date = $1",
                &[&SYNC_FROM.to_string()],
            )
            .await
            .unwrap()
            .get(0);
        if n != 2 {
            missing.push(format!("两把 sync start 应各插 1 行同步记录，实得 {n}"));
        }
        assert!(
            missing.is_empty(),
            "admin 放行侧必须逐格真落笔（否则门等于一律拒，测的是空气）：{missing:?}"
        );

        // 归属为空的行由 admin 刷得动（require_owner 的 admin 放行支路），行不存在同样只回 Ok(error)。
        let edge_admin = call_edge(&shared::testing::test_pool(), Some(ADMIN_PERSON)).await;
        let bad: Vec<String> = edge_admin
            .iter()
            .filter(|(_, r)| r.is_err())
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(bad.is_empty(), "admin 对归属为空与行不存在两格都应放行：{bad:?}");
        let empty_touch = cell(&c, "x_attendance_v2_group", "update_time", GROUP_EMPTY_ID).await;
        assert_ne!(
            empty_touch, SENTINEL,
            "空归属那行必须被 admin 刷掉哨兵，实得 {empty_touch:?}"
        );

        unseed().await;
        let residue = residue_count().await;
        assert_eq!(residue, 0, "收口后 b21-* 造数必须零残留，实得 {residue} 行");
    }

    async fn residue_count() -> i64 {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        let mut total = 0i64;
        for (tbl, key) in [
            ("x_attendance_appeal_info", APPEAL_ID),
            ("x_attendance_detail", DETAIL_ID),
            ("x_attendance_setting", SETTING_CODE),
        ] {
            let sql = format!("SELECT COUNT(*) FROM {tbl} WHERE id = $1");
            total += c
                .query_one(sql.as_str(), &[&key.to_string()])
                .await
                .unwrap()
                .get::<_, i64>(0);
        }
        for sql in [
            "SELECT COUNT(*) FROM x_attendance_v2_group WHERE id LIKE 'b21-%'",
            "SELECT COUNT(*) FROM auth_role WHERE id = 'b21-role-admin'",
            "SELECT COUNT(*) FROM auth_person WHERE id = 'b21-admin-person'",
            "SELECT COUNT(*) FROM auth_person_role WHERE person_id = 'b21-admin-person'",
        ] {
            total += c
                .query_one(sql, &[])
                .await
                .unwrap()
                .get::<_, i64>(0);
        }
        total += c
            .query_one(
                "SELECT COUNT(*) FROM x_attendance_sync_record WHERE start_date = $1",
                &[&SYNC_FROM.to_string()],
            )
            .await
            .unwrap()
            .get::<_, i64>(0);
        total
    }

    // 回归（四轮审计 Defect #1）：require_owner 以归一化口径比较本人身份。
    // 历史缺陷：session.person_unique 与 owner_id 原始字符串比较，当身份标识存在大小写 /
    // 空白差异时，本人操作会被误判为越权；归一化后同一身份的变体视为本人，且不引入越权
    // （归一化后确为他者仍一律 Forbidden）。
    #[tokio::test]
    async fn r1_require_owner_case_insensitive_self_match() {
        if !shared::testing::is_db_available().await {
            eprintln!(
                "skipping r1_require_owner_case_insensitive_self_match: DATABASE_URL not reachable"
            );
            return;
        }
        // 用一个明确不在库、非 admin 的身份，隔离 is_admin 支路，单测归一化比较本身。
        let pool = shared::testing::test_pool();
        let s = shared::testing::test_session("R1-Case-Owner-9999");
        // 同一身份的小写变体 -> 视为本人放行
        assert!(
            shared::middleware::require_owner(&pool, &s, "r1-case-owner-9999")
                .await
                .is_ok(),
            "同一身份的小写变体应视为本人放行"
        );
        // 前后空白 + 大小写 -> trim + lower 后仍视为本人放行
        assert!(
            shared::middleware::require_owner(&pool, &s, "  r1-CASE-owner-9999  ")
                .await
                .is_ok(),
            "trim 后的小写变体同样应视为本人放行"
        );
        // 归一化后确为他者 -> 一律 Forbidden，不因归一化放松越权判定
        assert!(
            matches!(
                shared::middleware::require_owner(&pool, &s, "r1-case-intruder-0001").await,
                Err(AppError::Forbidden)
            ),
            "他者身份必须 403"
        );
    }

    // 回归（四轮审计 Defect #3）：三张考勤表的主键已由 migration 109 补齐。
    // 早前 migration 032 未建主键，`ON CONFLICT (id)` 会撞 42P10；此处直接在真库校验
    // 主键约束存在，防止未来迁移回退。
    #[tokio::test]
    async fn r3_attendance_tables_have_primary_key() {
        if !shared::testing::is_db_available().await {
            eprintln!("skipping r3_attendance_tables_have_primary_key: DATABASE_URL not reachable");
            return;
        }
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        for t in [
            "x_attendance_appeal_info",
            "x_attendance_detail",
            "x_attendance_setting",
        ] {
            let n: i64 = c
                .query_one(
                    "SELECT COUNT(*) FROM pg_constraint c \
                     JOIN pg_class t ON t.oid = c.conrelid \
                     WHERE t.relname = $1 AND c.contype = 'p'",
                    &[&t.to_string()],
                )
                .await
                .unwrap()
                .get(0);
            assert_eq!(n, 1, "表 {t} 应恰好有一个主键约束（migration 109 补齐）");
        }
    }

    // 第五十批（R7 收口）：考勤控制规则 3 把（admin）+ 申诉/明细接收 4 把（会话存在）
    // 无会话必须 401（门在 pool.get() 之前，dead pool 即证）。
    #[tokio::test]
    async fn b50_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let ext = || Extension(pool.clone());
        let p = || Path("x".to_string());
        let j = || Json(json!({}));
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "create_control_rule",
                crate::create_control_rule(ext(), None, j()).await.map(|_| ()),
            ),
            (
                "delete_control_rule",
                crate::delete_control_rule(ext(), None, p()).await.map(|_| ()),
            ),
            (
                "toggle_control_rule",
                crate::toggle_control_rule(ext(), None, p(), j()).await.map(|_| ()),
            ),
            (
                "attendanceappealInfo_appeal_id",
                crate::attendanceappealInfo_appeal_id(ext(), None, p(), j())
                    .await
                    .map(|_| ()),
            ),
            (
                "attendancedetail_mobile_recive",
                crate::attendancedetail_mobile_recive(ext(), None, j())
                    .await
                    .map(|_| ()),
            ),
            (
                "attendancedetail_recive",
                crate::attendancedetail_recive(ext(), None, j()).await.map(|_| ()),
            ),
            (
                "attendancedetail_reciveSingle",
                crate::attendancedetail_reciveSingle(ext(), None, j())
                    .await
                    .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 7, "本批 7 把必须逐把点名");
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
