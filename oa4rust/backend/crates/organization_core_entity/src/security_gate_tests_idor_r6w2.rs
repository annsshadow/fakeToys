// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 审计第六轮写 IDOR（第二批：`organization_core_entity` 的 definition / identity
//! update·delete，共 4 条）。
//!
//! 判据统一双向：A 操作 B → 403，B 操作 A → 403，本人操作自己 → 200 且落库。
//! 只验「A 改 B 被拒」会放过「admin 分支误放行 / 方向不对称」的假修复。
//! 另验归属来源：create 的 `creator` 一律取会话人，不在请求体里接受。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};
    use serde_json::json;
    use tower::ServiceExt;

    const A: &str = "r6w2-a";
    const B: &str = "r6w2-b";

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
        .expect("r6w2 fixture sql");
    }

    async fn seed_definition(db: &DatabaseConnection, id: &str, creator: &str) {
        exec(
            db,
            // name 取 id：`uq_x_org_definition_name` 是唯一约束，固定字符串会撞。
            "INSERT INTO x_org_definition (id, name, category, type, creator) \
             VALUES ($1, $1, 'r6w2seed', 'r6w2seed', $2)",
            vec![Value::from(id), Value::from(creator)],
        )
        .await;
    }

    async fn seed_identity(db: &DatabaseConnection, id: &str, creator: &str) {
        exec(
            db,
            // person_id / name 取 id，避免任何唯一约束撞车。
            "INSERT INTO x_org_identity (id, person_id, name, type, creator) \
             VALUES ($1, $1, $1, 'r6w2seed', $2)",
            vec![Value::from(id), Value::from(creator)],
        )
        .await;
    }

    async fn seed_admin(db: &DatabaseConnection, person_id: &str, unique_id: &str) {
        exec(
            db,
            "INSERT INTO auth_role (id, name) VALUES ('r6w2-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $2, 'R6W2 Admin', 'r6w2-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![Value::from(person_id), Value::from(unique_id)],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'r6w2-admin-role', 'r6w2-unit') ON CONFLICT DO NOTHING",
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
        let mut app = crate::organization_core_entity_router(db.clone());
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

    async fn definition_name(db: &DatabaseConnection, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT name FROM x_org_definition WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w2 read definition")
        .map(|row| row.try_get::<String>("", "name").expect("name column"))
    }

    async fn definition_deleted(db: &DatabaseConnection, id: &str) -> bool {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT deleted_at FROM x_org_definition WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w2 read definition deleted_at")
        .and_then(|row| {
            row.try_get::<Option<chrono::NaiveDateTime>>("", "deleted_at")
                .ok()
        })
        .flatten()
        .is_some()
    }

    async fn identity_name(db: &DatabaseConnection, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT name FROM x_org_identity WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w2 read identity")
        .map(|row| row.try_get::<String>("", "name").expect("name column"))
    }

    async fn identity_deleted(db: &DatabaseConnection, id: &str) -> bool {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT deleted_at FROM x_org_identity WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w2 read identity deleted_at")
        .and_then(|row| {
            row.try_get::<Option<chrono::NaiveDateTime>>("", "deleted_at")
                .ok()
        })
        .flatten()
        .is_some()
    }

    async fn creator_of(db: &DatabaseConnection, table: &str, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            format!("SELECT creator FROM {table} WHERE id = $1"),
            vec![Value::from(id)],
        ))
        .await
        .expect("r6w2 read creator")
        .and_then(|row| row.try_get::<Option<String>>("", "creator").ok().flatten())
    }

    // ── definition update ──────────────────────────────────────────

    #[tokio::test]
    async fn r6w2_definition_update_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let fa = format!("r6w2-fa-{}", uuid::Uuid::new_v4());
        let fb = format!("r6w2-fb-{}", uuid::Uuid::new_v4());
        seed_definition(&db, &fa, A).await;
        seed_definition(&db, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::PUT,
                &format!("/api/organization/definition/{target}"),
                Some(json!({ "name": "r6w2-hacked" })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 改 {target} 必须 403（双向）"
            );
        }
        // 被拒方向不得落库
        assert_eq!(
            definition_name(&db, &fb).await.as_deref(),
            Some(fb.as_str())
        );
        assert_eq!(
            definition_name(&db, &fa).await.as_deref(),
            Some(fa.as_str())
        );

        // 本人操作自己 → 200 且落库
        let (status, _) = send(
            &db,
            Some(A),
            Method::PUT,
            &format!("/api/organization/definition/{fa}"),
            Some(json!({ "name": "r6w2-owned" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            definition_name(&db, &fa).await.as_deref(),
            Some("r6w2-owned")
        );

        for id in [&fa, &fb] {
            exec(
                &db,
                "DELETE FROM x_org_definition WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }

    // ── definition delete ──────────────────────────────────────────

    #[tokio::test]
    async fn r6w2_definition_delete_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let fa = format!("r6w2-fda-{}", uuid::Uuid::new_v4());
        let fb = format!("r6w2-fdb-{}", uuid::Uuid::new_v4());
        seed_definition(&db, &fa, A).await;
        seed_definition(&db, &fb, B).await;

        for (who, target) in [(A, fb.as_str()), (B, fa.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::DELETE,
                &format!("/api/organization/definition/{target}"),
                None,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）"
            );
        }
        assert!(!definition_deleted(&db, &fb).await);
        assert!(!definition_deleted(&db, &fa).await);

        let (status, _) = send(
            &db,
            Some(A),
            Method::DELETE,
            &format!("/api/organization/definition/{fa}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(definition_deleted(&db, &fa).await);

        for id in [&fa, &fb] {
            exec(
                &db,
                "DELETE FROM x_org_definition WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }

    // ── identity update ────────────────────────────────────────────

    #[tokio::test]
    async fn r6w2_identity_update_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let ia = format!("r6w2-ia-{}", uuid::Uuid::new_v4());
        let ib = format!("r6w2-ib-{}", uuid::Uuid::new_v4());
        seed_identity(&db, &ia, A).await;
        seed_identity(&db, &ib, B).await;

        for (who, target) in [(A, ib.as_str()), (B, ia.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::PUT,
                &format!("/api/organization/identity/{target}"),
                Some(json!({ "name": "r6w2-hacked" })),
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 改 {target} 必须 403（双向）"
            );
        }
        assert_eq!(identity_name(&db, &ib).await.as_deref(), Some(ib.as_str()));
        assert_eq!(identity_name(&db, &ia).await.as_deref(), Some(ia.as_str()));

        let (status, _) = send(
            &db,
            Some(A),
            Method::PUT,
            &format!("/api/organization/identity/{ia}"),
            Some(json!({ "name": "r6w2-owned" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(identity_name(&db, &ia).await.as_deref(), Some("r6w2-owned"));

        for id in [&ia, &ib] {
            exec(
                &db,
                "DELETE FROM x_org_identity WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }

    // ── identity delete ────────────────────────────────────────────

    #[tokio::test]
    async fn r6w2_identity_delete_both_directions() {
        let Some(db) = connect().await else {
            return;
        };
        let ia = format!("r6w2-ida-{}", uuid::Uuid::new_v4());
        let ib = format!("r6w2-idb-{}", uuid::Uuid::new_v4());
        seed_identity(&db, &ia, A).await;
        seed_identity(&db, &ib, B).await;

        for (who, target) in [(A, ib.as_str()), (B, ia.as_str())] {
            let (status, _) = send(
                &db,
                Some(who),
                Method::DELETE,
                &format!("/api/organization/identity/{target}"),
                None,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{who} 删 {target} 必须 403（双向）"
            );
        }
        assert!(!identity_deleted(&db, &ib).await);
        assert!(!identity_deleted(&db, &ia).await);

        let (status, _) = send(
            &db,
            Some(A),
            Method::DELETE,
            &format!("/api/organization/identity/{ia}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(identity_deleted(&db, &ia).await);

        for id in [&ia, &ib] {
            exec(
                &db,
                "DELETE FROM x_org_identity WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }

    // ── admin 分支必须真的可写（避免「修成谁都写不了」） ─────────

    #[tokio::test]
    async fn r6w2_admin_branch_still_allows_write() {
        let Some(db) = connect().await else {
            return;
        };
        let admin = format!("r6w2-admin-{}", uuid::Uuid::new_v4());
        seed_admin(&db, &admin, &admin).await;
        let fd = format!("r6w2-afd-{}", uuid::Uuid::new_v4());
        let fi = format!("r6w2-afi-{}", uuid::Uuid::new_v4());
        seed_definition(&db, &fd, B).await;
        seed_identity(&db, &fi, B).await;

        let (status, _) = send(
            &db,
            Some(&admin),
            Method::PUT,
            &format!("/api/organization/definition/{fd}"),
            Some(json!({ "name": "r6w2-admin-ok" })),
        )
        .await;
        assert_eq!(
            status,
            StatusCode::OK,
            "管理员分支必须仍可改他人 definition（否则修成谁都写不了）"
        );

        let (status, _) = send(
            &db,
            Some(&admin),
            Method::PUT,
            &format!("/api/organization/identity/{fi}"),
            Some(json!({ "name": "r6w2-admin-ok" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "管理员分支必须仍可改他人 identity");

        for id in [&fd, &fi] {
            exec(
                &db,
                "DELETE FROM x_org_definition WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
            exec(
                &db,
                "DELETE FROM x_org_identity WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }

    // ── create 的 creator 取会话人，且新行按归属生效 ────────────────

    #[tokio::test]
    async fn r6w2_create_sets_creator_from_session() {
        let Some(db) = connect().await else {
            return;
        };
        // 字段名照结构体：`DefinitionCreateRequest { name, category, type_ }`（无 serde rename）。
        let uniq = uuid::Uuid::new_v4().to_string();
        let (status, body) = send(
            &db,
            Some(A),
            Method::POST,
            "/api/organization/definition",
            Some(json!({
                "name": format!("r6w2-create-{uniq}"),
                "category": format!("r6w2-create-{uniq}"),
                "type_": format!("r6w2-create-{uniq}")
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let id = body["data"]["id"].as_str().expect("created id").to_string();
        assert_eq!(
            creator_of(&db, "x_org_definition", &id).await.as_deref(),
            Some(A),
            "definition 的 creator 必须取会话人"
        );

        // 非本人改新建行必须 403（证明谓词对新行真的生效）
        let (status, _) = send(
            &db,
            Some(B),
            Method::PUT,
            &format!("/api/organization/definition/{id}"),
            Some(json!({ "name": "r6w2-hacked" })),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, body) = send(
            &db,
            Some(A),
            Method::POST,
            "/api/organization/identity",
            // 字段名照结构体：`IdentityCreateRequest { person_id, name, type_ }`。
            // 归属收口后只能为本人创建身份，person_id 必须是会话主体 A。
            Some(json!({
                "person_id": A,
                "name": format!("r6w2-identity-{uniq}"),
                "type_": format!("r6w2-create-{uniq}")
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let iid = body["data"]["id"].as_str().expect("created id").to_string();
        assert_eq!(
            creator_of(&db, "x_org_identity", &iid).await.as_deref(),
            Some(A),
            "identity 的 creator 必须取会话人"
        );

        let (status, _) = send(
            &db,
            Some(B),
            Method::PUT,
            &format!("/api/organization/identity/{iid}"),
            Some(json!({ "name": "r6w2-hacked" })),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        for id in [&id, &iid] {
            exec(
                &db,
                "DELETE FROM x_org_definition WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
            exec(
                &db,
                "DELETE FROM x_org_identity WHERE id = $1",
                vec![Value::from(id.clone())],
            )
            .await;
        }
    }
}
