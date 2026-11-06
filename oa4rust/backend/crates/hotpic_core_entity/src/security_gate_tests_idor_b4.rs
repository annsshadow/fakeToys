// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第四批·hotpic 核心实体）：`delete_by_id`。
//!
//! 判据统一双向：A 删 B → 403，B 删 A → 403，本人删自己 → 200 且真软删；
//! 全局行（`application`/`info_id` 两列皆空）只有管理员可删，专属行本人或
//! 管理员可删。归属条件写在 `UPDATE ... WHERE`，不做先查后判。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use sea_orm::DatabaseConnection;
    use tower::ServiceExt;

    const A: &str = "r6b4h-a";
    const B: &str = "r6b4h-b";
    const ADMIN: &str = "r6b4h-admin";

    async fn connect() -> Option<(Pool, DatabaseConnection)> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        let pool = shared::testing::test_pool();
        let db = shared::testing::test_sea_orm_pool()
            .await
            .expect("r6b4h sea pool");
        Some((pool, db))
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b4h conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b4h fixture sql");
    }

    async fn seed_hotpic(pool: &Pool, id: &str, app: &str, info: &str, creator: &str) {
        exec(
            pool,
            "INSERT INTO x_hotpic (id, application, info_id, title, creator) \
             VALUES ($1, $2, $3, 'r6b4seed', $4)",
            vec![
                id.to_string(),
                app.to_string(),
                info.to_string(),
                creator.to_string(),
            ],
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
        id: &str,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = crate::hotpic_core_entity_router(db.clone());
        if let Some(person) = actor {
            app = app.layer(Extension(shared::testing::test_session(person)));
        }
        let request = Request::builder()
            .uri(format!("/api/hotpic/core/entity/delete/{id}"))
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
        let client = pool.get().await.expect("r6b4h conn");
        let row = client
            .query_one(
                "SELECT (deleted_at IS NOT NULL) AS deleted FROM x_hotpic WHERE id = $1",
                &[&id.to_string()],
            )
            .await
            .expect("r6b4h hotpic row exists");
        row.get::<_, bool>("deleted")
    }

    async fn cleanup(pool: &Pool, id: &str) {
        exec(
            pool,
            "DELETE FROM x_hotpic WHERE id = $1",
            vec![id.to_string()],
        )
        .await;
    }

    #[tokio::test]
    async fn b4_hotpic_core_delete_both_directions() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        let fa = format!("r6b4h-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b4h-fb-{}", uuid::Uuid::new_v4());
        seed_hotpic(&pool, &fa, "r6b4app", "r6b4info", A).await;
        seed_hotpic(&pool, &fb, "r6b4app", "r6b4info", B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(&db, Some(who), target).await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）"
            );
        }
        assert!(!deleted_state(&pool, &fb).await, "非属主删除不得落下");

        // 本人删自己 → 200 且真软删
        let (status, _) = send(&db, Some(A), &fa).await;
        assert_eq!(status, StatusCode::OK);
        assert!(deleted_state(&pool, &fa).await, "本人删除必须真落库");

        for id in [&fa, &fb] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b4_hotpic_core_delete_global_is_admin_only() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        seed_admin(&pool).await;
        let g = format!("r6b4h-global-{}", uuid::Uuid::new_v4());
        // 空键 = 部署级全局热点图；creator 设成 A，验证「creator 是他本人也不算数」
        seed_hotpic(&pool, &g, "", "", A).await;

        let (status, _) = send(&db, Some(A), &g).await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "全局热点图非管理员不得删除（即便 creator 是本人）"
        );
        assert!(!deleted_state(&pool, &g).await);

        let (status, _) = send(&db, Some(ADMIN), &g).await;
        assert_eq!(status, StatusCode::OK, "管理员必须能维护全局热点图");
        assert!(deleted_state(&pool, &g).await);

        cleanup(&pool, &g).await;
    }

    #[tokio::test]
    async fn b4_hotpic_core_delete_no_session_and_missing() {
        let Some((pool, db)) = connect().await else {
            return;
        };
        let f = format!("r6b4h-ns-{}", uuid::Uuid::new_v4());
        seed_hotpic(&pool, &f, "r6b4app", "r6b4info", A).await;

        let (status, _) = send(&db, None, &f).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "无会话必须 401");
        assert!(!deleted_state(&pool, &f).await);

        let missing = format!("r6b4h-missing-{}", uuid::Uuid::new_v4());
        let (status, _) = send(&db, Some(A), &missing).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "行不存在必须 404");

        cleanup(&pool, &f).await;
    }
}
