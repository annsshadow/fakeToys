#[cfg(test)]
mod tests {
    use crate::password::verify_password;
    use crate::RateLimiter;
    use crate::SessionManager;
    use base64::Engine;
    use shared::response::ActionResult;

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
            let session = manager.create_session("user1".to_string(), "token1".to_string()).await;
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
            manager.create_session("user1".to_string(), "token1".to_string()).await;
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
            manager.create_session("user1".to_string(), "token1".to_string()).await;
            manager.create_session("user2".to_string(), "token2".to_string()).await;

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
            manager.create_session("user1".to_string(), "token1".to_string()).await;
            manager.create_session("user1".to_string(), "token2".to_string()).await;
            // 为 user2 创建 session
            manager.create_session("user2".to_string(), "token3".to_string()).await;

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

    // Helper function for DES test
    fn des_encrypt_for_test(plain: &str, key: &str) -> Vec<u8> {
        crate::password::des_encrypt(plain, key).unwrap()
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
        let result = crate::password::des3_decrypt_ede2(&encrypted, "wrongkey12345678");
        assert!(result.is_err());
    }

    #[test]
    fn test_des3_encrypt_short_key() {
        let result = crate::password::des3_encrypt_ede2("test", "short");
        assert!(result.is_err());
    }
}
