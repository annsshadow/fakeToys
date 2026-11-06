// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第五批·cms 装配控制台）：三个「POST 动作别名但实际是读」的端点。
//!
//! `appinfo_appId_icon_size_size` / `categoryinfo_id_execute_projection` 按 `creator` 收口：
//! 非属主 403、本人 200、管理员 200。`data_document_id_array_data` 按 E13 口径：
//! 已发布公开（含匿名），登录用户可读本人（`creator`/`author_id`），管理员可读全部。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6b5c-a";
    const B: &str = "r6b5c-b";
    const ADMIN: &str = "r6b5c-admin";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b5c conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b5c fixture sql");
    }

    async fn seed_admin(pool: &Pool) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6b5c-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6B5C Admin', 'r6b5c-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6b5c-admin-role', 'r6b5c-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string()],
        )
        .await;
    }

    async fn icon(pool: &Pool, actor: Option<&str>, id: &str) -> Result<(), AppError> {
        crate::appinfo_appId_icon_size_size(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path((id.to_string(), "64".to_string())),
        )
        .await
        .map(|_| ())
    }

    async fn projection(pool: &Pool, actor: Option<&str>, id: &str) -> Result<(), AppError> {
        crate::categoryinfo_id_execute_projection(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|_| ())
    }

    async fn array_data(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::data_document_id_array_data(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b5_cms_appinfo_icon_both_directions() {
        let Some(pool) = pool().await else {
            return;
        };
        let fa = format!("r6b5c-app-a-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b5c-app-b-{}", uuid::Uuid::new_v4());
        for (id, creator) in [(&fa, A), (&fb, B)] {
            exec(
                &pool,
                "INSERT INTO x_cms_appinfo (id, app_type, creator) VALUES ($1, 'r6b5seed', $2)",
                vec![id.to_string(), creator.to_string()],
            )
            .await;
        }

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            assert!(
                matches!(
                    icon(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403（双向）"
            );
        }
        assert!(icon(&pool, Some(A), &fa).await.is_ok(), "本人必须可读");
        assert!(
            icon(&pool, None, &fb).await.is_err(),
            "无会话必须 401（门在查询前）"
        );

        for id in [&fa, &fb] {
            exec(
                &pool,
                "DELETE FROM x_cms_appinfo WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b5_cms_categoryinfo_projection_both_directions() {
        let Some(pool) = pool().await else {
            return;
        };
        let fa = format!("r6b5c-cat-a-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b5c-cat-b-{}", uuid::Uuid::new_v4());
        for (id, creator) in [(&fa, A), (&fb, B)] {
            exec(
                &pool,
                "INSERT INTO x_cms_categoryinfo (id, name, creator) VALUES ($1, 'r6b5seed', $2)",
                vec![id.to_string(), creator.to_string()],
            )
            .await;
        }

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            assert!(
                matches!(
                    projection(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 必须 403（双向）"
            );
        }
        assert!(
            projection(&pool, Some(A), &fa).await.is_ok(),
            "本人必须可读"
        );

        for id in [&fa, &fb] {
            exec(
                &pool,
                "DELETE FROM x_cms_categoryinfo WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b5_cms_data_document_array_data_public_and_owned() {
        let Some(pool) = pool().await else {
            return;
        };
        let draft = format!("r6b5c-doc-draft-{}", uuid::Uuid::new_v4());
        let pubdoc = format!("r6b5c-doc-pub-{}", uuid::Uuid::new_v4());
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, author_id, creator) \
             VALUES ($1, 'r6b5seed', $2, $2)",
            vec![draft.to_string(), A.to_string()],
        )
        .await;
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, author_id, creator, status) \
             VALUES ($1, 'r6b5seed', $2, $2, 'published')",
            vec![pubdoc.to_string(), A.to_string()],
        )
        .await;

        // 双向：B 读 A 的草稿 403；A 读 B 的草稿 403
        let draft_b = format!("r6b5c-doc-b-{}", uuid::Uuid::new_v4());
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, author_id, creator) \
             VALUES ($1, 'r6b5seed', $2, $2)",
            vec![draft_b.to_string(), B.to_string()],
        )
        .await;
        assert!(
            matches!(
                array_data(&pool, Some(B), &draft).await,
                Err(AppError::Forbidden)
            ),
            "B 读 A 的草稿必须 403"
        );
        assert!(
            matches!(
                array_data(&pool, Some(A), &draft_b).await,
                Err(AppError::Forbidden)
            ),
            "A 读 B 的草稿必须 403"
        );

        // 本人读自己草稿 200；匿名读草稿 403；匿名读已发布 200
        assert!(array_data(&pool, Some(A), &draft).await.is_ok());
        assert!(
            matches!(
                array_data(&pool, None, &draft).await,
                Err(AppError::Forbidden)
            ),
            "匿名读草稿必须 403"
        );
        assert!(
            array_data(&pool, None, &pubdoc).await.is_ok(),
            "匿名读已发布必须放行"
        );

        for id in [&draft, &draft_b, &pubdoc] {
            exec(
                &pool,
                "DELETE FROM x_cms_data_document WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b5_cms_reads_admin_allowed() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let app_id = format!("r6b5c-adm-app-{}", uuid::Uuid::new_v4());
        let cat_id = format!("r6b5c-adm-cat-{}", uuid::Uuid::new_v4());
        let doc_id = format!("r6b5c-adm-doc-{}", uuid::Uuid::new_v4());
        exec(
            &pool,
            "INSERT INTO x_cms_appinfo (id, app_type, creator) VALUES ($1, 'r6b5seed', $2)",
            vec![app_id.clone(), A.to_string()],
        )
        .await;
        exec(
            &pool,
            "INSERT INTO x_cms_categoryinfo (id, name, creator) VALUES ($1, 'r6b5seed', $2)",
            vec![cat_id.clone(), A.to_string()],
        )
        .await;
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, author_id, creator) \
             VALUES ($1, 'r6b5seed', $2, $2)",
            vec![doc_id.clone(), A.to_string()],
        )
        .await;

        assert!(icon(&pool, Some(ADMIN), &app_id).await.is_ok());
        assert!(projection(&pool, Some(ADMIN), &cat_id).await.is_ok());
        assert!(array_data(&pool, Some(ADMIN), &doc_id).await.is_ok());

        exec(
            &pool,
            "DELETE FROM x_cms_appinfo WHERE id = $1",
            vec![app_id],
        )
        .await;
        exec(
            &pool,
            "DELETE FROM x_cms_categoryinfo WHERE id = $1",
            vec![cat_id],
        )
        .await;
        exec(
            &pool,
            "DELETE FROM x_cms_data_document WHERE id = $1",
            vec![doc_id],
        )
        .await;
    }
}
