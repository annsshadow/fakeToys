#[cfg(test)]
mod tests {
    use crate::reset::ResetCodeStore;
    use crate::EditPersonRequest;
    use shared::response::ActionResult;

    #[test]
    fn test_reset_code_store() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = ResetCodeStore::new();

            store.store("user1".to_string(), "code123".to_string()).await;
            assert!(store.verify("user1", "code123").await);
            assert!(!store.verify("user1", "code456").await);
            assert!(!store.verify("user2", "code123").await);

            store.remove("user1").await;
            assert!(!store.verify("user1", "code123").await);
        });
    }

    #[test]
    fn test_reset_code_store_multiple() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = ResetCodeStore::new();

            store.store("user1".to_string(), "code1".to_string()).await;
            store.store("user2".to_string(), "code2".to_string()).await;

            assert!(store.verify("user1", "code1").await);
            assert!(store.verify("user2", "code2").await);
            assert!(!store.verify("user1", "code2").await);
            assert!(!store.verify("user2", "code1").await);
        });
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
