#[cfg(test)]
mod tests {
    use crate::password::verify_password;
    use crate::RateLimiter;
    use crate::SessionManager;
    use shared::response::ActionResult;

    #[test]
    fn test_verify_password_md5() {
        let password = "admin123";
        let md5_hash = format!("{:x}", md5::compute(password.as_bytes()));
        assert!(verify_password(password, &md5_hash, "", None));
    }

    #[test]
    fn test_verify_password_wrong() {
        let password = "admin123";
        let wrong_hash = "wrong_hash";
        assert!(!verify_password(password, wrong_hash, "", None));
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
}
