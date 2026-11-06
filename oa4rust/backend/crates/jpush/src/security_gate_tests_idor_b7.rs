// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读 IDOR 批次 7·jpush：`device_get` 按 `x_jpush_device.user_id` 收归属。
//!
//! 谓词进 `SELECT ... WHERE`：本人（`user_id`）或管理员；归属为空 → 仅管理员。
//! A/B 双向都断言 403；本人 200 且响应体含该行 id；无会话 401 先于读库。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::error::AppError;

    const A: &str = "b7jpd-a";
    const B: &str = "b7jpd-b";
    const ADMIN: &str = "b7jpd-admin";
    const ADMIN_ROLE: &str = "b7jpd-admin-role";

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("b7jpd conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("b7jpd fixture sql");
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
             VALUES ($1, $1, 'B7JPD Admin', 'b7jpd-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, 'b7jpd-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string(), ADMIN_ROLE.to_string()],
        )
        .await;
    }

    async fn seed_device(pool: &Pool, id: &str, owner: &str) {
        exec(
            pool,
            "INSERT INTO x_jpush_device (id, user_id, platform, token) \
             VALUES ($1, $2, 'b7jpd', 'b7jpd-token')",
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
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let res = crate::device_get(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await?;
        Ok(serde_json::to_value(&res.0).expect("b7jpd body"))
    }

    #[tokio::test]
    async fn b7_jpush_device_both_directions_and_owner() {
        let Some(pool) = pool().await else {
            return;
        };
        let fa = format!("b7jpd-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("b7jpd-fb-{}", uuid::Uuid::new_v4());
        seed_device(&pool, &fa, A).await;
        seed_device(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            assert!(
                matches!(
                    read(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403（双向）"
            );
        }

        let body = read(&pool, Some(A), &fa).await.expect("owner read ok");
        assert!(
            body.to_string().contains(&fa),
            "本人读自己的设备必须返回该行 id"
        );

        for id in [&fa, &fb] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b7_jpush_device_admin_anon_and_missing() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let fb = format!("b7jpd-adm-{}", uuid::Uuid::new_v4());
        seed_device(&pool, &fb, B).await;

        let body = read(&pool, Some(ADMIN), &fb).await.expect("admin read ok");
        assert!(body.to_string().contains(&fb), "管理员必须可读他人设备");

        let dead = dead_pool();
        assert!(
            matches!(read(&dead, None, &fb).await, Err(AppError::Unauthorized)),
            "无会话必须 401（门在读库前）"
        );

        let missing = format!("b7jpd-missing-{}", uuid::Uuid::new_v4());
        assert!(
            matches!(
                read(&pool, Some(A), &missing).await,
                Err(AppError::NotFound)
            ),
            "不存在的行保留 404"
        );

        cleanup(&pool, &fb).await;
    }
}
