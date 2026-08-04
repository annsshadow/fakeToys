use axum::{Json, Router, routing::get};
use serde_json::Value;

use shared::{error::AppError, response::ActionResult};

pub async fn get_portal(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String("Portal".to_string())),
        ("description".to_string(), Value::String("Portal description".to_string())),
    ])))))
}

pub async fn list_portal() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("portal-1".to_string())),
            ("name".to_string(), Value::String("Portal 1".to_string())),
        ]))
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_portal_category() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("cat-1".to_string())),
            ("name".to_string(), Value::String("Category 1".to_string())),
            ("count".to_string(), Value::Number(serde_json::Number::from(0_i64))),
        ]))
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub fn portal_router() -> Router {
    Router::new()
        .route("/jaxrs/portal/{id}", get(get_portal))
        .route("/jaxrs/portal/list", get(list_portal))
        .route("/jaxrs/portalcategory/list", get(list_portal_category))
}
