// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第六十批）：mind_assemble_control 写面 5 把。
//!
//! - `update_control_config`（UPDATE `x_mind_assemble_control_config`）→ admin 档；
//! - `save_folder` / `update_folder` / `folder_move_folderId` / `folder_id_force`
//!   （INSERT/UPDATE/软删 `x_mind`）→ 会话存在档；
//! - 分片A第三批：`save_folder` 归属改落会话身份（原硬写 "system"），
//!   用例 `save_folder_stamps_session_creator` 守住「新行 creator = 会话」。
//!
//! 无会话一律 401（先于任何 SQL，离线可证）。
//!
//! DB 依赖臂：`cargo test --lib -p mind_assemble_control -- --ignored`
#[cfg(test)]
#[allow(clippy::module_inception)]
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
        shared::testing::test_session("b60-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，5 把必须全 401。
    #[tokio::test]
    async fn b60_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "update_control_config",
                crate::update_control_config(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "id": "global", "configData": "{}" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "save_folder",
                crate::save_folder(
                    Extension(pool.clone()),
                    None,
                    Json(serde_json::from_value(json!({ "name": "n" })).unwrap()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "update_folder",
                crate::update_folder(
                    Extension(pool.clone()),
                    None,
                    Path("f1".to_string()),
                    Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "folder_move_folderId",
                crate::folder_move_folderId(
                    Extension(pool.clone()),
                    None,
                    Path("f1".to_string()),
                    Json(json!({ "parentId": "p" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "folder_id_force",
                crate::folder_id_force(Extension(pool.clone()), None, Path("f1".to_string()))
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

    /// admin 臂：全局配置写非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b60_update_control_config_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let res = crate::update_control_config(
            Extension(pool),
            Some(Extension(non_admin())),
            Json(json!({ "id": "global", "configData": "{}" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 全局配置写必须 403，实得 {res:?}"
        );
    }

    /// 分片A第三批：`save_folder` 的新行 creator 必须是会话身份，不能再落 `system`。
    /// 破坏实验：把 `creator` 改回 `let creator = "system";`，本用例必红。
    #[tokio::test]
    async fn save_folder_stamps_session_creator() {
        let pool = shared::testing::test_pool();
        let me = "u-sec-read-a-writer";
        let name = "sec-read-a-write-owner";
        {
            let c = pool.get().await.unwrap();
            c.execute("DELETE FROM x_mind WHERE name = $1", &[&name])
                .await
                .unwrap();
        }
        let res = crate::save_folder(
            Extension(pool.clone()),
            Some(Extension(shared::testing::test_session(me))),
            Json(serde_json::from_value(json!({ "name": name, "content": "c" })).unwrap()),
        )
        .await;
        assert!(res.is_ok(), "会话本人建文件夹应成功，实得错误");
        let c = pool.get().await.unwrap();
        let row = c
            .query_one("SELECT creator FROM x_mind WHERE name = $1", &[&name])
            .await
            .unwrap();
        let creator: Option<String> = row.get("creator");
        assert_eq!(
            creator.as_deref(),
            Some(me),
            "写侧归属必须落地会话身份，实得 {creator:?}"
        );
        assert_ne!(
            creator.as_deref(),
            Some("system"),
            "不得再以 system 落库（任何登录用户都可冒名）"
        );
    }

    /// 分片A第四批：脑图行读（`view` / `get`）双向 403；
    /// 属主、`shared=true`、`x_mind_share` 接收人均 200，共享边不放宽。
    #[tokio::test]
    async fn mind_read_idor_is_blocked_both_ways() {
        let pool = shared::testing::test_pool();
        let (a, b) = ("u-sec-read-a-owner", "u-sec-read-a-other");
        {
            let c = pool.get().await.unwrap();
            for (id, creator, shared) in [
                ("sec-read-a-mind-a", a, false),
                ("sec-read-a-mind-b", b, false),
                ("sec-read-a-mind-b-only", b, false),
                ("sec-read-a-mind-pub", b, true),
            ] {
                c.execute("DELETE FROM x_mind_share WHERE mind_id = $1", &[&id])
                    .await
                    .unwrap();
                c.execute("DELETE FROM x_mind WHERE id = $1", &[&id])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO x_mind (id, name, content, creator, shared) VALUES ($1, $2, $3, $4, $5)",
                    &[&id, &format!("n-{id}"), &format!("c-{id}"), &creator, &shared],
                )
                .await
                .unwrap();
            }
            c.execute(
                "INSERT INTO x_mind_share (id, mind_id, person) VALUES ($1, 'sec-read-a-mind-b', $2)",
                &[&uuid::Uuid::new_v4().to_string(), &a],
            )
            .await
            .unwrap();
        }
        let view = |who: &str, id: &str| {
            crate::u2::mind_view(
                Extension(pool.clone()),
                Some(Extension(shared::testing::test_session(who))),
                Path(id.to_string()),
            )
        };
        let get = |who: &str, id: &str| {
            crate::u2::mind_get(
                Extension(pool.clone()),
                Some(Extension(shared::testing::test_session(who))),
                Path(id.to_string()),
            )
        };

        let own = view(a, "sec-read-a-mind-a").await;
        let body = &own.unwrap().0.data.clone().unwrap();
        assert_eq!(
            body["name"], "n-sec-read-a-mind-a",
            "属主读本人脑图必须返回落库内容"
        );
        assert!(
            matches!(
                view(a, "sec-read-a-mind-b-only").await,
                Err(AppError::Forbidden)
            ),
            "A 越权读 B 的私有脑图必须 403"
        );
        assert!(
            matches!(view(b, "sec-read-a-mind-a").await, Err(AppError::Forbidden)),
            "B 越权读 A 的脑图必须 403（双向）"
        );
        assert!(
            matches!(
                get(a, "sec-read-a-mind-b-only").await,
                Err(AppError::Forbidden)
            ),
            "get 同样不得越权读"
        );
        assert!(
            get(b, "sec-read-a-mind-b-only").await.is_ok(),
            "属主 get 本人脑图应成功"
        );
        assert!(
            view(a, "sec-read-a-mind-b").await.is_ok(),
            "共享接收人（x_mind_share.person）应可读"
        );
        assert!(
            view(a, "sec-read-a-mind-pub").await.is_ok(),
            "shared=true 的脑图应可读（保持既有公开共享语义）"
        );
        assert!(
            matches!(
                crate::u2::mind_view(
                    Extension(pool.clone()),
                    None,
                    Path("sec-read-a-mind-a".to_string())
                )
                .await,
                Err(AppError::Unauthorized)
            ),
            "无会话读脑图必须 401"
        );
    }

    /// 分片A第四批：版本读（列表 / 首个版本）按所属脑图归属判定，
    /// 双向 403；属主 200，共享接收人 200。
    #[tokio::test]
    async fn mind_version_read_idor_is_blocked_both_ways() {
        let pool = shared::testing::test_pool();
        let (a, b) = ("u-sec-read-a-owner", "u-sec-read-a-other");
        {
            let c = pool.get().await.unwrap();
            for (mid, creator) in [
                ("sec-read-a-ver-a", a),
                ("sec-read-a-ver-b", b),
                ("sec-read-a-ver-b-only", b),
            ] {
                c.execute(
                    "DELETE FROM x_mind_version_info WHERE mind_id = $1",
                    &[&mid],
                )
                .await
                .unwrap();
                c.execute("DELETE FROM x_mind_share WHERE mind_id = $1", &[&mid])
                    .await
                    .unwrap();
                c.execute("DELETE FROM x_mind WHERE id = $1", &[&mid])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO x_mind (id, name, content, creator, shared) VALUES ($1, $2, 'c', $3, false)",
                    &[&mid, &format!("n-{mid}"), &creator],
                )
                .await
                .unwrap();
                c.execute(
                    "INSERT INTO x_mind_version_info (id, mind_id, name, folder_id, creator, file_version, shared) \
                     VALUES ($1, $2, 'v1', '', $3, 1, false)",
                    &[&format!("ver-{mid}"), &mid, &creator],
                )
                .await
                .unwrap();
            }
            c.execute(
                "INSERT INTO x_mind_share (id, mind_id, person) VALUES ($1, 'sec-read-a-ver-b', $2)",
                &[&uuid::Uuid::new_v4().to_string(), &a],
            )
            .await
            .unwrap();
        }
        let list = |who: &str, id: &str| {
            crate::u2::mind_version_list(
                Extension(pool.clone()),
                Some(Extension(shared::testing::test_session(who))),
                Path(id.to_string()),
            )
        };
        let get = |who: &str, id: &str| {
            crate::u2::mind_version_get(
                Extension(pool.clone()),
                Some(Extension(shared::testing::test_session(who))),
                Path(id.to_string()),
            )
        };

        let own = list(a, "sec-read-a-ver-a").await;
        let data = own.unwrap().0.data.clone().unwrap();
        assert!(
            data.as_array()
                .unwrap()
                .iter()
                .any(|v| v["id"] == "ver-sec-read-a-ver-a"),
            "属主版本列表必须包含落库版本，实得 {data}"
        );
        assert!(
            matches!(
                list(a, "sec-read-a-ver-b-only").await,
                Err(AppError::Forbidden)
            ),
            "越权读他人版本列表必须 403"
        );
        assert!(
            matches!(list(b, "sec-read-a-ver-a").await, Err(AppError::Forbidden)),
            "反向越权读版本列表必须 403"
        );
        let own_ver = get(a, "sec-read-a-ver-a").await;
        assert_eq!(
            own_ver.unwrap().0.data.clone().unwrap()["id"],
            "ver-sec-read-a-ver-a",
            "属主读本人版本应返回落库行"
        );
        assert!(
            matches!(
                get(a, "sec-read-a-ver-b-only").await,
                Err(AppError::Forbidden)
            ),
            "越权读他人版本必须 403"
        );
        assert!(
            list(a, "sec-read-a-ver-b").await.is_ok(),
            "共享接收人应可读版本列表（共享边不放宽、也不误杀）"
        );
        assert!(
            matches!(
                crate::u2::mind_version_list(
                    Extension(pool.clone()),
                    None,
                    Path("sec-read-a-ver-a".to_string())
                )
                .await,
                Err(AppError::Unauthorized)
            ),
            "无会话读版本列表必须 401"
        );
    }

    /// R6：u2 `mind_save` 收会话存在档（无会话 401，门在 pool.get() 之前）。
    #[tokio::test]
    async fn r6_mind_save_rejects_without_session() {
        let pool = dead_pool();
        let res = crate::u2::mind_save(
            Extension(pool),
            None,
            Json(json!({ "name": "n", "content": "c" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 mind_save 必须 401（门在 pool.get() 之前）"
        );
    }
}
