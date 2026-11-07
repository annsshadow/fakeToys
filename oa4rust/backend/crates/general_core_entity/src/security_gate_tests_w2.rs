// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 写 IDOR 第 W-b2 批：`general_core_entity` 的 file / invoice 读面与创建面。
//!
//! 判据：
//! 1. 非管理员读他人归属行 → 403；本人读自己行 → 200。
//! 2. 创建时请求体写他人 `creator`：非管理员 → 403；管理员代建 → 允许且按请求者落库。
//! 3. 创建时不带 `creator` → 落会话人。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};
    use tower::ServiceExt;

    const VICTIM: &str = "w2-victim";
    const ATTACKER: &str = "w2-attacker";
    const ADMIN: &str = "w2-admin";

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
        .expect("w2 fixture sql");
    }

    async fn send(
        db: &DatabaseConnection,
        actor: Option<&str>,
        method: Method,
        uri: &str,
        body: Option<serde_json::Value>,
    ) -> (StatusCode, serde_json::Value) {
        let mut app = crate::general_core_entity_router(db.clone());
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
            "INSERT INTO auth_role (id, name) VALUES ('w2-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'W2 Admin', 'w2-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![Value::from(ADMIN)],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'w2-admin-role', 'w2-unit') ON CONFLICT DO NOTHING",
            vec![Value::from(ADMIN)],
        )
        .await;
    }

    async fn creator_of(db: &DatabaseConnection, table: &str, id: &str) -> String {
        let row = db
            .query_one(Statement::from_sql_and_values(
                DbBackend::Postgres,
                format!("SELECT creator FROM {table} WHERE id = $1"),
                vec![Value::from(id)],
            ))
            .await
            .expect("w2 read creator")
            .expect("w2 row exists");
        row.try_get::<Option<String>>("", "creator")
            .unwrap_or_default()
            .unwrap_or_default()
    }

    #[tokio::test]
    async fn w2_reads_scope_to_owner_or_admin() {
        let Some(db) = connect().await else {
            return;
        };
        for (table, id, extra) in [
            ("x_general_file", "w2-file", ", name, mime_type, size"),
            (
                "x_general_invoice",
                "w2-inv",
                ", number, date, amount, status",
            ),
        ] {
            exec(
                &db,
                &format!("DELETE FROM {table} WHERE id = $1"),
                vec![Value::from(id)],
            )
            .await;
            let values = if table == "x_general_file" {
                ", 'w2', 'w2', 1"
            } else {
                ", 'w2', 'w2', 0, 'w2'"
            };
            exec(
                &db,
                &format!("INSERT INTO {table} (id, creator{extra}) VALUES ($1, $2{values})"),
                vec![Value::from(id), Value::from(VICTIM)],
            )
            .await;
        }

        for uri in [
            "/api/general/file/w2-file",
            "/api/general/file/download/w2-file",
            "/api/general/invoice/w2-inv",
        ] {
            let (status, body) = send(&db, Some(ATTACKER), Method::GET, uri, None).await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{ATTACKER} 读他人行 {uri} 必须 403，实际 body={body}"
            );
            let (status, body) = send(&db, Some(VICTIM), Method::GET, uri, None).await;
            assert_eq!(status, StatusCode::OK, "{VICTIM} 读自己行 {uri}");
            assert!(
                body.get("creator").is_some() || body.pointer("/data/creator").is_some(),
                "{uri} 本人读应返回归属列，实际 body={body}"
            );
        }

        // 精确按本用例的 id 清理：并行跑 w2_creates_reject_impersonation 时，
        // 宽口径 `WHERE creator = ...` 会把对方刚建的行删掉，读侧于是拿到 404
        // 而不是「存在但无权」应得的 403。
        for (table, id) in [
            ("x_general_file", "w2-file"),
            ("x_general_invoice", "w2-inv"),
        ] {
            exec(
                &db,
                &format!("DELETE FROM {table} WHERE id = $1"),
                vec![Value::from(id)],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn w2_creates_reject_impersonation() {
        let Some(db) = connect().await else {
            return;
        };
        seed_admin(&db).await;

        // 非管理员冒名 → 403，且不落库。
        for (uri, id_prefix) in [
            ("/api/general/file/create", "w2-file"),
            ("/api/general/invoice/create", "w2-inv"),
        ] {
            let (status, _) = send(
                &db,
                Some(ATTACKER),
                Method::POST,
                uri,
                Some(serde_json::json!({
                    "creator": VICTIM,
                    "name": id_prefix,
                    "number": id_prefix,
                })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "非管理员在 {uri} 写他人 creator 必须 403"
            );
        }

        // 不带 creator → 落会话人。
        let (status, body) = send(
            &db,
            Some(ATTACKER),
            Method::POST,
            "/api/general/file/create",
            Some(serde_json::json!({ "name": "w2-self" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "本人建文件");
        let file_id = body["data"]["id"].as_str().expect("file id").to_string();
        assert_eq!(creator_of(&db, "x_general_file", &file_id).await, ATTACKER);

        // 管理员代建 → 允许，按请求者落库（§6.2）。
        let (status, body) = send(
            &db,
            Some(ADMIN),
            Method::POST,
            "/api/general/invoice/create",
            Some(serde_json::json!({ "creator": VICTIM, "number": "w2-admin" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "管理员代建发票");
        let id = body["data"]["id"].as_str().expect("invoice id").to_string();
        assert_eq!(creator_of(&db, "x_general_invoice", &id).await, VICTIM);

        // 只删本用例创建的那一行，避免影响并行运行的 w2_reads_scope_to_owner_or_admin。
        exec(
            &db,
            "DELETE FROM x_general_file WHERE id = $1",
            vec![Value::from(file_id)],
        )
        .await;
        exec(
            &db,
            "DELETE FROM x_general_invoice WHERE number = 'w2-admin'",
            vec![],
        )
        .await;
    }
}
