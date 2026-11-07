// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b3（`cms_assemble_control` 六条读端点）。
//!
//! 口径 `cms §3`：`x_cms_appinfo` / `x_cms_categoryinfo` 按 `creator` 本人与管理员；
//! `x_cms_commend` 无 `creator` 列，唯一人员列 `person_id` 即归属列（台账已登记该映射）。
//! 三张表都没有 `published` 概念，因此不存在匿名可读分支。每条都验双向拒绝、
//! 本人/管理员正控制，正控制必须真的返回 canary（不接受空体）。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a3-a";
    const B: &str = "r6a3-b";
    // 三个用例并行跑，管理员种子用各自独立的人员行，避免 unique_id 唯一约束在
    // 「ON CONFLICT (id)」仲裁器之外并发冲突。
    const ADMIN_APP: &str = "r6a3-admin-app";
    const ADMIN_CAT: &str = "r6a3-admin-cat";
    const ADMIN_COMMEND: &str = "r6a3-admin-commend";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a3 conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6a3 fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a3-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A3 Admin', 'r6a3-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a3-admin-role', 'r6a3-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn application(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::application_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn appinfo(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::appinfo_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn appinfo_control(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        crate::appinfo_id_control(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn category(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::categoryinfo_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn category_control(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        crate::categoryinfo_id_control(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn commend(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::commend_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b3_cms_appinfo_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN_APP).await;
        let aa = format!("r6a3-app-a-{}", uuid::Uuid::new_v4());
        let ab = format!("r6a3-app-b-{}", uuid::Uuid::new_v4());
        for (id, creator, alias) in [(&aa, A, "r6a3-canary-app-a"), (&ab, B, "r6a3-canary-app-b")] {
            exec(
                &pool,
                "INSERT INTO x_cms_appinfo (id, app_type, alias, creator) \
                 VALUES ($1, 'r6a3-type', $2, $3)",
                vec![id.to_string(), alias.to_string(), creator.to_string()],
            )
            .await;
        }

        // 双向 + 匿名：三个端点共用同一判据
        for (who, target) in [(B, aa.as_str()), (A, ab.as_str())] {
            assert!(
                matches!(
                    application(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target}（application）必须 403"
            );
            assert!(
                matches!(
                    appinfo(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target}（appinfo）必须 403"
            );
            assert!(
                matches!(
                    appinfo_control(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target}（appinfo control）必须 403"
            );
        }
        for (label, gate) in [
            ("application", application(&pool, None, &aa).await),
            ("appinfo", appinfo(&pool, None, &aa).await),
            ("appinfo control", appinfo_control(&pool, None, &aa).await),
        ] {
            assert!(
                matches!(gate, Err(AppError::Forbidden)),
                "匿名读 {label} 必须 403"
            );
        }

        // 正控制：本人读自己的行，端点必须真的返回 alias
        for body in [
            application(&pool, Some(A), &aa)
                .await
                .expect("本人读 application"),
            appinfo(&pool, Some(A), &aa).await.expect("本人读 appinfo"),
            appinfo_control(&pool, Some(A), &aa)
                .await
                .expect("本人读 appinfo control"),
        ] {
            assert!(
                body.contains("r6a3-canary-app-a"),
                "本人读 appinfo 必须返回 alias，实际: {body}"
            );
        }

        // 管理员全部可读
        for body in [
            application(&pool, Some(ADMIN_APP), &aa).await,
            appinfo(&pool, Some(ADMIN_APP), &aa).await,
            appinfo_control(&pool, Some(ADMIN_APP), &aa).await,
        ] {
            assert!(body.is_ok(), "管理员必须可读他人 appinfo");
        }

        for id in [&aa, &ab] {
            exec(
                &pool,
                "DELETE FROM x_cms_appinfo WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b3_cms_categoryinfo_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN_CAT).await;
        let ca = format!("r6a3-cat-a-{}", uuid::Uuid::new_v4());
        let cb = format!("r6a3-cat-b-{}", uuid::Uuid::new_v4());
        for (id, creator, name) in [(&ca, A, "r6a3-canary-cat-a"), (&cb, B, "r6a3-canary-cat-b")] {
            exec(
                &pool,
                "INSERT INTO x_cms_categoryinfo (id, name, app_id, creator) \
                 VALUES ($1, $2, 'r6a3-app', $3)",
                vec![id.to_string(), name.to_string(), creator.to_string()],
            )
            .await;
        }

        for (who, target) in [(B, ca.as_str()), (A, cb.as_str())] {
            assert!(
                matches!(
                    category(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target}（categoryinfo）必须 403"
            );
            assert!(
                matches!(
                    category_control(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target}（categoryinfo control）必须 403"
            );
        }
        for (label, gate) in [
            ("categoryinfo", category(&pool, None, &ca).await),
            (
                "categoryinfo control",
                category_control(&pool, None, &ca).await,
            ),
        ] {
            assert!(
                matches!(gate, Err(AppError::Forbidden)),
                "匿名读 {label} 必须 403"
            );
        }

        for body in [
            category(&pool, Some(A), &ca)
                .await
                .expect("本人读 categoryinfo"),
            category_control(&pool, Some(A), &ca)
                .await
                .expect("本人读 categoryinfo control"),
        ] {
            assert!(
                body.contains("r6a3-canary-cat-a"),
                "本人读 categoryinfo 必须返回 name，实际: {body}"
            );
        }

        for body in [
            category(&pool, Some(ADMIN_CAT), &ca).await,
            category_control(&pool, Some(ADMIN_CAT), &ca).await,
        ] {
            assert!(body.is_ok(), "管理员必须可读他人 categoryinfo");
        }

        for id in [&ca, &cb] {
            exec(
                &pool,
                "DELETE FROM x_cms_categoryinfo WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b3_cms_commend_reads_person_id_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN_COMMEND).await;
        let ma = format!("r6a3-cmd-a-{}", uuid::Uuid::new_v4());
        let mb = format!("r6a3-cmd-b-{}", uuid::Uuid::new_v4());
        for (id, person_id) in [(&ma, A), (&mb, B)] {
            exec(
                &pool,
                "INSERT INTO x_cms_commend (id, doc_id, person_id) \
                 VALUES ($1, 'r6a3-doc', $2)",
                vec![id.to_string(), person_id.to_string()],
            )
            .await;
        }

        // `x_cms_commend` 无 creator：归属列映射到 person_id
        for (who, target) in [(B, ma.as_str()), (A, mb.as_str())] {
            assert!(
                matches!(
                    commend(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 认可（person_id）必须 403"
            );
        }
        assert!(
            matches!(commend(&pool, None, &ma).await, Err(AppError::Forbidden)),
            "匿名读认可必须 403"
        );

        // 正控制：本人读到的 personId 必须等于会话人
        let own = commend(&pool, Some(A), &ma).await.expect("本人读认可");
        assert!(
            own.contains("\"personId\":\"r6a3-a\""),
            "本人读认可必须返回 personId，实际: {own}"
        );
        assert!(
            commend(&pool, Some(ADMIN_COMMEND), &ma).await.is_ok(),
            "管理员必须可读他人认可"
        );

        for id in [&ma, &mb] {
            exec(
                &pool,
                "DELETE FROM x_cms_commend WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
