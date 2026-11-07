// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第一批：`general_core_entity` 的 file / invoice update/delete）。
//!
//! 判据统一双向：A 操作 B → 403，B 操作 A → 403，本人操作自己 → 200 且落库。
//! 只验「A 改 B 被拒」会放过「admin 分支误放行 / 方向不对称」的假修复。
//! 另验归属来源：create 忽略请求体 `creator`，一律取会话人（防创建时冒名）。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};
    use serde_json::json;
    use tower::ServiceExt;

    const A: &str = "r6w-a";
    const B: &str = "r6w-b";

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
        .expect("r6w fixture sql");
    }

    async fn seed_file(db: &DatabaseConnection, id: &str, creator: &str) {
        exec(
            db,
            "INSERT INTO x_general_file (id, name, mime_type, size, creator) \
             VALUES ($1, 'r6seed', 'r6seed', 1, $2)",
            vec![Value::from(id), Value::from(creator)],
        )
        .await;
    }

    async fn seed_invoice(db: &DatabaseConnection, id: &str, creator: &str) {
        exec(
            db,
            "INSERT INTO x_general_invoice (id, number, date, amount, status, creator) \
             VALUES ($1, 'r6seed', 'r6seed', 0, 'r6seed', $2)",
            vec![Value::from(id), Value::from(creator)],
        )
        .await;
    }

    async fn seed_admin(db: &DatabaseConnection, person_id: &str, unique_id: &str) {
        exec(
            db,
            "INSERT INTO auth_role (id, name) VALUES ('r6w-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $2, 'R6W Admin', 'r6w-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![Value::from(person_id), Value::from(unique_id)],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6w-admin-role', 'r6w-unit') ON CONFLICT DO NOTHING",
            vec![Value::from(person_id)],
        )
        .await;
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

    async fn file_name(db: &DatabaseConnection, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT name FROM x_general_file WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w read file")
        .map(|row| row.try_get::<String>("", "name").expect("name column"))
    }

    async fn invoice_number(db: &DatabaseConnection, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT number FROM x_general_invoice WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w read invoice")
        .map(|row| row.try_get::<String>("", "number").expect("number column"))
    }

    async fn file_creator(db: &DatabaseConnection, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT creator FROM x_general_file WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w read creator")
        .and_then(|row| row.try_get::<Option<String>>("", "creator").ok().flatten())
    }

    async fn invoice_creator(db: &DatabaseConnection, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT creator FROM x_general_invoice WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w read creator")
        .and_then(|row| row.try_get::<Option<String>>("", "creator").ok().flatten())
    }

    // ── file update ───────────────────────────────────────────────

    #[tokio::test]
    async fn r6w_file_update_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let fa = format!("r6w-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6w-fb-{}", uuid::Uuid::new_v4());
        seed_file(&db, &fa, A).await;
        seed_file(&db, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::POST,
                &format!("/api/general/file/update/{target}"),
                Some(json!({ "name": "r6w-hacked", "mimeType": "r6w-hacked" })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 改 {target} 必须 403（双向）"
            );
        }
        // 被拒方向不得落库
        assert_eq!(file_name(&db, &fb).await.as_deref(), Some("r6seed"));

        // 本人操作自己 → 200 且落库
        let (status, _) = send(
            &db,
            Some(A),
            Method::POST,
            &format!("/api/general/file/update/{fa}"),
            Some(json!({ "name": "r6w-owned", "mimeType": "text/plain" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(file_name(&db, &fa).await.as_deref(), Some("r6w-owned"));

        for id in [&fa, &fb] {
            exec(
                &db,
                "DELETE FROM x_general_file WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn r6w_file_delete_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let fa = format!("r6w-fda-{}", uuid::Uuid::new_v4());
        let fb = format!("r6w-fdb-{}", uuid::Uuid::new_v4());
        seed_file(&db, &fa, A).await;
        seed_file(&db, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::POST,
                &format!("/api/general/file/delete/{target}"),
                None,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）"
            );
            assert!(
                file_name(&db, target).await.is_some(),
                "被拒方向的行必须仍在"
            );
        }

        let (status, _) = send(
            &db,
            Some(A),
            Method::POST,
            &format!("/api/general/file/delete/{fa}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(file_name(&db, &fa).await.is_none(), "本人删除应落库");

        exec(
            &db,
            "DELETE FROM x_general_file WHERE id = $1",
            vec![Value::from(fb.clone())],
        )
        .await;
    }

    // ── invoice update / delete ───────────────────────────────────

    #[tokio::test]
    async fn r6w_invoice_update_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let ia = format!("r6w-ia-{}", uuid::Uuid::new_v4());
        let ib = format!("r6w-ib-{}", uuid::Uuid::new_v4());
        seed_invoice(&db, &ia, A).await;
        seed_invoice(&db, &ib, B).await;

        for (who, target) in [(A, ib.as_str()), (B, ia.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::POST,
                &format!("/api/general/invoice/update/{target}"),
                Some(json!({
                    "number": "r6w-hacked",
                    "date": "2026-01-01",
                    "amount": 99999.0,
                    "status": "approved"
                })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 改 {target} 必须 403（双向）"
            );
        }
        assert_eq!(invoice_number(&db, &ib).await.as_deref(), Some("r6seed"));

        let (status, _) = send(
            &db,
            Some(A),
            Method::POST,
            &format!("/api/general/invoice/update/{ia}"),
            Some(json!({
                "number": "r6w-owned",
                "date": "2026-01-02",
                "amount": 1.0,
                "status": "draft"
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(invoice_number(&db, &ia).await.as_deref(), Some("r6w-owned"));

        for id in [&ia, &ib] {
            exec(
                &db,
                "DELETE FROM x_general_invoice WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn r6w_invoice_delete_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let ia = format!("r6w-ida-{}", uuid::Uuid::new_v4());
        let ib = format!("r6w-idb-{}", uuid::Uuid::new_v4());
        seed_invoice(&db, &ia, A).await;
        seed_invoice(&db, &ib, B).await;

        for (who, target) in [(A, ib.as_str()), (B, ia.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::POST,
                &format!("/api/general/invoice/delete/{target}"),
                None,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）"
            );
            assert!(
                invoice_number(&db, target).await.is_some(),
                "被拒方向的行必须仍在"
            );
        }

        let (status, _) = send(
            &db,
            Some(A),
            Method::POST,
            &format!("/api/general/invoice/delete/{ia}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(invoice_number(&db, &ia).await.is_none(), "本人删除应落库");

        exec(
            &db,
            "DELETE FROM x_general_invoice WHERE id = $1",
            vec![Value::from(ib.clone())],
        )
        .await;
    }

    // ── 归属来源：create 不得听请求体 ─────────────────────────────

    #[tokio::test]
    async fn r6w_create_rejects_payload_creator() {
        let Some(db) = connect().await else {
            return;
        };

        // 请求体 creator≠会话人：非管理员一律 403，不得冒名落库。
        let (status, body) = send(
            &db,
            Some(A),
            Method::POST,
            "/api/general/file/create",
            Some(json!({ "name": "r6w-c", "mimeType": "text/plain", "size": 1, "creator": B })),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{body}");

        let (status, body) = send(
            &db,
            Some(A),
            Method::POST,
            "/api/general/invoice/create",
            Some(json!({ "number": "r6w-c", "creator": B })),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{body}");

        // 缺省 owner = 会话人：本人创建成功且 creator 落会话人。
        let (status, body) = send(
            &db,
            Some(A),
            Method::POST,
            "/api/general/file/create",
            Some(json!({ "name": "r6w-c-own", "mimeType": "text/plain", "size": 1 })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let file_id = body["data"]["id"]
            .as_str()
            .expect("file create returns id")
            .to_string();
        assert_eq!(
            file_creator(&db, &file_id).await.as_deref(),
            Some(A),
            "file_create 缺省 creator 必须落会话人"
        );

        let (status, body) = send(
            &db,
            Some(A),
            Method::POST,
            "/api/general/invoice/create",
            Some(json!({ "number": "r6w-c-own" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let invoice_id = body["data"]["id"]
            .as_str()
            .expect("invoice create returns id")
            .to_string();
        assert_eq!(
            invoice_creator(&db, &invoice_id).await.as_deref(),
            Some(A),
            "invoice_create 缺省 creator 必须落会话人"
        );

        exec(
            &db,
            "DELETE FROM x_general_file WHERE id = $1",
            vec![Value::from(file_id)],
        )
        .await;
        exec(
            &db,
            "DELETE FROM x_general_invoice WHERE id = $1",
            vec![Value::from(invoice_id)],
        )
        .await;
    }

    // ── admin 分支必须真的可写（避免「修成谁都写不了」） ─────────

    #[tokio::test]
    async fn r6w_admin_branch_still_allows_write() {
        let Some(db) = connect().await else {
            return;
        };
        let admin = format!("r6w-admin-{}", uuid::Uuid::new_v4());
        seed_admin(&db, &admin, &admin).await;
        let fb = format!("r6w-afb-{}", uuid::Uuid::new_v4());
        seed_file(&db, &fb, B).await;

        let (status, _) = send(
            &db,
            Some(&admin),
            Method::POST,
            &format!("/api/general/file/update/{fb}"),
            Some(json!({ "name": "r6w-admin-ok", "mimeType": "text/plain" })),
        )
        .await;
        assert_eq!(
            status,
            StatusCode::OK,
            "管理员分支必须仍可改他人行（否则修成谁都写不了）"
        );

        exec(
            &db,
            "DELETE FROM x_general_file WHERE id = $1",
            vec![Value::from(fb)],
        )
        .await;
    }
}
