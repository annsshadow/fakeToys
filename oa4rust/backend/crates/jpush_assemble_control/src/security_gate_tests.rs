// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第五十五批）：jpush_assemble_control 剩余写面 5 把。
//!
//! 这批是推送模块的写面：`save_jpush`/`delete_jpush`（改/软删 x_jpush 推送配置，
//! 按 `x_jpush.creator` 归属）、`device_bind`/`device_unbind_deviceName_deviceType`
//! （绑定/解绑设备，收会话存在档）、`message_send`（群发推送落 x_jpush，收 admin 档，
//! 与既有 `device_admin_unbind_all_person` 同口径）。原全部不取会话、无门禁。
//!
//! 余 1 条 `message_test_send`（只 SELECT x_jpush + 走内存 MockPushGateway）为只读
//! false-positive，按口径不动。
//!
//! 归属臂需真实 PostgreSQL：`cargo test --lib -p jpush_assemble_control -- --ignored`
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, Path};
    use axum::Json;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn non_admin() -> shared::session::Session {
        shared::testing::test_session("b55-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，5 把必须全 401（门在 pool.get() 之前）。
    #[tokio::test]
    async fn b55_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "save_jpush",
                crate::save_jpush(
                    Extension(pool.clone()),
                    None,
                    Path("j1".to_string()),
                    Json(serde_json::from_value(json!({ "title": "t" })).unwrap()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "delete_jpush",
                crate::delete_jpush(Extension(pool.clone()), None, Path("j1".to_string()))
                    .await
                    .map(|_| ()),
            ),
            (
                "device_bind",
                crate::device_bind(Extension(pool.clone()), None, Json(json!({})))
                    .await
                    .map(|_| ()),
            ),
            (
                "device_unbind_deviceName_deviceType",
                crate::device_unbind_deviceName_deviceType(
                    Extension(pool.clone()),
                    None,
                    Path(("d".to_string(), "android".to_string())),
                )
                .await
                .map(|_| ()),
            ),
            (
                "message_send",
                crate::message_send(Extension(pool.clone()), None, Json(json!({})))
                    .await
                    .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 5, "本批 5 把必须逐把点名");
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

    /// admin 臂：群发推送非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b55_message_send_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let res = crate::message_send(
            Extension(pool),
            Some(Extension(non_admin())),
            Json(json!({ "title": "t", "content": "c", "target": "all" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 群发推送必须 403，实得 {res:?}"
        );
    }

    /// 归属臂（需真实 PostgreSQL）：x_jpush 归属他人且本会话非 admin → 改/删 403。
    #[tokio::test]
    #[ignore = "需真实 PostgreSQL（DATABASE_URL）"]
    async fn b55_owner_writes_non_owner_forbidden() {
        let pool = shared::testing::test_pool();
        {
            let c = pool.get().await.unwrap();
            // x_jpush.id 无唯一约束（schema 缺陷，见账本 §五），不能用 ON CONFLICT；先清后插。
            c.execute("DELETE FROM x_jpush WHERE id = 'b55-jpush-owned-by-other'", &[])
                .await
                .unwrap();
            c.execute(
                "INSERT INTO x_jpush (id, title, content, target, creator, create_time) \
                 VALUES ('b55-jpush-owned-by-other','t','c','all','b55-owner-elsewhere',NOW())",
                &[],
            )
            .await
            .unwrap();
        }
        let sess = non_admin();
        let jid = || Path("b55-jpush-owned-by-other".to_string());
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "save_jpush",
                crate::save_jpush(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    jid(),
                    Json(serde_json::from_value(json!({ "title": "t2" })).unwrap()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "delete_jpush",
                crate::delete_jpush(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    jid(),
                )
                .await
                .map(|_| ()),
            ),
        ];
        for (name, res) in cases {
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "{name} 非属主且非 admin 必须 403，实得 {res:?}"
            );
        }
        pool.get()
            .await
            .unwrap()
            .execute("DELETE FROM x_jpush WHERE id='b55-jpush-owned-by-other'", &[])
            .await
            .unwrap();
    }
}
