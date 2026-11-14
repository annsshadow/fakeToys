// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! P1-1 的守卫：attendance 族的写侧**归属事实两列同写**
//!
//! # 这一格原来是什么形状
//!
//! 交接文档 §4 的 **P1-1**：「attendance 五张配置表写侧只写 `creator_person`」。
//! 2026-11-13 复核（按 `INSERT INTO x_attendance*` 的列名清单逐点重数，改前改后各量一遍）：
//!
//! * 写侧共 **25** 个 `INSERT INTO x_attendance*` 站点，其中改前 **19** 个只落
//!   `creator_person`（`creator` 列空着）；读侧谓词是
//!   `owner_scope(&["creator", "creator_person"], …)` ⇒ **两列都认**
//!   ⇒ 写侧不写 `creator` **不造成读写不一致**，但「归属事实只落一半列」仍在。
//! * 反过来也有 **3** 个站点只落 `creator`：全在 `x_attendance_config` 上 ——
//!   其中两个（`v2_config_post` / `v2_config_person_post`）落的是会话身份，
//!   本轮改成两列同写；另一个 `attendanceconfig_save` 的形参里**根本没有会话**
//!   （`creator` 取请求体，缺省 `system`）⇒ 进豁免表。
//! * 所以改前「只落半边」共 **20** 个（19 + 1），本轮改了 **13** 个 ⇒
//!   改后只剩 **6** 个，逐格对应下面 `SOURCE_EXEMPTIONS` 那张具名表。
//!
//! # 判决（2026-11-13）
//!
//! **写侧两列同写、落同一个会话身份**。理由三条：
//!
//! 1. **同族先例**：`x_message_mass` 早就这么写（`creator` 与 `creator_person`
//!    同时落会话身份，见 `message_assemble_communicate` 的 `mass_create`），
//!    而 P0-2 那一格修的就是「`creator` 从不写入」⇒ attendance 族是**同一族
//!    的漏网**，不是一种设计。
//! 2. **不造成行为变更**：读侧两列都认 ⇒ 写侧多写一列 = 多一份**事实**，
//!    现有判决一句不改。
//! 3. **半列事实的代价**：任何只读 `creator` 的新读者（`cal_calendar` 那一族
//!    读的就是 `createor`）都会看到 NULL，而 NULL 在归属谓词里与「不是属主」
//!    同形 ⇒ 静默失效而不是响亮失败。
//!
//! # 本文件的两把尺子
//!
//! 1. **源码面**（不需要库）：扫 `lib.rs` 里每一个 `INSERT INTO x_attendance*`
//!    的列名清单 ⇒ 「落了 `creator_person` 就必须也落 `creator`、落了 `creator`
//!    就必须也落 `creator_person`」，**例外逐个具名**（例外理由写在具名表旁边）。
//! 2. **真库面**：真打一次写端点，回读那一行的**两列** ⇒ 两列都等于会话身份。
//!    （没有这一支，源码面那条规则只是「文本形状对」，与「真的落库了」无关。）

#[cfg(test)]
mod creator_columns_tests {
    use axum::extract::{Extension, Json};
    use deadpool_postgres::Client;
    use serde_json::json;

    const ADMIN_PERSON: &str = "p11-attendance-admin";
    const ROLE_ID: &str = "p11-role-admin";
    const UNIT_ID: &str = "p11-unit";
    const EMPLOYEE_NAME: &str = "p11-employee";

    /// 源码面豁免表：**逐个具名 + 写明理由**。
    ///
    /// * `x_attendance_v2_group` / `x_attendance_v2_shift` / `x_attendance_v2_leave`
    ///   —— 迁移 061 给这三张表建的列**只有 `creator_person`**（没有 `creator`）
    ///   ⇒ 要两列同写得先加列（迁移）⇒ 那一格单独一轮，不在本轮射程。
    /// * `x_attendance_v2_group_schedule` / `x_attendance_v2_checkin_record`
    ///   —— 同族（表本身没有 `creator` 列）。
    /// * `x_attendance_config`（`attendanceconfig_save` 那一格）
    ///   —— 该 handler 的形参里**没有会话**（`pool` + `Json(payload>`），
    ///   `creator` 取的是请求体里的 `creator`（缺省 `system`）⇒ 没有「会话身份」
    ///   可落 ⇒ 那一格要先把会话加进形参（对外契约变更），单独一轮。
    const SOURCE_EXEMPTIONS: &[(&str, &str)] = &[
        (
            "x_attendance_v2_group",
            "迁移 061 建表时只有 creator_person，没有 creator 列",
        ),
        (
            "x_attendance_v2_shift",
            "迁移 061 建表时只有 creator_person，没有 creator 列",
        ),
        (
            "x_attendance_v2_leave",
            "迁移 061 建表时只有 creator_person，没有 creator 列",
        ),
        ("x_attendance_v2_group_schedule", "表本身没有 creator 列"),
        ("x_attendance_v2_checkin_record", "表本身没有 creator 列"),
        (
            "x_attendance_config",
            "attendanceconfig_save 的形参里没有会话（creator 取请求体）",
        ),
    ];

    /// 从 `lib.rs` 里抽出每一个 `INSERT INTO x_attendance*` 的 (表名, 列名清单)。
    fn insert_sites(source: &str) -> Vec<(String, Vec<String>)> {
        let mut out = Vec::new();
        let mut i = 0usize;
        while let Some(found) = source[i..].find("INSERT INTO ") {
            let start = i + found;
            let rest = &source[start + "INSERT INTO ".len()..];
            let table_end = rest.find([' ', '(']).unwrap_or(rest.len());
            let table = rest[..table_end].to_string();
            if !table.starts_with("x_attendance") {
                i = start + "INSERT INTO ".len();
                continue;
            }
            let open = match rest.find('(') {
                Some(pos) => pos,
                None => break,
            };
            let close = match rest[open..].find(')') {
                Some(pos) => open + pos,
                None => break,
            };
            let cols: Vec<String> = rest[open + 1..close]
                .split(',')
                .map(|c| c.trim().replace('\\', "").trim().to_string())
                .filter(|c| !c.is_empty())
                .collect();
            out.push((table, cols));
            i = start + "INSERT INTO ".len();
        }
        out
    }

    fn is_exempt(table: &str) -> bool {
        SOURCE_EXEMPTIONS.iter().any(|(t, _)| *t == table)
    }

    #[test]
    fn every_write_site_lands_both_owner_columns() {
        let source = include_str!("lib.rs");
        let sites = insert_sites(source);
        assert!(
            !sites.is_empty(),
            "没扫到任何 INSERT ⇒ 判据失效（源码形状变了？）"
        );
        let mut bad: Vec<String> = Vec::new();
        for (table, cols) in &sites {
            if is_exempt(table) {
                continue;
            }
            let has_creator = cols.iter().any(|c| c == "creator");
            let has_creator_person = cols.iter().any(|c| c == "creator_person");
            if has_creator != has_creator_person {
                bad.push(format!(
                    "{table}: creator={has_creator} creator_person={has_creator_person}"
                ));
            }
        }
        assert!(
            bad.is_empty(),
            "以下写点只落了归属事实的一半（P1-1）：{bad:?}"
        );
    }

    #[test]
    fn the_exemption_table_is_exactly_the_named_five() {
        // 豁免表不许悄悄变长：多一个豁免 = 多一个「归属事实只落一半」的洞
        let source = include_str!("lib.rs");
        let sites = insert_sites(source);
        let exempted: Vec<String> = sites
            .iter()
            .map(|(t, _)| t.clone())
            .filter(|t| is_exempt(t))
            .collect();
        // 豁免表里的每个表名都必须在源码里真的出现（防「豁免写了个不存在的表」）
        for (table, reason) in SOURCE_EXEMPTIONS {
            assert!(
                !reason.is_empty(),
                "{table} 的豁免理由是空的 ⇒ 豁免没有依据"
            );
            assert!(
                source.contains(table),
                "{table} 在源码里已经不存在了 ⇒ 这条豁免该删"
            );
        }
        assert!(
            exempted
                .iter()
                .all(|t| SOURCE_EXEMPTIONS.iter().any(|(x, _)| x == t)),
            "源码里出现了不在豁免表里的半列写点：{exempted:?}"
        );
    }

    /// 真库臂：真打一次写端点，回读两列。
    #[tokio::test]
    async fn the_real_db_row_carries_both_owner_columns() {
        if !shared::testing::is_db_available().await {
            eprintln!("skipping the_real_db_row_carries_both_owner_columns: DATABASE_URL 不可达");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();
        // **顺序**：先清上一轮残留（`unseed`），**再**种管理员 ⇒ 上一轮崩在半路时
        // 留下的行不会顶掉这一轮的种子。
        // （反过来「先种后清」会把刚种进去的种子一起清掉 ⇒ 端点回 403 Forbidden ——
        // 那是夹具顺序的错，不是产品的错。）
        unseed(&client).await;
        seed_admin(&client).await;

        let payload = json!({
            "topUnitName": "p11-unit",
            "unitName": "p11-unit",
            "employeeName": EMPLOYEE_NAME,
            "employeeNumber": "p11-no",
            "configType": "shift",
        });
        let result = crate::attendanceemployeeconfig_create(
            Extension(pool.clone()),
            Extension(shared::testing::test_session(ADMIN_PERSON)),
            Json(payload),
        )
        .await
        .expect("写端点应当成功");
        let id = result.0.data.unwrap()["id"].as_str().unwrap().to_string();

        let row = client
            .query_opt(
                "SELECT creator, creator_person FROM x_attendance_employee_config WHERE id = $1",
                &[&id],
            )
            .await
            .unwrap()
            .expect("刚写的行不在库里");
        let creator: Option<String> = row.get(0);
        let creator_person: Option<String> = row.get(1);
        assert_eq!(
            creator.as_deref(),
            Some(ADMIN_PERSON),
            "creator 列没落会话身份（P1-1）"
        );
        assert_eq!(
            creator_person.as_deref(),
            Some(ADMIN_PERSON),
            "creator_person 列没落会话身份"
        );

        unseed(&client).await;
    }

    async fn seed_admin(c: &Client) {
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ($1, 'admin', false) \
             ON CONFLICT (id) DO UPDATE SET name = 'admin', disable = false",
            &[&ROLE_ID.to_string()],
        )
        .await
        .unwrap();
        shared::testing::upsert_fixture_person(
            c,
            ADMIN_PERSON,
            ADMIN_PERSON,
            "p11 attendance admin",
        )
        .await;
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            &[
                &ADMIN_PERSON.to_string(),
                &ROLE_ID.to_string(),
                &UNIT_ID.to_string(),
            ],
        )
        .await
        .unwrap();
    }

    async fn unseed(c: &Client) {
        c.execute(
            "DELETE FROM x_attendance_employee_config WHERE employee_name = $1",
            &[&EMPLOYEE_NAME.to_string()],
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
        c.execute(
            "DELETE FROM auth_role WHERE id = $1",
            &[&ROLE_ID.to_string()],
        )
        .await
        .unwrap();
    }
}
