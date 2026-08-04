use crate::{exists_check, get_by_id, list_by_application_and_info_id};
use axum::extract::{Extension, Path};
use shared::response::ActionResult;

fn mock_pool() -> deadpool_postgres::Pool {
    let mgr = deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    deadpool_postgres::Pool::builder(mgr).max_size(1).build().unwrap()
}

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(serde_json::json!({"count": 2, "data": []}));

    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_exists_check_returns_success() {
    let result = exists_check().await.unwrap();
    let action: ActionResult<serde_json::Value> = result.0;
    assert_eq!(action.r#type, Some("success".to_string()));
    assert!(action.data.is_some());
    let data = action.data.unwrap();
    assert!(data.get("allExists").is_some());
}

#[tokio::test]
async fn test_get_by_id_existing() {
    let result = get_by_id(Extension(mock_pool()), Path("hotpic-001".to_string()))
        .await
        .unwrap();
    let action: ActionResult<serde_json::Value> = result.0;
    assert_eq!(action.r#type, Some("success".to_string()));
    let data = action.data.unwrap();
    assert_eq!(data.get("id").and_then(|v| v.as_str()), Some("hotpic-001"));
}

#[tokio::test]
async fn test_get_by_id_empty_returns_error() {
    let result = get_by_id(Extension(mock_pool()), Path("".to_string())).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_by_application_and_info_id_returns_success() {
    let result =
        list_by_application_and_info_id(Extension(mock_pool()), Path(("CMS".to_string(), "doc-123".to_string())))
            .await
            .unwrap();
    let action: ActionResult<serde_json::Value> = result.0;
    assert_eq!(action.r#type, Some("success".to_string()));
    let data = action.data.unwrap();
    assert!(data.get("count").is_some());
    assert!(data.get("data").is_some());
}
