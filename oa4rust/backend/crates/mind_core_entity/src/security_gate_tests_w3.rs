// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 写 IDOR 第 W-b3 批：`mind_core_entity` 的 create_mind / create_folder /
//! create_version。
//!
//! 判据：
//! 1. 非管理员请求体写他人 `creator` → 403，且不落库。
//! 2. 不带 `creator` → 落会话人。
//! 3. 管理员代建 → 允许，按请求体落库（§6.2）。
//! 4. `creatorUnit` 以会话单位为事实源：请求体伪造不得落库（§13.5）。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};
    use tower::ServiceExt;

    const VICTIM: &str = "w3-victim";
    const ATTACKER: &str = "w3-attacker";
    const ADMIN: &str = "w3-admin";

    async fn connect() -> Option<DatabaseConnection> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        shared::testing::test_sea_orm_pool().await.ok()
    }

    async fn exec(db: &DatabaseConnection, sql: &str, params: Vec<Value>) {
        db.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            params,
        ))
        .await
        .expect("w3 fixture sql");
    }

    async fn send(
        db: &DatabaseConnection,
        actor: Option<&str>,
        method: Method,
        uri: &str,
        body: Option<serde_json::Value>,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = crate::mind_core_entity_router(db.clone());
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
    async fn seed_person(db: &DatabaseConnection, id: &str, unique_id: &str) -> Result<(), String> {
        db.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $2, 'W3 Admin', 'w3-fixture-noop') ON CONFLICT DO NOTHING",
            vec![Value::from(id), Value::from(unique_id)],
        ))
        .await
        .map_err(|e| format!("w3 seed 插入失败: {e}"))?;
        let row = db
            .query_one(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "SELECT COUNT(*) AS n FROM auth_person WHERE id = $1 AND unique_id = $2",
                vec![Value::from(id), Value::from(unique_id)],
            ))
            .await
            .map_err(|e| format!("w3 seed 身份反查失败: {e}"))?
            .ok_or_else(|| "w3 seed 身份反查无结果".to_string())?;
        let n: i64 = row
            .try_get("", "n")
            .map_err(|e| format!("w3 seed 身份反查取值失败: {e}"))?;
        if n != 1 {
            return Err(format!(
                "w3 seed 真实冲突被吞：id={id} unique_id={unique_id} 反查行数={n}"
            ));
        }
        Ok(())
    }

    async fn seed_admin(db: &DatabaseConnection) {
        exec(
            db,
            "INSERT INTO auth_role (id, name) VALUES ('w3-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        // 无 arbiter：id 与 unique_id 各有唯一索引，并行 seed 的冲突可能落在
        // auth_person_unique_id_key 上，`ON CONFLICT (id)` 接不住。
        seed_person(db, ADMIN, ADMIN)
            .await
            .expect("w3 admin seed 必须建立 id/unique_id 一致的本人行");
        exec(
            db,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'w3-admin-role', 'w3-unit') ON CONFLICT DO NOTHING",
            vec![Value::from(ADMIN)],
        )
        .await;
    }

    /// 攻击者挂到固定单位（`creator_unit` 事实源用例）。
    async fn seed_attacker_unit(db: &DatabaseConnection) {
        exec(
            db,
            "INSERT INTO auth_role (id, name) VALUES ('w3-user-role','user') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_unit (id, name, parent_id, level, created_at) \
             VALUES ('w3-unit', 'W3 Unit', '', 1, NOW()) ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'W3 Attacker', 'w3-fixture-noop') \
             ON CONFLICT (id) DO NOTHING",
            vec![Value::from(ATTACKER)],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'w3-user-role', 'w3-unit') ON CONFLICT DO NOTHING",
            vec![Value::from(ATTACKER)],
        )
        .await;
    }

    async fn col_of(db: &DatabaseConnection, table: &str, id: &str, col: &str) -> String {
        let row = db
            .query_one(Statement::from_sql_and_values(
                DbBackend::Postgres,
                format!("SELECT {col} FROM {table} WHERE id = $1"),
                vec![Value::from(id)],
            ))
            .await
            .expect("w3 read col")
            .expect("w3 row exists");
        row.try_get::<Option<String>>("", col)
            .unwrap_or_default()
            .unwrap_or_default()
    }

    #[tokio::test]
    async fn w3_mind_and_folder_create_use_session_creator() {
        let Some(db) = connect().await else {
            return;
        };
        seed_admin(&db).await;

        for (uri, table) in [
            ("/api/mind/core/entity/mind", "x_mind_base_info"),
            ("/api/mind/core/entity/folder", "x_mind_folder_info"),
        ] {
            // 非管理员冒名 → 403，且不落库。
            let (status, _) = send(
                &db,
                Some(ATTACKER),
                Method::POST,
                uri,
                Some(serde_json::json!({ "name": "w3", "creator": VICTIM })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "非管理员在 {uri} 写他人 creator 必须 403"
            );

            // 不带 creator → 落会话人。
            let (status, body) = send(
                &db,
                Some(ATTACKER),
                Method::POST,
                uri,
                Some(serde_json::json!({ "name": "w3" })),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "本人经 {uri} 创建");
            let id = body["data"]["id"].as_str().expect("id").to_string();
            assert_eq!(col_of(&db, table, &id, "creator").await, ATTACKER);
            exec(
                &db,
                &format!("DELETE FROM {table} WHERE id = $1"),
                vec![Value::from(id)],
            )
            .await;

            // 管理员代建 → 允许，按请求体落库（§6.2）。
            let (status, body) = send(
                &db,
                Some(ADMIN),
                Method::POST,
                uri,
                Some(serde_json::json!({ "name": "w3", "creator": VICTIM })),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "管理员经 {uri} 代建");
            let id = body["data"]["id"].as_str().expect("id").to_string();
            assert_eq!(col_of(&db, table, &id, "creator").await, VICTIM);
            exec(
                &db,
                &format!("DELETE FROM {table} WHERE id = $1"),
                vec![Value::from(id)],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn w3_version_create_uses_session_creator_and_unit() {
        let Some(db) = connect().await else {
            return;
        };
        seed_admin(&db).await;
        seed_attacker_unit(&db).await;
        let uri = "/api/mind/core/entity/version";

        // 非管理员冒名 → 403，且不落库。
        let (status, _) = send(
            &db,
            Some(ATTACKER),
            Method::POST,
            uri,
            Some(serde_json::json!({
                "mindId": "w3-mind",
                "name": "w3",
                "folderId": "w3-folder",
                "creator": VICTIM,
            })),
        )
        .await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "非管理员写他人 creator 必须 403"
        );

        // 不带 creator；请求体伪造 creatorUnit 不得落库 → 落会话人 + 会话单位。
        let (status, body) = send(
            &db,
            Some(ATTACKER),
            Method::POST,
            uri,
            Some(serde_json::json!({
                "mindId": "w3-mind",
                "name": "w3",
                "folderId": "w3-folder",
                "creatorUnit": "w3-spoof-unit",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "本人建版本");
        let id = body["data"]["id"].as_str().expect("version id").to_string();
        assert_eq!(
            col_of(&db, "x_mind_version_info", &id, "creator").await,
            ATTACKER
        );
        assert_eq!(
            col_of(&db, "x_mind_version_info", &id, "creator_unit").await,
            "W3 Unit",
            "creatorUnit 必须取会话单位而非请求体"
        );
        exec(
            &db,
            "DELETE FROM x_mind_version_info WHERE id = $1",
            vec![Value::from(id)],
        )
        .await;
    }

    /// 反例：真实业务冲突（id 或 unique_id 已被不同身份占用）必须报错，
    /// 不能被无 arbiter 的 `ON CONFLICT DO NOTHING` 吞成「seed 成功」。
    /// 探针 id 专用前缀，避免干扰并行用例的 w3-admin。
    #[tokio::test]
    async fn w3_seed_reports_real_conflict_instead_of_swallowing_it() {
        let Some(db) = connect().await else {
            return;
        };
        const CLEAN: &str = "DELETE FROM auth_person \
             WHERE id LIKE 'w3-probe-mind%' OR unique_id LIKE 'w3-probe-mind%'";
        exec(&db, CLEAN, vec![]).await;

        // 方向一：unique_id 已被别的 id 占用。
        exec(
            &db,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ('w3-probe-mind-a-other','w3-probe-mind-a','probe','w3-fixture-noop')",
            vec![],
        )
        .await;
        let err = seed_person(&db, "w3-probe-mind-a", "w3-probe-mind-a").await;
        assert!(err.is_err(), "unique_id 被他人占用时必须报错，实际 {err:?}");

        // 方向二：id 已被占用且 unique_id 不同。
        exec(
            &db,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ('w3-probe-mind-b','w3-probe-mind-b-other','probe','w3-fixture-noop')",
            vec![],
        )
        .await;
        let err = seed_person(&db, "w3-probe-mind-b", "w3-probe-mind-b").await;
        assert!(err.is_err(), "id 被他人占用时必须报错，实际 {err:?}");

        exec(&db, CLEAN, vec![]).await;
    }
}
