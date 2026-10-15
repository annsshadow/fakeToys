// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第二十二批）：message 的 GET 写面 5 条里的 3 把。
//!
//! 修前这 3 把的函数体第一条语句就是写 SQL，一把都不读会话，而路由把 GET 与 DELETE/POST
//! **同名双注册**（csrf / requires_admin 两道中间件只认写方法，GET 那条整族绕过）：
//!
//! - 归属档 2 把：`consume/{id}/type/{type}`（消费记录是本人对某条消息的消费形态，
//!   按 id 改他人那行＝伪造他人已读态）、`im/msg/revoke/{id}`（撤回是发送者本人的动作，
//!   按 id 撤他人消息＝抹掉他人发言）。两把的归属事实都取 `sender`——这两张表不写 creator_person。
//! - 管理员档 1 把：`im/conversation/{id}/group/mockdeletetoget`（DELETE 删的是**整群全部成员行**，
//!   解散级破坏动作）。刻意不收归属：本表族没有「群主」事实可判——`x_message_conversation`
//!   从不落 creator_person、成员表的 role 没有任何写路径，收归属会静默退化成「一律管理员」，
//!   那不如直接把档位写成它实际的样子。
//!
//! 另 2 条（`quit/self`、`single/mockdeletetoget`）**不加门**：两把体内都已用
//! `is_conversation_member(..)` 做过成员判定、且只写自己那行，属于「有实质判定、
//! 形态是业务错」而非「无门禁」。把它们也套一层门是刷读数不是修漏洞，故留名单并记账。
//!
//! 两臂：无会话臂用**连不上库**的 pool 跑，5 次调用必须全 401——出现 `Internal` 就说明门插在了
//! `pool.get()` 之后；真库臂跑「他人 403 且不落笔 → 本人只放行归属那两把（解散群那把本人也拒）→
//! 管理员全档落笔」，落笔与不落笔都逐格回读。

#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Client, Manager, Pool};
    use shared::error::AppError;

    const CONSUME_OWN: &str = "b22-consume-own";
    const CONSUME_EMPTY: &str = "b22-consume-empty";
    const MSG_OWN: &str = "b22-msg-own";
    const MSG_EMPTY: &str = "b22-msg-empty";
    const CONV_GROUP: &str = "b22-conv-group";
    const MEM_OWN: &str = "b22-mem-own";
    const MEM_OTHER: &str = "b22-mem-other";
    const MISSING: &str = "b22-does-not-exist";
    const SENTINEL: &str = "b22-before";
    /// 归属两把放行后要写进去的值——钉成字面量，好让「落笔」是可断言的确切值而不是「变了」。
    const AFTER: &str = "b22-after";

    const ADMIN_PERSON: &str = "b22-admin-person";
    const OWNER_PERSON: &str = "b22-owner-person";
    const OTHER_PERSON: &str = "b22-other-person";

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

    /// 5 次具名调用（3 把门 + 归属两把的空归属格）。表长由 `assert_eq!(cases.len(), 5)` 钉住。
    async fn call_all(
        pool: &Pool,
        person: Option<&str>,
        msg_type: &str,
    ) -> Vec<(&'static str, Result<(), AppError>)> {
        let sess = || person.map(|w| Extension(shared::testing::test_session(w)));
        let ext = || Extension(pool.clone());
        vec![
            (
                "consume_own",
                crate::consume_id_type_type(ext(), sess(), p2(CONSUME_OWN, msg_type))
                    .await
                    .map(|_| ()),
            ),
            (
                "consume_empty_owner",
                crate::consume_id_type_type(ext(), sess(), p2(CONSUME_EMPTY, msg_type))
                    .await
                    .map(|_| ()),
            ),
            (
                "msg_revoke_own",
                crate::im_msg_revoke_id(ext(), sess(), p(MSG_OWN)).await.map(|_| ()),
            ),
            (
                "msg_revoke_empty_owner",
                crate::im_msg_revoke_id(ext(), sess(), p(MSG_EMPTY)).await.map(|_| ()),
            ),
            (
                "group_members_delete",
                crate::im_conversation_id_group_mockdeletetoget(ext(), sess(), p(CONV_GROUP))
                    .await
                    .map(|_| ()),
            ),
        ]
    }

    /// 缺行格：归属那两把对不存在的 id 必须回**业务错**而不是 403——不能把「没有这行」读成权限结论。
    async fn call_missing(pool: &Pool, person: &str) -> Vec<(&'static str, Result<(), AppError>)> {
        let sess = || Some(Extension(shared::testing::test_session(person)));
        let ext = || Extension(pool.clone());
        vec![
            (
                "consume_missing",
                crate::consume_id_type_type(ext(), sess(), p2(MISSING, AFTER))
                    .await
                    .map(|_| ()),
            ),
            (
                "msg_missing",
                crate::im_msg_revoke_id(ext(), sess(), p(MISSING)).await.map(|_| ()),
            ),
        ]
    }

    async fn cell(c: &Client, tbl: &str, col: &str, key: &str) -> String {
        let sql = format!("SELECT {col}::text FROM {tbl} WHERE id = $1");
        c.query_opt(sql.as_str(), &[&key.to_string()])
            .await
            .unwrap()
            .map(|r| r.get::<_, Option<String>>(0).unwrap_or_default())
            .unwrap_or_default()
    }

    async fn member_count(c: &Client) -> i64 {
        c.query_one(
            "SELECT COUNT(*) FROM x_message_conversation_member WHERE conversation_id = $1",
            &[&CONV_GROUP.to_string()],
        )
        .await
        .unwrap()
        .get::<_, i64>(0)
    }

    /// 逐格回读「有没有落笔」，并核成员行数等于 `members`。
    async fn untouched(members: i64) -> Vec<String> {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        let mut broke = Vec::new();
        for (tbl, key, col) in [
            ("x_message_consume", CONSUME_OWN, "type"),
            ("x_message_consume", CONSUME_EMPTY, "type"),
            ("x_message", MSG_OWN, "revoke_time"),
            ("x_message", MSG_EMPTY, "revoke_time"),
        ] {
            let got = cell(&c, tbl, col, key).await;
            if got != SENTINEL {
                broke.push(format!("{tbl}/{key}/{col} => {got:?} 期望仍是哨兵 {SENTINEL:?}"));
            }
        }
        let n = member_count(&c).await;
        if n != members {
            broke.push(format!(
                "x_message_conversation_member/{CONV_GROUP} 行数 => {n} 期望 {members}"
            ));
        }
        broke
    }

    /// 反向读数：管理员放行后，四格哨兵都必须被刷掉、整群成员行都必须清零。
    /// 它存在的意义是证明门不是「一律拒」——只测拒，门写成空气也是绿的。
    async fn moved() -> Vec<String> {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        let mut broke = Vec::new();
        for (tbl, key, col) in [
            ("x_message_consume", CONSUME_OWN, "type"),
            ("x_message_consume", CONSUME_EMPTY, "type"),
            ("x_message", MSG_OWN, "revoke_time"),
            ("x_message", MSG_EMPTY, "revoke_time"),
        ] {
            let got = cell(&c, tbl, col, key).await;
            if got == SENTINEL {
                broke.push(format!("{tbl}/{key}/{col} 仍是哨兵 {SENTINEL:?}，管理员那步没落笔"));
            }
        }
        let n = member_count(&c).await;
        if n != 0 {
            broke.push(format!("管理员清空整群成员后仍剩 {n} 行，期望 0"));
        }
        broke
    }

    async fn seed_admin_role(c: &Client) {
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ('b22-role-admin', 'admin', false) \
             ON CONFLICT (id) DO UPDATE SET name = 'admin', disable = false",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'b22 message admin', 'x') ON CONFLICT (id) DO NOTHING",
            &[&ADMIN_PERSON.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'b22-role-admin', 'b22-unit') ON CONFLICT DO NOTHING",
            &[&ADMIN_PERSON.to_string()],
        )
        .await
        .unwrap();
    }

    /// 先删后插，不用 `ON CONFLICT`：本 crate 这几张表按 migration 建的是裸 `id TEXT`，
    /// 主键缺失时 `ON CONFLICT (id)` 当场 42P10（与第二十一批同源）。
    async fn seed() {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        unseed_with(&c).await;

        c.execute(
            "INSERT INTO x_message_consume (id, consume, content, sender, consumed, create_time, type) \
             VALUES ($1, 'ws', 'b22 body', $2, false, NOW(), $3)",
            &[&CONSUME_OWN.to_string(), &OWNER_PERSON.to_string(), &SENTINEL.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO x_message_consume (id, consume, content, sender, consumed, create_time, type) \
             VALUES ($1, 'ws', 'b22 body', '', false, NOW(), $2)",
            &[&CONSUME_EMPTY.to_string(), &SENTINEL.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO x_message (id, conversation_id, content, sender, type, create_time, revoked, revoke_time) \
             VALUES ($1, $2, 'b22 body', $3, 'text', NOW(), false, $4)",
            &[&MSG_OWN.to_string(), &CONV_GROUP.to_string(), &OWNER_PERSON.to_string(), &SENTINEL.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO x_message (id, conversation_id, content, sender, type, create_time, revoked, revoke_time) \
             VALUES ($1, $2, 'b22 body', '', 'text', NOW(), false, $3)",
            &[&MSG_EMPTY.to_string(), &CONV_GROUP.to_string(), &SENTINEL.to_string()],
        )
        .await
        .unwrap();
        for (id, person) in [(MEM_OWN, OWNER_PERSON), (MEM_OTHER, OTHER_PERSON)] {
            c.execute(
                "INSERT INTO x_message_conversation_member (id, conversation_id, person_id, create_time) \
                 VALUES ($1, $2, $3, NOW())",
                &[&id.to_string(), &CONV_GROUP.to_string(), &person.to_string()],
            )
            .await
            .unwrap();
        }
        seed_admin_role(&c).await;
    }

    async fn unseed_with(c: &Client) {
        for sql in [
            "DELETE FROM x_message_consume WHERE id LIKE 'b22-%'",
            "DELETE FROM x_message WHERE id LIKE 'b22-%'",
            "DELETE FROM x_message_conversation_member WHERE id LIKE 'b22-%'",
            "DELETE FROM x_message_conversation WHERE id LIKE 'b22-%'",
            "DELETE FROM auth_person_role WHERE role_id = 'b22-role-admin'",
            "DELETE FROM auth_role WHERE id = 'b22-role-admin'",
            "DELETE FROM auth_person WHERE id LIKE 'b22-%'",
        ] {
            c.execute(sql, &[]).await.unwrap();
        }
    }

    async fn unseed() {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        unseed_with(&c).await;
    }

    #[tokio::test]
    async fn b22_get_writes_reject_without_session() {
        let pool = dead_pool();
        let cases = call_all(&pool, None, AFTER).await;
        assert_eq!(cases.len(), 5, "本批 5 格必须逐格点名，少一格即漏门");
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "以下 GET 写面无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }

    #[tokio::test]
    async fn b22_get_writes_gate_lands() {
        if !shared::testing::is_db_available().await {
            eprintln!("skipping b22_get_writes_gate_lands: DATABASE_URL not reachable");
            return;
        }
        seed().await;

        // 段一：他人会话五格全拒，且一格都不落笔。
        let other = call_all(&shared::testing::test_pool(), Some(OTHER_PERSON), AFTER).await;
        let leaked: Vec<String> = other
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "他人会话改他人消费记录 / 撤他人消息 / 清空整群成员必须 403：{leaked:?}"
        );
        let broke = untouched(2).await;
        assert!(broke.is_empty(), "他人被拒之后不得有任何落笔：{broke:?}");

        // 段二：本人只放行归属那两把（非空 sender），空归属两格与解散那把仍拒。
        let owner = call_all(&shared::testing::test_pool(), Some(OWNER_PERSON), AFTER).await;
        let owner_read: Vec<String> = owner
            .iter()
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert_eq!(
            owner_read,
            vec![
                "consume_own => None".to_string(),
                "consume_empty_owner => Some(Forbidden)".to_string(),
                "msg_revoke_own => None".to_string(),
                "msg_revoke_empty_owner => Some(Forbidden)".to_string(),
                "group_members_delete => Some(Forbidden)".to_string(),
            ],
            "本人只应放行 sender 属于他的那两把；空归属行与解散级动作仍归管理员"
        );
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        assert_eq!(
            cell(&c, "x_message_consume", "type", CONSUME_OWN).await,
            AFTER,
            "本人改自己的消费记录必须真落笔"
        );
        assert_ne!(
            cell(&c, "x_message", "revoke_time", MSG_OWN).await,
            SENTINEL,
            "本人撤自己的消息必须真落笔"
        );
        // 本人段被拒的三格（空归属两格 + 解散那把）必须一格都没落笔，成员行仍须是 2 行。
        let mut still = Vec::new();
        for (tbl, key, col) in [
            ("x_message_consume", CONSUME_EMPTY, "type"),
            ("x_message", MSG_EMPTY, "revoke_time"),
        ] {
            let got = cell(&c, tbl, col, key).await;
            if got != SENTINEL {
                still.push(format!("{tbl}/{key}/{col} => {got:?} 期望仍是哨兵"));
            }
        }
        let n = member_count(&c).await;
        if n != 2 {
            still.push(format!("本人不是群主档的放行对象，成员行数 => {n} 期望 2"));
        }
        assert!(
            still.is_empty(),
            "本人段只应放行 sender 属于他的两把，其余三把不得落笔：{still:?}"
        );

        // 段三：管理员五格全放行，且逐格真落笔。
        let admin = call_all(&shared::testing::test_pool(), Some(ADMIN_PERSON), AFTER).await;
        let broke: Vec<String> = admin
            .iter()
            .filter(|(_, r)| r.is_err())
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            broke.is_empty(),
            "管理员应在全部 5 格放行（含空归属两格与解散那把）：{broke:?}"
        );
        let moved_rows = moved().await;
        assert!(
            moved_rows.is_empty(),
            "管理员放行后哨兵必须被刷掉、整群成员行必须被清空：{moved_rows:?}"
        );

        // 段四：缺行格回业务错，不回 403。
        let missing = call_missing(&shared::testing::test_pool(), OWNER_PERSON).await;
        let wrong: Vec<String> = missing
            .iter()
            .filter(|(_, r)| r.is_err())
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            wrong.is_empty(),
            "不存在的 id 应回业务错（Ok + error 体），不能读成权限结论：{wrong:?}"
        );

        unseed().await;
        let residue = residue_count().await;
        assert_eq!(residue, 0, "收口后 b22-* 造数必须零残留，实得 {residue} 行");
    }

    async fn residue_count() -> i64 {
        let pool = shared::testing::test_pool();
        let c = pool.get().await.unwrap();
        let mut total = 0i64;
        for sql in [
            "SELECT COUNT(*) FROM x_message_consume WHERE id LIKE 'b22-%'",
            "SELECT COUNT(*) FROM x_message WHERE id LIKE 'b22-%'",
            "SELECT COUNT(*) FROM x_message_conversation_member WHERE id LIKE 'b22-%'",
            "SELECT COUNT(*) FROM x_message_conversation WHERE id LIKE 'b22-%'",
            "SELECT COUNT(*) FROM auth_role WHERE id = 'b22-role-admin'",
            "SELECT COUNT(*) FROM auth_person WHERE id LIKE 'b22-%'",
            "SELECT COUNT(*) FROM auth_person_role WHERE role_id = 'b22-role-admin'",
        ] {
            total += c
                .query_one(sql, &[])
                .await
                .unwrap()
                .get::<_, i64>(0);
        }
        total
    }
}
