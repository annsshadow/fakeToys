// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! mind 属主资源越权修复——思维导图/文件夹 的 update/delete 此前任一登录用户
//! 可按任意 id 物理删除他人数据（mind_base_info/mind_folder_info 有 creator 列
//! 但无属主校验）。修复后非属主 403、属主放行；create 的 creator 改取会话。
//!
//! 属主门禁需先查库取 creator，故用 DB 门控：有库时种真实行断言非属主 403、
//! 属主放行；无库时跳过（与 meeting_assemble_control u2 先例一致）。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::Extension;
    use axum::extract::Path;
    use axum::Json;
    use serde_json::Value;
    use shared::error::AppError;
    use shared::response::ActionResult;

    use crate::{delete_folder, delete_mind, update_folder, update_mind};

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn mind_writes_require_owner() {
        if !shared::testing::is_db_available().await {
            eprintln!("SKIP (no PG): mind_writes_require_owner");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();

        let mind_id = uuid::Uuid::new_v4().to_string();
        let folder_id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO mind_base_info (id, name, folder_id, creator) VALUES ($1, 'sec-gate', '', 'u-owner')",
                &[&mind_id],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO mind_folder_info (id, name, creator) VALUES ($1, 'sec-gate', 'u-owner')",
                &[&folder_id],
            )
            .await
            .unwrap();

        // 非属主 → 403
        assert!(
            matches!(
                delete_mind(
                    Extension(pool.clone()),
                    Extension(build_session("u-other")),
                    Path(mind_id.clone())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_mind 非属主必须 403"
        );
        assert!(
            matches!(
                update_mind(
                    Extension(pool.clone()),
                    Extension(build_session("u-other")),
                    Path(mind_id.clone()),
                    axum::Json(serde_json::json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "update_mind 非属主必须 403"
        );
        assert!(
            matches!(
                delete_folder(
                    Extension(pool.clone()),
                    Extension(build_session("u-other")),
                    Path(folder_id.clone())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_folder 非属主必须 403"
        );
        assert!(
            matches!(
                update_folder(
                    Extension(pool.clone()),
                    Extension(build_session("u-other")),
                    Path(folder_id.clone()),
                    axum::Json(serde_json::json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "update_folder 非属主必须 403"
        );

        // 属主 → 放行（不落 403）
        assert!(
            !matches!(
                delete_mind(
                    Extension(pool.clone()),
                    Extension(build_session("u-owner")),
                    Path(mind_id.clone())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_mind 属主不得 403"
        );

        client
            .execute("DELETE FROM mind_base_info WHERE id = $1", &[&mind_id])
            .await
            .unwrap();
        client
            .execute("DELETE FROM mind_folder_info WHERE id = $1", &[&folder_id])
            .await
            .unwrap();
    }

    /// 死池：`pool.get()` 必败，用于离线证明"门在库前返回"。
    fn dead_pool() -> deadpool_postgres::Pool {
        let mgr = deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        deadpool_postgres::Pool::builder(mgr).build().unwrap()
    }

    /// R6：create_mind / create_folder 收会话存在档（无会话 401，门在 pool.get() 之前）。
    #[tokio::test]
    async fn r6_create_rejects_without_session() {
        let pool = dead_pool();
        let mind_res = crate::create_mind(
            Extension(pool.clone()),
            None,
            axum::Json(serde_json::json!({ "name": "n" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(mind_res, Err(AppError::Unauthorized)),
            "无会话调用 create_mind 必须 401（门在 pool.get() 之前）"
        );
        let folder_res = crate::create_folder(
            Extension(pool),
            None,
            axum::Json(serde_json::json!({ "name": "f" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(folder_res, Err(AppError::Unauthorized)),
            "无会话调用 create_folder 必须 401（门在 pool.get() 之前）"
        );
    }

    /// 分片 A 读越权（IDOR）回归：`get_mind_with_id`（GET `/api/mind/mind/{id}`）
    /// 原无会话、无归属过滤，任一登录用户凭 id 可读他人思维导图。修复后归属谓词
    /// 写在 SQL WHERE 内。双向：A 读 B 的 403；B 读 A 的 403；本人 200 且真返回该行。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn mind_read_idor_is_blocked_both_ways() {
        let pool = shared::testing::test_pool();
        {
            let c = pool.get().await.unwrap();
            for (id, name, creator) in [
                ("sec-read-a-mind-own", "own-name", "u-sec-read-owner"),
                ("sec-read-a-mind-other", "other-name", "u-sec-read-other"),
            ] {
                // mind_base_info 的历史表没有主键/唯一约束，不能用 ON CONFLICT。
                c.execute("DELETE FROM mind_base_info WHERE id = $1", &[&id])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO mind_base_info (id, name, creator) VALUES ($1, $2, $3)",
                    &[&id, &name, &creator],
                )
                .await
                .unwrap();
            }
        }
        let owner = build_session("u-sec-read-owner");
        let other = build_session("u-sec-read-other");
        let path = |id: &str| Path(id.to_string());

        // 正控制：本人读自己的思维导图 200 且真返回落库内容。
        let body = crate::get_mind_with_id(
            Extension(pool.clone()),
            Some(Extension(owner.clone())),
            path("sec-read-a-mind-own"),
        )
        .await
        .expect("本人读自己的思维导图应成功");
        let data = body.0.data.expect("成功响应必须带 data");
        assert_eq!(
            data.get("id").and_then(|v| v.as_str()),
            Some("sec-read-a-mind-own")
        );
        assert_eq!(data.get("name").and_then(|v| v.as_str()), Some("own-name"));

        // 双向：A→B 与 B→A 都必须 403。
        for (sess, id) in [
            (&owner, "sec-read-a-mind-other"),
            (&other, "sec-read-a-mind-own"),
        ] {
            let res = crate::get_mind_with_id(
                Extension(pool.clone()),
                Some(Extension(sess.clone())),
                path(id),
            )
            .await;
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "越权读 {id} 必须 403"
            );
        }

        // 无会话：401。
        let res =
            crate::get_mind_with_id(Extension(pool.clone()), None, path("sec-read-a-mind-own"))
                .await;
        assert!(matches!(res, Err(AppError::Unauthorized)));
    }

    /// 分片 A 列表读越权：`list_my_folders` 原无会话、无归属过滤，返回全员文件夹。
    /// 修复后 `creator = 会话 OR admin`（NULL fail-closed）。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn folder_list_idor_is_blocked_both_ways() {
        let pool = shared::testing::test_pool();
        {
            let c = pool.get().await.unwrap();
            for (id, name, creator) in [
                ("sec-read-a-folder-own", "own-folder", "u-sec-read-owner"),
                (
                    "sec-read-a-folder-other",
                    "other-folder",
                    "u-sec-read-other",
                ),
            ] {
                c.execute("DELETE FROM mind_folder_info WHERE id = $1", &[&id])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO mind_folder_info (id, name, creator) VALUES ($1, $2, $3)",
                    &[&id, &name, &creator],
                )
                .await
                .unwrap();
            }
        }
        let owner = build_session("u-sec-read-owner");
        let other = build_session("u-sec-read-other");

        let ids = |body: &Json<ActionResult<Value>>| -> Vec<String> {
            body.0
                .data
                .as_ref()
                .and_then(|d| d.as_array())
                .map(|rows| {
                    rows.iter()
                        .filter_map(|r| r.get("id").and_then(|v| v.as_str()).map(str::to_string))
                        .collect()
                })
                .unwrap_or_default()
        };

        // 正控制 + 双向：A 只看得到自己的，看不到 B 的；B 反之亦然。
        for (sess, own, theirs) in [
            (&owner, "sec-read-a-folder-own", "sec-read-a-folder-other"),
            (&other, "sec-read-a-folder-other", "sec-read-a-folder-own"),
        ] {
            let body =
                crate::list_my_folders(Extension(pool.clone()), Some(Extension(sess.clone())))
                    .await
                    .expect("列表读取应成功");
            let got = ids(&body);
            assert!(got.contains(&own.to_string()), "必须返回本人文件夹 {own}");
            assert!(
                !got.contains(&theirs.to_string()),
                "不得返回他人文件夹 {theirs}"
            );
        }

        // 无会话：401。
        let res = crate::list_my_folders(Extension(pool.clone()), None).await;
        assert!(matches!(res, Err(AppError::Unauthorized)));
    }

    /// 分片 A 列表读越权：`list_versions_with_mind_id` 原无会话、无归属过滤，
    /// 任一登录用户凭 mind id 读他人版本历史。修复后归属取父思维导图 creator。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn version_list_idor_is_blocked_both_ways() {
        let pool = shared::testing::test_pool();
        {
            let c = pool.get().await.unwrap();
            for (mind_id, creator, version_id, vname) in [
                (
                    "sec-read-a-mind-own",
                    "u-sec-read-owner",
                    "sec-read-a-version-own",
                    "own-v1",
                ),
                (
                    "sec-read-a-mind-other",
                    "u-sec-read-other",
                    "sec-read-a-version-other",
                    "other-v1",
                ),
            ] {
                c.execute(
                    "DELETE FROM mind_version_info WHERE id = $1",
                    &[&version_id],
                )
                .await
                .unwrap();
                c.execute("DELETE FROM mind_base_info WHERE id = $1", &[&mind_id])
                    .await
                    .unwrap();
                c.execute(
                    "INSERT INTO mind_base_info (id, name, creator) VALUES ($1, 'n', $2)",
                    &[&mind_id, &creator],
                )
                .await
                .unwrap();
                c.execute(
                    "INSERT INTO mind_version_info (id, mind_id, name, creator) VALUES ($1, $2, $3, $4)",
                    &[&version_id, &mind_id, &vname, &creator],
                )
                .await
                .unwrap();
            }
        }
        let owner = build_session("u-sec-read-owner");
        let other = build_session("u-sec-read-other");
        let path = |id: &str| Path(id.to_string());

        // 正控制：本人读自己思维导图的版本，200 且真返回落库版本行。
        let body = crate::list_versions_with_mind_id(
            Extension(pool.clone()),
            Some(Extension(owner.clone())),
            path("sec-read-a-mind-own"),
        )
        .await
        .expect("本人读自己的版本应成功");
        let rows = body
            .0
            .data
            .as_ref()
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].get("id").and_then(|v| v.as_str()),
            Some("sec-read-a-version-own")
        );

        // 双向：A 读 B 的思维导图版本 403；B 读 A 的 403。
        for (sess, id) in [
            (&owner, "sec-read-a-mind-other"),
            (&other, "sec-read-a-mind-own"),
        ] {
            let res = crate::list_versions_with_mind_id(
                Extension(pool.clone()),
                Some(Extension(sess.clone())),
                path(id),
            )
            .await;
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "越权读 {id} 的版本必须 403"
            );
        }
    }
}
