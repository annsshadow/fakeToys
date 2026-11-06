// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读 IDOR 批次 7·component_assemble_control：`get_component` 按 creator 收归属。
//!
//! 自定义组件：本人（`creator`）或管理员；`type='system'` / `visible=true` 的内置
//! 共享组件保留全员可读（防止共享组件被 creator 门误挡）。归属为空且非共享 → 仅管理员。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::error::AppError;

    const A: &str = "b7cmp-a";
    const B: &str = "b7cmp-b";
    const ADMIN: &str = "b7cmp-admin";
    const ADMIN_ROLE: &str = "b7cmp-admin-role";

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
        let client = pool.get().await.expect("b7cmp conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("b7cmp fixture sql");
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
             VALUES ($1, $1, 'B7CMP Admin', 'b7cmp-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, 'b7cmp-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string(), ADMIN_ROLE.to_string()],
        )
        .await;
    }

    async fn seed_component(pool: &Pool, id: &str, creator: &str, kind: &str, visible: bool) {
        let sql = format!(
            "INSERT INTO x_component (id, name, type, creator, visible) \
             VALUES ($1, 'b7cmp', $2, $3, {visible})"
        );
        exec(
            pool,
            &sql,
            vec![id.to_string(), kind.to_string(), creator.to_string()],
        )
        .await;
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_component WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    async fn read(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let res = crate::get_component(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await?;
        Ok(serde_json::to_value(&res.0).expect("b7cmp body"))
    }

    #[tokio::test]
    async fn b7_component_both_directions_and_shared() {
        let Some(pool) = pg().await else {
            return;
        };
        let fa = format!("b7cmp-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("b7cmp-fb-{}", uuid::Uuid::new_v4());
        let fsystem = format!("b7cmp-sys-{}", uuid::Uuid::new_v4());
        let fvisible = format!("b7cmp-vis-{}", uuid::Uuid::new_v4());
        seed_component(&pool, &fa, A, "custom", false).await;
        seed_component(&pool, &fb, B, "custom", false).await;
        seed_component(&pool, &fsystem, B, "system", false).await;
        seed_component(&pool, &fvisible, B, "custom", true).await;

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
        assert!(body.to_string().contains(&fa), "本人必须可读自己的组件");

        // 共享语义：系统内置组件与 visible 组件不能被 creator 门误挡。
        for id in [&fsystem, &fvisible] {
            let body = read(&pool, Some(A), id).await.expect("shared read ok");
            assert!(
                body.to_string().contains(id),
                "system/visible 共享组件必须对非属主保持可读"
            );
        }

        for id in [&fa, &fb, &fsystem, &fvisible] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b7_component_admin_anon_and_missing() {
        let Some(pool) = pg().await else {
            return;
        };
        seed_admin(&pool).await;
        let fb = format!("b7cmp-adm-{}", uuid::Uuid::new_v4());
        seed_component(&pool, &fb, B, "custom", false).await;

        let body = read(&pool, Some(ADMIN), &fb).await.expect("admin read ok");
        assert!(body.to_string().contains(&fb), "管理员必须可读他人组件");

        let dead = dead_pool();
        assert!(
            matches!(read(&dead, None, &fb).await, Err(AppError::Unauthorized)),
            "无会话必须 401（门在读库前）"
        );

        let missing = format!("b7cmp-missing-{}", uuid::Uuid::new_v4());
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
