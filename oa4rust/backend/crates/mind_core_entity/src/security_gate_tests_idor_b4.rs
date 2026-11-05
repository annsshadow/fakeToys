// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第四批·mind）：`update_mind` / `update_folder`。
//!
//! 判据统一双向：A 改 B → 403，B 改 A → 403，本人改自己 → 200 且真落库；
//! 管理员按设计可改。归属条件写在 `UPDATE ... WHERE`，不做先查后判。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use sea_orm::DatabaseConnection;
    use serde_json::json;
    use tower::ServiceExt;

    const A: &str = "r6b4m-a";
    const B: &str = "r6b4m-b";
    const ADMIN: &str = "r6b4m-admin";

    async fn connect() -> Option<(Pool, DatabaseConnection)> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        let pool = shared::testing::test_pool();
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("r6b4m sea pool");
        Some((pool, db))
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b4m conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b4m fixture sql");
    }

    async fn seed_mind(pool: &Pool, id: &str, creator: &str) {
        exec(
            pool,
            "INSERT INTO x_mind_base_info (id, name, folder_id, description, creator) \
             VALUES ($1, 'r6b4seed', 'r6b4fld', 'r6b4seed', $2)",
            vec![id.to_string(), creator.to_string()],
        )
        .await;
    }

    async fn seed_folder(pool: &Pool, id: &str, creator: &str) {
        exec(
            pool,
            "INSERT INTO x_mind_folder_info (id, name, parent_id, order_number, description, creator) \
             VALUES ($1, 'r6b4seed', '', 0, 'r6b4seed', $2)",
            vec![id.to_string(), creator.to_string()],
        )
        .await;
    }

    async fn seed_admin(pool: &Pool) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6b4-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6B4 Admin', 'r6b4-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![ADMIN.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6b4-admin-role', 'r6b4-unit') ON CONFLICT DO NOTHING",
            vec![ADMIN.to_string()],
        )
        .await;
    }

    async fn send(
        db: &DatabaseConnection,
        actor: Option<&str>,
        uri: &str,
        body: serde_json::Value,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = crate::router(db.clone());
        if let Some(person) = actor {
            app = app.layer(Extension(shared::testing::test_session(person)));
        }
        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    async fn mind_name(pool: &Pool, id: &str) -> String {
        let client = pool.get().await.expect("r6b4m conn");
        let row = client
            .query_one(
                "SELECT name FROM x_mind_base_info WHERE id = $1",
                &[&id.to_string()],
            )
            .await
            .expect("r6b4m mind row exists");
        row.get::<_, String>("name")
    }

    async fn folder_name(pool: &Pool, id: &str) -> String {
        let client = pool.get().await.expect("r6b4m conn");
        let row = client
            .query_one(
                "SELECT name FROM x_mind_folder_info WHERE id = $1",
                &[&id.to_string()],
            )
            .await
            .expect("r6b4m folder row exists");
        row.get::<_, String>("name")
    }

    async fn cleanup(pool: &Pool, table: &str, id: &str) {
        exec(
            pool,
            &format!("DELETE FROM {table} WHERE id = $1"),
            vec![id.to_string()],
        )
        .await;
    }

    #[tokio::test]
    async fn b4_mind_update_mind_both_directions() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        let fa = format!("r6b4m-ma-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b4m-mb-{}", uuid::Uuid::new_v4());
        seed_mind(&pool, &fa, A).await;
        seed_mind(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                &format!("/api/mind/core/entity/mind/{target}"),
                json!({ "name": "r6b4-hacked" }),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 改 {target} 必须 403（双向）"
            );
        }
        assert_eq!(
            mind_name(&pool, &fb).await,
            "r6b4seed",
            "非属主改动不得落下"
        );

        let (status, _) = send(
            &db,
            Some(A),
            &format!("/api/mind/core/entity/mind/{fa}"),
            json!({ "name": "r6b4-owned" }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            mind_name(&pool, &fa).await,
            "r6b4-owned",
            "本人改动必须真落库"
        );

        for id in [&fa, &fb] {
            cleanup(&pool, "x_mind_base_info", id).await;
        }
    }

    #[tokio::test]
    async fn b4_mind_update_folder_both_directions() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        let fa = format!("r6b4m-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b4m-fb-{}", uuid::Uuid::new_v4());
        seed_folder(&pool, &fa, A).await;
        seed_folder(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                &format!("/api/mind/core/entity/folder/{target}"),
                json!({ "name": "r6b4-hacked" }),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 改 {target} 必须 403（双向）"
            );
        }
        assert_eq!(
            folder_name(&pool, &fb).await,
            "r6b4seed",
            "非属主改动不得落下"
        );

        let (status, _) = send(
            &db,
            Some(A),
            &format!("/api/mind/core/entity/folder/{fa}"),
            json!({ "name": "r6b4-owned" }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            folder_name(&pool, &fa).await,
            "r6b4-owned",
            "本人改动必须真落库"
        );

        for id in [&fa, &fb] {
            cleanup(&pool, "x_mind_folder_info", id).await;
        }
    }

    #[tokio::test]
    async fn b4_mind_admin_branch_allows_write() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        seed_admin(&pool).await;
        let fm = format!("r6b4m-am-{}", uuid::Uuid::new_v4());
        let ff = format!("r6b4m-af-{}", uuid::Uuid::new_v4());
        seed_mind(&pool, &fm, A).await;
        seed_folder(&pool, &ff, A).await;

        let (status, _) = send(
            &db,
            Some(ADMIN),
            &format!("/api/mind/core/entity/mind/{fm}"),
            json!({ "name": "r6b4-admin" }),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "管理员按设计可改");
        assert_eq!(mind_name(&pool, &fm).await, "r6b4-admin");

        let (status, _) = send(
            &db,
            Some(ADMIN),
            &format!("/api/mind/core/entity/folder/{ff}"),
            json!({ "name": "r6b4-admin" }),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "管理员按设计可改");
        assert_eq!(folder_name(&pool, &ff).await, "r6b4-admin");

        cleanup(&pool, "x_mind_base_info", &fm).await;
        cleanup(&pool, "x_mind_folder_info", &ff).await;
    }

    #[tokio::test]
    async fn b4_mind_no_session_and_missing() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        let fm = format!("r6b4m-ns-{}", uuid::Uuid::new_v4());
        seed_mind(&pool, &fm, A).await;

        let (status, _) = send(
            &db,
            None,
            &format!("/api/mind/core/entity/mind/{fm}"),
            json!({ "name": "r6b4-nosession" }),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "无会话必须 401");
        assert_eq!(mind_name(&pool, &fm).await, "r6b4seed");

        let missing = format!("r6b4m-missing-{}", uuid::Uuid::new_v4());
        let (status, _) = send(
            &db,
            Some(A),
            &format!("/api/mind/core/entity/mind/{missing}"),
            json!({ "name": "r6b4-x" }),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "行不存在必须 404");

        cleanup(&pool, "x_mind_base_info", &fm).await;
    }
}
