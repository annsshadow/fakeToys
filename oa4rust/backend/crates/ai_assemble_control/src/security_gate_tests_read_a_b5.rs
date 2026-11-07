// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b5（`ai_assemble_control::config_get_mcp_flag`）。
//!
//! 口径 E24 默认：`x_ai_mcp_config.creator` 本人或管理员；未命中保留
//! `mcp config not found`。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a5-ai-a";
    const B: &str = "r6a5-ai-b";
    const ADMIN: &str = "r6a5-admin-ai";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a5-ai conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a5-ai fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a5-admin-role-ai','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A5 Ai Admin', 'r6a5-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a5-admin-role-ai', 'r6a5-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn config_get_mcp_flag(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        crate::config_get_mcp_flag(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b5_ai_mcp_config_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN).await;

        let ca = format!("r6a5-ai-a-{}", uuid::Uuid::new_v4());
        let cb = format!("r6a5-ai-b-{}", uuid::Uuid::new_v4());
        for (id, creator, name) in [(&ca, A, "r6a5-canary-ai-a"), (&cb, B, "r6a5-canary-ai-b")] {
            exec(
                &pool,
                "INSERT INTO x_ai_mcp_config (id, name, url, enabled, creator) \
                 VALUES ($1, $2, 'r6a5-url', true, $3)",
                vec![id.to_string(), name.to_string(), creator.to_string()],
            )
            .await;
        }

        for (who, target) in [(B, ca.as_str()), (A, cb.as_str())] {
            assert!(
                matches!(
                    config_get_mcp_flag(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403"
            );
        }
        assert!(
            matches!(
                config_get_mcp_flag(&pool, None, &ca).await,
                Err(AppError::Forbidden)
            ),
            "匿名读 mcp 配置必须 403"
        );

        let own = config_get_mcp_flag(&pool, Some(A), &ca)
            .await
            .expect("本人读 mcp 配置");
        assert!(
            own.contains("r6a5-canary-ai-a"),
            "本人读 mcp 配置必须返回 name，实际: {own}"
        );
        assert!(
            config_get_mcp_flag(&pool, Some(ADMIN), &ca).await.is_ok(),
            "管理员必须可读他人 mcp 配置"
        );

        let missing = config_get_mcp_flag(&pool, Some(A), "r6a5-ai-missing")
            .await
            .expect("未知 id 保持 200 error 文案");
        assert!(
            missing.contains("mcp config not found"),
            "未知 id 必须返回 mcp config not found，实际: {missing}"
        );

        for id in [&ca, &cb] {
            exec(
                &pool,
                "DELETE FROM x_ai_mcp_config WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
