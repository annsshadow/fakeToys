#[cfg(test)]
mod tests {
    use crate::password::ChangePasswordRequest;
    use crate::reset::{ResetCodeError, ResetCodeStore, MAX_ATTEMPTS};
    use crate::EditPersonRequest;
    use shared::response::ActionResult;

    #[test]
    fn test_reset_code_issue_and_consume() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = ResetCodeStore::new();
            let code = store.issue("user1").await;
            assert_eq!(code.len(), 6);
            assert!(code.chars().all(|c| c.is_ascii_digit()));

            // 校验通过后立即消费（一次性）
            assert!(store.verify_and_consume("user1", &code).await.is_ok());
            assert_eq!(
                store.verify_and_consume("user1", &code).await,
                Err(ResetCodeError::NotFound)
            );
        });
    }

    #[test]
    fn test_reset_code_wrong_then_right() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = ResetCodeStore::new();
            let code = store.issue("user1").await;

            // 错误码不消耗正确尝试次数上限本身，但递减可尝试次数
            let wrong = if code == "000000" { "000001" } else { "000000" };
            assert_eq!(
                store.verify_and_consume("user1", wrong).await,
                Err(ResetCodeError::WrongCode)
            );
            assert!(store.verify_and_consume("user1", &code).await.is_ok());
            assert_eq!(
                store.verify_and_consume("user1", &code).await,
                Err(ResetCodeError::NotFound)
            );
        });
    }

    #[test]
    fn test_reset_code_attempts_exhausted() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = ResetCodeStore::new();
            let code = store.issue("user1").await;
            let wrong = if code == "000000" { "000001" } else { "000000" };

            for i in 0..(MAX_ATTEMPTS - 1) {
                let _ = i;
                assert_eq!(
                    store.verify_and_consume("user1", wrong).await,
                    Err(ResetCodeError::WrongCode)
                );
            }
            // 最后一次错误尝试后触达上限
            assert_eq!(
                store.verify_and_consume("user1", wrong).await,
                Err(ResetCodeError::TooManyAttempts)
            );
            // 条目已移除
            assert_eq!(
                store.verify_and_consume("user1", wrong).await,
                Err(ResetCodeError::NotFound)
            );
        });
    }

    #[test]
    fn test_reset_code_expired() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = ResetCodeStore::new();
            store.insert_expired("user1", "123456").await;
            assert_eq!(
                store.verify_and_consume("user1", "123456").await,
                Err(ResetCodeError::Expired)
            );
        });
    }

    #[test]
    fn test_reset_code_unknown_key() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = ResetCodeStore::new();
            assert_eq!(
                store.verify_and_consume("nobody", "123456").await,
                Err(ResetCodeError::NotFound)
            );
        });
    }

    #[test]
    fn test_is_password_acceptable() {
        assert!(!crate::reset::is_password_acceptable("12345"));
        assert!(!crate::reset::is_password_acceptable("123456"));
        assert!(crate::reset::is_password_acceptable("123456a"));
        let long = format!("{}1", "x".repeat(63));
        assert!(crate::reset::is_password_acceptable(&long));
        assert!(!crate::reset::is_password_acceptable(&format!("{}x", long)));
    }

    #[test]
    fn test_password_hash_roundtrip_bcrypt() {
        let hash = auth::password::hash_password("new-secret");
        assert!(hash.starts_with("{bcrypt}"));
        assert!(auth::password::verify_password("new-secret", &hash, "", None));
        assert!(!auth::password::verify_password("wrong", &hash, "", None));
    }

    #[test]
    fn test_edit_person_request() {
        let req = EditPersonRequest {
            name: Some("new_name".to_string()),
            mobile: Some("123456".to_string()),
            email: Some("test@example.com".to_string()),
        };

        assert_eq!(req.name, Some("new_name".to_string()));
        assert_eq!(req.mobile, Some("123456".to_string()));
        assert_eq!(req.email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_edit_person_request_partial() {
        let req = EditPersonRequest {
            name: None,
            mobile: Some("123456".to_string()),
            email: None,
        };

        assert_eq!(req.name, None);
        assert_eq!(req.mobile, Some("123456".to_string()));
        assert_eq!(req.email, None);
    }

    #[test]
    fn test_change_password_request_deserialize() {
        let req: ChangePasswordRequest =
            serde_json::from_str(r#"{"old_password":"old","new_password":"new"}"#).unwrap();
        assert_eq!(req.old_password, "old");
        assert_eq!(req.new_password, "new");
    }

    #[test]
    fn test_reset_password_request_deserialize() {
        let req: crate::reset::ResetPasswordRequest =
            serde_json::from_str(r#"{"credential":"u1","code":"123456","password":"newpass"}"#)
                .unwrap();
        assert_eq!(req.credential, "u1");
        assert_eq!(req.code, "123456");
        assert_eq!(req.password, "newpass");
    }

    #[test]
    fn test_action_result_personal_serialization() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "id": "person-1",
            "name": "admin"
        }));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["id"], "person-1");
        assert_eq!(json["data"]["name"], "admin");
    }

    #[test]
    fn test_signature_info_struct() {
        use crate::signature::SignatureInfo;
        let info = SignatureInfo {
            id: "sig-1".to_string(),
            name: "SIGNATURE_sig-1".to_string(),
            person: "user1".to_string(),
            value: "base64data".to_string(),
            created_at: None,
        };
        assert_eq!(info.id, "sig-1");
        assert_eq!(info.name, "SIGNATURE_sig-1");
        assert_eq!(info.person, "user1");
        assert_eq!(info.value, "base64data");
    }

    #[test]
    fn test_signature_upload_result_serialization() {
        let result: ActionResult<crate::signature::SignatureInfo> = ActionResult::success(
            crate::signature::SignatureInfo {
                id: "sig-2".to_string(),
                name: "SIGNATURE_sig-2".to_string(),
                person: "user2".to_string(),
                value: "encoded_data".to_string(),
                created_at: None,
            },
        );

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["id"], "sig-2");
        assert_eq!(json["data"]["name"], "SIGNATURE_sig-2");
        assert_eq!(json["data"]["person"], "user2");
        assert_eq!(json["data"]["value"], "encoded_data");
    }

    #[test]
    fn test_signature_list_result_serialization() {
        use serde_json::json;
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "signatures": [
                {"id": "sig-1", "name": "SIGNATURE_sig-1", "person": "user1", "value": "data1"},
                {"id": "sig-2", "name": "SIGNATURE_sig-2", "person": "user1", "value": "data2"}
            ]
        }));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["signatures"].as_array().unwrap().len(), 2);
        assert_eq!(json["data"]["signatures"][0]["id"], "sig-1");
        assert_eq!(json["data"]["signatures"][1]["id"], "sig-2");
    }

    #[test]
    fn test_icon_get_result_serialization_with_icon() {
        use serde_json::json;
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "icon": "iVBORw0KGgo=",
            "exists": true,
        }));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["icon"], "iVBORw0KGgo=");
        assert_eq!(json["data"]["exists"], true);
    }

    #[test]
    fn test_icon_get_result_serialization_without_icon() {
        use serde_json::json;
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "icon": "",
            "exists": false,
        }));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["icon"], "");
        assert_eq!(json["data"]["exists"], false);
    }

    #[test]
    fn test_icon_upload_result_serialization() {
        use serde_json::json;
        let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
            "icon": "base64encodeddata",
            "exists": true,
        }));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["icon"], "base64encodeddata");
        assert_eq!(json["data"]["exists"], true);
    }

    #[test]
    fn test_register_request_deserialize() {
        let req: crate::regist::RegisterRequest = serde_json::from_str(
            r#"{"credential":"newuser","password":"Abcdef1","name":"新用户","mobile":"13800138000","email":"new@example.com","code":"123456"}"#,
        )
        .unwrap();
        assert_eq!(req.credential, "newuser");
        assert_eq!(req.password, "Abcdef1");
        assert_eq!(req.name, "新用户");
        assert_eq!(req.mobile, Some("13800138000".to_string()));
        assert_eq!(req.email, Some("new@example.com".to_string()));
        assert_eq!(req.code, "123456");
    }

    #[test]
    fn test_register_request_deserialize_no_optional_fields() {
        let req: crate::regist::RegisterRequest = serde_json::from_str(
            r#"{"credential":"u1","password":"Pass1word","name":"User One","code":"654321"}"#,
        )
        .unwrap();
        assert_eq!(req.credential, "u1");
        assert_eq!(req.mobile, None);
        assert_eq!(req.email, None);
    }

    #[test]
    fn test_register_response_serialization() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "id": "person-abc-123",
            "unique": "newuser",
            "name": "新用户",
            "mobile": "13800138000",
            "email": "new@example.com",
        }));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["id"], "person-abc-123");
        assert_eq!(json["data"]["unique"], "newuser");
        assert_eq!(json["data"]["name"], "新用户");
        assert_eq!(json["data"]["mobile"], "13800138000");
        assert_eq!(json["data"]["email"], "new@example.com");
    }

    #[test]
    fn test_register_password_strength() {
        // Weak: too short
        assert!(!crate::reset::is_password_acceptable("12345"));
        // Weak: no letter
        assert!(!crate::reset::is_password_acceptable("123456"));
        // Weak: no digit
        assert!(!crate::reset::is_password_acceptable("abcdef"));
        // Valid: minimum length with letter and digit
        assert!(crate::reset::is_password_acceptable("Abcdef1"));
        // Valid: longer
        assert!(crate::reset::is_password_acceptable("MyPass1234"));
        // Weak: too long (>64)
        let long = format!("{}1", "x".repeat(64));
        assert!(!crate::reset::is_password_acceptable(&long));
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// plan002 U2：x_organization_assemble_personal 残余契约端点（u2 模块）行为测试
//
// 这些测试编码业务意图：
//  1. custom/definition 的写入是归一化 upsert —— 同名重复写入不得产生多行；
//  2. 用户级数据端点必须认证（无 token → 401），管理员级必须校验角色；
//  3. empowerlog 分页对非管理员强制收敛到本人数据（对齐 Java 分支语义）；
//  4. exmail 被动读取只信回调落库数据，未登录返回零值而非报错。
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod u2_contract {
    use super::*;
    use auth::SessionManager;
    use axum::extract::Path;
    use crate::u2;
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    fn app(pool: shared::Pool, sm: SessionManager) -> axum::Router {
        crate::router(pool, sm)
    }

    async fn body_bytes(response: axum::response::Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        serde_json::from_slice(&bytes).unwrap()
    }

    /// 幂等建表 + 管理员/普通用户会话夹具；返回 (user_token, admin_token)
    async fn fixture() -> (shared::Pool, String, String, String, String, SessionManager) {
        let pool = test_pool();
        let client = pool.get().await.unwrap();

        // 基础表（宿主库可能已有，全部幂等）
        client.execute(
            "CREATE TABLE IF NOT EXISTS auth_person (
                id VARCHAR(255) PRIMARY KEY,
                unique_id VARCHAR(255) UNIQUE,
                name VARCHAR(255),
                mobile VARCHAR(50),
                email VARCHAR(255),
                icon TEXT,
                password_hash VARCHAR(255) DEFAULT '',
                locked BOOLEAN DEFAULT FALSE,
                deleted_at TIMESTAMP,
                created_at TIMESTAMP DEFAULT NOW(),
                updated_at TIMESTAMP DEFAULT NOW()
             )", &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS auth_role (
                id VARCHAR(255) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                description TEXT,
                disable BOOLEAN DEFAULT FALSE,
                deleted_at TIMESTAMP
             )", &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS auth_person_role (
                id VARCHAR(255) PRIMARY KEY,
                person_id VARCHAR(255),
                role_id VARCHAR(255)
             )", &[]).await.unwrap();
        client.execute(
            "ALTER TABLE auth_person ADD COLUMN IF NOT EXISTS icon TEXT",
            &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_custom (
                id VARCHAR(255) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                person VARCHAR(255) NOT NULL,
                value TEXT,
                created_at TIMESTAMP DEFAULT NOW(),
                updated_at TIMESTAMP DEFAULT NOW(),
                deleted_at TIMESTAMP
             )", &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_org_definition (
                id VARCHAR(36) PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT,
                creator TEXT,
                create_time TIMESTAMP NOT NULL DEFAULT NOW(),
                update_time TIMESTAMP NOT NULL DEFAULT NOW()
             )", &[]).await.unwrap();
        // 宿主库可能存在旧形态表：缺列补齐
        for col_def in [
            "data TEXT",
            "creator TEXT",
            "create_time TIMESTAMP NOT NULL DEFAULT NOW()",
            "update_time TIMESTAMP NOT NULL DEFAULT NOW()",
        ] {
            client.execute(
                &format!("ALTER TABLE x_org_definition ADD COLUMN IF NOT EXISTS {col_def}"),
                &[],
            ).await.unwrap();
        }
        client.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS uq_x_org_definition_name \
             ON x_org_definition (name)", &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_org_person_extend (
                id VARCHAR(36) PRIMARY KEY,
                person TEXT NOT NULL,
                type TEXT NOT NULL,
                extend JSONB NOT NULL DEFAULT '{}'::jsonb,
                create_time TIMESTAMP NOT NULL DEFAULT NOW(),
                update_time TIMESTAMP NOT NULL DEFAULT NOW()
             )", &[]).await.unwrap();
        client.execute(
            "CREATE TABLE IF NOT EXISTS x_org_empower_log (
                id VARCHAR(255) PRIMARY KEY,
                application VARCHAR(255),
                process VARCHAR(255),
                work VARCHAR(255),
                from_identity VARCHAR(255),
                to_identity VARCHAR(255),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
             )", &[]).await.unwrap();
        // 与 077 一致的补列（宿主库旧表缺列时兜底）
        for col_def in [
            "from_person VARCHAR(255)",
            "to_person VARCHAR(255)",
            "title VARCHAR(255)",
            "empower_time TIMESTAMP",
            "activity VARCHAR(255)",
            "activity_name VARCHAR(255)",
        ] {
            client.execute(
                &format!("ALTER TABLE x_org_empower_log ADD COLUMN IF NOT EXISTS {col_def}"),
                &[],
            ).await.unwrap();
        }

        // 人员夹具
        let user_uid = "u2-user@P";
        let admin_uid = "u2-admin@P";
        for uid in [user_uid, admin_uid] {
            let pid = format!("u2-person-{}", uid.trim_end_matches("@P"));
            client.execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ($1, $2, $3, 'u2-test-noop') \
                 ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
                &[&pid, &uid.to_string(), &uid.to_string()],
            ).await.unwrap();
        }
        // admin 角色
        client.execute(
            "INSERT INTO auth_role (id, name) VALUES ('u2-role-admin', 'admin') \
             ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name",
            &[]).await.unwrap();
        let link = client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 SELECT 'u2-person-u2-admin', 'u2-role-admin', '' \
                 WHERE NOT EXISTS (\
                     SELECT 1 FROM auth_person_role \
                     WHERE person_id = 'u2-person-u2-admin' AND role_id = 'u2-role-admin')",
                &[],
            )
            .await;
        if link.is_err() {
            client
                .execute(
                    "INSERT INTO auth_person_role (person_id, role_id) \
                     SELECT 'u2-person-u2-admin', 'u2-role-admin' \
                     WHERE NOT EXISTS (\
                         SELECT 1 FROM auth_person_role \
                         WHERE person_id = 'u2-person-u2-admin' AND role_id = 'u2-role-admin')",
                    &[],
                )
                .await
                .unwrap();
        }

        let sm = SessionManager::with_pool(pool.clone());
        let user_token = format!("u2-tok-{}", uuid::Uuid::new_v4());
        let admin_token = format!("u2-tok-{}", uuid::Uuid::new_v4());
        sm.create_session(user_uid.to_string(), user_token.clone()).await.unwrap();
        sm.create_session(admin_uid.to_string(), admin_token.clone()).await.unwrap();

        (pool, user_token, admin_token, user_uid.to_string(), admin_uid.to_string(), sm)
    }

    #[tokio::test]
    async fn u2_custom_requires_authentication() {
        let (pool, _, _, _, _, sm) = fixture().await;
        let response = app(pool, sm)
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/custom/u2cfg")
                    .method("GET")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn u2_custom_put_get_delete_roundtrip() {
        let (pool, user_token, _, user_uid, _, sm) = fixture().await;
        let auth = format!("Bearer {user_token}");
        let router = app(pool.clone(), sm);

        // PUT 创建
        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/custom/u2cfg")
                    .method("PUT")
                    .header("authorization", &auth)
                    .header("content-type", "text/plain")
                    .body(axum::body::Body::from("hello-custom"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let v = body_bytes(response).await;
        assert_eq!(v["type"], "success", "PUT custom 失败: status={status} body={v}");

        // GET 读取
        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/custom/u2cfg")
                    .header("authorization", &auth)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"], "hello-custom");

        // 再次 PUT：归一化查重，同名单行
        let _ = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/custom/u2cfg")
                    .method("PUT")
                    .header("authorization", &auth)
                    .body(axum::body::Body::from("v2"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let client = pool.get().await.unwrap();
        let n: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_custom WHERE person=$1 AND name='u2cfg' AND deleted_at IS NULL",
                &[&user_uid],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 1, "同 (person,name) 不得产生重复行");

        // DELETE 删除
        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/custom/u2cfg")
                    .method("DELETE")
                    .header("authorization", &auth)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["value"], true);
    }

    #[tokio::test]
    async fn u2_definition_roundtrip() {
        let (pool, user_token, _, _, _, sm) = fixture().await;
        let auth = format!("Bearer {user_token}");
        let router = app(pool.clone(), sm.clone());

        let put = |router: &axum::Router, body: &'static str| {
            router.clone().oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/definition/u2def")
                    .method("PUT")
                    .header("authorization", &auth)
                    .body(axum::body::Body::from(body))
                    .unwrap(),
            )
        };
        let v = body_bytes(put(&router, "def-v1").await.unwrap()).await;
        assert_eq!(v["type"], "success", "definition PUT body={v}");

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/definition/u2def")
                    .header("authorization", &auth)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"], "def-v1");

        let _ = put(&router, "def-v2").await.unwrap();
        let client = pool.get().await.unwrap();
        let n: i64 = client
            .query_one("SELECT COUNT(*) AS c FROM x_org_definition WHERE name='u2def'", &[])
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 1, "definition 唯一名约束下仅一行");

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/definition/u2def")
                    .method("DELETE")
                    .header("authorization", &auth)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["value"], true, "delete def: status={status} body={v}");
    }

    #[tokio::test]
    async fn u2_regist_check_password_policy_hint() {
        let weak = u2::regist_check_password(Path("123".into())).await.unwrap();
        assert!(
            serde_json::to_value(&weak.0).unwrap()["data"]
                .as_str()
                .map(|s| !s.is_empty())
                .unwrap_or(false),
            "弱密码应返回策略提示"
        );

        let strong = u2::regist_check_password(Path("abc123456".into())).await.unwrap();
        let strong_json = serde_json::to_value(&strong.0).unwrap();
        let hint = strong_json["data"].as_str().unwrap_or("");
        assert!(hint.is_empty(), "强密码不应有提示");
    }

    #[tokio::test]
    async fn u2_empowerlog_paging_scopes_non_admin() {
        let (pool, user_token, _, user_uid, _, sm) = fixture().await;
        let auth = format!("Bearer {user_token}");
        let router = app(pool.clone(), sm);

        // 种两行日志：一行属于当前用户、一行属于他人
        {
            let client = pool.get().await.unwrap();
            client.execute(
                "DELETE FROM x_org_empower_log WHERE id IN ('u2-log-me','u2-log-other')",
                &[],
            ).await.unwrap();
            client.execute(
                "INSERT INTO x_org_empower_log (id, from_person, to_person, title, from_identity, to_identity) \
                 VALUES ('u2-log-me', $1, 'x@P', 'mine', $1, 'x@I')",
                &[&user_uid],
            ).await.unwrap();
            client.execute(
                "INSERT INTO x_org_empower_log (id, from_person, to_person, title, from_identity, to_identity) \
                 VALUES ('u2-log-other', 'someoneelse@P', $1, 'other', 'someoneelse@I', $1)",
                &[&user_uid],
            ).await.unwrap();
        }

        // 非管理员走 currentperson 分页：只能看到 from_person=自己的行
        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/empowerlog/list/currentperson/paging/1/size/10")
                    .method("POST")
                    .header("content-type", "application/json")
                    .header("authorization", &auth)
                    .body(axum::body::Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let v = body_bytes(response).await;
        eprintln!("empowerlog paging resp: status={status} body={v}");
        let titles: Vec<&str> = v["data"]["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["title"].as_str().unwrap_or(""))
            .collect();
        assert!(titles.contains(&"mine"), "应包含本人日志");
        assert!(!titles.contains(&"other"), "不得泄露他人日志");
    }

    #[tokio::test]
    async fn u2_exmail_passive_reads_extend() {
        let (pool, user_token, _, user_uid, _, sm) = fixture().await;
        {
            let client = pool.get().await.unwrap();
            client.execute(
                "DELETE FROM x_org_person_extend WHERE person=$1 AND type='exmail'",
                &[&user_uid],
            ).await.unwrap();
            client.execute(
                "INSERT INTO x_org_person_extend (id, person, type, extend) \
                 VALUES ('u2-ext-1', $1, 'exmail', '{\"unreadCount\":7,\"titleList\":[\"t1\"]}')",
                &[&user_uid],
            ).await.unwrap();
        }
        let auth = format!("Bearer {user_token}");
        let router = app(pool.clone(), sm);

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/exmail/new/count/passive")
                    .header("authorization", &auth)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["value"], 7);

        // 未登录 → 返回零值（对齐 Java anonymous 分支），不报错
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/exmail/new/count/passive")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let v = body_bytes(response).await;
        assert_eq!(v["data"]["value"], 0);
    }

    #[tokio::test]
    async fn u2_signature_list_person_requires_admin() {
        let (pool, user_token, _, _, _, sm) = fixture().await;
        let auth = format!("Bearer {user_token}"); // 普通用户
        let router = app(pool, sm);
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/person/signature/list/person/u2-admin@P")
                    .header("authorization", &auth)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn u2_regist_mode_defaults_disabled() {
        std::env::remove_var("PERSON_REGISTER");
        let v = u2::regist_mode().await.unwrap();
        let j = serde_json::to_value(&v.0).unwrap();
        assert_eq!(j["data"], "false");
    }
}
