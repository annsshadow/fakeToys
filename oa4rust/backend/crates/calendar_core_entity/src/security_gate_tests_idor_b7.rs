// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读 IDOR 批次 7·calendar_core_entity：`calendar_get` 按 createor/target/公开旗标收读权限。
//!
//! 谓词进 `SELECT ... WHERE`：创建人、共享目标（`target`）、公开日历（`is_public`）
//! 或管理员；归属为空且非公开 → 仅管理员。A/B 双向 403，公开/共享行保留可读。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "b7cal-a";
    const B: &str = "b7cal-b";
    const ADMIN: &str = "b7cal-admin";
    const ADMIN_ROLE: &str = "b7cal-admin-role";

    async fn pg() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("b7cal conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("b7cal fixture sql");
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
             VALUES ($1, $1, 'B7CAL Admin', 'b7cal-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, 'b7cal-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string(), ADMIN_ROLE.to_string()],
        )
        .await;
    }

    async fn seed_calendar(pool: &Pool, id: &str, createor: &str, target: &str, is_public: bool) {
        let sql = format!(
            "INSERT INTO x_cal_calendar \
             (id, name, type, target, color, description, createor, is_public, status) \
             VALUES ($1, 'b7cal', 'PERSONAL', $2, '#ffffff', 'b7cal', $3, {is_public}, 'OPEN')"
        );
        exec(
            pool,
            &sql,
            vec![id.to_string(), target.to_string(), createor.to_string()],
        )
        .await;
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_cal_calendar WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    async fn read(
        db: &sea_orm::DatabaseConnection,
        actor: Option<&str>,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let res = crate::calendar_get(
            Extension(db.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await?;
        Ok(serde_json::to_value(&res.0).expect("b7cal body"))
    }

    #[tokio::test]
    async fn b7_calendar_both_directions_and_sharing() {
        let Some(pool) = pg().await else {
            return;
        };
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("b7cal sea_orm db");
        let fa = format!("b7cal-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("b7cal-fb-{}", uuid::Uuid::new_v4());
        let fpublic = format!("b7cal-pub-{}", uuid::Uuid::new_v4());
        let ftarget = format!("b7cal-tgt-{}", uuid::Uuid::new_v4());
        seed_calendar(&pool, &fa, A, A, false).await;
        seed_calendar(&pool, &fb, B, B, false).await;
        seed_calendar(&pool, &fpublic, B, B, true).await;
        // 共享给 A 的日历：createor 是 B，target 指向 A。
        seed_calendar(&pool, &ftarget, B, A, false).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            assert!(
                matches!(read(&db, Some(who), target).await, Err(AppError::Forbidden)),
                "{who} 读 {target} 必须 403（双向）"
            );
        }

        let body = read(&db, Some(A), &fa).await.expect("owner read ok");
        assert!(body.to_string().contains(&fa), "本人必须可读自己的日历");

        // 共享语义：公开日历与 target 共享行都不能被归属门挡掉。
        for id in [&fpublic, &ftarget] {
            let body = read(&db, Some(A), id).await.expect("shared read ok");
            assert!(body.to_string().contains(id), "共享目标/公开日历必须可读");
        }

        for id in [&fa, &fb, &fpublic, &ftarget] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b7_calendar_admin_anon_and_missing() {
        let Some(pool) = pg().await else {
            return;
        };
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("b7cal sea_orm db");
        seed_admin(&pool).await;
        let fb = format!("b7cal-adm-{}", uuid::Uuid::new_v4());
        seed_calendar(&pool, &fb, B, B, false).await;

        let body = read(&db, Some(ADMIN), &fb).await.expect("admin read ok");
        assert!(body.to_string().contains(&fb), "管理员必须可读他人日历");

        assert!(
            matches!(read(&db, None, &fb).await, Err(AppError::Unauthorized)),
            "无会话必须 401（门在读库前）"
        );

        let missing = format!("b7cal-missing-{}", uuid::Uuid::new_v4());
        let body = read(&db, Some(A), &missing)
            .await
            .expect("missing keeps error body");
        assert!(
            body.to_string().contains("not found"),
            "不存在的行保留原 error body"
        );

        cleanup(&pool, &fb).await;
    }
}
