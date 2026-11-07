// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b4（`cms_core_entity::article_get`）。
//!
//! 口径 `cms §3`：`x_cms_article.status = 'published'` 全员（含匿名）可读；
//! 其余状态（草稿/空）仅 `author_id` 本人与管理员。双向拒绝 + 匿名草稿拒绝 +
//! 本人/管理员/匿名公开三组正控制，正控制必须真的返回 canary。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use sea_orm::DatabaseConnection;
    use shared::error::AppError;

    const A: &str = "r6a4-art-a";
    const B: &str = "r6a4-art-b";
    const ADMIN: &str = "r6a4-admin-art";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a4-art conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client
            .execute(sql, &refs)
            .await
            .expect("r6a4-art fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a4-admin-role-art','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A4 Art Admin', 'r6a4-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a4-admin-role-art', 'r6a4-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn article_get(
        pool: &Pool,
        db: &DatabaseConnection,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        crate::article_get(
            Extension(db.clone()),
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b4_article_reads_published_or_author_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("r6a4-art sea-orm");
        seed_admin(&pool, ADMIN).await;

        let da = format!("r6a4-art-draft-a-{}", uuid::Uuid::new_v4());
        let db_id = format!("r6a4-art-draft-b-{}", uuid::Uuid::new_v4());
        let pa = format!("r6a4-art-pub-a-{}", uuid::Uuid::new_v4());
        for (id, author, status, title) in [
            (&da, A, "draft", "r6a4-canary-art-draft-a"),
            (&db_id, B, "draft", "r6a4-canary-art-draft-b"),
            (&pa, A, "published", "r6a4-canary-art-pub-a"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_cms_article (id, category_id, title, author_id, status) \
                 VALUES ($1, 'r6a4-cat', $2, $3, $4)",
                vec![
                    id.to_string(),
                    title.to_string(),
                    author.to_string(),
                    status.to_string(),
                ],
            )
            .await;
        }

        // 双向：他人草稿一律 403
        for (who, target) in [(B, da.as_str()), (A, db_id.as_str())] {
            assert!(
                matches!(
                    article_get(&pool, &db, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target}（草稿）必须 403"
            );
        }
        // 匿名读草稿同样 403
        assert!(
            matches!(
                article_get(&pool, &db, None, &da).await,
                Err(AppError::Forbidden)
            ),
            "匿名读草稿必须 403"
        );

        // 正控制：作者读自己的草稿，必须真的返回 title
        let own = article_get(&pool, &db, Some(A), &da)
            .await
            .expect("作者读草稿");
        assert!(
            own.contains("r6a4-canary-art-draft-a"),
            "作者读草稿必须返回 title，实际: {own}"
        );
        // 管理员可读他人草稿
        assert!(
            article_get(&pool, &db, Some(ADMIN), &da).await.is_ok(),
            "管理员必须可读他人草稿"
        );

        // published：匿名与任意登录用户都可读
        for actor in [None, Some(B)] {
            let body = article_get(&pool, &db, actor, &pa)
                .await
                .expect("published 公开可读");
            assert!(
                body.contains("r6a4-canary-art-pub-a"),
                "published 文章必须返回 title，实际: {body}"
            );
        }

        // 不存在：保留原 not found 文案
        let missing = article_get(&pool, &db, Some(A), "r6a4-art-missing")
            .await
            .expect("未知 id 保持 200 error 文案");
        assert!(
            missing.contains("article not found"),
            "未知 id 必须返回 article not found，实际: {missing}"
        );

        for id in [&da, &db_id, &pa] {
            exec(
                &pool,
                "DELETE FROM x_cms_article WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
