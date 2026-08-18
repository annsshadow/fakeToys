use shared::response::ActionResult;

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
fn test_list_ids_handler_mock() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "count": 0,
            "data": []
        }));
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data["count"], 0);
    });
}

#[test]
fn test_get_by_flag_handler_mock() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let flag = "test_flag";
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "flag": flag,
            "name": "mock_process",
            "description": "mock_description"
        }));
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data["flag"], flag);
    });
}
