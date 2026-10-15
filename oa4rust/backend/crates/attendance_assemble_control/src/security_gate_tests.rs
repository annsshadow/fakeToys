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
//! 明列豁免 1 处：`checkDetailWithPersonByCycle` 是**早于本批存在**的必然 500——
//! `cycle_year` / `cycle_month` 在真库是 integer，handler 却把路径字符串直接绑进
//! `WHERE cycle_year = $1`（42883 operator does not exist: integer = text，已用
//! `PREPARE tc(text,text)` 探针验真）。UPDATE 那步被 `.unwrap_or(0)` 吞掉，回读 COUNT 那步
//! `map_err(Internal)?` 当场炸。本批只加门不修语义，所以 admin 格对该把只允许 `Internal`，
//! 并把它的 checked 落笔状态一并钉住；一旦有人修好类型口径，这条豁免会自己喊红提醒撤掉。

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
    const CYCLE_YEAR: &str = "9999";
    const CYCLE_MONTH: &str = "99";
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
                    p2(CYCLE_YEAR, CYCLE_MONTH),
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

    /// 造数前先 `unseed()` 按 id 删净，再普通 INSERT——不用 `ON CONFLICT`：
    /// `x_attendance_appeal_info` / `x_attendance_detail` / `x_attendance_setting` 三张表
    /// 在 migration 032 里**根本没建主键**（`"id" TEXT` 一列而已），`ON CONFLICT (id)` 当场 42P10。
    /// 本臂连跑两遍要读数一致，靠的是「先删后插」这条可重复序列，不是上一遍留下的数据。
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
        let broke: Vec<String> = admin
            .iter()
            .filter(|(k, r)| {
                if *k == "detail_check_by_cycle" {
                    // 明列豁免：integer 列绑字符串参数必撞 42883，早于本批存在，本批只加门不修
                    return !matches!(r, Err(AppError::Internal));
                }
                r.is_err()
            })
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            broke.is_empty(),
            "admin 应在 8 把上放行、并在 detail_check_by_cycle 上恰好撞既有 500；\
             该把一旦变成 Ok 就是类型口径被修好，本豁免要跟着撤：{broke:?}"
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
            // 豁免那把不落笔（42883 在 UPDATE 之前就把语句打死）
            ("x_attendance_detail", "checked", DETAIL_ID, "false"),
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
}
