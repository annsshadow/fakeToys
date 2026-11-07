// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读越权分片 A / A-b2（`cms_assemble_control` 六条读端点）。
//!
//! 口径 `cms §3`：`x_cms_form` / `x_cms_form_v2` 有 `status` 列，`published` 全员
//! （含匿名）可读、`draft` 仅 `creator` 本人与管理员；`x_cms_script` 无 `status`，
//! 按 `creator` 本人与管理员。每条都验双向拒绝、本人/管理员正控制，正控制必须
//! 真的返回 canary（不接受空体）。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    const A: &str = "r6a2-a";
    const B: &str = "r6a2-b";
    // 三个用例并行跑，管理员种子用各自独立的人员行，避免 unique_id 唯一约束在
    // 「ON CONFLICT (id)」仲裁器之外并发冲突。
    const ADMIN_FORM: &str = "r6a2-admin-form";
    const ADMIN_FV: &str = "r6a2-admin-fv";
    const ADMIN_SCRIPT: &str = "r6a2-admin-script";

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6a2 conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6a2 fixture sql");
    }

    async fn seed_admin(pool: &Pool, admin: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6a2-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6A2 Admin', 'r6a2-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6a2-admin-role', 'r6a2-unit') ON CONFLICT DO NOTHING",
            vec![admin.to_string()],
        )
        .await;
    }

    async fn anon_form(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::anonymous_form_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn form(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::form_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn form_version(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::formversion_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn script(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::script_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn script_version(
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<String, AppError> {
        crate::scriptversion_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    async fn template_form(pool: &Pool, actor: Option<&str>, id: &str) -> Result<String, AppError> {
        crate::templateform_id(
            Extension(pool.clone()),
            actor.map(|p| Extension(shared::testing::test_session(p))),
            Path(id.to_string()),
        )
        .await
        .map(|json| serde_json::to_string(&json.0).unwrap_or_default())
    }

    #[tokio::test]
    async fn b2_cms_form_reads_published_public_and_draft_owner() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN_FORM).await;
        let fa = format!("r6a2-form-a-{}", uuid::Uuid::new_v4());
        let fb = format!("r6a2-form-b-{}", uuid::Uuid::new_v4());
        let fpub = format!("r6a2-form-pub-{}", uuid::Uuid::new_v4());
        for (id, creator, definition, status) in [
            (&fa, A, r#"{"name":"r6a2-canary-form-a"}"#, "draft"),
            (&fb, B, r#"{"name":"r6a2-canary-form-b"}"#, "draft"),
            (&fpub, A, r#"{"name":"r6a2-canary-form-pub"}"#, "published"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_cms_form (id, app_id, name, definition, status, creator) \
                 VALUES ($1, 'r6a2-app', 'r6a2-name', $2, $3, $4)",
                vec![
                    id.to_string(),
                    definition.to_string(),
                    status.to_string(),
                    creator.to_string(),
                ],
            )
            .await;
        }

        // 双向：他人草稿一律 403（含匿名），两个端点同判据
        for (who, target) in [(B, fa.as_str()), (A, fb.as_str())] {
            assert!(
                matches!(
                    anon_form(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 草稿（anonymous）必须 403"
            );
            assert!(
                matches!(
                    form(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 草稿（form_id）必须 403"
            );
        }
        assert!(
            matches!(anon_form(&pool, None, &fa).await, Err(AppError::Forbidden)),
            "匿名读草稿（anonymous）必须 403"
        );
        assert!(
            matches!(form(&pool, None, &fa).await, Err(AppError::Forbidden)),
            "匿名读草稿（form_id）必须 403"
        );

        // 正控制：本人读自己的草稿，两个端点都必须真的返回定义
        for body in [
            anon_form(&pool, Some(A), &fa)
                .await
                .expect("本人读草稿 anonymous"),
            form(&pool, Some(A), &fa).await.expect("本人读草稿 form_id"),
        ] {
            assert!(
                body.contains("r6a2-canary-form-a"),
                "本人读草稿必须返回定义，实际: {body}"
            );
        }

        // published 公开：匿名与任意登录用户都可读
        for body in [
            anon_form(&pool, None, &fpub).await.expect("匿名读已发布"),
            form(&pool, Some(B), &fpub).await.expect("登录读已发布"),
        ] {
            assert!(
                body.contains("r6a2-canary-form-pub"),
                "已发布表单必须返回定义，实际: {body}"
            );
        }

        // 管理员全部可读
        assert!(
            anon_form(&pool, Some(ADMIN_FORM), &fa).await.is_ok(),
            "管理员必须可读他人草稿（anonymous）"
        );
        assert!(
            form(&pool, Some(ADMIN_FORM), &fa).await.is_ok(),
            "管理员必须可读他人草稿（form_id）"
        );

        for id in [&fa, &fb, &fpub] {
            exec(
                &pool,
                "DELETE FROM x_cms_form WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b2_cms_form_v2_reads_published_public_and_draft_owner() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN_FV).await;
        let va = format!("r6a2-fv-a-{}", uuid::Uuid::new_v4());
        let vb = format!("r6a2-fv-b-{}", uuid::Uuid::new_v4());
        let vpub = format!("r6a2-fv-pub-{}", uuid::Uuid::new_v4());
        for (id, creator, name, definition, status) in [
            (&va, A, "r6a2-name-a", "r6a2-canary-fv-a", "draft"),
            (&vb, B, "r6a2-name-b", "r6a2-canary-fv-b", "draft"),
            (&vpub, A, "r6a2-canary-fv-pub", "r6a2-def-pub", "published"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_cms_form_v2 (id, app_id, name, definition, status, creator) \
                 VALUES ($1, 'r6a2-app', $2, $3, $4, $5)",
                vec![
                    id.to_string(),
                    name.to_string(),
                    definition.to_string(),
                    status.to_string(),
                    creator.to_string(),
                ],
            )
            .await;
        }

        // 三个读端点共用同一张表与同一判据：双向 + 匿名读草稿一律 403
        for (who, target) in [(B, va.as_str()), (A, vb.as_str())] {
            assert!(
                matches!(
                    form_version(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 草稿（formversion）必须 403"
            );
            assert!(
                matches!(
                    script_version(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 草稿（scriptversion）必须 403"
            );
            assert!(
                matches!(
                    template_form(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 草稿（templateform）必须 403"
            );
        }
        for (label, gate) in [
            ("formversion", form_version(&pool, None, &va).await),
            ("scriptversion", script_version(&pool, None, &va).await),
            ("templateform", template_form(&pool, None, &va).await),
        ] {
            assert!(
                matches!(gate, Err(AppError::Forbidden)),
                "匿名读草稿（{label}）必须 403"
            );
        }

        // 正控制：本人读自己草稿，端点各自返回其 canary 字段
        let fv = form_version(&pool, Some(A), &va)
            .await
            .expect("本人读 formversion");
        assert!(
            fv.contains("r6a2-canary-fv-a"),
            "formversion 本人必须返回定义，实际: {fv}"
        );
        let sv = script_version(&pool, Some(A), &va)
            .await
            .expect("本人读 scriptversion");
        assert!(
            sv.contains("r6a2-name-a"),
            "scriptversion 本人必须返回名称，实际: {sv}"
        );
        let tf = template_form(&pool, Some(A), &va)
            .await
            .expect("本人读 templateform");
        assert!(
            tf.contains("r6a2-canary-fv-a"),
            "templateform 本人必须返回定义，实际: {tf}"
        );

        // published 公开：匿名可读（三个端点）
        assert!(
            form_version(&pool, None, &vpub)
                .await
                .expect("匿名读 formversion")
                .contains("r6a2-def-pub"),
            "匿名读已发布 formversion 必须返回定义"
        );
        assert!(
            script_version(&pool, None, &vpub)
                .await
                .expect("匿名读 scriptversion")
                .contains("r6a2-canary-fv-pub"),
            "匿名读已发布 scriptversion 必须返回名称"
        );
        assert!(
            template_form(&pool, None, &vpub)
                .await
                .expect("匿名读 templateform")
                .contains("r6a2-def-pub"),
            "匿名读已发布 templateform 必须返回定义"
        );

        // 管理员放行
        for body in [
            form_version(&pool, Some(ADMIN_FV), &va).await,
            script_version(&pool, Some(ADMIN_FV), &va).await,
            template_form(&pool, Some(ADMIN_FV), &va).await,
        ] {
            assert!(body.is_ok(), "管理员必须可读他人草稿");
        }

        for id in [&va, &vb, &vpub] {
            exec(
                &pool,
                "DELETE FROM x_cms_form_v2 WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn b2_cms_script_reads_creator_guard() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool, ADMIN_SCRIPT).await;
        let sa = format!("r6a2-script-a-{}", uuid::Uuid::new_v4());
        let sb = format!("r6a2-script-b-{}", uuid::Uuid::new_v4());
        for (id, creator, content) in [
            (&sa, A, "r6a2-canary-script-a"),
            (&sb, B, "r6a2-canary-script-b"),
        ] {
            exec(
                &pool,
                "INSERT INTO x_cms_script (id, app_id, name, unique_name, script_content, creator) \
                 VALUES ($1, 'r6a2-app', 'r6a2-name', 'r6a2-unique', $2, $3)",
                vec![
                    id.to_string(),
                    content.to_string(),
                    creator.to_string(),
                ],
            )
            .await;
        }

        // 双向拒绝 + 匿名拒绝：脚本表无 published 概念
        for (who, target) in [(B, sa.as_str()), (A, sb.as_str())] {
            assert!(
                matches!(
                    script(&pool, Some(who), target).await,
                    Err(AppError::Forbidden)
                ),
                "{who} 读 {target} 脚本必须 403"
            );
        }
        assert!(
            matches!(script(&pool, None, &sa).await, Err(AppError::Forbidden)),
            "匿名读脚本必须 403"
        );

        // 正控制：本人与外层管理员都真返回脚本正文
        let own = script(&pool, Some(A), &sa).await.expect("本人读脚本");
        assert!(
            own.contains("r6a2-canary-script-a"),
            "本人读脚本必须返回正文，实际: {own}"
        );
        assert!(
            script(&pool, Some(ADMIN_SCRIPT), &sa).await.is_ok(),
            "管理员必须可读他人脚本"
        );

        for id in [&sa, &sb] {
            exec(
                &pool,
                "DELETE FROM x_cms_script WHERE id = $1",
                vec![id.to_string()],
            )
            .await;
        }
    }
}
