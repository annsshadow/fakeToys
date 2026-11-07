// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b1（`cms_assemble_control` 六条读端点）。
//!
//! 口径 `cms §3`：`published` 全员（含匿名）可读；`draft` 仅 `creator`/`author_id` 本人；
//! 附件按 `creator`、评论按「评论作者 OR 可读文档 OR 管理员」。每条都验双向拒绝、
//! 本人/管理员正控制，且正控制必须真的返回 canary（不接受空体）。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a1-a";
    const B: &str = "r6a1-b";
    const ADMIN: &str = "r6a1-admin";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a1 conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6a1 fixture sql");
    }

    async fn seed_admin(pool: &Pool) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a1-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A1 Admin', 'r6a1-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a1-admin-role', 'r6a1-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string()],
        )
        .await;
    }

    async fn view(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::anonymous_document_id_view(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn persons(pool: &Pool, actor: &str, id: &str) -> Result<String, AppError> {
        crate::document_persons_u3(
            Extension(pool.clone()),
            Extension(shared::testing::test_session(actor)),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn file_meta(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::file_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn file_content(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::file_id_content(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn file_download(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::file_id_download(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn comment(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::comment_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b1_cms_document_view_published_public_and_draft_owner() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let draft = format!("r6a1-doc-draft-{}", uuid::Uuid::new_v4());
        let draft_b = format!("r6a1-doc-draft-b-{}", uuid::Uuid::new_v4());
        let pubdoc = format!("r6a1-doc-pub-{}", uuid::Uuid::new_v4());
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, content, author_id, creator) \
             VALUES ($1, 'r6a1-title', 'r6a1-canary-doc-a', $2, $2)",
            vec![draft.clone(), A.to_string()],
        )
        .await;
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, content, author_id, creator) \
             VALUES ($1, 'r6a1-title', 'r6a1-canary-doc-b', $2, $2)",
            vec![draft_b.clone(), B.to_string()],
        )
        .await;
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, content, author_id, creator, status) \
             VALUES ($1, 'r6a1-title', 'r6a1-canary-doc-pub', $2, $2, 'published')",
            vec![pubdoc.clone(), A.to_string()],
        )
        .await;

        // 双向：他人草稿一律 403（含匿名）
        assert!(
            matches!(view(&pool, Some(B), &draft).await, Err(AppError::Forbidden)),
            "B 读 A 的草稿必须 403"
        );
        assert!(
            matches!(
                view(&pool, Some(A), &draft_b).await,
                Err(AppError::Forbidden)
            ),
            "A 读 B 的草稿必须 403"
        );
        assert!(
            matches!(view(&pool, None, &draft).await, Err(AppError::Forbidden)),
            "匿名读草稿必须 403"
        );

        // 正控制：本人读自己草稿，必须真的返回正文
        let own = view(&pool, Some(A), &draft).await.expect("本人读草稿");
        assert!(
            own.contains("r6a1-canary-doc-a"),
            "本人读草稿必须返回正文，实际: {own}"
        );
        assert!(
            persons(&pool, A, &draft).await.is_ok(),
            "本人读文档人员必须放行"
        );
        assert!(
            matches!(persons(&pool, B, &draft).await, Err(AppError::Forbidden)),
            "他人读文档人员必须 403"
        );

        // published 公开：匿名与任意登录用户都可读
        let anon_pub = view(&pool, None, &pubdoc).await.expect("匿名读已发布");
        assert!(
            anon_pub.contains("r6a1-canary-doc-pub"),
            "匿名读已发布必须返回正文，实际: {anon_pub}"
        );
        assert!(
            view(&pool, Some(B), &pubdoc).await.is_ok(),
            "登录用户读已发布"
        );

        // 管理员全部可读
        assert!(
            view(&pool, Some(ADMIN), &draft).await.is_ok(),
            "管理员必须可读他人草稿"
        );
        assert!(
            persons(&pool, ADMIN, &draft).await.is_ok(),
            "管理员必须可读文档人员"
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
    async fn b1_cms_file_reads_owner_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let fa = format!("r6a1-file-a-{}", uuid::Uuid::new_v4());
        let fb = format!("r6a1-file-b-{}", uuid::Uuid::new_v4());
        for (id, creator, body) in [
            (&fa, A, "r6a1-file-a-content"),
            (&fb, B, "r6a1-file-b-content"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_cms_file (id, name, size, content_type, content_base64, creator) \
                 VALUES ($1, 'r6a1-name', 1, 'text/plain', $2, $3)",
                vec![id.to_string(), body.to_string(), creator.to_string()],
            )
            .await;
        }

        // 双向拒绝：元数据、正文、下载三个端点都要 403
        for (who, target) in [(B, fa.as_str()), (A, fb.as_str())] {
            assert!(
                matches!(
                    file_meta(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 元数据必须 403"
            );
            assert!(
                matches!(
                    file_content(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 正文必须 403"
            );
            assert!(
                matches!(
                    file_download(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 下载 {target} 必须 403"
            );
        }
        assert!(
            matches!(file_meta(&pool, None, &fa).await, Err(AppError::Forbidden)),
            "匿名读文件元数据必须 403"
        );
        assert!(
            matches!(
                file_content(&pool, None, &fa).await,
                Err(AppError::Forbidden)
            ),
            "匿名读文件正文必须 403"
        );
        assert!(
            matches!(
                file_download(&pool, None, &fa).await,
                Err(AppError::Forbidden)
            ),
            "匿名下载文件必须 403"
        );

        // 正控制：本人三个端点都返回 canary；管理员放行
        let meta = file_meta(&pool, Some(A), &fa).await.expect("本人读元数据");
        assert!(
            meta.contains("r6a1-name"),
            "本人元数据必须真返回，实际: {meta}"
        );
        let content = file_content(&pool, Some(A), &fa).await.expect("本人读正文");
        assert!(
            content.contains("r6a1-file-a-content"),
            "本人正文必须真返回，实际: {content}"
        );
        let download = file_download(&pool, Some(A), &fa).await.expect("本人下载");
        assert!(
            download.contains("r6a1-file-a-content"),
            "本人下载必须真返回，实际: {download}"
        );
        assert!(
            file_meta(&pool, Some(ADMIN), &fa).await.is_ok(),
            "管理员必须可读他人文件"
        );

        for id in [&fa, &fb] {
            exec(
                &pool,
                "DELETE FROM x_cms_file WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b1_cms_comment_reads_document_acl() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let draft = format!("r6a1-doc-c-draft-{}", uuid::Uuid::new_v4());
        let draft_b = format!("r6a1-doc-c-draft-b-{}", uuid::Uuid::new_v4());
        let pubdoc = format!("r6a1-doc-c-pub-{}", uuid::Uuid::new_v4());
        let c_own = format!("r6a1-comment-own-{}", uuid::Uuid::new_v4());
        let c_other = format!("r6a1-comment-other-{}", uuid::Uuid::new_v4());
        let c_guest = format!("r6a1-comment-guest-{}", uuid::Uuid::new_v4());
        let c_pub = format!("r6a1-comment-pub-{}", uuid::Uuid::new_v4());
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, content, author_id, creator) \
             VALUES ($1, 'r6a1-title', 'r6a1-canary', $2, $2)",
            vec![draft.clone(), A.to_string()],
        )
        .await;
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, content, author_id, creator) \
             VALUES ($1, 'r6a1-title', 'r6a1-canary', $2, $2)",
            vec![draft_b.clone(), B.to_string()],
        )
        .await;
        exec(
            &pool,
            "INSERT INTO x_cms_data_document (id, title, content, author_id, creator, status) \
             VALUES ($1, 'r6a1-title', 'r6a1-canary', $2, $2, 'published')",
            vec![pubdoc.clone(), A.to_string()],
        )
        .await;
        // c_own：A 在 A 的草稿上；c_other：B 在 B 自己的草稿上；
        // c_guest：B 在 A 的草稿上；c_pub：A 在已发布文档上
        for (id, doc, person) in [
            (&c_own, draft.as_str(), A),
            (&c_other, draft_b.as_str(), B),
            (&c_guest, draft.as_str(), B),
            (&c_pub, pubdoc.as_str(), A),
        ] {
            exec(
                &pool,
                "INSERT INTO x_cms_comment (id, doc_id, person_id, content) \
                 VALUES ($1, $2, $3, 'r6a1-comment-body')",
                vec![id.to_string(), doc.to_string(), person.to_string()],
            )
            .await;
        }

        // 双向：既非评论作者、也读不到其所在草稿 → 403
        assert!(
            matches!(
                comment(&pool, Some(B), &c_own).await,
                Err(AppError::Forbidden)
            ),
            "B 读 A 在草稿上的评论必须 403"
        );
        assert!(
            matches!(
                comment(&pool, Some(A), &c_other).await,
                Err(AppError::Forbidden)
            ),
            "A 读 B 自己草稿上的评论必须 403"
        );
        assert!(
            matches!(comment(&pool, None, &c_own).await, Err(AppError::Forbidden)),
            "匿名读评论必须 403"
        );

        // 正控制：评论作者可读自己；草稿作者可读其文档下他人评论；已发布文档的评论登录可读
        for (who, id) in [
            (A, &c_own),
            (B, &c_other),
            (A, &c_guest),
            (B, &c_guest),
            (B, &c_pub),
        ] {
            let body = comment(&pool, Some(who), id).await.expect("评论可读");
            assert!(
                body.contains("r6a1-comment-body"),
                "{who} 读 {id} 必须返回评论正文，实际: {body}"
            );
        }
        // 管理员放行
        assert!(
            comment(&pool, Some(ADMIN), &c_own).await.is_ok(),
            "管理员必须可读他人评论"
        );

        for id in [&c_own, &c_other, &c_guest, &c_pub] {
            exec(
                &pool,
                "DELETE FROM x_cms_comment WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
        for id in [&draft, &draft_b, &pubdoc] {
            exec(
                &pool,
                "DELETE FROM x_cms_data_document WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
