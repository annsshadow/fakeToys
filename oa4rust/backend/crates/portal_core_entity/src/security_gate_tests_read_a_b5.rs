// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b5（`portal_core_entity::page_get`）。
//!
//! 口径 E23：`x_portal_page.creator` 本人或管理员；未命中保留 `page not found`。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use sea_orm::DatabaseConnection;
    use shared::error::AppError;

    const A: &str = "r6a5-pge-a";
    const B: &str = "r6a5-pge-b";
    const ADMIN: &str = "r6a5-admin-pge";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a5-pge conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a5-pge fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a5-admin-role-pge','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A5 Pge Admin', 'r6a5-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a5-admin-role-pge', 'r6a5-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn page_get(
        db: &DatabaseConnection,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        crate::page_get(
            Extension(db.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b5_portal_page_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("r6a5-pge sea-orm");
        seed_admin(&pool, ADMIN).await;

        let pa = format!("r6a5-pge-a-{}", uuid::Uuid::new_v4());
        let pb = format!("r6a5-pge-b-{}", uuid::Uuid::new_v4());
        for (id, creator, name) in [(&pa, A, "r6a5-canary-pge-a"), (&pb, B, "r6a5-canary-pge-b")] {
            exec(
                &pool,
                "INSERT INTO x_portal_page (id, portal_id, name, status, creator) \
                 VALUES ($1, 'r6a5-portal', $2, 'published', $3)",
                vec![id.to_string(), name.to_string(), creator.to_string()],
            )
            .await;
        }

        for (who, target) in [(B, pa.as_str()), (A, pb.as_str())] {
            assert!(
                matches!(
                    page_get(&db, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403"
            );
        }
        assert!(
            matches!(page_get(&db, None, &pa).await, Err(AppError::Forbidden)),
            "匿名读页面必须 403"
        );

        let own = page_get(&db, Some(A), &pa).await.expect("本人读页面");
        assert!(
            own.contains("r6a5-canary-pge-a"),
            "本人读页面必须返回 name，实际: {own}"
        );
        assert!(
            page_get(&db, Some(ADMIN), &pa).await.is_ok(),
            "管理员必须可读他人页面"
        );

        let missing = page_get(&db, Some(A), "r6a5-pge-missing")
            .await
            .expect("未知 id 保持 200 error 文案");
        assert!(
            missing.contains("page not found"),
            "未知 id 必须返回 page not found，实际: {missing}"
        );

        for id in [&pa, &pb] {
            exec(
                &pool,
                "DELETE FROM x_portal_page WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
