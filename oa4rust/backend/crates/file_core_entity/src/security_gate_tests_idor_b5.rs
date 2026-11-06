// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第五批·file 核心实体）：`folder_delete`（`file_folder`）。
//!
//! 判据统一双向：A 删 B → 403，B 删 A → 403，本人删自己 → 200 且真软删；
//! 管理员可维护他人目录。归属条件写在 `UPDATE ... WHERE`，不做先查后判。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use sea_orm::DatabaseConnection;
    use tower::ServiceExt;

    const A: &str = "r6b5f-a";
    const B: &str = "r6b5f-b";
    const ADMIN: &str = "r6b5f-admin";

    async fn connect() -> Option<(Pool, DatabaseConnection)> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        let pool = shared::testing::test_pool();
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("r6b5f sea pool");
        Some((pool, db))
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b5f conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b5f fixture sql");
    }

    async fn seed_folder(pool: &Pool, id: &str, person: &str) {
        exec(
            pool,
            "INSERT INTO file_folder (id, name, person) VALUES ($1, 'r6b5seed', $2)",
            vec![id.to_string(), person.to_string()],
        )
        .await;
    }

    async fn seed_admin(pool: &Pool) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6b5-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6B5 Admin', 'r6b5-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6b5-admin-role', 'r6b5-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string()],
        )
        .await;
    }

    async fn send(
        db: &DatabaseConnection,
        actor: Option<&str>,
        id: &str,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = crate::file_core_entity_router(db.clone());
        if let Some(person) = actor {
            app = app.layer(Extension(shared::testing::test_session(person)));
        }
        let request = Request::builder()
            .uri(format!("/api/file/core/entity/folder/{id}"))
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    async fn deleted_state(pool: &Pool, id: &str) -> bool {
        let client = pool.get().await.expect("r6b5f conn");
        let row = client
            .query_one(
                "SELECT (deleted_at IS NOT NULL) AS deleted FROM file_folder WHERE id = $1",
                &[&id.to_string()],
            )
            .await
            .expect("r6b5f folder row exists");
        row.get::<_, bool>("deleted")
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM file_folder WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    #[tokio::test]
    async fn b5_file_folder_delete_both_directions() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        let fa = format!("r6b5f-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b5f-fb-{}", uuid::Uuid::new_v4());
        seed_folder(&pool, &fa, A).await;
        seed_folder(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, body) = send(&db, Some(who), target).await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）body={body}"
            );
        }
        assert!(!deleted_state(&pool, &fb).await, "非属主删除不得落下");

        // 本人删自己 → 200 且真软删（写后必读）
        let (status, _) = send(&db, Some(A), &fa).await;
        assert_eq!(status, StatusCode::OK);
        assert!(deleted_state(&pool, &fa).await, "本人删除必须真落库");

        for id in [&fa, &fb] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b5_file_folder_delete_admin_and_missing() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        seed_admin(&pool).await;
        let fa = format!("r6b5f-adm-{}", uuid::Uuid::new_v4());
        seed_folder(&pool, &fa, A).await;

        let (status, _) = send(&db, Some(ADMIN), &fa).await;
        assert_eq!(status, StatusCode::OK, "管理员必须能维护他人目录");
        assert!(deleted_state(&pool, &fa).await);

        let missing = format!("r6b5f-missing-{}", uuid::Uuid::new_v4());
        let (status, _) = send(&db, Some(A), &missing).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "行不存在必须 404");

        cleanup(&pool, &fa).await;
    }

    #[tokio::test]
    async fn b5_file_folder_delete_no_session() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        let fa = format!("r6b5f-ns-{}", uuid::Uuid::new_v4());
        seed_folder(&pool, &fa, A).await;

        let (status, _) = send(&db, None, &fa).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "无会话必须 401");
        assert!(!deleted_state(&pool, &fa).await);

        cleanup(&pool, &fa).await;
    }
}
