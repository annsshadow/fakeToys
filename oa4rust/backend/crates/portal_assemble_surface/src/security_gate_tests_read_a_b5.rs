// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b5（`portal_assemble_surface` 余下四个读端点）。
//!
//! 口径 E23：`x_portal` / `x_portal_surface` 读 = `creator` 本人或管理员。
//! 覆盖 `portal_id_icon`、`portal_id_icon_base64`、`preview_surface`、`surface_preview`。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a5-srf-a";
    const B: &str = "r6a5-srf-b";
    const ADMIN: &str = "r6a5-admin-srf";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a5-srf conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a5-srf fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a5-admin-role-srf','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A5 Srf Admin', 'r6a5-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a5-admin-role-srf', 'r6a5-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn call(
        which: &str,
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        let session = actor.map(|p| Extension(shared::testing::test_session(p)));
        let path = Path(id.to_string());
        let pool = Extension(pool.clone());
        let result = match which {
            "portal_id_icon" => crate::portal_id_icon(pool, session, path).await,
            "portal_id_icon_base64" => crate::portal_id_icon_base64(pool, session, path).await,
            "preview_surface" => crate::preview_surface(pool, session, path).await,
            "surface_preview" => crate::surface_preview(pool, session, path).await,
            other => panic!("unknown endpoint {other}"),
        };
        result.map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b5_surface_remaining_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN).await;

        let pa = format!("r6a5-por-a-{}", uuid::Uuid::new_v4());
        let pb = format!("r6a5-por-b-{}", uuid::Uuid::new_v4());
        for (id, creator, logo, logo64) in [
            (&pa, A, "r6a5-canary-logo-a", "r6a5-canary-logo64-a"),
            (&pb, B, "r6a5-canary-logo-b", "r6a5-canary-logo64-b"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_portal (id, name, alias, description, portal_category, creator, logo, logo_base64) \
                 VALUES ($1, 'r6a5-portal', 'r6a5-alias', 'r6a5-desc', 'r6a5-cat', $2, $3, $4)",
                vec![
                    id.to_string(),
                    creator.to_string(),
                    logo.to_string(),
                    logo64.to_string(),
                ],
            )
            .await;
        }

        let sa = format!("r6a5-srf-a-{}", uuid::Uuid::new_v4());
        let sb = format!("r6a5-srf-b-{}", uuid::Uuid::new_v4());
        for (id, creator, html) in [
            (&sa, A, "r6a5-canary-html-a"),
            (&sb, B, "r6a5-canary-html-b"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_portal_surface (id, name, html, creator) \
                 VALUES ($1, 'r6a5-surface', $2, $3)",
                vec![id.to_string(), html.to_string(), creator.to_string()],
            )
            .await;
        }

        // x_portal 两个端点
        for endpoint in ["portal_id_icon", "portal_id_icon_base64"] {
            for (who, target) in [(B, pa.as_str()), (A, pb.as_str())] {
                assert!(
                    matches!(
                        call(endpoint, &pool, Some(who), target).await,
                        Err(AppError::Forbidden)
                    ),
                    "{who} 读 {target}（{endpoint}）必须 403"
                );
            }
            assert!(
                matches!(
                    call(endpoint, &pool, None, &pa).await,
                    Err(AppError::Forbidden)
                ),
                "匿名读 {endpoint} 必须 403"
            );
            assert!(
                call(endpoint, &pool, Some(ADMIN), &pa).await.is_ok(),
                "管理员必须可读他人 {endpoint}"
            );
        }
        for (endpoint, canary) in [
            ("portal_id_icon", "r6a5-canary-logo-a"),
            ("portal_id_icon_base64", "r6a5-canary-logo64-a"),
        ] {
            let body = call(endpoint, &pool, Some(A), &pa)
                .await
                .expect("本人读门户");
            assert!(
                body.contains(canary),
                "本人读 {endpoint} 必须返回 {canary}，实际: {body}"
            );
        }

        // x_portal_surface 两个端点
        for endpoint in ["preview_surface", "surface_preview"] {
            for (who, target) in [(B, sa.as_str()), (A, sb.as_str())] {
                assert!(
                    matches!(
                        call(endpoint, &pool, Some(who), target).await,
                        Err(AppError::Forbidden)
                    ),
                    "{who} 读 {target}（{endpoint}）必须 403"
                );
            }
            assert!(
                matches!(
                    call(endpoint, &pool, None, &sa).await,
                    Err(AppError::Forbidden)
                ),
                "匿名读 {endpoint} 必须 403"
            );
            assert!(
                call(endpoint, &pool, Some(ADMIN), &sa).await.is_ok(),
                "管理员必须可读他人 {endpoint}"
            );
            let body = call(endpoint, &pool, Some(A), &sa)
                .await
                .expect("本人读 surface");
            assert!(
                body.contains("r6a5-canary-html-a"),
                "本人读 {endpoint} 必须返回 html，实际: {body}"
            );
        }

        let missing = call("surface_preview", &pool, Some(A), "r6a5-srf-missing")
            .await
            .expect("未知 id 保持 200 error 文案");
        assert!(
            missing.contains("surface not found"),
            "未知 id 必须返回 surface not found，实际: {missing}"
        );

        for id in [&pa, &pb] {
            exec(
                &pool,
                "DELETE FROM x_portal WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
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
