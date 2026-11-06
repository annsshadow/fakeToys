// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 写 IDOR 第 W-b1 批：`general_assemble_control` 的 area / attendscope / invoice /
//! qrcode / securityclearance 五个读端点。
//!
//! 判据：
//! 1. 非管理员读他人归属行 → 403；本人读自己行 → 200。
//! 2. 三条管理写端点的 `creator` 以会话为事实源：请求体伪造 `creator` 不得落库。
//!
//! 历史行（`creator='system'`/NULL）按 fail-closed 仅管理员可读，不回填。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use tower::ServiceExt;

    const VICTIM: &str = "w1-victim";
    const ATTACKER: &str = "w1-attacker";
    const ADMIN: &str = "w1-admin";

    async fn pool() -> Option<deadpool_postgres::Pool> {
        if !shared::testing::is_db_available().await {
            return None;
        }
        Some(shared::testing::test_pool())
    }

    async fn exec(client: &deadpool_postgres::Object, sql: &str) {
        client.execute(sql, &[]).await.expect("w1 fixture sql");
    }

    async fn exec_one(client: &deadpool_postgres::Object, sql: &str, p: &[&str]) {
        let params: Vec<&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)> = p
            .iter()
            .map(|s| s as &(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync))
            .collect();
        client.execute(sql, &params).await.expect("w1 fixture sql");
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

    async fn seed_admin(client: &deadpool_postgres::Object) {
        exec(
            client,
            "INSERT INTO auth_role (id, name) VALUES ('w1-admin-role','admin') \
             ON CONFLICT (id) DO NOTHING",
        )
        .await;
        exec_one(
            client,
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1, $1, 'W1 Admin', 'w1-fixture-noop') \
             ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
            &[ADMIN],
        )
        .await;
        exec(
            client,
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ('w1-admin','w1-admin-role','w1-unit') ON CONFLICT DO NOTHING",
        )
        .await;
    }

    #[tokio::test]
    async fn w1_reads_scope_to_owner_or_admin() {
        let Some(pool) = pool().await else {
            return;
        };
        let client = shared::db::acquire(&pool).await.expect("w1 acquire");

        let fixtures = [
            ("x_general_assemble_area", "w1-area"),
            ("x_general_attend_scope", "w1-scope"),
            ("x_general_assemble_invoice", "w1-inv"),
            ("x_general_assemble_qrcode", "w1-qr"),
            ("x_general_assemble_security_clearance", "w1-sec"),
        ];
        for (table, id) in fixtures {
            let del = format!("DELETE FROM \"{table}\" WHERE id = '{id}'");
            exec(&client, &del).await;
            let ins = format!("INSERT INTO \"{table}\" (id, creator) VALUES ('{id}', '{VICTIM}')");
            exec(&client, &ins).await;
        }

        let cases = [
            ("/api/general/assemble/control/area/w1-area", "w1-area"),
            (
                "/api/general/assemble/control/attendscope/w1-scope",
                "w1-scope",
            ),
            ("/api/general/assemble/control/invoice/get/w1-inv", "w1-inv"),
            ("/api/general/assemble/control/qrcode/w1-qr", "w1-qr"),
            (
                "/api/general/assemble/control/securityclearance/w1-sec",
                "w1-sec",
            ),
        ];
        for (uri, canary) in cases {
            let (status, body) = send(&pool, Some(ATTACKER), Method::GET, uri, None).await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{ATTACKER} 读他人行 {uri} 必须 403，实际 body={body}"
            );
            assert!(
                !body.to_string().contains(canary),
                "{uri} 的 403 响应不得回显受害者数据"
            );

            let (status, body) = send(&pool, Some(VICTIM), Method::GET, uri, None).await;
            assert_eq!(status, StatusCode::OK, "{VICTIM} 读自己行 {uri}");
            assert!(
                body.to_string().contains(canary),
                "{uri} 本人读应返回自己的行，实际 body={body}"
            );
        }

        for (table, id) in fixtures {
            let sql = format!("DELETE FROM \"{table}\" WHERE id = '{id}'");
            exec(&client, &sql).await;
        }
    }

    #[tokio::test]
    async fn w1_creates_use_session_creator() {
        let Some(pool) = pool().await else {
            return;
        };
        let client = shared::db::acquire(&pool).await.expect("w1 acquire");
        seed_admin(&client).await;

        let spoof = Some(serde_json::json!({ "creator": VICTIM, "name": "w1-new" }));

        let (status, body) = send(
            &pool,
            Some(ADMIN),
            Method::POST,
            "/api/general/assemble/control/area/create",
            spoof.clone(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "admin 建地区");
        let id = body["data"]["id"].as_str().expect("area id").to_string();
        assert_eq!(
            row_creator(&client, "x_general_assemble_area", &id).await,
            ADMIN
        );
        exec(
            &client,
            &format!("DELETE FROM x_general_assemble_area WHERE id = '{id}'"),
        )
        .await;

        let (status, body) = send(
            &pool,
            Some(ADMIN),
            Method::POST,
            "/api/general/assemble/control/attendscope/create",
            spoof,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "admin 建考勤范围");
        let id = body["data"]["id"].as_str().expect("scope id").to_string();
        assert_eq!(
            row_creator(&client, "x_general_attend_scope", &id).await,
            ADMIN
        );
        exec(
            &client,
            &format!("DELETE FROM x_general_attend_scope WHERE id = '{id}'"),
        )
        .await;

        let spoof = Some(serde_json::json!({ "creator": VICTIM, "name": "w1-sec-new" }));
        let (status, body) = send(
            &pool,
            Some(ADMIN),
            Method::POST,
            "/api/general/assemble/control/securityclearance/create",
            spoof,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "admin 建安全等级");
        let id = body["data"]["id"].as_str().expect("sec id").to_string();
        assert_eq!(
            row_creator(&client, "x_general_assemble_security_clearance", &id).await,
            ADMIN
        );
        exec(
            &client,
            &format!("DELETE FROM x_general_assemble_security_clearance WHERE id = '{id}'"),
        )
        .await;
    }

    async fn row_creator(client: &deadpool_postgres::Object, table: &str, id: &str) -> String {
        let sql = format!("\"{table}\"");
        let row = client
            .query_one(&format!("SELECT creator FROM {sql} WHERE id = $1"), &[&id])
            .await
            .expect("w1 read creator");
        row.get::<_, Option<String>>("creator").unwrap_or_default()
    }
}
