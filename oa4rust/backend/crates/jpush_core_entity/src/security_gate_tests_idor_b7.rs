// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读 IDOR 批次 7·jpush_core_entity：`device_get` 按 `x_jpush_device.user_id` 收归属。
//!
//! sea-orm `Entity::find_by_id(...).filter(Expr::cust_with_values(...))`：本人或管理员；
//! 归属为空 → 仅管理员。行存在但无权 403，不存在保留 404；A/B 双向都断言。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "b7jce-a";
    const B: &str = "b7jce-b";
    const ADMIN: &str = "b7jce-admin";
    const ADMIN_ROLE: &str = "b7jce-admin-role";

    async fn pg() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("b7jce conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("b7jce fixture sql");
    }

    async fn seed_admin(pool: &Pool) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ($1, 'admin') ON CONFLICT (id) DO NOTHING",
            vec![ADMIN_ROLE.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'B7JCE Admin', 'b7jce-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, 'b7jce-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string(), ADMIN_ROLE.to_string()],
        )
        .await;
    }

    async fn seed_device(pool: &Pool, id: &str, owner: &str) {
        exec(
            pool,
            "INSERT INTO x_jpush_device (id, user_id, platform, token) \
             VALUES ($1, $2, 'b7jce', 'b7jce-token')",
            vec![id.to_string(), owner.to_string()],
        )
        .await;
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_jpush_device WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    async fn read(
        db: &sea_orm::DatabaseConnection,
        actor: Option<&str>,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let res = crate::device_get(
            Extension(db.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await?;
        Ok(serde_json::to_value(&res.0).expect("b7jce body"))
    }

    #[tokio::test]
    async fn b7_jpush_core_device_both_directions_and_owner() {
        let Some(pool) = pg().await else {
            return;
        };
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("b7jce sea_orm db");
        let fa = format!("b7jce-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("b7jce-fb-{}", uuid::Uuid::new_v4());
        seed_device(&pool, &fa, A).await;
        seed_device(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            assert!(
                matches!(read(&db, Some(who), target).await, Err(AppError::Forbidden)),
                "{who} 读 {target} 必须 403（双向）"
            );
        }

        let body = read(&db, Some(A), &fa).await.expect("owner read ok");
        assert!(
            body.to_string().contains(&fa),
            "本人读自己的设备必须返回该行 id"
        );

        for id in [&fa, &fb] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b7_jpush_core_device_admin_anon_and_missing() {
        let Some(pool) = pg().await else {
            return;
        };
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("b7jce sea_orm db");
        seed_admin(&pool).await;
        let fb = format!("b7jce-adm-{}", uuid::Uuid::new_v4());
        seed_device(&pool, &fb, B).await;

        let body = read(&db, Some(ADMIN), &fb).await.expect("admin read ok");
        assert!(body.to_string().contains(&fb), "管理员必须可读他人设备");

        assert!(
            matches!(read(&db, None, &fb).await, Err(AppError::Unauthorized)),
            "无会话必须 401（门在读库前）"
        );

        let missing = format!("b7jce-missing-{}", uuid::Uuid::new_v4());
        assert!(
            matches!(read(&db, Some(A), &missing).await, Err(AppError::NotFound)),
            "不存在的行保留 404"
        );

        cleanup(&pool, &fb).await;
    }
}
