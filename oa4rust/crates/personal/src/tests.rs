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