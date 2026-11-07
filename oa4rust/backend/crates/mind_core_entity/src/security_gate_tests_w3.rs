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

    async fn seed_admin(db: &DatabaseConnection) {
        exec(
            db,
            "INSERT INTO auth_role (id, name) VALUES ('w3-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            db,
            // 无 arbiter：id 与 unique_id 各有唯一索引，并行 seed 的冲突可能落在
            // auth_person_unique_id_key 上，`ON CONFLICT (id)` 接不住。
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'W3 Admin', 'w3-fixture-noop') \
             ON CONFLICT DO NOTHING",
            vec![Value::from(ADMIN)],
        )
        .await;
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
}
