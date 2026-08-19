#[cfg(test)]
mod tests {
    use crate::password::verify_password;
    use crate::RateLimiter;
    use crate::SessionManager;
    use base64::Engine;
    use shared::response::ActionResult;
    use shared::testing::test_pool;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[test]
    fn test_verify_password_md5() {
        let password = "admin123";
        let md5_hash = format!("{:x}", md5::compute(password.as_bytes()));
        assert!(verify_password(password, &md5_hash, "", None));
    }

    #[test]
    fn test_verify_password_des() {
        let password = "admin123";
        let key = "testkey123";
        let encrypted = des_encrypt_for_test(password, key);
        let base64 = base64::engine::general_purpose::URL_SAFE.encode(encrypted);
        let url_encoded = urlencoding::encode(&base64);
        assert!(verify_password(password, &url_encoded, key, None));
    }

    #[test]
    fn test_verify_password_wrong() {
        let password = "admin123";
        let wrong_hash = "wrong_hash";
        assert!(!verify_password(password, wrong_hash, "", None));
    }

    #[test]
    fn test_verify_password_des_wrong_key() {
        let password = "admin123";
        let key = "testkey123";
        let encrypted = des_encrypt_for_test(password, key);
        let base64 = base64::engine::general_purpose::URL_SAFE.encode(encrypted);
        let url_encoded = urlencoding::encode(&base64);
        assert!(!verify_password(password, &url_encoded, "wrongkey", None));
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
        assert_eq!(result.message, None);
    }

    #[test]
    fn test_action_result_error() {
        let result: ActionResult<String> = ActionResult::error("test error");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("test error".to_string()));
    }

    #[test]
    fn test_action_result_all_fields() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({"key": "value"}));
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        assert_eq!(result.message, None);
        assert_eq!(result.date, None);
        assert_eq!(result.spent, None);
        assert_eq!(result.size, None);
        assert_eq!(result.count, None);
        assert_eq!(result.position, None);
        assert_eq!(result.prompt, None);
    }

    #[test]
    fn test_session_manager() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            let session = manager.create_session("user1".to_string(), "token1".to_string()).await.unwrap();
            assert_eq!(session.person_unique, "user1");
            assert_eq!(session.token, "token1");

            let found = manager.validate_session("token1").await;
            assert!(found.is_some());
            assert_eq!(found.unwrap().person_unique, "user1");

            let not_found = manager.validate_session("nonexistent").await;
            assert!(not_found.is_none());
        });
    }

    #[test]
    fn test_session_manager_remove() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            manager.create_session("user1".to_string(), "token1".to_string()).await.unwrap();
            assert!(manager.validate_session("token1").await.is_some());

            manager.remove_session("token1").await;
            assert!(manager.validate_session("token1").await.is_none());
        });
    }

    #[test]
    fn test_session_manager_multiple() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            manager.create_session("user1".to_string(), "token1".to_string()).await.unwrap();
            manager.create_session("user2".to_string(), "token2".to_string()).await.unwrap();

            assert!(manager.validate_session("token1").await.is_some());
            assert!(manager.validate_session("token2").await.is_some());
            assert!(manager.validate_session("token3").await.is_none());
        });
    }

    #[test]
    fn test_session_manager_remove_sessions_by_person() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            // 为 user1 创建多个 session
            manager.create_session("user1".to_string(), "token1".to_string()).await.unwrap();
            manager.create_session("user1".to_string(), "token2".to_string()).await.unwrap();
            // 为 user2 创建 session
            manager.create_session("user2".to_string(), "token3".to_string()).await.unwrap();

            // 验证三个 session 都有效
            assert!(manager.validate_session("token1").await.is_some());
            assert!(manager.validate_session("token2").await.is_some());
            assert!(manager.validate_session("token3").await.is_some());

            // 批量注销 user1 的所有 session
            manager.remove_sessions_by_person("user1").await;

            // user1 的 session 应全部失效
            assert!(manager.validate_session("token1").await.is_none());
            assert!(manager.validate_session("token2").await.is_none());
            // user2 的 session 不受影响
            assert!(manager.validate_session("token3").await.is_some());
        });
    }

    #[test]
    fn test_code_store_issue_and_verify() {
        let store = crate::CodeStore::new();
        let code = store.issue("user1");
        assert!(!code.is_empty());
        assert!(store.verify("user1", &code));
        // 验证码一次性，再次验证应失败
        assert!(!store.verify("user1", &code));
    }

    #[test]
    fn test_code_store_expired() {
        let store = crate::CodeStore::new();
        let code = store.issue("user1");
        // 模拟过期（直接修改内部存储）
        {
            let mut entries = store.entries.lock().unwrap();
            if let Some(entry) = entries.get_mut("user1") {
                use chrono::{Duration, Utc};
                entry.expires_at = Utc::now() - Duration::minutes(1);
            }
        }
        assert!(!store.verify("user1", &code));
    }

    #[test]
    fn test_rate_limiter() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let limiter = RateLimiter::new();
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_err());
        });
    }

    #[test]
    fn test_rate_limiter_reset() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let limiter = RateLimiter::new();
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_err());

            limiter.reset("key1").await;
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
        });
    }

    #[test]
    fn test_rate_limiter_different_keys() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let limiter = RateLimiter::new();
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key2", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key1", 3, 1).await.is_ok());
            assert!(limiter.check_rate_limit("key2", 3, 1).await.is_ok());
        });
    }

    // --- U7 新增测试用例：OIDC 路由注册与 id_token 验证 ---

    #[tokio::test]
    async fn test_oidc_authorize_route_registered() {
        let pool = shared::testing::test_pool();
        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/oidc/authorize?client_id=test&redirect_uri=http://localhost&response_type=code&scope=openid&state=abc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_oidc_callback_route_registered() {
        let pool = shared::testing::test_pool();
        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/oidc/callback?code=testcode&state=abc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 路由已注册：不应返回 404（缺少 OIDC 配置时返回 500，这是正常的）
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_oidc_get_or_create_person_inserts() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_oidc_get_or_create_person_inserts: DATABASE_URL not reachable");
            return;
        }

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();

        if let Some(c) = &client {
            let _ = c
                .execute(
                    "DELETE FROM auth_person WHERE unique_id = $1",
                    &[&"oidc_test_create_user_001"],
                )
                .await;
        }

        let result = crate::oidc::get_or_create_person(&pool, "test_create_user_001")
            .await
            .expect("get_or_create_person should succeed");

        assert_eq!(
            result.get("unique_id").and_then(|v| v.as_str()),
            Some("oidc_test_create_user_001")
        );
        assert!(
            result.get("id").and_then(|v| v.as_str()).is_some(),
            "new person should have an id"
        );
    }

    #[tokio::test]
    async fn test_oidc_get_or_create_person_existing() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_oidc_get_or_create_person_existing: DATABASE_URL not reachable");
            return;
        }

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();

        let unique_id = "oidc_test_existing_user_001";

        if let Some(c) = &client {
            let _ = c
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                     VALUES ($1, $2, $3, $4) \
                     ON CONFLICT (unique_id) DO UPDATE SET name = EXCLUDED.name",
                    &[&"person-oidc-existing", &unique_id, &"Pre-existing OIDC User", &"{bcrypt}$2b$12$dummy"],
                )
                .await;
        }

        let result = crate::oidc::get_or_create_person(&pool, "test_existing_user_001")
            .await
            .expect("get_or_create_person should succeed");

        assert_eq!(
            result.get("unique_id").and_then(|v| v.as_str()),
            Some(unique_id)
        );
        assert_eq!(
            result.get("name").and_then(|v| v.as_str()),
            Some("Pre-existing OIDC User")
        );
    }

    fn des_encrypt_for_test(plain: &str, key: &str) -> Vec<u8> {
        crate::password::des_encrypt(plain, key).unwrap()
    }

    struct EnvGuard {
        vars: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn new() -> Self {
            Self { vars: Vec::new() }
        }
        fn set(mut self, key: &'static str, value: impl Into<String>) -> Self {
            let prev = std::env::var(key).ok();
            self.vars.push((key, prev));
            std::env::set_var(key, value.into());
            self
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, prev) in self.vars.drain(..).rev() {
                match prev {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    #[test]
    fn test_des3_encrypt_decrypt_ede2_roundtrip() {
        let plain = "testuser#1700000000000";
        let key = "0123456789abcdef"; // 16 bytes
        let encrypted = crate::password::des3_encrypt_ede2(plain, key).unwrap();
        let decrypted = crate::password::des3_decrypt_ede2(&encrypted, key).unwrap();
        assert_eq!(String::from_utf8(decrypted).unwrap(), plain);
    }

    #[test]
    fn test_des3_decrypt_wrong_key() {
        let plain = "testuser#1700000000000";
        let key = "0123456789abcdef";
        let encrypted = crate::password::des3_encrypt_ede2(plain, key).unwrap();
        // 错误 key 解密产生乱码（非 UTF-8），无法还原原始内容
        let decrypted = crate::password::des3_decrypt_ede2(&encrypted, "wrongkey12345678").unwrap();
        let result = String::from_utf8(decrypted);
        // 乱码通常不是合法 UTF-8
        match result {
            Ok(s) => assert_ne!(s, plain),
            Err(_) => {} // 非 UTF-8 也符合预期
        }
    }

    #[test]
    fn test_des3_encrypt_short_key() {
        let result = crate::password::des3_encrypt_ede2("test", "short");
        assert!(result.is_err());
    }

    // --- U1 新增测试用例 ---

    #[test]
    fn test_person_info_full_fields() {
        let person = crate::PersonInfo {
            id: "person-001".to_string(),
            unique: "user001".to_string(),
            name: "Test User".to_string(),
            mobile: Some("13800138000".to_string()),
            email: Some("test@example.com".to_string()),
            icon: Some("base64icon".to_string()),
            job: Some("Engineer".to_string()),
            department: Some("IT".to_string()),
            unit: Some("Tech Unit".to_string()),
            position: Some("Senior".to_string()),
        };
        assert_eq!(person.id, "person-001");
        assert_eq!(person.unique, "user001");
        assert_eq!(person.name, "Test User");
        assert_eq!(person.mobile, Some("13800138000".to_string()));
        assert_eq!(person.email, Some("test@example.com".to_string()));
        assert_eq!(person.icon, Some("base64icon".to_string()));
        assert_eq!(person.job, Some("Engineer".to_string()));
        assert_eq!(person.department, Some("IT".to_string()));
        assert_eq!(person.unit, Some("Tech Unit".to_string()));
        assert_eq!(person.position, Some("Senior".to_string()));
    }

    #[test]
    fn test_login_response_full_fields() {
        let person = crate::PersonInfo {
            id: "person-001".to_string(),
            unique: "user001".to_string(),
            name: "Test User".to_string(),
            mobile: Some("13800138000".to_string()),
            email: None,
            icon: None,
            job: None,
            department: None,
            unit: None,
            position: None,
        };
        let response = crate::LoginResponse {
            token: "test-token".to_string(),
            token_type: "Bearer".to_string(),
            role_list: vec!["admin".to_string(), "user".to_string()],
            password_expired: false,
            identity_list: vec!["identity1".to_string()],
            person,
        };
        assert_eq!(response.token, "test-token");
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.role_list, vec!["admin".to_string(), "user".to_string()]);
        assert_eq!(response.password_expired, false);
        assert_eq!(response.identity_list, vec!["identity1".to_string()]);
        assert_eq!(response.person.unique, "user001");
    }

    #[test]
    fn test_login_response_password_expired() {
        let person = crate::PersonInfo {
            id: "person-002".to_string(),
            unique: "user002".to_string(),
            name: "Expired User".to_string(),
            mobile: None,
            email: None,
            icon: None,
            job: None,
            department: None,
            unit: None,
            position: None,
        };
        let response = crate::LoginResponse {
            token: "expired-token".to_string(),
            token_type: "Bearer".to_string(),
            role_list: vec![],
            password_expired: true,
            identity_list: vec![],
            person,
        };
        assert_eq!(response.password_expired, true);
        assert!(response.role_list.is_empty());
        assert!(response.identity_list.is_empty());
    }

    #[test]
    fn test_check_token_request() {
        let req = crate::check_token::CheckTokenRequest {
            token: "test-token".to_string(),
        };
        assert_eq!(req.token, "test-token");
    }

    #[test]
    fn test_switch_user_request() {
        let req = crate::switch_user::SwitchUserRequest {
            credential: "target-user".to_string(),
        };
        assert_eq!(req.credential, "target-user");
    }

    #[test]
    fn test_two_factor_login_request() {
        let req = crate::two_factor::TwoFactorLoginRequest {
            credential: "user".to_string(),
            password: "pass".to_string(),
        };
        assert_eq!(req.credential, "user");
        assert_eq!(req.password, "pass");
    }

    #[test]
    fn test_two_factor_phase1_response() {
        let response = crate::two_factor::TwoFactorPhase1Response {
            value: true,
            password_expired: false,
            temp_token: "test-temp-token".to_string(),
        };
        assert_eq!(response.value, true);
        assert_eq!(response.password_expired, false);
        assert_eq!(response.temp_token, "test-temp-token");
    }

    #[test]
    fn test_temp_token_store_issue_and_verify() {
        let store = crate::TempTokenStore::new();
        let token = store.issue("user1");
        assert!(!token.is_empty());
        let credential = store.verify(&token);
        assert_eq!(credential, Some("user1".to_string()));
        assert!(store.verify(&token).is_none());
    }

    #[test]
    fn test_temp_token_store_expired() {
        let store = crate::TempTokenStore::new();
        let token = store.issue("user1");
        {
            let mut entries = store.entries.lock().unwrap();
            if let Some(entry) = entries.get_mut(&token) {
                use chrono::{Duration, Utc};
                entry.expires_at = Utc::now() - Duration::minutes(1);
            }
        }
        assert!(store.verify(&token).is_none());
    }

    #[test]
    fn test_temp_token_store_credential_binding() {
        let store = crate::TempTokenStore::new();
        let token = store.issue("victim");
        let credential = store.verify(&token);
        assert_eq!(credential, Some("victim".to_string()));
        assert_ne!(credential, Some("attacker".to_string()));
    }

    #[test]
    fn test_action_result_with_login_response() {
        let person = crate::PersonInfo {
            id: "p1".to_string(),
            unique: "u1".to_string(),
            name: "n".to_string(),
            mobile: None,
            email: None,
            icon: None,
            job: None,
            department: None,
            unit: None,
            position: None,
        };
        let response = crate::LoginResponse {
            token: "t".to_string(),
            token_type: "Bearer".to_string(),
            role_list: vec!["admin".to_string()],
            password_expired: false,
            identity_list: vec![],
            person,
        };
        let result: ActionResult<crate::LoginResponse> = ActionResult::success(response);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data.token, "t");
        assert_eq!(data.token_type, "Bearer");
    }

    // --- U3 新增测试用例：安全注销广播 ---

    #[test]
    fn test_broadcast_logout_single_instance() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            manager.create_session("user1".to_string(), "token1".to_string()).await.unwrap();
            manager.create_session("user1".to_string(), "token2".to_string()).await.unwrap();

            // 单实例模式（无 pool）：broadcast_logout 应正常返回，不报错
            manager.broadcast_logout("user1").await;

            // session 仍然存在（单实例下 broadcast 不做本地清理，由 remove_sessions_by_person 负责）
            assert!(manager.validate_session("token1").await.is_some());
            assert!(manager.validate_session("token2").await.is_some());
        });
    }

    #[test]
    fn test_remove_sessions_by_person_then_broadcast() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            manager.create_session("user1".to_string(), "token1".to_string()).await.unwrap();
            manager.create_session("user1".to_string(), "token2".to_string()).await.unwrap();
            manager.create_session("user2".to_string(), "token3".to_string()).await.unwrap();

            // 先批量移除
            manager.remove_sessions_by_person("user1").await;

            // 再广播（单实例模式，无实际操作）
            manager.broadcast_logout("user1").await;

            // user1 的 session 应全部失效
            assert!(manager.validate_session("token1").await.is_none());
            assert!(manager.validate_session("token2").await.is_none());
            // user2 的 session 不受影响
            assert!(manager.validate_session("token3").await.is_some());
        });
    }

    #[test]
    fn test_check_token_threshold_no_threshold() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            // 无 pool 时，check_token_threshold 应返回 false（允许创建）
            let now = chrono::Utc::now().naive_utc();
            let result = manager.check_token_threshold(now, "user1").await;
            assert!(!result);
        });
    }

    #[test]
    fn test_broadcast_logout_nonexistent_user() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = SessionManager::new();
            // 对不存在的用户调用 broadcast_logout，应正常返回不报错
            manager.broadcast_logout("nonexistent").await;
        });
    }

    #[tokio::test]
    async fn test_unit_list_db_connected() {
        let pool = shared::testing::test_pool();
        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/unit/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // seed data includes 2 units (unit-root, unit-dept1)
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_role_list_db_connected() {
        let pool = shared::testing::test_pool();
        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/role/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // seed data includes 2 roles (role-admin, role-user)
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_code_send_db_connected() {
        let pool = shared::testing::test_pool();
        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/code/credential/admin")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // seed data includes person with unique_id='admin'
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_end_to_end_db_connected() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_login_end_to_end: DATABASE_URL not reachable");
            return;
        }

        let _guard = EnvGuard::new().set("LDAP_ENABLE", "false");

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();

        // Seed a test user with a known bcrypt password hash
        if let Some(c) = &client {
            let _ = c
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, deleted_at) \
                     VALUES ($1, $2, $3, $4, false, NULL) \
                     ON CONFLICT (unique_id) DO UPDATE SET password_hash = EXCLUDED.password_hash",
                    &[
                        &"person-it-login",
                        &"it-login",
                        &"IT Login User",
                        &format!("{}{}", crate::password::BCRYPT_PREFIX, bcrypt::hash("testpass123", bcrypt::DEFAULT_COST).unwrap().as_str()),
                    ],
                )
                .await;

            // 清理 auth_token_threshold 中可能残留的测试数据，避免阈值拦截登录
            let _ = c
                .execute(
                    "DELETE FROM auth_token_threshold WHERE person_unique = $1",
                    &[&"it-login"],
                )
                .await;
        }

        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let body = serde_json::json!({
            "credential": "it-login",
            "password": "testpass123"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/login")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"]["token"].as_str().is_some());
        assert!(!json["data"]["token"].as_str().unwrap().is_empty());
    }

    // --- U6 新增测试用例：LDAP + two_factor 安全验证 ---

    #[tokio::test]
    async fn test_login_ldap_connection_error_returns_500() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_login_ldap_connection_error: DATABASE_URL not reachable");
            return;
        }

        let _guard = EnvGuard::new()
            .set("LDAP_ENABLE", "true")
            .set("LDAP_URL", "ldap://192.0.2.1:389")
            .set("LDAP_BASE_DN", "dc=example,dc=com")
            .set("LDAP_BIND_USER", "")
            .set("LDAP_BIND_PWD", "");

        assert!(
            ldap::LdapConfig::from_env().is_some(),
            "LDAP should be enabled in test"
        );

        let direct = crate::ldap_auth::try_ldap_authenticate("ldap-err-user", "testpass123").await;
        eprintln!("direct ldap_auth result: {:?}", direct);

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();
        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);
        if let Some(c) = &client {
            let _ = c
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, deleted_at) \
                     VALUES ($1, $2, $3, $4, false, NULL) \
                     ON CONFLICT (unique_id) DO UPDATE SET password_hash = EXCLUDED.password_hash",
                    &[
                        &"person-ldap-err",
                        &"ldap-err-user",
                        &"LDAP Error User",
                        &format!(
                            "{}{}",
                            crate::password::BCRYPT_PREFIX,
                            bcrypt::hash("testpass123", bcrypt::DEFAULT_COST)
                                .unwrap()
                                .as_str()
                        ),
                    ],
                )
                .await;
        }

        let body = serde_json::json!({
            "credential": "ldap-err-user",
            "password": "testpass123"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/login")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_two_factor_login_phase1_db_connected() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!(
                "skipping test_two_factor_login_phase1: DATABASE_URL not reachable"
            );
            return;
        }

        let _guard = EnvGuard::new().set("LDAP_ENABLE", "false");

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();

        if let Some(c) = &client {
            let _ = c
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, deleted_at) \
                     VALUES ($1, $2, $3, $4, false, NULL) \
                     ON CONFLICT (unique_id) DO UPDATE SET password_hash = EXCLUDED.password_hash",
                    &[
                        &"person-2fa-phase1",
                        &"2fa-phase1-user",
                        &"2FA Phase1 User",
                        &format!(
                            "{}{}",
                            crate::password::BCRYPT_PREFIX,
                            bcrypt::hash("testpass123", bcrypt::DEFAULT_COST)
                                .unwrap()
                                .as_str()
                        ),
                    ],
                )
                .await;
        }

        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let body = serde_json::json!({
            "credential": "2fa-phase1-user",
            "password": "testpass123"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/two_factor")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["value"], true);
        assert!(!json["data"]["temp_token"].as_str().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_two_factor_full_flow_db_connected() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_two_factor_full_flow: DATABASE_URL not reachable");
            return;
        }

        let _guard = EnvGuard::new().set("LDAP_ENABLE", "false");

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();

        if let Some(c) = &client {
            let _ = c
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash, locked, deleted_at) \
                     VALUES ($1, $2, $3, $4, false, NULL) \
                     ON CONFLICT (unique_id) DO UPDATE SET password_hash = EXCLUDED.password_hash",
                    &[
                        &"person-2fa-full",
                        &"2fa-full-user",
                        &"2FA Full User",
                        &format!(
                            "{}{}",
                            crate::password::BCRYPT_PREFIX,
                            bcrypt::hash("testpass123", bcrypt::DEFAULT_COST)
                                .unwrap()
                                .as_str()
                        ),
                    ],
                )
                .await;
        }

        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let body = serde_json::json!({
            "credential": "2fa-full-user",
            "password": "testpass123"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/two_factor")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        let temp_token = json["data"]["temp_token"].as_str().unwrap().to_string();

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/code/credential/2fa-full-user")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let known_code = crate::code_store().issue("2fa-full-user");

        let body = serde_json::json!({
            "credential": "2fa-full-user",
            "codeAnswer": known_code,
            "tempToken": temp_token
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/code")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "success");
        assert!(!json["data"]["token"].as_str().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_two_factor_invalid_temp_token_returns_error() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!(
                "skipping test_two_factor_invalid_temp_token: DATABASE_URL not reachable"
            );
            return;
        }

        let pool = shared::testing::test_pool();
        let rate_limiter = RateLimiter::new();
        let session_manager = SessionManager::with_pool(pool.clone());
        let app = crate::router(pool, rate_limiter, session_manager);

        let body = serde_json::json!({
            "credential": "2fa-invalid-temp-user",
            "codeAnswer": "123456",
            "tempToken": "invalid-temp-token"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/authentication/code")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "error");
    }
}