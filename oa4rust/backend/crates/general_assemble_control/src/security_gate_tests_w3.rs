// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 写 IDOR 第 W-b3 批：`general_assemble_control` 的 office 创建面
//! （`/office` 与 `/office/html/to/word` 为同一 handler，门禁两条用例）。
//!
//! 判据：
//! 1. 非管理员请求体写他人 `creator` → 403，且不落库。
//! 2. 不带 `creator` → 落会话人。
//! 3. 管理员代建 → 允许，按请求体落库（§6.2）。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use tower::ServiceExt;

    const VICTIM: &str = "w3-victim";
    const ATTACKER: &str = "w3-attacker";
    const ADMIN: &str = "w3-admin";

    async fn pool() -> Option<deadpool_postgres::Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(client: &deadpool_postgres::Object, sql: &str) {
        client.execute(sql, &[]).await.expect("w3 fixture sql");
    }

    async fn send(
        pool: &deadpool_postgres::Pool,
        actor: Option<&str>,
        method: Method,
        uri: &str,
        body: Option<serde_json::Value>,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = crate::general_assemble_control_router(pool.clone());
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

    /// 无 arbiter 的 `ON CONFLICT DO NOTHING` 会把任何唯一冲突静默吞掉。
    /// 插入后反查「id 与 unique_id 必须互相指认」，真实冲突就必须报错，
    /// 只有并行 seed 恰好已建好同一行时才允许通过。
    async fn seed_person(
        client: &deadpool_postgres::Object,
        id: &str,
        unique_id: &str,
    ) -> Result<(), String> {
        client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ($1, $2, 'W3 Admin', 'w3-fixture-noop') ON CONFLICT DO NOTHING",
                &[&id, &unique_id],
            )
            .await
            .map_err(|e| format!("w3 seed 插入失败: {e}"))?;
        let row = client
            .query_one(
                "SELECT count(*) AS n FROM auth_person WHERE id = $1 AND unique_id = $2",
                &[&id, &unique_id],
            )
            .await
            .map_err(|e| format!("w3 seed 身份反查失败: {e}"))?;
        let n: i64 = row.get("n");
        if n != 1 {
            return Err(format!(
                "w3 seed 真实冲突被吞：id={id} unique_id={unique_id} 反查行数={n}"
            ));
        }
        Ok(())
    }

    async fn seed_admin(client: &deadpool_postgres::Object) {
        exec(
            client,
            "INSERT INTO auth_role (id, name) VALUES ('w3-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
        )
        .await;
        // 无 arbiter：id 与 unique_id 各有唯一索引，并行 seed 的冲突可能落在
        // auth_person_unique_id_key 上，`ON CONFLICT (id)` 接不住。
        seed_person(client, ADMIN, ADMIN)
            .await
            .expect("w3 admin seed 必须建立 id/unique_id 一致的本人行");
        exec(
            client,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ('w3-admin','w3-admin-role','w3-unit') ON CONFLICT DO NOTHING",
        )
        .await;
    }

    async fn creator_of(client: &deadpool_postgres::Object, id: &str) -> String {
        let row = client
            .query_one(
                "SELECT creator FROM x_general_assemble_office WHERE id = $1",
                &[&id],
            )
            .await
            .expect("w3 read creator");
        row.get::<_, Option<String>>("creator").unwrap_or_default()
    }

    #[tokio::test]
    async fn w3_office_create_uses_session_creator() {
        let Some(pool) = pool().await else {
            return;
        };
        let client = shared::db::acquire(&pool).await.expect("w3 acquire");
        seed_admin(&client).await;

        let uris = [
            "/api/general/assemble/control/office",
            "/api/general/assemble/control/office/html/to/word",
        ];

        // 非管理员冒名 → 403，且不落库。
        for uri in uris {
            let (status, _) = send(
                &pool,
                Some(ATTACKER),
                Method::POST,
                uri,
                Some(serde_json::json!({ "creator": VICTIM, "htmlContent": "<p>w3</p>" })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "非管理员在 {uri} 写他人 creator 必须 403"
            );
        }

        // 不带 creator → 落会话人。
        for uri in uris {
            let (status, body) = send(
                &pool,
                Some(ATTACKER),
                Method::POST,
                uri,
                Some(serde_json::json!({ "htmlContent": "<p>w3</p>" })),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "本人经 {uri} 建 office 行");
            let id = body["data"]["id"].as_str().expect("office id").to_string();
            assert_eq!(
                creator_of(&client, &id).await,
                ATTACKER,
                "{uri} 必须落会话人"
            );
            exec(
                &client,
                &format!("DELETE FROM x_general_assemble_office WHERE id = '{id}'"),
            )
            .await;
        }

        // 管理员代建 → 允许，按请求体落库（§6.2）。
        let (status, body) = send(
            &pool,
            Some(ADMIN),
            Method::POST,
            uris[0],
            Some(serde_json::json!({ "creator": VICTIM, "htmlContent": "<p>w3</p>" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "管理员代建 office 行");
        let id = body["data"]["id"].as_str().expect("office id").to_string();
        assert_eq!(creator_of(&client, &id).await, VICTIM);
        exec(
            &client,
            &format!("DELETE FROM x_general_assemble_office WHERE id = '{id}'"),
        )
        .await;
    }

    /// 反例：真实业务冲突（id 或 unique_id 已被不同身份占用）必须报错，
    /// 不能被无 arbiter 的 `ON CONFLICT DO NOTHING` 吞成「seed 成功」。
    /// 探针 id 专用前缀，避免干扰并行用例的 w3-admin。
    #[tokio::test]
    async fn w3_seed_reports_real_conflict_instead_of_swallowing_it() {
        let Some(pool) = pool().await else {
            return;
        };
        let client = shared::db::acquire(&pool).await.expect("w3 acquire");
        const CLEAN: &str = "DELETE FROM auth_person \
             WHERE id LIKE 'w3-probe-ga%' OR unique_id LIKE 'w3-probe-ga%'";
        exec(&client, CLEAN).await;

        // 方向一：unique_id 已被别的 id 占用。
        exec(
            &client,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ('w3-probe-ga-a-other','w3-probe-ga-a','probe','w3-fixture-noop')",
        )
        .await;
        let err = seed_person(&client, "w3-probe-ga-a", "w3-probe-ga-a").await;
        assert!(err.is_err(), "unique_id 被他人占用时必须报错，实际 {err:?}");

        // 方向二：id 已被占用且 unique_id 不同。
        exec(
            &client,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ('w3-probe-ga-b','w3-probe-ga-b-other','probe','w3-fixture-noop')",
        )
        .await;
        let err = seed_person(&client, "w3-probe-ga-b", "w3-probe-ga-b").await;
        assert!(err.is_err(), "id 被他人占用时必须报错，实际 {err:?}");

        exec(&client, CLEAN).await;
    }
}
