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
    use shared::error::AppError;

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
}
