// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 读 IDOR E19·portal_assemble_designer：5 条详情读按 `creator` 收归属。
//!
//! 谓词进 `SELECT ... WHERE`：本人（`creator`）或管理员；归属为空 → 仅管理员（fail-closed）。
//! A/B 双向 403；管理员可读他人；无会话 401 先于读库；行不存在保留原 not-found 错误体。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use shared::error::AppError;

    const A: &str = "e19pd-a";
    const B: &str = "e19pd-b";
    const ADMIN: &str = "e19pd-admin";
    const ADMIN_ROLE: &str = "e19pd-admin-role";

    const TABLES: [&str; 5] = [
        "x_portal_design",
        "x_portal_dict",
        "x_portal_file",
        "x_portal_script_version",
        "x_portal_template_page",
    ];

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    async fn pool() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("e19pd conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("e19pd fixture sql");
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
             VALUES ($1, $1, 'E19PD Admin', 'e19pd-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, $2, 'e19pd-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string(), ADMIN_ROLE.to_string()],
        )
        .await;
    }

    async fn seed_row(pool: &Pool, table: &str, id: &str, creator: &str) {
        exec(
            pool,
            &format!("INSERT INTO {table} (id, creator) VALUES ($1, $2)"),
            vec![id.to_string(), creator.to_string()],
        )
        .await;
    }

    async fn cleanup(pool: &Pool, table: &str, id: &str) {
        exec(
            pool,
            &format!("DELETE FROM {table} WHERE id = $1"),
            vec![id.to_string()],
        )
        .await;
    }

    async fn call(
        endpoint: &str,
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let res = match endpoint {
            "get_design" => {
                crate::get_design(
                    Extension(pool.clone()),
                    actor.map(|p| Extension(shared::testing::test_session(p))),
                    Path(id.to_string()),
                )
                .await?
            }
            "dict_id" => {
                crate::dict_id(
                    Extension(pool.clone()),
                    actor.map(|p| Extension(shared::testing::test_session(p))),
                    Path(id.to_string()),
                )
                .await?
            }
            "file_id_download" => {
                crate::file_id_download(
                    Extension(pool.clone()),
                    actor.map(|p| Extension(shared::testing::test_session(p))),
                    Path(id.to_string()),
                )
                .await?
            }
            "scriptversion_id" => {
                crate::scriptversion_id(
                    Extension(pool.clone()),
                    actor.map(|p| Extension(shared::testing::test_session(p))),
                    Path(id.to_string()),
                )
                .await?
            }
            "templatepage_id" => {
                crate::templatepage_id(
                    Extension(pool.clone()),
                    actor.map(|p| Extension(shared::testing::test_session(p))),
                    Path(id.to_string()),
                )
                .await?
            }
            other => panic!("unknown endpoint {other}"),
        };
        Ok(serde_json::to_value(&res.0).expect("e19pd body"))
    }

    #[tokio::test]
    async fn e19_portal_designer_read_both_directions() {
        let Some(pool) = pool().await else {
            return;
        };
        let rows_a: Vec<String> = TABLES
            .iter()
            .map(|t| format!("e19pd-a-{}-{t}", uuid::Uuid::new_v4()))
            .collect();
        let rows_b: Vec<String> = TABLES
            .iter()
            .map(|t| format!("e19pd-b-{}-{t}", uuid::Uuid::new_v4()))
            .collect();

        for (idx, table) in TABLES.iter().enumerate() {
            seed_row(&pool, table, &rows_a[idx], A).await;
            seed_row(&pool, table, &rows_b[idx], B).await;
        }

        for (idx, endpoint) in [
            "get_design",
            "dict_id",
            "file_id_download",
            "scriptversion_id",
            "templatepage_id",
        ]
        .iter()
        .enumerate()
        {
            assert!(
                matches!(
                    call(endpoint, &pool, Some(A), &rows_b[idx]).await,
                    Err(AppError::Forbidden)
                ),
                "{endpoint}: A 读 B 的行必须 403"
            );
            assert!(
                matches!(
                    call(endpoint, &pool, Some(B), &rows_a[idx]).await,
                    Err(AppError::Forbidden)
                ),
                "{endpoint}: B 读 A 的行必须 403（双向）"
            );

            let own_a = call(endpoint, &pool, Some(A), &rows_a[idx])
                .await
                .expect("owner read ok");
            assert!(
                own_a.to_string().contains(&rows_a[idx]),
                "{endpoint}: A 必须可读自己的行"
            );
            let own_b = call(endpoint, &pool, Some(B), &rows_b[idx])
                .await
                .expect("owner read ok");
            assert!(
                own_b.to_string().contains(&rows_b[idx]),
                "{endpoint}: B 必须可读自己的行"
            );
        }

        for (idx, table) in TABLES.iter().enumerate() {
            cleanup(&pool, table, &rows_a[idx]).await;
            cleanup(&pool, table, &rows_b[idx]).await;
        }
    }

    #[tokio::test]
    async fn e19_portal_designer_read_admin_anon_and_missing() {
        let Some(pool) = pool().await else {
            return;
        };
        seed_admin(&pool).await;
        let rows_b: Vec<String> = TABLES
            .iter()
            .map(|t| format!("e19pd-adm-{}-{t}", uuid::Uuid::new_v4()))
            .collect();
        for (idx, table) in TABLES.iter().enumerate() {
            seed_row(&pool, table, &rows_b[idx], B).await;
        }

        let endpoints = [
            "get_design",
            "dict_id",
            "file_id_download",
            "scriptversion_id",
            "templatepage_id",
        ];
        for (idx, endpoint) in endpoints.iter().enumerate() {
            let body = call(endpoint, &pool, Some(ADMIN), &rows_b[idx])
                .await
                .expect("admin read ok");
            assert!(
                body.to_string().contains(&rows_b[idx]),
                "{endpoint}: 管理员必须可读他人行"
            );
        }

        let dead = dead_pool();
        for endpoint in endpoints {
            assert!(
                matches!(
                    call(endpoint, &dead, None, &rows_b[0]).await,
                    Err(AppError::Unauthorized)
                ),
                "{endpoint}: 无会话必须 401（门在读库前）"
            );
        }

        for endpoint in endpoints {
            let missing = format!("e19pd-missing-{}-{endpoint}", uuid::Uuid::new_v4());
            let body = call(endpoint, &pool, Some(A), &missing)
                .await
                .expect("missing keeps error body");
            assert!(
                body.to_string().contains("not found"),
                "{endpoint}: 不存在的行保留原 error body"
            );
        }

        for (idx, table) in TABLES.iter().enumerate() {
            cleanup(&pool, table, &rows_b[idx]).await;
        }
    }
}
