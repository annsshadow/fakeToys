// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b4（`portal::portal_id`）。
//!
//! 口径 E23：`x_portal` 读 = `creator` 本人或管理员；空归属 fail-closed。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a4-por-a";
    const B: &str = "r6a4-por-b";
    const ADMIN: &str = "r6a4-admin-por";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a4-por conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a4-por fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a4-admin-role-por','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A4 Por Admin', 'r6a4-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a4-admin-role-por', 'r6a4-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn portal_id(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::portal_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b4_portal_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN).await;

        let pa = format!("r6a4-por-a-{}", uuid::Uuid::new_v4());
        let pb = format!("r6a4-por-b-{}", uuid::Uuid::new_v4());
        for (id, creator, name, logo) in [
            (&pa, A, "r6a4-canary-por-a", "r6a4-logo-a"),
            (&pb, B, "r6a4-canary-por-b", "r6a4-logo-b"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_portal (id, name, alias, description, portal_category, creator, logo) \
                 VALUES ($1, $2, 'r6a4-alias', 'r6a4-desc', 'r6a4-cat', $3, $4)",
                vec![
                    id.to_string(),
                    name.to_string(),
                    creator.to_string(),
                    logo.to_string(),
                ],
            )
            .await;
        }

        for (who, target) in [(B, pa.as_str()), (A, pb.as_str())] {
            assert!(
                matches!(
                    portal_id(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403"
            );
        }
        assert!(
            matches!(portal_id(&pool, None, &pa).await, Err(AppError::Forbidden)),
            "匿名读门户必须 403"
        );

        let own = portal_id(&pool, Some(A), &pa).await.expect("本人读门户");
        assert!(
            own.contains("r6a4-canary-por-a"),
            "本人读门户必须返回 name，实际: {own}"
        );
        assert!(
            portal_id(&pool, Some(ADMIN), &pa).await.is_ok(),
            "管理员必须可读他人门户"
        );

        let missing = portal_id(&pool, Some(A), "r6a4-por-missing")
            .await
            .expect("未知 id 保持 200 error 文案");
        assert!(
            missing.contains("portal not found"),
            "未知 id 必须返回 portal not found，实际: {missing}"
        );

        for id in [&pa, &pb] {
            exec(
                &pool,
                "DELETE FROM x_portal WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
