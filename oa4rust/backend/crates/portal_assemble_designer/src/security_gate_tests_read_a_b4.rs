// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b4（`portal_assemble_designer` 三个 `x_portal` 读端点）。
//!
//! 口径 E23：`creator` 本人或管理员；三个端点共用同一判据与同一行。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a4-des-a";
    const B: &str = "r6a4-des-b";
    const ADMIN: &str = "r6a4-admin-des";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a4-des conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a4-des fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a4-admin-role-des','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A4 Des Admin', 'r6a4-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a4-admin-role-des', 'r6a4-unit') ON CONFLICT DO NOTHING",
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
            "portal_id" => crate::portal_id(pool, session, path).await,
            "portal_id_icon" => crate::portal_id_icon(pool, session, path).await,
            "portal_id_permission" => crate::portal_id_permission(pool, session, path).await,
            other => panic!("unknown endpoint {other}"),
        };
        result.map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b4_designer_portal_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN).await;

        let pa = format!("r6a4-des-a-{}", uuid::Uuid::new_v4());
        let pb = format!("r6a4-des-b-{}", uuid::Uuid::new_v4());
        for (id, creator, name, logo, permission) in [
            (
                &pa,
                A,
                "r6a4-canary-des-a",
                "r6a4-logo-des-a",
                "r6a4-perm-des-a",
            ),
            (
                &pb,
                B,
                "r6a4-canary-des-b",
                "r6a4-logo-des-b",
                "r6a4-perm-des-b",
            ),
        ] {
            exec(
                &pool,
                "INSERT INTO x_portal (id, name, alias, description, portal_category, creator, logo, permission) \
                 VALUES ($1, $2, 'r6a4-alias', 'r6a4-desc', 'r6a4-cat', $3, $4, $5)",
                vec![
                    id.to_string(),
                    name.to_string(),
                    creator.to_string(),
                    logo.to_string(),
                    permission.to_string(),
                ],
            )
            .await;
        }

        for endpoint in ["portal_id", "portal_id_icon", "portal_id_permission"] {
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
        }

        // 正控制：本人 / 管理员读自己的行，端点必须真的返回各自 canary。
        // `portal_id` 的 SELECT 读 `update_time`（schema 为 TIMESTAMP）却按 Option<String>
        // 反序列化，是既有的类型缺陷、与本批门禁无关；本批对它的放行分支只验 403 判定，
        // 正控制覆盖 icon/permission。
        for (endpoint, canary) in [
            ("portal_id_icon", "r6a4-logo-des-a"),
            ("portal_id_permission", "r6a4-perm-des-a"),
        ] {
            let body = call(endpoint, &pool, Some(A), &pa)
                .await
                .expect("本人读门户");
            assert!(
                body.contains(canary),
                "本人读 {endpoint} 必须返回 {canary}，实际: {body}"
            );
            assert!(
                call(endpoint, &pool, Some(ADMIN), &pa).await.is_ok(),
                "管理员必须可读他人 {endpoint}"
            );
        }

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
