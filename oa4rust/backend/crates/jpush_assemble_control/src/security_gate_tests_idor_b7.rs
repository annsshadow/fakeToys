// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读 IDOR 批次 7·jpush_assemble_control：`get_jpush` 按 `x_jpush.creator` 收归属。
//!
//! 谓词进 `SELECT ... WHERE`：本人（`creator`）、广播（`target = 'all'`）或管理员；
//! 归属为空且非广播 → 仅管理员。A/B 双向 403；广播行保留全员可读（共享语义）。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::error::AppError;

    const A: &str = "b7jpa-a";
    const B: &str = "b7jpa-b";
    const ADMIN: &str = "b7jpa-admin";
    const ADMIN_ROLE: &str = "b7jpa-admin-role";

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
        let client = pool.get().await.expect("b7jpa conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("b7jpa fixture sql");
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
             VALUES ($1, $1, 'B7JPA Admin', 'b7jpa-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, 'b7jpa-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string(), ADMIN_ROLE.to_string()],
        )
        .await;
    }

    async fn seed_jpush(pool: &Pool, id: &str, creator: &str, target: &str) {
        exec(
            pool,
            "INSERT INTO x_jpush (id, title, content, target, creator) \
             VALUES ($1, 'b7jpa', 'b7jpa', $2, $3)",
            vec![id.to_string(), target.to_string(), creator.to_string()],
        )
        .await;
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_jpush WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    async fn read(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let res = crate::get_jpush(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await?;
        Ok(serde_json::to_value(&res.0).expect("b7jpa body"))
    }

    #[tokio::test]
    async fn b7_jpush_read_both_directions_and_broadcast_share() {
        let Some(pool) = pool().await else {
            return;
        };
        let fa = format!("b7jpa-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("b7jpa-fb-{}", uuid::Uuid::new_v4());
        let fbroadcast = format!("b7jpa-all-{}", uuid::Uuid::new_v4());
        seed_jpush(&pool, &fa, A, "b7jpa-single").await;
        seed_jpush(&pool, &fb, B, "b7jpa-single").await;
        seed_jpush(&pool, &fbroadcast, B, "all").await;

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
        assert!(body.to_string().contains(&fa), "本人必须可读自己的推送");

        // 共享语义：target='all' 的广播是面向全员的，不能被归属门挡掉。
        let shared = read(&pool, Some(A), &fbroadcast)
            .await
            .expect("broadcast read ok");
        assert!(
            shared.to_string().contains(&fbroadcast),
            "target='all' 广播必须对非属主保持可读"
        );

        for id in [&fa, &fb, &fbroadcast] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b7_jpush_read_admin_anon_and_missing() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let fb = format!("b7jpa-adm-{}", uuid::Uuid::new_v4());
        seed_jpush(&pool, &fb, B, "b7jpa-single").await;

        let body = read(&pool, Some(ADMIN), &fb).await.expect("admin read ok");
        assert!(body.to_string().contains(&fb), "管理员必须可读他人推送");

        let dead = dead_pool();
        assert!(
            matches!(read(&dead, None, &fb).await, Err(AppError::Unauthorized)),
            "无会话必须 401（门在读库前）"
        );

        let missing = format!("b7jpa-missing-{}", uuid::Uuid::new_v4());
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
