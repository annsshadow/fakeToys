// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b4（`cms_core_express::content_detail`）。
//!
//! 口径 `cms §3`：`x_cms_content.status = 'published'` 全员（含匿名）可读；
//! 其余状态仅 `creator` 本人与管理员。未知 id 保持 `NotFound`。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a4-cnt-a";
    const B: &str = "r6a4-cnt-b";
    const ADMIN: &str = "r6a4-admin-cnt";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a4-cnt conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a4-cnt fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a4-admin-role-cnt','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A4 Cnt Admin', 'r6a4-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a4-admin-role-cnt', 'r6a4-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn content_detail(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        crate::content_detail(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b4_content_reads_published_or_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN).await;

        let da = format!("r6a4-cnt-draft-a-{}", uuid::Uuid::new_v4());
        let db_id = format!("r6a4-cnt-draft-b-{}", uuid::Uuid::new_v4());
        let pa = format!("r6a4-cnt-pub-a-{}", uuid::Uuid::new_v4());
        for (id, creator, status, title) in [
            (&da, A, "draft", "r6a4-canary-cnt-draft-a"),
            (&db_id, B, "draft", "r6a4-canary-cnt-draft-b"),
            (&pa, A, "published", "r6a4-canary-cnt-pub-a"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_cms_content (id, title, category_id, status, content, creator) \
                 VALUES ($1, $2, 'r6a4-cat', $3, 'r6a4-body', $4)",
                vec![
                    id.to_string(),
                    title.to_string(),
                    status.to_string(),
                    creator.to_string(),
                ],
            )
            .await;
        }

        for (who, target) in [(B, da.as_str()), (A, db_id.as_str())] {
            assert!(
                matches!(
                    content_detail(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target}（草稿）必须 403"
            );
        }
        assert!(
            matches!(
                content_detail(&pool, None, &da).await,
                Err(AppError::Forbidden)
            ),
            "匿名读草稿必须 403"
        );

        let own = content_detail(&pool, Some(A), &da)
            .await
            .expect("作者读草稿");
        assert!(
            own.contains("r6a4-canary-cnt-draft-a"),
            "作者读草稿必须返回 title，实际: {own}"
        );
        assert!(
            content_detail(&pool, Some(ADMIN), &da).await.is_ok(),
            "管理员必须可读他人草稿"
        );

        for actor in [None, Some(B)] {
            let body = content_detail(&pool, actor, &pa)
                .await
                .expect("published 公开可读");
            assert!(
                body.contains("r6a4-canary-cnt-pub-a"),
                "published 内容必须返回 title，实际: {body}"
            );
        }

        assert!(
            matches!(
                content_detail(&pool, Some(A), "r6a4-cnt-missing").await,
                Err(AppError::NotFound)
            ),
            "未知 id 必须 NotFound"
        );

        for id in [&da, &db_id, &pa] {
            exec(
                &pool,
                "DELETE FROM x_cms_content WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
