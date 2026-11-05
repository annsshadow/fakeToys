// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第三批：`program_center` 的 collect save/delete + module 两条
//! remove/write）。
//!
//! 判据统一双向：A 操作 B → 403，B 操作 A → 403，本人操作自己 → 200 且落库。
//! 只验「A 改 B 被拒」会放过「admin 分支误放行 / 方向不对称」的假修复。
//! collect 的归属列是 `person_id`，module 的是 `creator`；两者都写进 SQL 的 WHERE。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::types::ToSql;
    use deadpool_postgres::Pool;
    use serde_json::json;
    use tower::ServiceExt;

    const A: &str = "r6b3-a";
    const B: &str = "r6b3-b";

    async fn connect() -> Option<Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(pool: &Pool, sql: &str, params: Vec<String>) {
        let client = pool.get().await.expect("r6b3 conn");
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        client.execute(sql, &refs).await.expect("r6b3 fixture sql");
    }

    async fn seed_collect(pool: &Pool, id: &str, owner: &str) {
        exec(
            pool,
            "INSERT INTO x_program_collect (id, person_id, creator, title, url) \
             VALUES ($1, $2, $2, 'r6b3seed', 'r6b3seed')",
            vec![id.to_string(), owner.to_string()],
        )
        .await;
    }

    async fn seed_module(pool: &Pool, id: &str, creator: &str) {
        exec(
            pool,
            "INSERT INTO x_program_module (id, name, entity, creator) \
             VALUES ($1, 'r6b3seed', 'r6b3seed', $2)",
            vec![id.to_string(), creator.to_string()],
        )
        .await;
    }

    async fn seed_admin(pool: &Pool, person_id: &str) {
        exec(
            pool,
            "INSERT INTO auth_role (id, name) VALUES ('r6b3-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'R6B3 Admin', 'r6b3-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![person_id.to_string()],
        )
        .await;
        exec(
            pool,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6b3-admin-role', 'r6b3-unit') ON CONFLICT DO NOTHING",
            vec![person_id.to_string()],
        )
        .await;
    }

    fn app(pool: &Pool) -> axum::Router {
        crate::router(pool.clone())
    }

    async fn send(
        pool: &Pool,
        actor: Option<&str>,
        method: Method,
        uri: &str,
        body: Option<serde_json::Value>,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = app(pool);
        if let Some(person) = actor {
            app = app.layer(Extension(shared::testing::test_session(person)));
        }
        let request = Request::builder()
            .uri(uri)
            .method(method)
            .header("content-type", "application/json")
            .body(body.map_or_else(Body::empty, |b| Body::from(b.to_string())))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    /// (title, person_id, 是否已软删)
    async fn collect_state(pool: &Pool, id: &str) -> (Option<String>, Option<String>, bool) {
        let client = pool.get().await.expect("r6b3 conn");
        let row = client
            .query_one(
                "SELECT title, person_id, (deleted_at IS NOT NULL) AS deleted \
                 FROM x_program_collect WHERE id = $1",
                &[&id.to_string()],
            )
            .await
            .expect("r6b3 collect row exists");
        (
            row.get::<_, Option<String>>("title"),
            row.get::<_, Option<String>>("person_id"),
            row.get::<_, bool>("deleted"),
        )
    }

    async fn cleanup(pool: &Pool, table: &str, id: &str) {
        exec(
            pool,
            &format!("DELETE FROM {table} WHERE id = $1"),
            vec![id.to_string()],
        )
        .await;
    }

    // ── collect save ──────────────────────────────────────────────

    #[tokio::test]
    async fn b3_collect_save_both_directions() {
        let Some(pool) = connect().await else {
            return;
        };
        let fa = format!("r6b3-cfa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b3-cfb-{}", uuid::Uuid::new_v4());
        seed_collect(&pool, &fa, A).await;
        seed_collect(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &pool,
                Some(who),
                Method::PUT,
                &format!("/api/program_center/collect/save/{target}"),
                Some(json!({ "title": "r6b3-hacked" })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 改 {target} 必须 403（双向）"
            );
        }
        assert_eq!(
            collect_state(&pool, &fb).await.0.as_deref(),
            Some("r6b3seed")
        );

        // 本人操作自己 → 200 且落库
        let (status, _) = send(
            &pool,
            Some(A),
            Method::PUT,
            &format!("/api/program_center/collect/save/{fa}"),
            Some(json!({ "title": "r6b3-owned", "personId": B })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let (title, person, _) = collect_state(&pool, &fa).await;
        assert_eq!(title.as_deref(), Some("r6b3-owned"));
        // 归属不可经 save 改写：载荷里塞他人的 personId 无效
        assert_eq!(person.as_deref(), Some(A));

        for id in [&fa, &fb] {
            cleanup(&pool, "x_program_collect", id).await;
        }
    }

    // ── collect delete ────────────────────────────────────────────

    #[tokio::test]
    async fn b3_collect_delete_both_directions() {
        let Some(pool) = connect().await else {
            return;
        };
        let fa = format!("r6b3-cda-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b3-cdb-{}", uuid::Uuid::new_v4());
        seed_collect(&pool, &fa, A).await;
        seed_collect(&pool, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &pool,
                Some(who),
                Method::DELETE,
                &format!("/api/program_center/collect/delete/{target}"),
                None,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）"
            );
        }
        assert!(!collect_state(&pool, &fb).await.2, "被拒方向不得软删");

        let (status, _) = send(
            &pool,
            Some(A),
            Method::DELETE,
            &format!("/api/program_center/collect/delete/{fa}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(collect_state(&pool, &fa).await.2, "本人删除必须落库");

        // 不存在的行 → 404（区分「无权」与「不存在」）
        let (status, _) = send(
            &pool,
            Some(A),
            Method::DELETE,
            "/api/program_center/collect/delete/r6b3-missing",
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);

        for id in [&fa, &fb] {
            cleanup(&pool, "x_program_collect", id).await;
        }
    }

    // ── module remove / write ─────────────────────────────────────

    #[tokio::test]
    async fn b3_module_remove_structure_both_directions() {
        let Some(pool) = connect().await else {
            return;
        };
        let ma = format!("r6b3-mra-{}", uuid::Uuid::new_v4());
        let mb = format!("r6b3-mrb-{}", uuid::Uuid::new_v4());
        seed_module(&pool, &ma, A).await;
        seed_module(&pool, &mb, B).await;

        for (who, target) in [(A, mb.as_str()), (B, ma.as_str())] {
            let (status, _) = send(
                &pool,
                Some(who),
                Method::DELETE,
                &format!("/api/program_center/module/remove/structure/{target}"),
                None,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 对 {target} 调 remove 必须 403（双向）"
            );
        }

        let (status, _) = send(
            &pool,
            Some(A),
            Method::DELETE,
            &format!("/api/program_center/module/remove/structure/{ma}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        for id in [&ma, &mb] {
            cleanup(&pool, "x_program_module", id).await;
        }
    }

    #[tokio::test]
    async fn b3_module_write_flag_both_directions() {
        let Some(pool) = connect().await else {
            return;
        };
        let ma = format!("r6b3-mwa-{}", uuid::Uuid::new_v4());
        let mb = format!("r6b3-mwb-{}", uuid::Uuid::new_v4());
        seed_module(&pool, &ma, A).await;
        seed_module(&pool, &mb, B).await;

        for (who, target) in [(A, mb.as_str()), (B, ma.as_str())] {
            let (status, _) = send(
                &pool,
                Some(who),
                Method::PUT,
                &format!("/api/program_center/module/write/{target}"),
                None,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 对 {target} 调 write 必须 403（双向）"
            );
        }

        let (status, _) = send(
            &pool,
            Some(A),
            Method::PUT,
            &format!("/api/program_center/module/write/{ma}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        for id in [&ma, &mb] {
            cleanup(&pool, "x_program_module", id).await;
        }
    }

    // ── admin 分支 / 无会话 / create 归属 ─────────────────────────

    #[tokio::test]
    async fn b3_admin_branch_still_allows_write() {
        let Some(pool) = connect().await else {
            return;
        };
        let admin = format!("r6b3-admin-{}", uuid::Uuid::new_v4());
        let fb = format!("r6b3-afb-{}", uuid::Uuid::new_v4());
        let mb = format!("r6b3-amb-{}", uuid::Uuid::new_v4());
        seed_admin(&pool, &admin).await;
        seed_collect(&pool, &fb, B).await;
        seed_module(&pool, &mb, B).await;

        let (status, _) = send(
            &pool,
            Some(&admin),
            Method::DELETE,
            &format!("/api/program_center/collect/delete/{fb}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "admin 分支应仍可写");
        assert!(collect_state(&pool, &fb).await.2);

        let (status, _) = send(
            &pool,
            Some(&admin),
            Method::PUT,
            &format!("/api/program_center/module/write/{mb}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "admin 分支应仍可写");

        cleanup(&pool, "x_program_collect", &fb).await;
        cleanup(&pool, "x_program_module", &mb).await;
    }

    #[tokio::test]
    async fn b3_no_session_is_401() {
        let Some(pool) = connect().await else {
            return;
        };
        let fa = format!("r6b3-nsa-{}", uuid::Uuid::new_v4());
        seed_collect(&pool, &fa, A).await;

        let (status, _) = send(
            &pool,
            None,
            Method::DELETE,
            &format!("/api/program_center/collect/delete/{fa}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        cleanup(&pool, "x_program_collect", &fa).await;
    }

    #[tokio::test]
    async fn b3_collect_create_sets_person_from_session() {
        let Some(pool) = connect().await else {
            return;
        };
        // 载荷里冒用 B 的 personId，落库必须是会话人 A
        let (status, body) = send(
            &pool,
            Some(A),
            Method::POST,
            "/api/program_center/collect/create",
            Some(json!({
                "title": "r6b3-create",
                "url": "r6b3-create",
                "personId": B
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let id = body["data"]["id"]
            .as_str()
            .expect("create 返回 id")
            .to_string();
        assert_eq!(
            collect_state(&pool, &id).await.1.as_deref(),
            Some(A),
            "create 的归属必须取会话人，忽略请求体 personId"
        );
        cleanup(&pool, "x_program_collect", &id).await;
    }
}
