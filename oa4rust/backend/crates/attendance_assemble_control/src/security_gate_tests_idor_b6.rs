// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 写 IDOR 批次 6·attendance：`attendanceappealInfo_appeal_id`（POST/PUT 同 handler）。
//!
//! 归属谓词进 `UPDATE ... WHERE`：申请人（`person_id` / `creator`）本人或管理员；
//! 非属主 403 且**不落笔**，双向都断言。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Json, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use serde_json::json;
    use shared::error::AppError;

    const A: &str = "r6b6a-a";
    const B: &str = "r6b6a-b";
    const ADMIN: &str = "r6b6a-admin";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b6a conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b6a fixture sql");
    }

    async fn seed_admin(pool: &Pool) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6b6a-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6B6A Admin', 'r6b6a-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6b6a-admin-role', 'r6b6a-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string()],
        )
        .await;
    }

    async fn seed_appeal(pool: &Pool, id: &str, person: &str) {
        exec(
            pool,
            "INSERT INTO x_attendance_appeal_info (id, person_id, creator, appeal_status) \
             VALUES ($1, $2, $2, 'pending')",
            vec![id.to_string(), person.to_string()],
        )
        .await;
    }

    async fn appeal_status(pool: &Pool, id: &str) -> Option<String> {
        let client = pool.get().await.expect("r6b6a conn");
        client
            .query_opt(
                "SELECT appeal_status FROM x_attendance_appeal_info WHERE id = $1",
                &[&id.to_string()],
            )
            .await
            .expect("r6b6a status read")
            .map(|r| {
                r.get::<_, Option<String>>("appeal_status")
                    .unwrap_or_default()
            })
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_attendance_appeal_info WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    async fn send(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
        status: &str,
    ) -> Result<(), AppError> {
        crate::attendanceappealInfo_appeal_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
            Json(json!({ "status": status })),
        )
        .await
        .map(|_| ())
    }

    #[tokio::test]
    async fn b6_appeal_both_directions_and_write() {
        let Some(pool) = pool().await else {
            return;
        };
        let fa = format!("r6b6a-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b6a-fb-{}", uuid::Uuid::new_v4());
        seed_appeal(&pool, &fa, A).await;
        seed_appeal(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            assert!(
                matches!(
                    send(&pool, Some(who), target, "approved").await,
                    Err(AppError::Forbidden)
                ),
                "{who} 处理 {target} 必须 403（双向）"
            );
        }
        assert_eq!(
            appeal_status(&pool, &fb).await.as_deref(),
            Some("pending"),
            "非属主必须一行不落"
        );

        // 本人处理自己的申诉 → 200 且真落库（写后必读）
        assert!(send(&pool, Some(A), &fa, "approved").await.is_ok());
        assert_eq!(
            appeal_status(&pool, &fa).await.as_deref(),
            Some("approved"),
            "本人处理必须真落库"
        );

        for id in [&fa, &fb] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b6_appeal_admin_session_and_missing() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let fa = format!("r6b6a-adm-{}", uuid::Uuid::new_v4());
        seed_appeal(&pool, &fa, A).await;

        assert!(send(&pool, Some(ADMIN), &fa, "approved").await.is_ok());
        assert_eq!(appeal_status(&pool, &fa).await.as_deref(), Some("approved"));

        assert!(
            matches!(
                send(&pool, None, &fa, "approved").await,
                Err(AppError::Unauthorized)
            ),
            "无会话必须 401"
        );

        let missing = format!("r6b6a-missing-{}", uuid::Uuid::new_v4());
        assert!(send(&pool, Some(A), &missing, "approved").await.is_ok());

        cleanup(&pool, &fa).await;
    }
}
