// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第四批·hotpic assemble control）：`user_hotpic_delete_by_ids`。
//!
//! 判据统一双向：A 删 B → 403，B 删 A → 403，本人删自己 → 200 且真软删；
//! 全局行（两列皆空）只有管理员可删。归属条件写在 `UPDATE ... WHERE`。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use tower::ServiceExt;

    const A: &str = "r6b4j-a";
    const B: &str = "r6b4j-b";
    const ADMIN: &str = "r6b4j-admin";

    async fn connect() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b4j conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b4j fixture sql");
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
        pool: &Pool,
        actor: Option<&str>,
        id: &str,
        id2: &str,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = crate::router(pool.clone());
        if let Some(person) = actor {
            app = app.layer(Extension(shared::testing::test_session(person)));
        }
        let request = Request::builder()
            .uri(format!(
                "/api/hotpic/assemble/control/user/hotpic/{id}/{id2}"
            ))
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
        let client = pool.get().await.expect("r6b4j conn");
        let row = client
            .query_one(
                "SELECT (deleted_at IS NOT NULL) AS deleted FROM x_hotpic WHERE id = $1",
                &[&id.to_string()],
            )
            .await
            .expect("r6b4j hotpic row exists");
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
    async fn b4_hotpic_assemble_delete_both_directions() {
        let Some(pool) = connect().await else {
            return;
        };
        let fa = format!("r6b4j-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b4j-fb-{}", uuid::Uuid::new_v4());
        let none = format!("r6b4j-none-{}", uuid::Uuid::new_v4());
        seed_hotpic(&pool, &fa, "r6b4app", "r6b4info", A).await;
        seed_hotpic(&pool, &fb, "r6b4app", "r6b4info", B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(&pool, Some(who), target, &none).await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）"
            );
        }
        assert!(!deleted_state(&pool, &fb).await, "非属主删除不得落下");

        // 本人一次删两条自己的 → 200 且两条都软删
        let own2 = format!("r6b4j-fa2-{}", uuid::Uuid::new_v4());
        seed_hotpic(&pool, &own2, "r6b4app", "r6b4info", A).await;
        let (status, json) = send(&pool, Some(A), &fa, &own2).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            json["data"]["deleted"].as_u64(),
            Some(2),
            "本人两条都应软删"
        );
        assert!(deleted_state(&pool, &fa).await);
        assert!(deleted_state(&pool, &own2).await);

        for id in [&fa, &fb, &own2] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b4_hotpic_assemble_delete_mixed_batch_spares_other_row() {
        let Some(pool) = connect().await else {
            return;
        };
        let fa = format!("r6b4j-mfa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b4j-mfb-{}", uuid::Uuid::new_v4());
        seed_hotpic(&pool, &fa, "r6b4app", "r6b4info", A).await;
        seed_hotpic(&pool, &fb, "r6b4app", "r6b4info", B).await;

        // 混入他人 id：只删自己的那条，他人行不受影响
        let (status, json) = send(&pool, Some(A), &fa, &fb).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["data"]["deleted"].as_u64(), Some(1));
        assert!(deleted_state(&pool, &fa).await);
        assert!(!deleted_state(&pool, &fb).await, "他人行不得被写");

        for id in [&fa, &fb] {
            cleanup(&pool, id).await;
        }
    }

    #[tokio::test]
    async fn b4_hotpic_assemble_delete_global_is_admin_only() {
        let Some(pool) = connect().await else {
            return;
        };
        seed_admin(&pool).await;
        let g = format!("r6b4j-global-{}", uuid::Uuid::new_v4());
        let none = format!("r6b4j-none-{}", uuid::Uuid::new_v4());
        seed_hotpic(&pool, &g, "", "", A).await;

        let (status, _) = send(&pool, Some(A), &g, &none).await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "全局热点图非管理员不得删除（即便 creator 是本人）"
        );
        assert!(!deleted_state(&pool, &g).await);

        let (status, _) = send(&pool, Some(ADMIN), &g, &none).await;
        assert_eq!(status, StatusCode::OK, "管理员必须能维护全局热点图");
        assert!(deleted_state(&pool, &g).await);

        cleanup(&pool, &g).await;
    }

    #[tokio::test]
    async fn b4_hotpic_assemble_delete_no_session_and_missing() {
        let Some(pool) = connect().await else {
            return;
        };
        let f = format!("r6b4j-ns-{}", uuid::Uuid::new_v4());
        let none = format!("r6b4j-none-{}", uuid::Uuid::new_v4());
        seed_hotpic(&pool, &f, "r6b4app", "r6b4info", A).await;

        let (status, _) = send(&pool, None, &f, &none).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "无会话必须 401");
        assert!(!deleted_state(&pool, &f).await);

        let m1 = format!("r6b4j-miss1-{}", uuid::Uuid::new_v4());
        let m2 = format!("r6b4j-miss2-{}", uuid::Uuid::new_v4());
        let (status, _) = send(&pool, Some(A), &m1, &m2).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "两行都不存在必须 404");

        cleanup(&pool, &f).await;
    }
}
