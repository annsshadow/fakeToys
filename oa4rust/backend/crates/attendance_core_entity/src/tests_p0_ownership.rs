// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! P0：考勤记录越权改/删（审计第四轮）。
//!
//! 判据统一为双向：A 操作 B → 403，B 操作 A → 403，本人操作自己 → 200。
//! 只验「A 改 B 被拒」会放过「admin 分支误放行 / 方向不对称」的假修复。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};
    use tower::ServiceExt;

    const BASE: &str = "/api/attendance/core/entity";

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
        .expect("p0 fixture sql");
    }

    /// 本地/CI 测试库可能还没跑到 117，先幂等补列，避免用例卡在 schema 差异。
    async fn ensure_schema(db: &DatabaseConnection) {
        exec(
            db,
            "ALTER TABLE \"x_attendance_record\" ADD COLUMN IF NOT EXISTS \"created_by\" TEXT",
            vec![],
        )
        .await;
    }

    async fn seed_record(db: &DatabaseConnection, id: &str, owner: &str) {
        exec(
            db,
            "INSERT INTO x_attendance_record \
             (id, user_id, check_in_time, status, created_by) \
             VALUES ($1, $2, '2026-01-01T09:00:00', 'normal', $2)",
            vec![Value::from(id), Value::from(owner)],
        )
        .await;
    }

    async fn seed_admin(db: &DatabaseConnection, person_id: &str, unique_id: &str) {
        exec(
            db,
            "INSERT INTO auth_role (id, name) VALUES ('p0-admin-role','admin') ON CONFLICT (id) DO NOTHING",
            vec![],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $2, 'P0 Admin', 'p0-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            vec![Value::from(person_id), Value::from(unique_id)],
        )
        .await;
        exec(
            db,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1, 'p0-admin-role', 'p0-unit') ON CONFLICT DO NOTHING",
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
        let mut app = crate::attendance_core_entity_router(db.clone());
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

    async fn row_status(db: &DatabaseConnection, id: &str) -> Option<String> {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT status FROM x_attendance_record WHERE id = $1",
            vec![Value::from(id)],
        ))
        .await
        .expect("p0 read status")
        .map(|row| row.try_get::<String>("", "status").expect("status column"))
    }

    async fn count_owned_by(db: &DatabaseConnection, owner: &str) -> i64 {
        db.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT count(*) AS n FROM x_attendance_record WHERE user_id = $1",
            vec![Value::from(owner)],
        ))
        .await
        .expect("p0 count rows")
        .expect("count row")
        .try_get::<i64>("", "n")
        .expect("count column")
    }

    #[tokio::test]
    async fn p0_record_update_is_owner_scoped_both_directions() {
        let Some(db) = connect().await else {
            eprintln!("skipping p0_record_update_is_owner_scoped_both_directions: DB unavailable");
            return;
        };
        ensure_schema(&db).await;
        let ida = format!("p0-upd-a-{}", uuid::Uuid::new_v4());
        let idb = format!("p0-upd-b-{}", uuid::Uuid::new_v4());
        seed_record(&db, &ida, "p0-owner-a").await;
        seed_record(&db, &idb, "p0-owner-b").await;

        // A → B 与 B → A 两个方向都必须 403，且落库不变
        let (status, _) = send(
            &db,
            Some("p0-owner-a"),
            Method::POST,
            &format!("{BASE}/record/{idb}/update"),
            Some(serde_json::json!({"status": "late"})),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "A 改 B 必须 403");
        assert_eq!(row_status(&db, &idb).await.as_deref(), Some("normal"));

        let (status, _) = send(
            &db,
            Some("p0-owner-b"),
            Method::POST,
            &format!("{BASE}/record/{ida}/update"),
            Some(serde_json::json!({"status": "late"})),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "B 改 A 必须 403");
        assert_eq!(row_status(&db, &ida).await.as_deref(), Some("normal"));

        // 本人改自己 → 200 且落库生效
        let (status, _) = send(
            &db,
            Some("p0-owner-a"),
            Method::POST,
            &format!("{BASE}/record/{ida}/update"),
            Some(serde_json::json!({"status": "late", "checkOutTime": "2026-01-01T18:00:00"})),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "本人改自己必须 200");
        assert_eq!(row_status(&db, &ida).await.as_deref(), Some("late"));

        for id in [&ida, &idb] {
            exec(
                &db,
                "DELETE FROM x_attendance_record WHERE id = $1",
                vec![Value::from(id.as_str())],
            )
            .await;
        }
    }

    #[tokio::test]
    async fn p0_record_delete_is_owner_scoped_both_directions() {
        let Some(db) = connect().await else {
            eprintln!("skipping p0_record_delete_is_owner_scoped_both_directions: DB unavailable");
            return;
        };
        ensure_schema(&db).await;
        let ida = format!("p0-del-a-{}", uuid::Uuid::new_v4());
        let idb = format!("p0-del-b-{}", uuid::Uuid::new_v4());
        seed_record(&db, &ida, "p0-owner-a").await;
        seed_record(&db, &idb, "p0-owner-b").await;

        let (status, _) = send(
            &db,
            Some("p0-owner-a"),
            Method::POST,
            &format!("{BASE}/record/{idb}/delete"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "A 删 B 必须 403");
        assert_eq!(row_status(&db, &idb).await.as_deref(), Some("normal"));

        let (status, _) = send(
            &db,
            Some("p0-owner-b"),
            Method::POST,
            &format!("{BASE}/record/{ida}/delete"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "B 删 A 必须 403");
        assert_eq!(row_status(&db, &ida).await.as_deref(), Some("normal"));

        let (status, _) = send(
            &db,
            Some("p0-owner-a"),
            Method::POST,
            &format!("{BASE}/record/{ida}/delete"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "本人删自己必须 200");
        assert_eq!(row_status(&db, &ida).await, None);

        exec(
            &db,
            "DELETE FROM x_attendance_record WHERE id = $1",
            vec![Value::from(idb.as_str())],
        )
        .await;
    }

    #[tokio::test]
    async fn p0_record_create_derives_owner_from_session() {
        let Some(db) = connect().await else {
            eprintln!("skipping p0_record_create_derives_owner_from_session: DB unavailable");
            return;
        };
        ensure_schema(&db).await;
        // 伪造目标用唯一身份，避免并行用例互相污染计数
        let forged = format!("p0-forged-{}", uuid::Uuid::new_v4());

        // 普通用户伪造他人 userId → 403，且不落库
        let (status, _) = send(
            &db,
            Some("p0-owner-a"),
            Method::POST,
            &format!("{BASE}/record/create"),
            Some(serde_json::json!({
                "userId": forged,
                "checkInTime": "2026-01-01T09:00:00"
            })),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "普通用户伪造他人必须 403");
        assert_eq!(count_owned_by(&db, &forged).await, 0, "被拒请求不得落库");

        // 不带 userId → owner 取会话人
        let (status, body) = send(
            &db,
            Some("p0-owner-a"),
            Method::POST,
            &format!("{BASE}/record/create"),
            Some(serde_json::json!({"checkInTime": "2026-01-01T09:00:00"})),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"]["user_id"], "p0-owner-a");
        assert_eq!(body["data"]["created_by"], "p0-owner-a");
        let created_self = body["data"]["id"].as_str().expect("created id").to_string();

        // 管理员代建他人 → 允许，user_id 是目标人，created_by 留痕代建人
        seed_admin(&db, "p0-admin-person", "p0-owner-admin").await;
        let (status, body) = send(
            &db,
            Some("p0-owner-admin"),
            Method::POST,
            &format!("{BASE}/record/create"),
            Some(serde_json::json!({
                "userId": forged,
                "checkInTime": "2026-01-01T09:00:00"
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "管理员代建必须放行");
        assert_eq!(body["data"]["user_id"], forged);
        assert_eq!(body["data"]["created_by"], "p0-owner-admin");
        let created_proxy = body["data"]["id"].as_str().expect("created id").to_string();

        // 只清本用例创建的两行
        for id in [&created_self, &created_proxy] {
            exec(
                &db,
                "DELETE FROM x_attendance_record WHERE id = $1",
                vec![Value::from(id.as_str())],
            )
            .await;
        }
    }
}
