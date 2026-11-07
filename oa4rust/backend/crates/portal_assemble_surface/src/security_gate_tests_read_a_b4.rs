// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b4（`portal_assemble_surface::get_surface`）。
//!
//! 口径 E23：`x_portal_surface` 读 = `creator` 本人或管理员；空归属 fail-closed。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a4-srf-a";
    const B: &str = "r6a4-srf-b";
    const ADMIN: &str = "r6a4-admin-srf";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a4-srf conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a4-srf fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a4-admin-role-srf','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A4 Srf Admin', 'r6a4-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a4-admin-role-srf', 'r6a4-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn get_surface(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::get_surface(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b4_surface_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN).await;

        let sa = format!("r6a4-srf-a-{}", uuid::Uuid::new_v4());
        let sb = format!("r6a4-srf-b-{}", uuid::Uuid::new_v4());
        for (id, creator, name, html) in [
            (&sa, A, "r6a4-canary-srf-a", "r6a4-html-srf-a"),
            (&sb, B, "r6a4-canary-srf-b", "r6a4-html-srf-b"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_portal_surface (id, name, html, creator) \
                 VALUES ($1, $2, $3, $4)",
                vec![
                    id.to_string(),
                    name.to_string(),
                    html.to_string(),
                    creator.to_string(),
                ],
            )
            .await;
        }

        for (who, target) in [(B, sa.as_str()), (A, sb.as_str())] {
            assert!(
                matches!(
                    get_surface(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403"
            );
        }
        assert!(
            matches!(
                get_surface(&pool, None, &sa).await,
                Err(AppError::Forbidden)
            ),
            "匿名读 surface 必须 403"
        );

        let own = get_surface(&pool, Some(A), &sa)
            .await
            .expect("本人读 surface");
        assert!(
            own.contains("r6a4-html-srf-a"),
            "本人读 surface 必须返回 html，实际: {own}"
        );
        assert!(
            get_surface(&pool, Some(ADMIN), &sa).await.is_ok(),
            "管理员必须可读他人 surface"
        );

        let missing = get_surface(&pool, Some(A), "r6a4-srf-missing")
            .await
            .expect("未知 id 保持 200 error 文案");
        assert!(
            missing.contains("surface not found"),
            "未知 id 必须返回 surface not found，实际: {missing}"
        );

        for id in [&sa, &sb] {
            exec(
                &pool,
                "DELETE FROM x_portal_surface WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
