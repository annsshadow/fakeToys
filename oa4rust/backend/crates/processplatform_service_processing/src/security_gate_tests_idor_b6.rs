// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 写 IDOR 批次 6·processplatform：`task_id_will`（`x_task`，POST/GET 同 handler）与
//! `attachment_edit_id_text`（`x_attachment`）。
//!
//! 归属谓词进 `SELECT ... WHERE`：task 取负责人（`x_task.person`）或发起人（`x_work.creator`）
//! 两列 OR；attachment 取 `creator`；管理员兜底，空归属只允许管理员。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6b6p-a";
    const B: &str = "r6b6p-b";
    const ADMIN: &str = "r6b6p-admin";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b6p conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b6p fixture sql");
    }

    async fn seed_admin(pool: &Pool) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6b6p-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6B6P Admin', 'r6b6p-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6b6p-admin-role', 'r6b6p-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string()],
        )
        .await;
    }

    async fn seed_task(pool: &Pool, task: &str, work: &str, person: &str, creator: &str) {
        exec(
            pool,
            "INSERT INTO x_work (id, title, process, creator) \
             VALUES ($1, 'r6b6seed', 'r6b6seed', $2)",
            vec![work.to_string(), creator.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO x_task (id, title, work, person, task_status) \
             VALUES ($1, 'r6b6seed', $2, $3, 'wait')",
            vec![task.to_string(), work.to_string(), person.to_string()],
        )
        .await;
    }

    async fn cleanup_task(pool: &Pool, task: &str, work: &str) {
        exec(
            pool,
            "DELETE FROM x_task WHERE id = $1",
            vec![task.to_string()],
        )
        .await;
        exec(
            pool,
            "DELETE FROM x_work WHERE id = $1",
            vec![work.to_string()],
        )
        .await;
    }

    async fn seed_attachment(pool: &Pool, id: &str, work: &str, creator: &str) {
        exec(
            pool,
            "INSERT INTO x_attachment (id, work_id, name, content, creator) \
             VALUES ($1, $2, 'r6b6seed', 'r6b6seed', $3)",
            vec![id.to_string(), work.to_string(), creator.to_string()],
        )
        .await;
    }

    async fn cleanup_attachment(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_attachment WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    async fn task_will(pool: &Pool, actor: Option<&str>, id: &str) -> Result<(), AppError> {
        crate::task_id_will(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|_| ())
    }

    async fn attachment(pool: &Pool, actor: Option<&str>, id: &str) -> Result<(), AppError> {
        crate::attachment_edit_id_text(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|_| ())
    }

    #[tokio::test]
    async fn b6_task_will_both_directions() {
        let Some(pool) = pool().await else {
            return;
        };
        let (ta, wa) = (
            format!("r6b6p-ta-{}", uuid::Uuid::new_v4()),
            format!("r6b6p-wa-{}", uuid::Uuid::new_v4()),
        );
        let (tb, wb) = (
            format!("r6b6p-tb-{}", uuid::Uuid::new_v4()),
            format!("r6b6p-wb-{}", uuid::Uuid::new_v4()),
        );
        // A 是任务 a 的负责人、work b 的发起人；B 反之（覆盖 person / creator 两列）。
        seed_task(&pool, &ta, &wa, A, A).await;
        seed_task(&pool, &tb, &wb, B, B).await;
        // 交叉行：work 发起人是 A、任务负责人是 B —— 两列各自必须能读。
        let (tc, wc) = (
            format!("r6b6p-tc-{}", uuid::Uuid::new_v4()),
            format!("r6b6p-wc-{}", uuid::Uuid::new_v4()),
        );
        seed_task(&pool, &tc, &wc, B, A).await;

        for (who, target) in [(A, tb.as_str()), (B, ta.as_str())] {
            assert!(
                matches!(
                    task_will(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403（双向）"
            );
        }

        assert!(
            task_will(&pool, Some(A), &ta).await.is_ok(),
            "负责人必须可读"
        );
        assert!(
            task_will(&pool, Some(B), &tb).await.is_ok(),
            "负责人必须可读"
        );
        // 两列 OR：work 发起人与任务负责人各自都能读同一行
        assert!(
            task_will(&pool, Some(A), &tc).await.is_ok(),
            "work 发起人必须可读"
        );
        assert!(
            task_will(&pool, Some(B), &tc).await.is_ok(),
            "任务负责人必须可读"
        );

        let missing = format!("r6b6p-missing-{}", uuid::Uuid::new_v4());
        assert!(
            matches!(
                task_will(&pool, Some(A), &missing).await,
                Err(AppError::NotFound)
            ),
            "行不存在必须 404"
        );
        assert!(
            matches!(
                task_will(&pool, None, &ta).await,
                Err(AppError::Unauthorized)
            ),
            "无会话必须 401"
        );

        cleanup_task(&pool, &ta, &wa).await;
        cleanup_task(&pool, &tb, &wb).await;
        cleanup_task(&pool, &tc, &wc).await;
    }

    #[tokio::test]
    async fn b6_attachment_text_both_directions() {
        let Some(pool) = pool().await else {
            return;
        };
        let (fa, wa) = (
            format!("r6b6p-fa-{}", uuid::Uuid::new_v4()),
            format!("r6b6p-wa2-{}", uuid::Uuid::new_v4()),
        );
        let (fb, wb) = (
            format!("r6b6p-fb-{}", uuid::Uuid::new_v4()),
            format!("r6b6p-wb2-{}", uuid::Uuid::new_v4()),
        );
        exec(
            &pool,
            "INSERT INTO x_work (id, title, process, creator) \
             VALUES ($1, 'r6b6seed', 'r6b6seed', $2), ($3, 'r6b6seed', 'r6b6seed', $4)",
            vec![wa.clone(), A.to_string(), wb.clone(), B.to_string()],
        )
        .await;
        seed_attachment(&pool, &fa, &wa, A).await;
        seed_attachment(&pool, &fb, &wb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            assert!(
                matches!(
                    attachment(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403（双向）"
            );
        }
        assert!(
            attachment(&pool, Some(A), &fa).await.is_ok(),
            "本人必须可读"
        );

        cleanup_attachment(&pool, &fa).await;
        cleanup_attachment(&pool, &fb).await;
        exec(&pool, "DELETE FROM x_work WHERE id = $1", vec![wa]).await;
        exec(&pool, "DELETE FROM x_work WHERE id = $1", vec![wb]).await;
    }

    #[tokio::test]
    async fn b6_task_attachment_admin_allowed() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let (t, w) = (
            format!("r6b6p-tadm-{}", uuid::Uuid::new_v4()),
            format!("r6b6p-wadm-{}", uuid::Uuid::new_v4()),
        );
        seed_task(&pool, &t, &w, A, A).await;
        let fa = format!("r6b6p-fadm-{}", uuid::Uuid::new_v4());
        seed_attachment(&pool, &fa, &w, A).await;

        assert!(
            task_will(&pool, Some(ADMIN), &t).await.is_ok(),
            "管理员必须可读"
        );
        assert!(
            attachment(&pool, Some(ADMIN), &fa).await.is_ok(),
            "管理员必须可读"
        );

        cleanup_attachment(&pool, &fa).await;
        cleanup_task(&pool, &t, &w).await;
    }
}
