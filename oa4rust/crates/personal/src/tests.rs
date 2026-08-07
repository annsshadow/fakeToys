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
        assert!(crate::reset::is_password_acceptable("123456"));
        let long = "x".repeat(64);
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
}