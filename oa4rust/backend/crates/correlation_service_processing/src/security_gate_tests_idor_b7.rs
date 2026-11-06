// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读 IDOR 批次 7·correlation_service_processing：`get_correlation` 按 person_id 收归属。
//!
//! 归属列 `person_id`（主体人），兼容早期 `creator`：本人或管理员；归属为空 → 仅管理员。
//! A/B 双向 403；本人 200 且响应体含该行 id；无会话 401 先于读库。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::error::AppError;

    const A: &str = "b7cor-a";
    const B: &str = "b7cor-b";
    const ADMIN: &str = "b7cor-admin";
    const ADMIN_ROLE: &str = "b7cor-admin-role";

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    async fn pg() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("b7cor conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("b7cor fixture sql");
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
             VALUES ($1, $1, 'B7COR Admin', 'b7cor-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, 'b7cor-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string(), ADMIN_ROLE.to_string()],
        )
        .await;
    }

    async fn seed_correlation(pool: &Pool, id: &str, person: &str) {
        exec(
            pool,
            "INSERT INTO x_correlation (id, person_id, creator) VALUES ($1, $2, 'system')",
            vec![id.to_string(), person.to_string()],
        )
        .await;
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_correlation WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    async fn read(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let res = crate::get_correlation(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await?;
        Ok(serde_json::to_value(&res.0).expect("b7cor body"))
    }

    #[tokio::test]
    async fn b7_correlation_both_directions_and_owner() {
        let Some(pool) = pg().await else {
            return;
        };
        let fa = format!("b7cor-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("b7cor-fb-{}", uuid::Uuid::new_v4());
        seed_correlation(&pool, &fa, A).await;
        seed_correlation(&pool, &fb, B).await;

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
            "本人读自己的关联记录必须返回该行 id"
        );

        for id in [&fa, &fb] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b7_correlation_admin_anon_and_missing() {
        let Some(pool) = pg().await else {
            return;
        };
        seed_admin(&pool).await;
        let fb = format!("b7cor-adm-{}", uuid::Uuid::new_v4());
        seed_correlation(&pool, &fb, B).await;

        let body = read(&pool, Some(ADMIN), &fb).await.expect("admin read ok");
        assert!(body.to_string().contains(&fb), "管理员必须可读他人关联");

        let dead = dead_pool();
        assert!(
            matches!(read(&dead, None, &fb).await, Err(AppError::Unauthorized)),
            "无会话必须 401（门在读库前）"
        );

        let missing = format!("b7cor-missing-{}", uuid::Uuid::new_v4());
        let body = read(&pool, Some(A), &missing)
            .await
            .expect("missing keeps error body");
        assert!(
            body.to_string().contains("not found"),
            "不存在的行保留原 error body"
        );

        cleanup(&pool, &fb).await;
    }
}
